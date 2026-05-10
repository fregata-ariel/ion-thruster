# Physics Model Reference

ion-craft CFDフレームワークの物理モデルリファレンス。EHD（電気流体力学）シミュレーションで使用する支配方程式、離散化手法、境界条件、および連成戦略を記述する。


## Governing Equations

EHDシミュレーションは4つの連成方程式系で構成される:

1. **Poisson方程式** — 空間電荷密度から電位分布を決定
2. **ドリフト拡散方程式** — 電場によるドリフトと拡散によるイオン輸送
3. **EHD体積力** — 空間電荷と電場の積
4. **Navier-Stokes方程式** — EHD体積力を外力とする非圧縮性流れ

物理的な因果連鎖は以下の通り:

$$
\text{コロナ放電} \xrightarrow{\text{イオン生成}} n_i
\xrightarrow{q \cdot n_i} \rho_q
\xrightarrow{\nabla \cdot (\varepsilon \nabla \phi) = -\rho_q} \phi
\xrightarrow{-\nabla \phi} \mathbf{E}
\xrightarrow{\rho_q \mathbf{E}} \mathbf{f}_{\text{EHD}}
\xrightarrow{\text{NS方程式}} \mathbf{u}
$$

イオン風は、コロナ放電で生成されたイオンが中性分子に運動量を渡して生じる巨視的な気流である。


## Poisson Equation

電位 $\phi$ はPoisson方程式で決定される:

$$
\nabla \cdot (\varepsilon_0 \nabla \phi) = -\rho_q
$$

ここで $\varepsilon_0$ = 8.8541878128×10⁻¹² F/m（真空の誘電率）、空間電荷密度 $\rho_q = q \cdot n_i$（$q$ = 1.602176634×10⁻¹⁹ C、電気素量）。

有限体積法による離散化では、セル $P$ に対して:

$$
\sum_f \varepsilon_0 A_f \frac{\phi_N - \phi_P}{|\mathbf{d}|} = -\rho_{q,P} V_P
$$

ここで $A_f$ は面積、$|\mathbf{d}|$ はowner-neighbor重心間距離、$V_P$ はセル体積。この離散化は`cfd-fvm`の`laplacian_kernel()`で記述される:

```rust
let kernel = laplacian_kernel("phi", epsilon_0, "poisson_rhs");
```

離散化した結果は対称正定値の疎行列系 $A\phi = b$ となり、Jacobi前処理付き共役勾配法で求解する。

電場は電位の負の勾配として計算する:

$$
\mathbf{E} = -\nabla \phi
$$

離散的にはセル中心でGreen-Gaussの定理を適用し、面値の重み付き和から勾配を近似する。


## Ion Transport

イオン密度 $n_i$ の時間発展はドリフト拡散方程式で記述される:

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot (\mu_i n_i \mathbf{E} - D_i \nabla n_i) = 0
$$

ここで $\mu_i$ = 2.0×10⁻⁴ m²/(V·s)（イオン移動度）、$D_i$ = 5.0×10⁻⁶ m²/s（イオン拡散係数）。

第1項 $\mu_i n_i \mathbf{E}$ はドリフト（電場による輸送）、第2項 $D_i \nabla n_i$ は拡散を表す。この2つの効果が競合するため、通常の中心差分や風上差分では数値振動や過度な数値拡散が生じる。Scharfetter-Gummelスキームがこの問題を解決する。


## Scharfetter-Gummel Scheme

Scharfetter-Gummelスキームは、面のローカルPeclet数に基づいて指数的な重み付けを行い、ドリフトと拡散の競合を安定に離散化する。

面 $f$（owner $O$、neighbor $N$）における面Peclet数:

$$
\text{Pe}_f = \frac{\mu_i \mathbf{E}_f \cdot \mathbf{d}}{D_i}
$$

ここで $\mathbf{d}$ はowner→neighbor重心間ベクトル。面フラックスはBernoulli関数 $B(x) = x/(e^x - 1)$ を用いて:

$$
J_f = \frac{D_i A_f}{|\mathbf{d}|} \left[ B(\text{Pe}_f) \, n_O - B(-\text{Pe}_f) \, n_N \right]
$$

Bernoulli関数の数値的安定性は3領域で確保する:

| $|x|$ の範囲 | 近似 | 実装 |
|---|---|---|
| $|x| < 10^{-6}$ | Taylor展開: $B(x) \approx 1 - x/2 + x^2/12$ | 桁落ち回避 |
| $x > 500$ | $B(x) \approx 0$ | オーバーフロー回避 |
| $x < -500$ | $B(x) \approx -x$ | オーバーフロー回避 |
| その他 | $B(x) = x/(e^x - 1)$ | 直接計算 |

```rust
fn bernoulli(x: f64) -> f64 {
    if x.abs() < 1e-6 {
        1.0 - x * 0.5 + x * x / 12.0    // Taylor
    } else if x > 500.0 {
        0.0                               // exp(x) >> x
    } else if x < -500.0 {
        -x                                // B(x) ≈ -x for x << 0
    } else {
        x / (x.exp() - 1.0)              // exact
    }
}
```

カーネルは`cfd-fvm`で構築される:

```rust
let kernel = scharfetter_gummel_kernel(
    "ion_density",      // concentration field
    "electric_field",   // E field
    2.0e-4,             // ion mobility μ_i
    5.0e-6,             // diffusion D_i
    "ion_rhs",          // target accumulator
);
```


## EHD Body Force

EHD体積力は空間電荷密度と電場の積:

$$
\mathbf{f}_{\text{EHD}} = \rho_q \mathbf{E}
$$

ここで $\rho_q = q \cdot n_i$。この体積力がNavier-Stokes方程式の外力項としてイオン風を駆動する。

離散的にはセルごとの要素積として計算する。`CellOp::Multiply`で $\rho_q$ と $\mathbf{E}$ の各成分を掛け合わせる。


## Incompressible Navier-Stokes

非圧縮性Navier-Stokes方程式にEHD体積力を外力として加える:

$$
\rho_g \frac{\partial \mathbf{u}}{\partial t} + \rho_g (\mathbf{u} \cdot \nabla)\mathbf{u} = -\nabla p + \mu_g \nabla^2 \mathbf{u} + \mathbf{f}_{\text{EHD}}
$$

$$
\nabla \cdot \mathbf{u} = 0
$$

ここで $\rho_g$ = 1.225 kg/m³（気体密度）、$\mu_g$ = 1.81×10⁻⁵ Pa·s（気体動粘度）（標準大気条件）。

圧力投影法で解く:

1. **予測ステップ**: 圧力項を除いた運動量方程式で仮速度 $\mathbf{u}^*$ を計算
2. **圧力Poisson方程式**: $\nabla^2 p = \frac{\rho_g}{\Delta t} \nabla \cdot \mathbf{u}^*$ を解く
3. **補正ステップ**: $\mathbf{u}^{n+1} = \mathbf{u}^* - \frac{\Delta t}{\rho_g} \nabla p$ で発散フリーに補正

移流項の離散化には風上差分（`AdvectionScheme::Upwind`）を使用する。将来的にTVDリミッター（Van Leer, MinMod, Superbee）を追加予定。

粘性項はラプラシアンカーネルで離散化:

```rust
let viscous = laplacian_kernel("velocity_x", mu_g, "momentum_rhs_x");
let advect  = advection_kernel("velocity_x", "velocity", AdvectionScheme::Upwind, "momentum_rhs_x");
```


## Boundary Conditions

境界条件は`BoundaryCondition` enumで定義され、TOMLファイルで境界パッチごとに指定する。EHDシミュレーションの典型的な3パッチ構成:

### Emitter（放電電極）

| 変数 | 条件 | 値 |
|---|---|---|
| $\phi$ | Dirichlet | 20,000 V |
| $n_i$ | Dirichlet | $1.0 \times 10^{15}$ m⁻³ |
| $\mathbf{u}$ | No-slip | $\mathbf{0}$ |

### Collector（集電電極）

| 変数 | 条件 | 値 |
|---|---|---|
| $\phi$ | Dirichlet | 0 V |
| $n_i$ | Absorbing | — |
| $\mathbf{u}$ | No-slip | $\mathbf{0}$ |

### Farfield（遠方境界）

| 変数 | 条件 | 値 |
|---|---|---|
| $\phi$ | Neumann | 0（零勾配） |
| $n_i$ | Outflow | — |
| $\mathbf{u}$ | Outflow | — |

TOML設定例:

```toml
[boundary.emitter]
phi = { type = "dirichlet", value = 20000.0 }
ion_density = { type = "dirichlet", value = 1.0e15 }
velocity = { type = "no_slip" }

[boundary.collector]
phi = { type = "dirichlet", value = 0.0 }
ion_density = { type = "absorbing" }
velocity = { type = "no_slip" }
```

`Absorbing`境界条件はイオンが電極に到達して消滅する物理を表す。`Outflow`は対流による自然流出を許容する。


## Operator Splitting Coupling

連成方程式系はオペレータ分割法で解く。各タイムステップで以下の4つのサブステップを逐次実行する:

```text
1. PoissonStep    : ρ_q → φ, E を計算（楕円型、CG求解）
2. IonTransportStep: E, u を使って n_i を更新（SGスキーム、陽的時間積分）
3. EhdForceStep   : ρ_q × E → f_EHD を計算（代数的、dtに依存しない）
4. FluidStep      : f_EHD を体積力として u, p を更新（圧力投影法）
```

この順序が重要な理由:

- PoissonStepは楕円型であり、$\rho_q$ から $\phi, \mathbf{E}$ を即座に決定する。先に解くことで、後続ステップが最新の電場を使える。
- IonTransportStepはドリフト拡散の時間発展であり、最新の $\mathbf{E}$ と $\mathbf{u}$ を必要とする。
- EhdForceStepは代数的な計算のみ（$\mathbf{f} = \rho_q \mathbf{E}$）であり、安定性制約がない。
- FluidStepは最後に実行し、更新された体積力で速度場を補正する。

時間刻み $\Delta t$ は全ステップの`max_dt()`の最小値にCFL係数を乗じて決定する:

$$
\Delta t = C_{\text{CFL}} \cdot \min_s \Delta t_s^{\max}
$$

楕円型ステップ（Poisson）は`f64::INFINITY`を返す（無条件安定）。IonTransportStepのCFL制約がドリフト速度に基づく最も厳しい制約となる。


## Physical Constants and Parameters

EHDシミュレーションで使用する物理定数とパラメータ:

### 物理定数

| 記号 | 名称 | 値 | 単位 |
|---|---|---|---|
| $\varepsilon_0$ | 真空の誘電率 | 8.8541878128×10⁻¹² | F/m |
| $q$ | 電気素量 | 1.602176634×10⁻¹⁹ | C |

### 気体パラメータ（標準大気条件）

| 記号 | 名称 | 典型値 | 単位 |
|---|---|---|---|
| $\rho_g$ | 気体密度 | 1.225 | kg/m³ |
| $\mu_g$ | 気体動粘度 | 1.81×10⁻⁵ | Pa·s |

### イオン輸送パラメータ

| 記号 | 名称 | 典型値 | 単位 |
|---|---|---|---|
| $\mu_i$ | イオン移動度 | 2.0×10⁻⁴ | m²/(V·s) |
| $D_i$ | イオン拡散係数 | 5.0×10⁻⁶ | m²/s |

これらのパラメータは`EhdConfig`構造体で保持され、TOML設定ファイルの`[physics]`セクションから読み込まれる:

```toml
[physics]
gas = "air"
rho_g = 1.225
mu_g = 1.81e-5
epsilon = 8.8541878128e-12
ion_mobility = 2.0e-4
ion_diffusion = 5.0e-6
```

