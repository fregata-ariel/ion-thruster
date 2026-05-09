# Physics Model Reference

::: {lang=ja}
ion-craft CFDフレームワークの物理モデルリファレンス。EHD（電気流体力学）シミュレーションで使用する支配方程式、離散化手法、境界条件、および連成戦略を記述する。
:::

::: {lang=en}
Physics model reference for the ion-craft CFD framework. Describes the governing equations, discretization methods, boundary conditions, and coupling strategy used in EHD (electrohydrodynamic) simulation.
:::

## Governing Equations

::: {lang=ja}
EHDシミュレーションは4つの連成方程式系で構成される:

1. **[]{.term id=poisson_eq}** — 空間電荷密度から電位分布を決定
2. **[]{.term id=drift_diffusion}** — 電場によるドリフトと拡散によるイオン輸送
3. **EHD体積力** — 空間電荷と電場の積
4. **[]{.term id=navier_stokes}** — EHD体積力を外力とする非圧縮性流れ

物理的な因果連鎖は以下の通り:

$$
\text{[]{.term id=corona_discharge}} \xrightarrow{\text{イオン生成}} n_i
\xrightarrow{q \cdot n_i} \rho_q
\xrightarrow{\nabla \cdot (\varepsilon \nabla \phi) = -\rho_q} \phi
\xrightarrow{-\nabla \phi} \mathbf{E}
\xrightarrow{\rho_q \mathbf{E}} \mathbf{f}_{\text{EHD}}
\xrightarrow{\text{NS方程式}} \mathbf{u}
$$

[]{.term id=ionic_wind}は、コロナ放電で生成されたイオンが中性分子に運動量を渡して生じる巨視的な気流である。
:::

::: {lang=en}
The EHD simulation consists of four coupled equation systems:

1. **[]{.term id=poisson_eq}** — determines the electric potential distribution from space charge density
2. **[]{.term id=drift_diffusion}** — ion transport via electric field drift and diffusion
3. **EHD body force** — product of space charge density and electric field
4. **[]{.term id=navier_stokes}** — incompressible flow with EHD body force as an external force

The physical causal chain is:

$$
\text{[]{.term id=corona_discharge}} \xrightarrow{\text{ion generation}} n_i
\xrightarrow{q \cdot n_i} \rho_q
\xrightarrow{\nabla \cdot (\varepsilon \nabla \phi) = -\rho_q} \phi
\xrightarrow{-\nabla \phi} \mathbf{E}
\xrightarrow{\rho_q \mathbf{E}} \mathbf{f}_{\text{EHD}}
\xrightarrow{\text{NS eqs.}} \mathbf{u}
$$

[]{.term id=ionic_wind} is the macroscopic gas flow produced when corona-generated ions transfer momentum to neutral molecules.
:::

## Poisson Equation

::: {lang=ja}
[]{.term id=electric_potential} $\phi$ は[]{.term id=poisson_eq}で決定される:

$$
\nabla \cdot (\varepsilon_0 \nabla \phi) = -\rho_q
$$

ここで $\varepsilon_0$ = 8.8541878128×10⁻¹² F/m（[]{.term id=permittivity}）、[]{.term id=charge_density} $\rho_q = q \cdot n_i$（$q$ = 1.602176634×10⁻¹⁹ C、[]{.term id=elementary_charge}）。

[]{.term id=fvm}による離散化では、セル $P$ に対して:

$$
\sum_f \varepsilon_0 A_f \frac{\phi_N - \phi_P}{|\mathbf{d}|} = -\rho_{q,P} V_P
$$

ここで $A_f$ は面積、$|\mathbf{d}|$ はowner-neighbor重心間距離、$V_P$ はセル体積。この離散化は`cfd-fvm`の`laplacian_kernel()`で記述される:

```rust
let kernel = laplacian_kernel("phi", epsilon_0, "poisson_rhs");
```

離散化した結果は対称正定値の疎行列系 $A\phi = b$ となり、Jacobi前処理付き[]{.term id=cg_solver}で求解する。

[]{.term id=electric_field}は電位の負の勾配として計算する:

$$
\mathbf{E} = -\nabla \phi
$$

離散的にはセル中心でGreen-Gaussの定理を適用し、面値の重み付き和から勾配を近似する。
:::

::: {lang=en}
The []{.term id=electric_potential} $\phi$ is determined by the []{.term id=poisson_eq}:

$$
\nabla \cdot (\varepsilon_0 \nabla \phi) = -\rho_q
$$

where $\varepsilon_0$ = 8.8541878128×10⁻¹² F/m ([]{.term id=permittivity}), and []{.term id=charge_density} $\rho_q = q \cdot n_i$ ($q$ = 1.602176634×10⁻¹⁹ C, []{.term id=elementary_charge}).

In the []{.term id=fvm} discretization, for cell $P$:

$$
\sum_f \varepsilon_0 A_f \frac{\phi_N - \phi_P}{|\mathbf{d}|} = -\rho_{q,P} V_P
$$

where $A_f$ is the face area, $|\mathbf{d}|$ is the owner-neighbor centroid distance, and $V_P$ is the cell volume. This discretization is described by `laplacian_kernel()` in `cfd-fvm`:

```rust
let kernel = laplacian_kernel("phi", epsilon_0, "poisson_rhs");
```

The discretized system yields a symmetric positive-definite sparse matrix system $A\phi = b$, solved by the Jacobi-preconditioned []{.term id=cg_solver}.

The []{.term id=electric_field} is computed as the negative gradient of the potential:

$$
\mathbf{E} = -\nabla \phi
$$

Discretely, the Green-Gauss theorem is applied at cell centers, approximating the gradient from weighted face-value sums.
:::

## Ion Transport

::: {lang=ja}
[]{.term id=ion_density} $n_i$ の時間発展は[]{.term id=drift_diffusion}で記述される:

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot (\mu_i n_i \mathbf{E} - D_i \nabla n_i) = 0
$$

ここで $\mu_i$ = 2.0×10⁻⁴ m²/(V·s)（[]{.term id=ion_mobility}）、$D_i$ = 5.0×10⁻⁶ m²/s（[]{.term id=ion_diffusion}）。

第1項 $\mu_i n_i \mathbf{E}$ はドリフト（電場による輸送）、第2項 $D_i \nabla n_i$ は拡散を表す。この2つの効果が競合するため、通常の中心差分や風上差分では数値振動や過度な数値拡散が生じる。[]{.term id=scharfetter_gummel}がこの問題を解決する。
:::

::: {lang=en}
The time evolution of []{.term id=ion_density} $n_i$ is described by the []{.term id=drift_diffusion}:

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot (\mu_i n_i \mathbf{E} - D_i \nabla n_i) = 0
$$

where $\mu_i$ = 2.0×10⁻⁴ m²/(V·s) ([]{.term id=ion_mobility}), and $D_i$ = 5.0×10⁻⁶ m²/s ([]{.term id=ion_diffusion}).

The first term $\mu_i n_i \mathbf{E}$ represents drift (transport by the electric field), and the second term $D_i \nabla n_i$ represents diffusion. Because these two effects compete, standard central or upwind differencing produces either numerical oscillation or excessive numerical diffusion. The []{.term id=scharfetter_gummel} resolves this problem.
:::

## Scharfetter-Gummel Scheme

::: {lang=ja}
[]{.term id=scharfetter_gummel}は、面のローカルPeclet数に基づいて指数的な重み付けを行い、ドリフトと拡散の競合を安定に離散化する。

面 $f$（owner $O$、neighbor $N$）における面Peclet数:

$$
\text{Pe}_f = \frac{\mu_i \mathbf{E}_f \cdot \mathbf{d}}{D_i}
$$

ここで $\mathbf{d}$ はowner→neighbor重心間ベクトル。面フラックスは[]{.term id=bernoulli_function} $B(x) = x/(e^x - 1)$ を用いて:

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
:::

::: {lang=en}
The []{.term id=scharfetter_gummel} applies exponential weighting based on the local face Peclet number, stably discretizing the competing drift and diffusion terms.

The face Peclet number at face $f$ (owner $O$, neighbor $N$):

$$
\text{Pe}_f = \frac{\mu_i \mathbf{E}_f \cdot \mathbf{d}}{D_i}
$$

where $\mathbf{d}$ is the owner→neighbor centroid vector. The face flux uses the []{.term id=bernoulli_function} $B(x) = x/(e^x - 1)$:

$$
J_f = \frac{D_i A_f}{|\mathbf{d}|} \left[ B(\text{Pe}_f) \, n_O - B(-\text{Pe}_f) \, n_N \right]
$$

Numerical stability of the Bernoulli function is ensured across three regions:

| $|x|$ range | Approximation | Purpose |
|---|---|---|
| $|x| < 10^{-6}$ | Taylor expansion: $B(x) \approx 1 - x/2 + x^2/12$ | Avoid cancellation |
| $x > 500$ | $B(x) \approx 0$ | Avoid overflow |
| $x < -500$ | $B(x) \approx -x$ | Avoid overflow |
| otherwise | $B(x) = x/(e^x - 1)$ | Direct computation |

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

The kernel is built in `cfd-fvm`:

```rust
let kernel = scharfetter_gummel_kernel(
    "ion_density",      // concentration field
    "electric_field",   // E field
    2.0e-4,             // ion mobility μ_i
    5.0e-6,             // diffusion D_i
    "ion_rhs",          // target accumulator
);
```
:::

## EHD Body Force

::: {lang=ja}
[]{.term id=ehd_body_force}は[]{.term id=charge_density}と[]{.term id=electric_field}の積:

$$
\mathbf{f}_{\text{EHD}} = \rho_q \mathbf{E}
$$

ここで $\rho_q = q \cdot n_i$。この体積力がNavier-Stokes方程式の外力項として[]{.term id=ionic_wind}を駆動する。

離散的にはセルごとの要素積として計算する。`CellOp::Multiply`で $\rho_q$ と $\mathbf{E}$ の各成分を掛け合わせる。
:::

::: {lang=en}
The []{.term id=ehd_body_force} is the product of []{.term id=charge_density} and []{.term id=electric_field}:

$$
\mathbf{f}_{\text{EHD}} = \rho_q \mathbf{E}
$$

where $\rho_q = q \cdot n_i$. This body force drives the []{.term id=ionic_wind} as an external force term in the Navier-Stokes equations.

Discretely, it is computed as a per-cell element-wise product. `CellOp::Multiply` multiplies $\rho_q$ with each component of $\mathbf{E}$.
:::

## Incompressible Navier-Stokes

::: {lang=ja}
非圧縮性[]{.term id=navier_stokes}に[]{.term id=ehd_body_force}を外力として加える:

$$
\rho_g \frac{\partial \mathbf{u}}{\partial t} + \rho_g (\mathbf{u} \cdot \nabla)\mathbf{u} = -\nabla p + \mu_g \nabla^2 \mathbf{u} + \mathbf{f}_{\text{EHD}}
$$

$$
\nabla \cdot \mathbf{u} = 0
$$

ここで $\rho_g$ = 1.225 kg/m³（[]{.term id=gas_density}）、$\mu_g$ = 1.81×10⁻⁵ Pa·s（[]{.term id=gas_viscosity}）（標準大気条件）。

[]{.term id=pressure_projection}で解く:

1. **予測ステップ**: 圧力項を除いた運動量方程式で仮速度 $\mathbf{u}^*$ を計算
2. **圧力Poisson方程式**: $\nabla^2 p = \frac{\rho_g}{\Delta t} \nabla \cdot \mathbf{u}^*$ を解く
3. **補正ステップ**: $\mathbf{u}^{n+1} = \mathbf{u}^* - \frac{\Delta t}{\rho_g} \nabla p$ で発散フリーに補正

移流項の離散化には風上差分（`AdvectionScheme::Upwind`）を使用する。将来的にTVDリミッター（Van Leer, MinMod, Superbee）を追加予定。

粘性項はラプラシアンカーネルで離散化:

```rust
let viscous = laplacian_kernel("velocity_x", mu_g, "momentum_rhs_x");
let advect  = advection_kernel("velocity_x", "velocity", AdvectionScheme::Upwind, "momentum_rhs_x");
```
:::

::: {lang=en}
The incompressible []{.term id=navier_stokes} with []{.term id=ehd_body_force} as an external force:

$$
\rho_g \frac{\partial \mathbf{u}}{\partial t} + \rho_g (\mathbf{u} \cdot \nabla)\mathbf{u} = -\nabla p + \mu_g \nabla^2 \mathbf{u} + \mathbf{f}_{\text{EHD}}
$$

$$
\nabla \cdot \mathbf{u} = 0
$$

where $\rho_g$ = 1.225 kg/m³ ([]{.term id=gas_density}), $\mu_g$ = 1.81×10⁻⁵ Pa·s ([]{.term id=gas_viscosity}) (standard atmospheric conditions).

Solved using the []{.term id=pressure_projection}:

1. **Predictor step**: compute tentative velocity $\mathbf{u}^*$ from the momentum equation without pressure
2. **Pressure Poisson equation**: solve $\nabla^2 p = \frac{\rho_g}{\Delta t} \nabla \cdot \mathbf{u}^*$
3. **Corrector step**: $\mathbf{u}^{n+1} = \mathbf{u}^* - \frac{\Delta t}{\rho_g} \nabla p$ to enforce divergence-free velocity

Advection discretization uses upwind differencing (`AdvectionScheme::Upwind`). TVD limiters (Van Leer, MinMod, Superbee) are planned for future addition.

The viscous term is discretized with the Laplacian kernel:

```rust
let viscous = laplacian_kernel("velocity_x", mu_g, "momentum_rhs_x");
let advect  = advection_kernel("velocity_x", "velocity", AdvectionScheme::Upwind, "momentum_rhs_x");
```
:::

## Boundary Conditions

::: {lang=ja}
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
:::

::: {lang=en}
Boundary conditions are defined by the `BoundaryCondition` enum and specified per boundary patch in the TOML file. Typical 3-patch configuration for EHD simulation:

### Emitter (discharge electrode)

| Variable | Condition | Value |
|---|---|---|
| $\phi$ | Dirichlet | 20,000 V |
| $n_i$ | Dirichlet | $1.0 \times 10^{15}$ m⁻³ |
| $\mathbf{u}$ | No-slip | $\mathbf{0}$ |

### Collector (collection electrode)

| Variable | Condition | Value |
|---|---|---|
| $\phi$ | Dirichlet | 0 V |
| $n_i$ | Absorbing | — |
| $\mathbf{u}$ | No-slip | $\mathbf{0}$ |

### Farfield (far-field boundary)

| Variable | Condition | Value |
|---|---|---|
| $\phi$ | Neumann | 0 (zero gradient) |
| $n_i$ | Outflow | — |
| $\mathbf{u}$ | Outflow | — |

TOML configuration example:

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

The `Absorbing` boundary condition represents the physics of ions reaching the electrode and being neutralized. `Outflow` permits natural convective outflow.
:::

## Operator Splitting Coupling

::: {lang=ja}
連成方程式系は[]{.term id=operator_splitting}で解く。各タイムステップで以下の4つのサブステップを逐次実行する:

```
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
:::

::: {lang=en}
The coupled equation system is solved using []{.term id=operator_splitting}. Four sub-steps are executed sequentially each timestep:

```
1. PoissonStep     : ρ_q → φ, E (elliptic, CG solve)
2. IonTransportStep: update n_i using E, u (SG scheme, explicit time integration)
3. EhdForceStep    : ρ_q × E → f_EHD (algebraic, dt-independent)
4. FluidStep       : update u, p using f_EHD (pressure projection)
```

The ordering matters:

- PoissonStep is elliptic and immediately determines $\phi, \mathbf{E}$ from $\rho_q$. Solving it first provides up-to-date electric field for subsequent steps.
- IonTransportStep performs drift-diffusion time evolution, requiring the latest $\mathbf{E}$ and $\mathbf{u}$.
- EhdForceStep is purely algebraic ($\mathbf{f} = \rho_q \mathbf{E}$) with no stability constraint.
- FluidStep runs last, correcting the velocity field with the updated body force.

The timestep $\Delta t$ is determined as the CFL coefficient times the minimum `max_dt()` across all steps:

$$
\Delta t = C_{\text{CFL}} \cdot \min_s \Delta t_s^{\max}
$$

Elliptic steps (Poisson) return `f64::INFINITY` (unconditionally stable). The IonTransportStep CFL constraint based on drift velocity is typically the most restrictive.
:::

## Physical Constants and Parameters

::: {lang=ja}
EHDシミュレーションで使用する物理定数とパラメータ:

### 物理定数

| 記号 | 名称 | 値 | 単位 |
|---|---|---|---|
| $\varepsilon_0$ | []{.term id=permittivity} | 8.8541878128×10⁻¹² | F/m |
| $q$ | []{.term id=elementary_charge} | 1.602176634×10⁻¹⁹ | C |

### 気体パラメータ（標準大気条件）

| 記号 | 名称 | 典型値 | 単位 |
|---|---|---|---|
| $\rho_g$ | []{.term id=gas_density} | 1.225 | kg/m³ |
| $\mu_g$ | []{.term id=gas_viscosity} | 1.81×10⁻⁵ | Pa·s |

### イオン輸送パラメータ

| 記号 | 名称 | 典型値 | 単位 |
|---|---|---|---|
| $\mu_i$ | []{.term id=ion_mobility} | 2.0×10⁻⁴ | m²/(V·s) |
| $D_i$ | []{.term id=ion_diffusion} | 5.0×10⁻⁶ | m²/s |

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
:::

::: {lang=en}
Physical constants and parameters used in EHD simulation:

### Physical Constants

| Symbol | Name | Value | Unit |
|---|---|---|---|
| $\varepsilon_0$ | []{.term id=permittivity} | 8.8541878128×10⁻¹² | F/m |
| $q$ | []{.term id=elementary_charge} | 1.602176634×10⁻¹⁹ | C |

### Gas Parameters (standard atmospheric conditions)

| Symbol | Name | Typical Value | Unit |
|---|---|---|---|
| $\rho_g$ | []{.term id=gas_density} | 1.225 | kg/m³ |
| $\mu_g$ | []{.term id=gas_viscosity} | 1.81×10⁻⁵ | Pa·s |

### Ion Transport Parameters

| Symbol | Name | Typical Value | Unit |
|---|---|---|---|
| $\mu_i$ | []{.term id=ion_mobility} | 2.0×10⁻⁴ | m²/(V·s) |
| $D_i$ | []{.term id=ion_diffusion} | 5.0×10⁻⁶ | m²/s |

These parameters are held in the `EhdConfig` struct and loaded from the `[physics]` section of the TOML configuration file:

```toml
[physics]
gas = "air"
rho_g = 1.225
mu_g = 1.81e-5
epsilon = 8.8541878128e-12
ion_mobility = 2.0e-4
ion_diffusion = 5.0e-6
```
:::
