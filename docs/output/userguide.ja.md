# User Guide

ion-craft CFDフレームワークのユーザーガイド。有限体積法に基づくイオン風シミュレーションについて、インストールからメッシュ生成、設定、実行、可視化までの手順を説明する。


## Prerequisites

以下のツールが必要:

| ツール | バージョン | 用途 |
|---|---|---|
| Rust (rustc + cargo) | 1.85+ (edition 2024) | フレームワークのビルド |
| Gmsh | 4.x | メッシュ生成 (`.geo` → `.msh`) |
| ParaView | 5.x | 結果の可視化 (`.vtu` / `.pvd`) |

Rustのインストール:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


## Installation

```bash
git clone <repository-url> ion-craft
cd ion-craft
cargo build --release
```

ビルド成功後、`target/release/ehd-sim` バイナリが生成される。

テストの実行:
```bash
cargo test --workspace
```


## Mesh Generation with Gmsh

Gmshの`.geo`ファイルでジオメトリを定義し、`.msh`ファイル（MSH 2.2 ASCII）を生成する。

### wire_plate_2d サンプル

`examples/wire_plate_2d/wire_plate.geo` は2Dワイヤー-プレート電極構成を定義する:

- **ドメイン**: 60mm × 40mm
- **ワイヤー（emitter）**: 半径1mm、ギャップ20mm位置
- **プレート（collector）**: 底辺
- **メッシュサイズ**: ワイヤー近傍 0.5mm、プレート近傍 1mm、遠方 5mm

物理グループの定義が重要。TOML設定の`[boundary.*]`セクション名と一致させる:

```text
Physical Curve("collector") = {1};
Physical Curve("emitter") = {5, 6, 7, 8};
Physical Curve("farfield") = {2, 3, 4};
Physical Surface("fluid") = {1};
```

メッシュ生成コマンド:
```bash
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 -o examples/wire_plate_2d/wire_plate.msh
```

`-format msh2` が必要（ion-craftはMSH 2.2 ASCIIのみ対応）。


## Configuration (TOML)

シミュレーション設定はTOMLファイルで定義する。`examples/wire_plate_2d/simulation.toml` を参考に:

### [mesh] セクション

```toml
[mesh]
path = "examples/wire_plate_2d/wire_plate.msh"
format = "gmsh"
```

### [physics] セクション

```toml
[physics]
gas = "air"
rho_g = 1.225          # 気体密度 (kg/m³)
mu_g = 1.81e-5         # 気体動粘度 (Pa·s)
epsilon = 8.8541878128e-12  # 誘電率 (F/m)
ion_mobility = 2.0e-4  # イオン移動度 (m²/V·s)
ion_diffusion = 5.0e-6 # イオン拡散係数 (m²/s)
```

### [boundary.*] セクション

各境界パッチに対して変数ごとの境界条件を指定する。パッチ名はGmshの物理グループ名と一致させる:

```toml
[boundary.emitter]
phi = { type = "dirichlet", value = 20000.0 }
ion_density = { type = "dirichlet", value = 1.0e15 }
velocity = { type = "no_slip" }

[boundary.collector]
phi = { type = "dirichlet", value = 0.0 }
ion_density = { type = "absorbing" }
velocity = { type = "no_slip" }

[boundary.farfield]
phi = { type = "neumann", value = 0.0 }
ion_density = { type = "outflow" }
velocity = { type = "outflow" }
```

対応する境界条件タイプ: `dirichlet`, `neumann`, `no_slip`, `outflow`, `absorbing`

### [fluid] セクション

```toml
[fluid]
model = "incompressible"
advection = "upwind"
pressure_solver = "cg"
dt = 1.0e-6        # 時間刻み (s)
steps = 100         # 最大ステップ数
cfl = 0.5           # CFL係数（省略可、適応dt時に使用）
max_time = 1.0e-4   # 最大シミュレーション時間（省略可、省略時は無制限）
```

`pressure_solver = "cg"` は共役勾配法を使用。時間積分はオペレータ分割法で連成方程式系を逐次的に解く。

### [output] セクション

```toml
[output]
format = "vtu"
every = 10          # 出力間隔（ステップ数）
path = "output/"
fields = ["phi", "electric_field", "ion_density", "charge_density",
          "velocity", "pressure", "ehd_force"]
```


## Running a Simulation

```bash
# メッシュ生成
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 \
     -o examples/wire_plate_2d/wire_plate.msh

# シミュレーション実行
cargo run --release --bin ehd-sim -- --config examples/wire_plate_2d/simulation.toml

# またはビルド済みバイナリを直接使用
./target/release/ehd-sim --config examples/wire_plate_2d/simulation.toml
```

### CLIオプション

```text
ehd-sim [OPTIONS]

Options:
  -c, --config <PATH>   設定ファイルパス [default: simulation.toml]
  -m, --mesh <PATH>     メッシュファイルパスの上書き
  -o, --output <PATH>   出力ディレクトリの上書き
  -h, --help            ヘルプ表示
```

### ログ制御

環境変数`RUST_LOG`でログレベルを制御:

```bash
RUST_LOG=debug cargo run --release --bin ehd-sim -- -c simulation.toml
RUST_LOG=info  cargo run --release --bin ehd-sim -- -c simulation.toml
```


## Visualization with ParaView

シミュレーション結果は`output/`ディレクトリにVTU形式で出力される。

### 手順

1. ParaViewで `output/output.pvd` を開く（File → Open）
2. 「Apply」をクリック
3. 表示するフィールドを選択（phi, velocity, ion_density 等）
4. 時系列アニメーションの再生ボタンで時間発展を確認

### 出力ファイル構造

```text
output/
├── output.pvd           # PVDコレクション（時系列インデックス）
├── frame_000000.vtu     # 初期状態
├── frame_000010.vtu     # ステップ10
├── frame_000020.vtu     # ステップ20
└── ...
```

PVDファイルはXML形式で各フレームのタイムスタンプとファイル名を記録。ParaViewが自動的に時系列として認識する。


## Adding a Physics Module

新しい物理モジュールを追加する手順。オペレータ分割法フレームワークにステップとして組み込む:

### 1. crateの作成

```bash
cargo new crates/thermal-physics --lib
```

`Cargo.toml`の`[workspace.members]`に追加。依存先として`cfd-core`, `cfd-fields`, `cfd-mesh`, `cfd-time`を追加。

### 2. モジュール構造体の実装

```rust
pub struct ThermalModule {
    pub config: ThermalConfig,
}

impl ThermalModule {
    pub fn new(config: ThermalConfig) -> Self { ... }

    pub fn register_fields(&self, registry: &mut FieldRegistry, mesh: &Mesh) {
        registry.register_scalar("temperature", mesh.n_cells, FieldLocation::Cell);
        registry.register_scalar("heat_flux", mesh.n_cells, FieldLocation::Cell);
    }

    pub fn initialize(&self, state: &mut SimState, mesh: &Mesh) { ... }

    pub fn splitting_steps(&self) -> Vec<Box<dyn SplittingStep>> {
        vec![Box::new(HeatDiffusionStep::new(self.config.clone()))]
    }
}
```

### 3. SplittingStepの実装

```rust
impl SplittingStep for HeatDiffusionStep {
    fn name(&self) -> &str { "heat_diffusion" }
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError> {
        // カーネル記述を構築し、バックエンドで実行
        let kernel = laplacian_kernel("temperature", self.conductivity, "heat_rhs");
        // ... 線形系の組立・求解
        Ok(())
    }
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64 { f64::INFINITY }
}
```

### 4. CLIへの統合

既存のEHDモジュールと同様に`main()`でwiring:

```rust
let thermal = ThermalModule::new(thermal_config);
thermal.register_fields(&mut state.fields, &mesh);
for step in thermal.splitting_steps() {
    splitting.add_step(step);
}
```

モジュール間の通信は`FieldRegistry`を介して行う。直接参照は不要。

