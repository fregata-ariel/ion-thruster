# User Guide

::: {lang=ja}
ion-craft CFDフレームワークのユーザーガイド。[]{.term id=fvm}に基づく[]{.term id=ionic_wind}シミュレーションについて、インストールからメッシュ生成、設定、実行、可視化までの手順を説明する。
:::

::: {lang=en}
User guide for the ion-craft CFD framework. Covers []{.term id=fvm}-based []{.term id=ionic_wind} simulation — from installation to mesh generation, configuration, execution, and visualization.
:::

## Prerequisites

::: {lang=ja}
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
:::

::: {lang=en}
The following tools are required:

| Tool | Version | Purpose |
|---|---|---|
| Rust (rustc + cargo) | 1.85+ (edition 2024) | Build the framework |
| Gmsh | 4.x | Mesh generation (`.geo` → `.msh`) |
| ParaView | 5.x | Result visualization (`.vtu` / `.pvd`) |

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
:::

## Installation

::: {lang=ja}
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
:::

::: {lang=en}
```bash
git clone <repository-url> ion-craft
cd ion-craft
cargo build --release
```

After a successful build, the `target/release/ehd-sim` binary is created.

Run tests:
```bash
cargo test --workspace
```
:::

## Mesh Generation with Gmsh

::: {lang=ja}
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
:::

::: {lang=en}
Define geometry in Gmsh `.geo` files and generate `.msh` files (MSH 2.2 ASCII).

### wire_plate_2d sample

`examples/wire_plate_2d/wire_plate.geo` defines a 2D wire-to-plate electrode configuration:

- **Domain**: 60mm × 40mm
- **Wire (emitter)**: radius 1mm, positioned at 20mm gap
- **Plate (collector)**: bottom edge
- **Mesh sizes**: 0.5mm near wire, 1mm near plate, 5mm far field

Physical group definitions are critical — they must match the `[boundary.*]` section names in the TOML config:

```text
Physical Curve("collector") = {1};
Physical Curve("emitter") = {5, 6, 7, 8};
Physical Curve("farfield") = {2, 3, 4};
Physical Surface("fluid") = {1};
```

Mesh generation command:
```bash
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 -o examples/wire_plate_2d/wire_plate.msh
```

`-format msh2` is required (ion-craft supports MSH 2.2 ASCII only).
:::

## Configuration (TOML)

::: {lang=ja}
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

`pressure_solver = "cg"` は[]{.term id=cg_solver}を使用。時間積分は[]{.term id=operator_splitting}で連成方程式系を逐次的に解く。

### [output] セクション

```toml
[output]
format = "vtu"
every = 10          # 出力間隔（ステップ数）
path = "output/"
fields = ["phi", "electric_field", "ion_density", "charge_density",
          "velocity", "pressure", "ehd_force"]
```
:::

::: {lang=en}
Simulation settings are defined in TOML files. Refer to `examples/wire_plate_2d/simulation.toml`:

### [mesh] section

```toml
[mesh]
path = "examples/wire_plate_2d/wire_plate.msh"
format = "gmsh"
```

### [physics] section

```toml
[physics]
gas = "air"
rho_g = 1.225          # gas density (kg/m³)
mu_g = 1.81e-5         # gas dynamic viscosity (Pa·s)
epsilon = 8.8541878128e-12  # permittivity (F/m)
ion_mobility = 2.0e-4  # ion mobility (m²/V·s)
ion_diffusion = 5.0e-6 # ion diffusion coefficient (m²/s)
```

### [boundary.*] section

Specify boundary conditions per variable for each boundary patch. Patch names must match Gmsh physical group names:

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

Supported boundary condition types: `dirichlet`, `neumann`, `no_slip`, `outflow`, `absorbing`

### [fluid] section

```toml
[fluid]
model = "incompressible"
advection = "upwind"
pressure_solver = "cg"
dt = 1.0e-6        # timestep (s)
steps = 100         # maximum steps
cfl = 0.5           # CFL coefficient (optional, used for adaptive dt)
max_time = 1.0e-4   # maximum simulation time (optional, unlimited if omitted)
```

`pressure_solver = "cg"` uses the []{.term id=cg_solver}. Time integration uses []{.term id=operator_splitting} to solve the coupled equation system sequentially.

### [output] section

```toml
[output]
format = "vtu"
every = 10          # output interval (steps)
path = "output/"
fields = ["phi", "electric_field", "ion_density", "charge_density",
          "velocity", "pressure", "ehd_force"]
```
:::

## Running a Simulation

::: {lang=ja}
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
:::

::: {lang=en}
```bash
# Generate mesh
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 \
     -o examples/wire_plate_2d/wire_plate.msh

# Run simulation
cargo run --release --bin ehd-sim -- --config examples/wire_plate_2d/simulation.toml

# Or use the pre-built binary directly
./target/release/ehd-sim --config examples/wire_plate_2d/simulation.toml
```

### CLI options

```text
ehd-sim [OPTIONS]

Options:
  -c, --config <PATH>   Config file path [default: simulation.toml]
  -m, --mesh <PATH>     Override mesh file path
  -o, --output <PATH>   Override output directory
  -h, --help            Show help
```

### Log control

Control log level with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run --release --bin ehd-sim -- -c simulation.toml
RUST_LOG=info  cargo run --release --bin ehd-sim -- -c simulation.toml
```
:::

## Visualization with ParaView

::: {lang=ja}
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
:::

::: {lang=en}
Simulation results are output in VTU format to the `output/` directory.

### Steps

1. Open `output/output.pvd` in ParaView (File → Open)
2. Click "Apply"
3. Select the field to display (phi, velocity, ion_density, etc.)
4. Use the play button for time series animation

### Output file structure

```text
output/
├── output.pvd           # PVD collection (time series index)
├── frame_000000.vtu     # initial state
├── frame_000010.vtu     # step 10
├── frame_000020.vtu     # step 20
└── ...
```

The PVD file is XML recording each frame's timestamp and filename. ParaView automatically recognizes it as a time series.
:::

## Adding a Physics Module

::: {lang=ja}
新しい物理モジュールを追加する手順。[]{.term id=operator_splitting}フレームワークにステップとして組み込む:

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
:::

::: {lang=en}
Steps to add a new physics module. Each module is integrated as steps in the []{.term id=operator_splitting} framework:

### 1. Create the crate

```bash
cargo new crates/thermal-physics --lib
```

Add to `[workspace.members]` in `Cargo.toml`. Add `cfd-core`, `cfd-fields`, `cfd-mesh`, `cfd-time` as dependencies.

### 2. Implement the module struct

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

### 3. Implement SplittingStep

```rust
impl SplittingStep for HeatDiffusionStep {
    fn name(&self) -> &str { "heat_diffusion" }
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError> {
        // Build kernel description and execute via backend
        let kernel = laplacian_kernel("temperature", self.conductivity, "heat_rhs");
        // ... assemble and solve linear system
        Ok(())
    }
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64 { f64::INFINITY }
}
```

### 4. Wire into the CLI

Same pattern as the EHD module in `main()`:

```rust
let thermal = ThermalModule::new(thermal_config);
thermal.register_fields(&mut state.fields, &mesh);
for step in thermal.splitting_steps() {
    splitting.add_step(step);
}
```

Inter-module communication is through `FieldRegistry` only. No direct references needed.
:::
