# Architecture Guide

ion-craft CFDフレームワークのアーキテクチャガイド。汎用CFDコアと物理モジュールの分離設計、ComputeBackend抽象レイヤー、データ指向設計の全体像を説明する。


## Overview

ion-craftは、Rustで構築された汎用CFDシミュレーションフレームワークである。最初のユースケースはEHD（電気流体力学）イオン風シミュレーションだが、フレームワーク自体は物理非依存に設計されている。

設計の3本柱は以下の通り:

1. **汎用CFDコア + プラグイン物理モジュール**: メッシュ、場、離散化、線形代数、I/Oは物理に依存しない。EHDは差し替え可能な物理モジュールとして実装。将来的に熱伝達、反応流など異なる物理の追加が可能。
2. **ComputeBackend抽象レイヤー**: 「何を計算するか」と「どう実行するか」を分離。カーネルを`FaceOp`/`CellOp`という**検査可能なデータ記述**として構築し、バックエンドが解釈・コンパイルする。将来のJITコンパイラ(Cranelift)やGPU(WGPU)バックエンドに対応可能。
3. **データ指向設計**: 連続`Vec<f64>`配列、CSR圧縮接続、面ベース有限体積法。キャッシュ効率とSIMD auto-vectorization対応を重視。

フレームワークは11のRust crateで構成され、Cargo workspaceで管理される。各crateの責務は明確に分離され、依存関係は厳密に一方向（下流→上流）である。

ソルバの全体フローは:

1. Gmshメッシュファイルを読み込み、面ベースFVMトポロジを構築
2. 物理モジュールが`FieldRegistry`にフィールドを登録・初期化
3. オペレータ分割法により連成方程式系を逐次サブステップに分割
4. 各タイムステップで`FaceKernel`/`CellKernel`記述を生成し、`ComputeBackend`で実行
5. VTU形式で結果を出力し、PVD時系列でParaView可視化に対応


## Crate Structure

フレームワークは11のRust crateで構成される。汎用CFDコア（7 crate）と物理固有部（2 crate）、およびCLIバイナリに分かれる。Cargo workspace（`resolver = "2"`）で依存関係とビルドを一括管理する。

### 汎用CFDコア

| crate | 役割 | 主な公開型 |
|---|---|---|
| `cfd-core` | 共有基盤型。依存なしのリーフcrate | `Vec3 = [f64; 3]`, `CellId(u32)`, `FaceId(u32)`, `NodeId(u32)`, `BoundaryCondition`, `CfdError`, `FieldLocation` |
| `cfd-mesh` | 非構造格子メッシュ、Gmsh MSH 2.2リーダー、面ベーストポロジ構築 | `Mesh`, `BoundaryPatch`, `CellType` |
| `cfd-fields` | 場の格納・レジストリ・シミュレーション状態 | `ScalarField`, `VectorField`, `Field`, `FieldRegistry`, `SimState` |
| `cfd-linalg` | CSR疎行列演算(sprs)、前処理付き共役勾配法 | `LinearSystem`, `ConjugateGradient`, `JacobiPreconditioner` |
| `cfd-compute` | ComputeBackend trait、カーネルIR記述型 | `ComputeBackend`, `FaceKernel`, `CellKernel`, `FaceOp`, `CellOp`, `FieldRef`, `ParamRef` |
| `cfd-compute-cpu` | CPUバックエンド（Rustループによる解釈実行） | `CpuBackend`, `CpuMesh`, `CpuFieldStore` |
| `cfd-fvm` | 有限体積法演算子のカーネルビルダー関数群 | `laplacian_kernel()`, `advection_kernel()`, `scharfetter_gummel_kernel()` 等 |
| `cfd-time` | 時間積分制御 | `SplittingStep`, `OperatorSplitting`, `SimulationDriver`, `FieldWriter` |
| `cfd-io` | 入出力（VTU出力、PVD時系列、TOML設定パーサ） | `VtuWriter`, `SimConfig` |

### 物理モジュール

| crate | 役割 | 主な公開型 |
|---|---|---|
| `ehd-physics` | EHD物理モジュール（Poisson方程式、イオン輸送、EHD体積力） | `EhdModule`, `EhdConfig` |
| `ehd-cli` | `ehd-sim` CLIバイナリ（clapベース） | — |

### 設計原則

crate依存の方向は一方向（下流→上流）であり、循環依存は存在しない。`cfd-core`はリーフcrateとして外部依存を最小限に保ち（`thiserror`のみ）、他の全crateがこれに依存する。物理crateは汎用コアに依存するが、汎用コアは物理crateを知らない。

`cfd-core`の型設計には特筆すべき点がある:

- **Vec3は型エイリアス**: `pub type Vec3 = [f64; 3]` とし、構造体ではなく配列として定義。auto-vectorization対応と、`&[Vec3]`を`&[[f64; 3]]`として直接扱える利便性を両立。ベクトル演算は`vec3::ops`モジュールのインライン自由関数群として提供。
- **newtype index**: `CellId(u32)`, `FaceId(u32)`, `NodeId(u32)` はマクロで生成されたnewtype。`From<usize>`, `as_usize()`で変換可能。u32にすることで、数十億セルまで対応しつつメモリ節約。
- **BoundaryCondition enum**: `Dirichlet`, `Neumann`, `ZeroGradient`, `NoSlip`, `Outflow`, `Absorbing`, `FixedFlux`, `Custom` の8種をサポート。
- **CfdError**: `thiserror`によるderive。`Mesh`, `Io`, `SolverNotConverged`, `FieldNotFound`, `DimensionMismatch`, `Config`, `Boundary`, `Other` の8バリアント。

外部依存は`[workspace.dependencies]`で集中管理される:

| 用途 | crate | バージョン |
|---|---|---|
| 疎行列 | `sprs` | 0.11 |
| VTU出力 | `vtkio` | 0.6 |
| シリアライゼーション | `serde` | 1 |
| TOML設定 | `toml` | 0.8 |
| CLI | `clap` | 4 |
| 構造化ログ | `tracing` | 0.1 |
| エラー型derive | `thiserror` | 2 |


## Data Flow

シミュレーション全体のデータフローは以下の通り:

```text
Gmsh (.geo/.msh)
  → cfd-mesh: Mesh構造体（面ベースFVMトポロジ）
    → cfd-fields: FieldRegistry（場の登録・共有）
      → ehd-physics: PhysicsModule（場の初期化・ステップ定義）
        → cfd-time: OperatorSplitting（時間ループ制御）
          → cfd-fvm: カーネル記述（FaceKernel/CellKernel）
            → cfd-compute-cpu: カーネル実行
          → cfd-linalg: 線形系の組立・求解
        → cfd-io: VTU/PVD出力
          → ParaView可視化
```

具体的には、`ehd-cli`の`main()`で以下のように接続される:

```rust
// 1. 設定読込
let config = SimConfig::load(&path)?;

// 2. メッシュ読込・トポロジ構築
let mesh = cfd_mesh::gmsh::read_msh(&config.mesh.path)?;

// 3. 状態初期化 + 物理モジュールのフィールド登録
let mut state = SimState::new();
let ehd = EhdModule::new(ehd_config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

// 4. オペレータ分割の構成
let mut splitting = OperatorSplitting::new(cfl);
for step in ehd.splitting_steps() {
    splitting.add_step(step);
}

// 5. 出力ライターの設定
let mut writer = VtuWriter::new(&config.output.path, scalars, vectors);

// 6. シミュレーション実行
let mut driver = SimulationDriver { splitting, ... };
driver.run(&mesh, &mut state, &mut writer)?;
```

物理モジュール間のデータ共有は`FieldRegistry`を介して行う。各モジュールは名前付きフィールドを登録・参照するだけで、直接的なモジュール間参照は持たない。EHDモジュールは9個のフィールド（`phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`）を登録する。


## ComputeBackend Abstraction

ComputeBackend抽象レイヤーは「何を計算するか」と「どう実行するか」を分離する。これがフレームワークの最も特徴的な設計判断である。

`ComputeBackend` traitは関連型`Mesh: MeshHandle`と`Fields: FieldStore`を持ち、各バックエンドが独自のデータ表現を使える:

```rust
pub trait ComputeBackend: Send + Sync {
    type Mesh: MeshHandle;
    type Fields: FieldStore;

    fn prepare_mesh(&self, mesh: &MeshData) -> Result<Self::Mesh, CfdError>;
    fn execute_face_kernel(&self, mesh: &Self::Mesh, kernel: &FaceKernel,
                           fields: &mut Self::Fields) -> Result<(), CfdError>;
    fn execute_cell_kernel(&self, mesh: &Self::Mesh, kernel: &CellKernel,
                           fields: &mut Self::Fields) -> Result<(), CfdError>;
    fn spmv(&self, values: &[f64], col_idx: &[u32], row_ptr: &[u32], x: &[f64], y: &mut [f64]);
    fn dot(&self, x: &[f64], y: &[f64]) -> f64;
    fn norm2(&self, x: &[f64]) -> f64;
}
```

`MeshData`は`cfd-compute`内で定義された軽量メッシュ構造体で、`cfd-mesh::Mesh`への直接依存を避ける。`Mesh::to_mesh_data()`で変換する。


### カーネルはデータであり、コードではない

FVM演算子（ラプラシアン、移流、Scharfetter-Gummelスキームフラックス等）は`FaceKernel`/`CellKernel`という**データ記述**を返す。実行はしない。

```rust
// cfd-fvm: カーネル「記述」を構築（計算は発生しない）
let kernel = laplacian_kernel("phi", 1.0, "residual");

// cfd-compute-cpu: カーネルを「実行」（バックエンドがops列を解釈）
backend.execute_face_kernel(mesh, &kernel, fields)?;
```

`FaceKernel`は名前、`FaceOp`列、読み書きフィールドリストを持つ。`FaceOp`には4種の面演算がある:

- `Diffusion` — 拡散フラックス: $\gamma_f \cdot A_f \cdot (\phi_N - \phi_O) / |d|$
- `Advection` — 移流フラックス（Upwind/Central/TVD選択）
- `ScharfetterGummel` — ドリフト拡散フラックス（Bernoulli関数重み付け）
- `Divergence` — ベクトル場の発散

`CellKernel`は`CellOp`列を持ち、6種のセル演算を提供: `Axpy`, `Scale`, `Clamp`, `Multiply`, `Fill`, `Copy`。

### なぜクロージャではなくデータ記述か

Rustのクロージャはコンパイル時に型が確定する。JITコンパイラ(Cranelift)やGPU(WGPU)には渡せない。データ記述なら:

- Craneliftで検査・コンパイルできる
- WGPUでcompute shaderに変換できる
- シリアライズしてキャッシュできる
- 最適化パスを走らせられる

### バックエンド階層

```text
FaceKernel / CellKernel（データ記述）
    │
    ├── CpuBackend      → 直接Rustループ（現在）
    ├── CraneliftBackend → JITネイティブコード（将来）
    └── WgpuBackend      → GPU compute shader（将来）
```



## Mesh Data Model

`Mesh`構造体は面ベースFVMトポロジをデータ指向レイアウトで保持する。全てのデータは連続配列であり、セルごとのヒープ確保は一切行わない。

```rust
pub struct Mesh {
    // 次元
    pub n_nodes: usize, pub n_cells: usize,
    pub n_faces: usize, pub n_internal_faces: usize,
    // ノード座標
    pub node_coords: Vec<Vec3>,
    // セルデータ
    pub cell_volumes: Vec<f64>, pub cell_centroids: Vec<Vec3>, pub cell_types: Vec<CellType>,
    // CSR圧縮接続 (セル→面、セル→ノード、面→ノード)
    pub cell_face_offsets: Vec<u32>,  pub cell_face_indices: Vec<FaceId>,
    pub cell_node_offsets: Vec<u32>,  pub cell_node_indices: Vec<NodeId>,
    pub face_node_offsets: Vec<u32>,  pub face_node_indices: Vec<NodeId>,
    // 面データ
    pub face_areas: Vec<f64>, pub face_normals: Vec<Vec3>, pub face_centroids: Vec<Vec3>,
    pub face_owner: Vec<CellId>,     // 全面
    pub face_neighbor: Vec<CellId>,  // 内部面のみ (長さ = n_internal_faces)
    // 境界パッチ
    pub boundary_patches: Vec<BoundaryPatch>,
    // 事前計算幾何量
    pub face_delta: Vec<Vec3>,       // owner→neighbor重心間ベクトル
    pub face_delta_mag: Vec<f64>,    // |face_delta|
    pub face_weight: Vec<f64>,       // 補間重み
}
```

`CellType`は`Triangle`, `Quad`, `Tetrahedron`, `Hexahedron`, `Wedge`, `Pyramid`の6種をサポートする。トポロジ構築は`topology::build_mesh()`で、Gmshから読んだ生要素データから面ベース構造を構築する。ソートされたノード集合でハッシュマッチングし、2セルが共有する面を内部面、1セルのみの面を境界面として識別する。


### 面の順序規約

内部面が先、境界面が後:

```text
面インデックス: [0 .. n_internal) [n_internal .. n_total)
                 ├─ 内部面 ─────┤ ├─ 境界面 ──────────┤
                                   ├─ patch 0 ─┤ ├─ patch 1 ─┤ ...
```

この順序はOpenFOAMの規約に一致し、最もホットな内部面ループをブランチフリー・連続メモリアクセスで実行可能にする。

### CSR圧縮接続

セル→面、セル→ノード、面→ノードの接続情報はCSR形式（offsets配列 + indices配列）で格納する。`Vec<Vec<FaceId>>`のようなヒープ散在を避け、キャッシュ効率とSIMD対応を確保する。

### 事前計算された幾何量

- 面積、法線、重心（面・セル）
- セル体積
- 面delta（owner→neighbor重心間ベクトル）
- 補間重み

これらはメッシュ読込時に一度計算し、ソルバ実行中は再計算しない。


## Field Registry and State

`FieldRegistry`は文字列キーの`HashMap<String, Field>`で場を管理する。物理モジュールはここにフィールドを登録し、他のモジュールと共有する。

```rust
// 場の登録（EHDモジュール内）
registry.register_scalar("phi", n_cells, FieldLocation::Cell);
registry.register_vector("velocity", n_cells, FieldLocation::Cell);

// 場へのアクセス（SplittingStep内）
let phi = state.fields.get_scalar("phi")?;
let vel = state.fields.get_vector_mut("velocity")?;
```

`Field`は`Scalar(ScalarField)`と`Vector(VectorField)`のenum。`ScalarField`は`Vec<f64>`、`VectorField`は`Vec<Vec3>`を内部に持つ。いずれも`FieldLocation`（`Cell`/`Face`/`Node`）を保持する。

2つのスカラーフィールドへの同時可変参照が必要な場合（例: PoissonソルバのRHS組立時）、`get_scalar_pair_mut()`が`unsafe`で安全に提供する:

```rust
let (phi, rhs) = state.fields.get_scalar_pair_mut("phi", "poisson_rhs")?;
```

`SimState`は`FieldRegistry` + 時刻(`time: f64`) + ステップ数(`step: usize`) + 時間刻み(`dt: f64`)を保持する。タイムステップ中の全サブステップが同一の`SimState`を逐次的に変更する。


### なぜ文字列キーか

- 物理モジュール間がコンパイル時に互いを知らなくてよい
- デバッグ・ログで場の名前が直接読める
- HashMap lookupのコストはタイムステップあたり1回/フィールドであり、ホットループ内ではない


## Physics Module System

物理モジュールは`PhysicsModule`パターンに従う具体的な構造体として実装する。traitではなく構造体パターンを採用する理由は、各モジュールが異なる設定型（`EhdConfig`等）を持ち、返すステップの型も異なるためである。3つの責務を持つ:

1. **場の登録**: `register_fields(&self, registry, mesh)` — 必要なフィールドをRegistryに追加
2. **初期化**: `initialize(&self, state, mesh)` — 初期条件の設定
3. **ステップ提供**: `splitting_steps(&self) -> Vec<Box<dyn SplittingStep>>` — OperatorSplittingに追加するサブステップのリストを返す

```rust
let ehd = EhdModule::new(config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

for step in ehd.splitting_steps() {
    splitting.add_step(step);
}
```

EHDモジュールは9個のフィールドを登録する: `phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`。`output_fields()`メソッドでVTU出力すべきフィールドの名前リストも返す。


### 将来の複数物理モジュール連成

```rust
let ehd = EhdModule::new(ehd_config);
let thermal = ThermalModule::new(thermal_config);

ehd.register_fields(&mut state.fields, &mesh);
thermal.register_fields(&mut state.fields, &mesh);

// ステップの順序が連成の仕方を決める
for step in ehd.splitting_steps() { splitting.add_step(step); }
for step in thermal.splitting_steps() { splitting.add_step(step); }
```

モジュール間の通信は`FieldRegistry`のみを介する。直接参照なし。


## Time Stepping and Operator Splitting

オペレータ分割法を使用して、連成方程式系を逐次的に解く。`OperatorSplitting`は`Vec<Box<dyn SplittingStep>>`を保持し、毎タイムステップで全ステップを順番に実行する。`SimulationDriver`が最外側の時間ループを制御し、初期状態出力、dt計算、ステップ実行、定期出力、終了判定を行う。

```rust
// SimulationDriver::run() の概略
writer.write_frame(mesh, state, 0)?;           // 初期状態
for step_num in 1..=max_steps {
    state.dt = fixed_dt.unwrap_or(splitting.compute_dt(mesh, state));
    splitting.advance(mesh, state)?;            // 全サブステップ実行
    state.time += state.dt;
    if step_num % output_interval == 0 {
        writer.write_frame(mesh, state, step_num)?;
    }
    if state.time >= max_time { break; }
}
```


### SplittingStep trait

各サブステップは以下のtraitを実装する:

```rust
pub trait SplittingStep: Send {
    fn name(&self) -> &str;
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<()>;
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}
```

`max_dt()`は各ステップの安定性制約を報告する。楕円型（Poisson）は`f64::INFINITY`を返す。

### EHDの分割順序

```text
1. PoissonStep    : ρ_q → φ, E を計算（楕円型、CG求解）
2. IonTransportStep: E, u を使って n_i を更新（SG scheme）
3. EhdForceStep   : ρ_q × E → f_EHD を計算
4. FluidStep      : f_EHD を体積力として u, p を更新（圧力投影）
```

### 時間刻み制御

全ステップの`max_dt()`の最小値にCFL係数を乗じて全体のdtを決定する。固定dt指定も可能。


## Performance Design

パフォーマンスは設計段階から考慮されている。以下の3つの軸で最適化する。


### データレイアウト

- 全フィールドは`Vec<f64>` / `Vec<[f64; 3]>`の連続配列
- メッシュ接続はCSR圧縮（`Vec<u32>` offsets + `Vec<Id>` indices）
- `HashMap`はホットループ外（場の取得は1回/タイムステップ/フィールド）

### アロケーション回避

- `LinearSystem`のスパーシティパターンは1回構築、値のみ毎ステップ上書き
- CGソルバのスクラッチバッファは`SplittingStep`構造体に事前確保
- 面フラックスの一時バッファも再利用

### 並列化への備え

- 面ループ・セルループはrayon並列化対応構造
- `[f64; 3]`ベクトルはauto-vectorization対応
- `SplittingStep: Send`でスレッド安全性を保証


## Dependency Graph

crate間の依存関係は厳密に一方向。循環依存は存在しない。依存グラフはDAG（有向非巡回グラフ）を形成する。

```text
crate                 依存先
─────────────────────────────────────────────────────
cfd-core              (なし — thiserrorのみ)
cfd-compute           cfd-core
cfd-fields            cfd-core
cfd-linalg            cfd-core (+sprs)
cfd-mesh              cfd-core, cfd-compute
cfd-fvm               cfd-core, cfd-compute
cfd-compute-cpu       cfd-core, cfd-compute, cfd-fields
cfd-time              cfd-core, cfd-mesh, cfd-fields
cfd-io                cfd-core, cfd-mesh, cfd-fields, cfd-time (+vtkio, toml, serde)
ehd-physics           cfd-core, cfd-mesh, cfd-fields, cfd-linalg, cfd-compute, cfd-fvm, cfd-time
ehd-cli               全crate (+clap, tracing-subscriber, anyhow)
```

`cfd-compute`は`cfd-mesh`に直接依存せず、`MeshData`という軽量構造体を自身の内部モジュール(`cfd_core_mesh_data`)で定義する。これにより、バックエンドcrateがメッシュの全実装を知る必要がなくなる。

### 外部依存

| 用途 | crate | 選定理由 |
|---|---|---|
| 疎行列 | `sprs` | 成熟、pure Rust、CSR/COO対応 |
| Gmsh読込 | 自前パーサ | MSH 2.2 ASCII、依存最小化 |
| VTU出力 | `vtkio` | XML VTU非構造格子対応 |
| 設定 | `toml` + `serde` | Rust標準的手法 |
| CLI | `clap` | derive macro対応 |
| ログ | `tracing` | 構造化ログ、スパン計測 |

