# Architecture Guide

::: {lang=ja}
ion-craft CFDフレームワークのアーキテクチャガイド。汎用CFDコアと物理モジュールの分離設計、ComputeBackend抽象レイヤー、データ指向設計の全体像を説明する。
:::

::: {lang=en}
Architecture guide for the ion-craft CFD framework. Describes the separation of generic CFD core and physics modules, the ComputeBackend abstraction layer, and the data-oriented design philosophy.
:::

## Overview

::: {lang=ja}
ion-craftは、Rustで構築された汎用CFDシミュレーションフレームワークである。最初のユースケースはEHD（電気流体力学）[]{.term id=ionic_wind}シミュレーションだが、フレームワーク自体は物理非依存に設計されている。

設計の3本柱は以下の通り:

1. **汎用CFDコア + プラグイン物理モジュール**: メッシュ、場、離散化、線形代数、I/Oは物理に依存しない。EHDは差し替え可能な物理モジュールとして実装。将来的に熱伝達、反応流など異なる物理の追加が可能。
2. **ComputeBackend抽象レイヤー**: 「何を計算するか」と「どう実行するか」を分離。カーネルを`FaceOp`/`CellOp`という**検査可能なデータ記述**として構築し、バックエンドが解釈・コンパイルする。将来のJITコンパイラ(Cranelift)やGPU(WGPU)バックエンドに対応可能。
3. **データ指向設計**: 連続`Vec<f64>`配列、CSR圧縮接続、面ベース[]{.term id=fvm}。キャッシュ効率とSIMD auto-vectorization対応を重視。

フレームワークは11のRust crateで構成され、Cargo workspaceで管理される。各crateの責務は明確に分離され、依存関係は厳密に一方向（下流→上流）である。

ソルバの全体フローは:

1. Gmshメッシュファイルを読み込み、面ベースFVMトポロジを構築
2. 物理モジュールが`FieldRegistry`にフィールドを登録・初期化
3. []{.term id=operator_splitting}により連成方程式系を逐次サブステップに分割
4. 各タイムステップで`FaceKernel`/`CellKernel`記述を生成し、`ComputeBackend`で実行
5. VTU形式で結果を出力し、PVD時系列でParaView可視化に対応
:::

::: {lang=en}
ion-craft is a general-purpose CFD simulation framework built in Rust. While the first use case is EHD (electrohydrodynamic) []{.term id=ionic_wind} simulation, the framework itself is physics-agnostic by design.

Three design pillars:

1. **Generic CFD core + pluggable physics modules**: Mesh, fields, discretization, linear algebra, and I/O are physics-independent. EHD is implemented as a swappable physics module. Additional physics (heat transfer, reactive flows, etc.) can be added in the future.
2. **ComputeBackend abstraction layer**: Separates "what to compute" from "how to execute." Kernels are constructed as **inspectable data descriptions** (`FaceOp`/`CellOp` enums), which backends interpret or compile. Enables future JIT compiler (Cranelift) and GPU (WGPU) backends.
3. **Data-oriented design**: Contiguous `Vec<f64>` arrays, CSR-compressed connectivity, face-based []{.term id=fvm}. Prioritizes cache efficiency and SIMD auto-vectorization readiness.

The framework comprises 11 Rust crates managed as a Cargo workspace. Each crate has clearly separated responsibilities, and dependencies flow strictly in one direction (downstream → upstream).

The overall solver flow is:

1. Load a Gmsh mesh file and build face-based FVM topology
2. Physics modules register and initialize fields in the `FieldRegistry`
3. []{.term id=operator_splitting} decomposes the coupled equation system into sequential sub-steps
4. Each timestep generates `FaceKernel`/`CellKernel` descriptions, executed by the `ComputeBackend`
5. Output results in VTU format with PVD time series for ParaView visualization
:::

## Crate Structure

::: {lang=ja}
フレームワークは11のRust crateで構成される。汎用CFDコア（7 crate）と物理固有部（2 crate）、およびCLIバイナリに分かれる。Cargo workspace（`resolver = "2"`）で依存関係とビルドを一括管理する。

### 汎用CFDコア

| crate | 役割 | 主な公開型 |
|---|---|---|
| `cfd-core` | 共有基盤型。依存なしのリーフcrate | `Vec3 = [f64; 3]`, `CellId(u32)`, `FaceId(u32)`, `NodeId(u32)`, `BoundaryCondition`, `CfdError`, `FieldLocation` |
| `cfd-mesh` | 非構造格子メッシュ、Gmsh MSH 2.2リーダー、面ベーストポロジ構築 | `Mesh`, `BoundaryPatch`, `CellType` |
| `cfd-fields` | 場の格納・レジストリ・シミュレーション状態 | `ScalarField`, `VectorField`, `Field`, `FieldRegistry`, `SimState` |
| `cfd-linalg` | CSR疎行列演算(sprs)、前処理付き[]{.term id=cg_solver} | `LinearSystem`, `ConjugateGradient`, `JacobiPreconditioner` |
| `cfd-compute` | ComputeBackend trait、カーネルIR記述型 | `ComputeBackend`, `FaceKernel`, `CellKernel`, `FaceOp`, `CellOp`, `FieldRef`, `ParamRef` |
| `cfd-compute-cpu` | CPUバックエンド（Rustループによる解釈実行） | `CpuBackend`, `CpuMesh`, `CpuFieldStore` |
| `cfd-fvm` | []{.term id=fvm}演算子のカーネルビルダー関数群 | `laplacian_kernel()`, `advection_kernel()`, `scharfetter_gummel_kernel()` 等 |
| `cfd-time` | 時間積分制御 | `SplittingStep`, `OperatorSplitting`, `SimulationDriver`, `FieldWriter` |
| `cfd-io` | 入出力（VTU出力、PVD時系列、TOML設定パーサ） | `VtuWriter`, `SimConfig` |

### 物理モジュール

| crate | 役割 | 主な公開型 |
|---|---|---|
| `ehd-physics` | EHD物理モジュール（[]{.term id=poisson_eq}、イオン輸送、[]{.term id=ehd_body_force}） | `EhdModule`, `EhdConfig` |
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
:::

::: {lang=en}
The framework comprises 11 Rust crates. These split into a generic CFD core (7 crates), physics-specific modules (2 crates), and a CLI binary. A Cargo workspace (`resolver = "2"`) centrally manages dependencies and builds.

### Generic CFD Core

| Crate | Role | Key Public Types |
|---|---|---|
| `cfd-core` | Shared foundational types. Leaf crate with no internal dependencies | `Vec3 = [f64; 3]`, `CellId(u32)`, `FaceId(u32)`, `NodeId(u32)`, `BoundaryCondition`, `CfdError`, `FieldLocation` |
| `cfd-mesh` | Unstructured mesh, Gmsh MSH 2.2 reader, face-based topology construction | `Mesh`, `BoundaryPatch`, `CellType` |
| `cfd-fields` | Field storage, registry, and simulation state | `ScalarField`, `VectorField`, `Field`, `FieldRegistry`, `SimState` |
| `cfd-linalg` | CSR sparse matrix ops (sprs), preconditioned []{.term id=cg_solver} | `LinearSystem`, `ConjugateGradient`, `JacobiPreconditioner` |
| `cfd-compute` | ComputeBackend trait, kernel IR description types | `ComputeBackend`, `FaceKernel`, `CellKernel`, `FaceOp`, `CellOp`, `FieldRef`, `ParamRef` |
| `cfd-compute-cpu` | CPU backend (interpreted Rust loop execution) | `CpuBackend`, `CpuMesh`, `CpuFieldStore` |
| `cfd-fvm` | []{.term id=fvm} operator kernel builder functions | `laplacian_kernel()`, `advection_kernel()`, `scharfetter_gummel_kernel()` etc. |
| `cfd-time` | Time integration control | `SplittingStep`, `OperatorSplitting`, `SimulationDriver`, `FieldWriter` |
| `cfd-io` | Input/output (VTU output, PVD time series, TOML config parser) | `VtuWriter`, `SimConfig` |

### Physics Modules

| Crate | Role | Key Public Types |
|---|---|---|
| `ehd-physics` | EHD physics module ([]{.term id=poisson_eq}, ion transport, []{.term id=ehd_body_force}) | `EhdModule`, `EhdConfig` |
| `ehd-cli` | `ehd-sim` CLI binary (clap-based) | — |

### Design Principles

Crate dependencies flow in one direction (downstream → upstream) with no circular dependencies. `cfd-core` is a leaf crate with minimal external dependencies (only `thiserror`), and all other crates depend on it. Physics crates depend on the generic core, but the generic core has no knowledge of physics crates.

Notable type design choices in `cfd-core`:

- **Vec3 as type alias**: `pub type Vec3 = [f64; 3]` — defined as an array rather than a struct. This enables auto-vectorization and allows `&[Vec3]` to be used directly as `&[[f64; 3]]`. Vector operations are provided as inline free functions in the `vec3::ops` module.
- **Newtype indices**: `CellId(u32)`, `FaceId(u32)`, `NodeId(u32)` generated via macro. Convertible via `From<usize>` and `as_usize()`. Using u32 supports up to billions of cells while saving memory.
- **BoundaryCondition enum**: Supports 8 variants: `Dirichlet`, `Neumann`, `ZeroGradient`, `NoSlip`, `Outflow`, `Absorbing`, `FixedFlux`, `Custom`.
- **CfdError**: Derived via `thiserror`. 8 variants: `Mesh`, `Io`, `SolverNotConverged`, `FieldNotFound`, `DimensionMismatch`, `Config`, `Boundary`, `Other`.

External dependencies are centrally managed via `[workspace.dependencies]`:

| Purpose | Crate | Version |
|---|---|---|
| Sparse matrices | `sprs` | 0.11 |
| VTU output | `vtkio` | 0.6 |
| Serialization | `serde` | 1 |
| TOML config | `toml` | 0.8 |
| CLI | `clap` | 4 |
| Structured logging | `tracing` | 0.1 |
| Error type derive | `thiserror` | 2 |
:::

## Data Flow

::: {lang=ja}
シミュレーション全体のデータフローは以下の通り:

```
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
:::

::: {lang=en}
The overall simulation data flow:

```
Gmsh (.geo/.msh)
  → cfd-mesh: Mesh struct (face-based FVM topology)
    → cfd-fields: FieldRegistry (field registration & sharing)
      → ehd-physics: PhysicsModule (field initialization & step definition)
        → cfd-time: OperatorSplitting (time loop control)
          → cfd-fvm: kernel descriptions (FaceKernel/CellKernel)
            → cfd-compute-cpu: kernel execution
          → cfd-linalg: linear system assembly & solve
        → cfd-io: VTU/PVD output
          → ParaView visualization
```

Concretely, the wiring happens in `ehd-cli`'s `main()`:

```rust
// 1. Load config
let config = SimConfig::load(&path)?;

// 2. Read mesh and build topology
let mesh = cfd_mesh::gmsh::read_msh(&config.mesh.path)?;

// 3. Initialize state + register physics module fields
let mut state = SimState::new();
let ehd = EhdModule::new(ehd_config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

// 4. Configure operator splitting
let mut splitting = OperatorSplitting::new(cfl);
for step in ehd.splitting_steps() {
    splitting.add_step(step);
}

// 5. Set up output writer
let mut writer = VtuWriter::new(&config.output.path, scalars, vectors);

// 6. Run simulation
let mut driver = SimulationDriver { splitting, ... };
driver.run(&mesh, &mut state, &mut writer)?;
```

Data sharing between physics modules occurs through the `FieldRegistry`. Each module only registers and references named fields — no direct inter-module references exist. The EHD module registers 9 fields (`phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`).
:::

## ComputeBackend Abstraction

::: {lang=ja}
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
:::

::: {lang=en}
The ComputeBackend abstraction layer separates "what to compute" from "how to execute." This is the framework's most distinctive design decision.

The `ComputeBackend` trait uses associated types `Mesh: MeshHandle` and `Fields: FieldStore`, allowing each backend its own data representation:

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

`MeshData` is a lightweight mesh struct defined within `cfd-compute` to avoid direct dependency on `cfd-mesh::Mesh`. Conversion is done via `Mesh::to_mesh_data()`.
:::

### カーネルはデータであり、コードではない

FVM演算子（ラプラシアン、移流、[]{.term id=scharfetter_gummel}フラックス等）は`FaceKernel`/`CellKernel`という**データ記述**を返す。実行はしない。

```rust
// cfd-fvm: カーネル「記述」を構築（計算は発生しない）
let kernel = laplacian_kernel("phi", 1.0, "residual");

// cfd-compute-cpu: カーネルを「実行」（バックエンドがops列を解釈）
backend.execute_face_kernel(mesh, &kernel, fields)?;
```

`FaceKernel`は名前、`FaceOp`列、読み書きフィールドリストを持つ。`FaceOp`には4種の面演算がある:

- `Diffusion` — 拡散フラックス: $\gamma_f \cdot A_f \cdot (\phi_N - \phi_O) / |d|$
- `Advection` — 移流フラックス（Upwind/Central/TVD選択）
- `ScharfetterGummel` — ドリフト拡散フラックス（[]{.term id=bernoulli_function}重み付け）
- `Divergence` — ベクトル場の発散

`CellKernel`は`CellOp`列を持ち、6種のセル演算を提供: `Axpy`, `Scale`, `Clamp`, `Multiply`, `Fill`, `Copy`。

### なぜクロージャではなくデータ記述か

Rustのクロージャはコンパイル時に型が確定する。JITコンパイラ(Cranelift)やGPU(WGPU)には渡せない。データ記述なら:

- Craneliftで検査・コンパイルできる
- WGPUでcompute shaderに変換できる
- シリアライズしてキャッシュできる
- 最適化パスを走らせられる

### バックエンド階層

```
FaceKernel / CellKernel（データ記述）
    │
    ├── CpuBackend      → 直接Rustループ（現在）
    ├── CraneliftBackend → JITネイティブコード（将来）
    └── WgpuBackend      → GPU compute shader（将来）
```

`ComputeBackend` traitは`MeshHandle`と`FieldStore`を関連型として持ち、各バックエンドが独自のデータ表現を使える。
:::

::: {lang=en}
The ComputeBackend abstraction layer separates "what to compute" from "how to execute." This is the framework's most distinctive design decision.

### Kernels are data, not code

FVM operators (Laplacian, advection, []{.term id=scharfetter_gummel} flux, etc.) return **data descriptions** as `FaceKernel`/`CellKernel`. They do not execute.

```rust
// cfd-fvm: build kernel "description" (no computation happens)
let kernel = laplacian_kernel("phi", 1.0, "residual");

// cfd-compute-cpu: "execute" the kernel (backend interprets the ops list)
backend.execute_face_kernel(mesh, &kernel, fields)?;
```

`FaceKernel` carries a name, a list of `FaceOp`s, and read/write field lists. Four face operation variants exist:

- `Diffusion` — diffusion flux: $\gamma_f \cdot A_f \cdot (\phi_N - \phi_O) / |d|$
- `Advection` — advection flux (Upwind/Central/TVD selectable)
- `ScharfetterGummel` — drift-diffusion flux ([]{.term id=bernoulli_function} weighted)
- `Divergence` — divergence of a vector field

`CellKernel` carries a list of `CellOp`s, providing 6 cell operations: `Axpy`, `Scale`, `Clamp`, `Multiply`, `Fill`, `Copy`.

### Why data descriptions instead of closures

Rust closures have types fixed at compile time. They cannot be passed to JIT compilers (Cranelift) or GPUs (WGPU). Data descriptions can be:

- Inspected and compiled by Cranelift
- Converted to compute shaders for WGPU
- Serialized and cached
- Optimized through transformation passes

### Backend hierarchy

```
FaceKernel / CellKernel (data descriptions)
    │
    ├── CpuBackend        → direct Rust loops (current)
    ├── CraneliftBackend   → JIT native code (future)
    └── WgpuBackend        → GPU compute shaders (future)
```

The `ComputeBackend` trait uses associated types for `MeshHandle` and `FieldStore`, allowing each backend to use its own data representation.
:::

## Mesh Data Model

::: {lang=ja}
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
:::

::: {lang=en}
The `Mesh` struct stores face-based FVM topology in a data-oriented layout. All data is in contiguous arrays — no per-cell heap allocations.

```rust
pub struct Mesh {
    // Dimensions
    pub n_nodes: usize, pub n_cells: usize,
    pub n_faces: usize, pub n_internal_faces: usize,
    // Node coordinates
    pub node_coords: Vec<Vec3>,
    // Cell data
    pub cell_volumes: Vec<f64>, pub cell_centroids: Vec<Vec3>, pub cell_types: Vec<CellType>,
    // CSR-compressed connectivity (cell→face, cell→node, face→node)
    pub cell_face_offsets: Vec<u32>,  pub cell_face_indices: Vec<FaceId>,
    pub cell_node_offsets: Vec<u32>,  pub cell_node_indices: Vec<NodeId>,
    pub face_node_offsets: Vec<u32>,  pub face_node_indices: Vec<NodeId>,
    // Face data
    pub face_areas: Vec<f64>, pub face_normals: Vec<Vec3>, pub face_centroids: Vec<Vec3>,
    pub face_owner: Vec<CellId>,     // all faces
    pub face_neighbor: Vec<CellId>,  // internal faces only (length = n_internal_faces)
    // Boundary patches
    pub boundary_patches: Vec<BoundaryPatch>,
    // Precomputed geometry
    pub face_delta: Vec<Vec3>,       // owner → neighbor centroid vector
    pub face_delta_mag: Vec<f64>,    // |face_delta|
    pub face_weight: Vec<f64>,       // interpolation weight
}
```

`CellType` supports 6 variants: `Triangle`, `Quad`, `Tetrahedron`, `Hexahedron`, `Wedge`, `Pyramid`. Topology construction is done by `topology::build_mesh()`, which builds the face-based structure from raw element data read from Gmsh. Shared faces between two cells are identified as internal faces by hash-matching sorted node sets; faces belonging to only one cell are boundary faces.
:::

### 面の順序規約

内部面が先、境界面が後:

```
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
:::

::: {lang=en}
The `Mesh` struct stores face-based FVM topology in a data-oriented layout.

### Face ordering convention

Internal faces first, boundary faces after:

```
Face indices: [0 .. n_internal) [n_internal .. n_total)
               ├─ internal ─────┤ ├─ boundary ──────────┤
                                   ├─ patch 0 ─┤ ├─ patch 1 ─┤ ...
```

This convention matches OpenFOAM and enables the hottest inner loop (internal face flux) to run branch-free with contiguous memory access.

### CSR-compressed connectivity

Cell-to-face, cell-to-node, and face-to-node connectivity use CSR format (offsets array + indices array). This avoids heap-scattered `Vec<Vec<FaceId>>` and ensures cache efficiency and SIMD readiness.

### Precomputed geometry

- Areas, normals, centroids (faces and cells)
- Cell volumes
- Face delta vectors (owner → neighbor centroid)
- Interpolation weights

These are computed once at mesh load time and never recomputed during solver execution.
:::

## Field Registry and State

::: {lang=ja}
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
:::

::: {lang=en}
`FieldRegistry` manages fields through a string-keyed `HashMap<String, Field>`. Physics modules register fields here and share them with other modules.

```rust
// Register fields (inside EHD module)
registry.register_scalar("phi", n_cells, FieldLocation::Cell);
registry.register_vector("velocity", n_cells, FieldLocation::Cell);

// Access fields (inside SplittingStep)
let phi = state.fields.get_scalar("phi")?;
let vel = state.fields.get_vector_mut("velocity")?;
```

`Field` is an enum of `Scalar(ScalarField)` and `Vector(VectorField)`. `ScalarField` wraps `Vec<f64>`, `VectorField` wraps `Vec<Vec3>`. Both carry a `FieldLocation` (`Cell`/`Face`/`Node`).

When simultaneous mutable access to two scalar fields is needed (e.g., during Poisson solver RHS assembly), `get_scalar_pair_mut()` safely provides it via `unsafe`:

```rust
let (phi, rhs) = state.fields.get_scalar_pair_mut("phi", "poisson_rhs")?;
```

`SimState` holds `FieldRegistry` + current time (`time: f64`) + step count (`step: usize`) + timestep size (`dt: f64`). All sub-steps within a timestep sequentially mutate the same `SimState`.
:::

### なぜ文字列キーか

- 物理モジュール間がコンパイル時に互いを知らなくてよい
- デバッグ・ログで場の名前が直接読める
- HashMap lookupのコストはタイムステップあたり1回/フィールドであり、ホットループ内ではない

`SimState`は`FieldRegistry` + 時刻 + ステップ数 + dtを保持する。タイムステップ中の全サブステップが同一の`SimState`を逐次的に変更する。
:::

::: {lang=en}
`FieldRegistry` manages fields through a string-keyed HashMap. Physics modules register fields here and share them with other modules.

```rust
// Register fields (inside EHD module)
registry.register_scalar("phi", n_cells, FieldLocation::Cell);
registry.register_vector("velocity", n_cells, FieldLocation::Cell);

// Access fields (inside SplittingStep)
let phi = state.fields.get_scalar("phi")?;
let vel = state.fields.get_vector_mut("velocity")?;
```

### Why string keys

- Physics modules don't need compile-time knowledge of each other
- Field names are directly readable in debug output and logs
- HashMap lookup cost is once per field per timestep — not in hot loops

`SimState` holds `FieldRegistry` + current time + step count + dt. All sub-steps within a timestep sequentially mutate the same `SimState`.
:::

## Physics Module System

::: {lang=ja}
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
:::

::: {lang=en}
Physics modules are implemented as concrete structs following the `PhysicsModule` pattern. A struct pattern is chosen over a trait because each module has different config types (`EhdConfig`, etc.) and returns different step types. Each module has three responsibilities:

1. **Field registration**: `register_fields(&self, registry, mesh)` — add required fields to the Registry
2. **Initialization**: `initialize(&self, state, mesh)` — set initial conditions
3. **Step provision**: `splitting_steps(&self) -> Vec<Box<dyn SplittingStep>>` — return a list of sub-steps for OperatorSplitting

```rust
let ehd = EhdModule::new(config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

for step in ehd.splitting_steps() {
    splitting.add_step(step);
}
```

The EHD module registers 9 fields: `phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`. It also provides `output_fields()` returning field name lists for VTU output.
:::

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
:::

::: {lang=en}
Physics modules are implemented as concrete structs following the `PhysicsModule` pattern. Rather than a monolithic trait object, each module has three responsibilities:

1. **Field registration**: `register_fields()` — add required fields to the Registry
2. **Initialization**: `initialize()` — set initial conditions
3. **Step provision**: `splitting_steps()` — return a list of sub-steps for OperatorSplitting

```rust
let ehd = EhdModule::new(config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

for step in ehd.splitting_steps() {
    splitting.add_step(step);
}
```

### Future multi-physics coupling

```rust
let ehd = EhdModule::new(ehd_config);
let thermal = ThermalModule::new(thermal_config);

ehd.register_fields(&mut state.fields, &mesh);
thermal.register_fields(&mut state.fields, &mesh);

// Step ordering determines coupling strategy
for step in ehd.splitting_steps() { splitting.add_step(step); }
for step in thermal.splitting_steps() { splitting.add_step(step); }
```

Inter-module communication occurs solely through `FieldRegistry`. No direct references.
:::

## Time Stepping and Operator Splitting

::: {lang=ja}
[]{.term id=operator_splitting}を使用して、連成方程式系を逐次的に解く。`OperatorSplitting`は`Vec<Box<dyn SplittingStep>>`を保持し、毎タイムステップで全ステップを順番に実行する。`SimulationDriver`が最外側の時間ループを制御し、初期状態出力、dt計算、ステップ実行、定期出力、終了判定を行う。

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
:::

::: {lang=en}
[]{.term id=operator_splitting} is used to solve the coupled equation system sequentially. `OperatorSplitting` holds `Vec<Box<dyn SplittingStep>>` and executes all steps in order each timestep. `SimulationDriver` controls the outermost time loop: initial state output, dt computation, step execution, periodic output, and termination checks.

```rust
// SimulationDriver::run() outline
writer.write_frame(mesh, state, 0)?;           // initial state
for step_num in 1..=max_steps {
    state.dt = fixed_dt.unwrap_or(splitting.compute_dt(mesh, state));
    splitting.advance(mesh, state)?;            // execute all sub-steps
    state.time += state.dt;
    if step_num % output_interval == 0 {
        writer.write_frame(mesh, state, step_num)?;
    }
    if state.time >= max_time { break; }
}
```
:::

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

```
1. PoissonStep    : ρ_q → φ, E を計算（楕円型、CG求解）
2. IonTransportStep: E, u を使って n_i を更新（SG scheme）
3. EhdForceStep   : ρ_q × E → f_EHD を計算
4. FluidStep      : f_EHD を体積力として u, p を更新（圧力投影）
```

### 時間刻み制御

全ステップの`max_dt()`の最小値にCFL係数を乗じて全体のdtを決定する。固定dt指定も可能。
:::

::: {lang=en}
[]{.term id=operator_splitting} is used to solve the coupled equation system sequentially.

### SplittingStep trait

Each sub-step implements the following trait:

```rust
pub trait SplittingStep: Send {
    fn name(&self) -> &str;
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<()>;
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}
```

`max_dt()` reports each step's stability constraint. Elliptic steps (Poisson) return `f64::INFINITY`.

### EHD splitting order

```
1. PoissonStep     : ρ_q → φ, E (elliptic, CG solve)
2. IonTransportStep: update n_i using E, u (SG scheme)
3. EhdForceStep    : ρ_q × E → f_EHD
4. FluidStep       : update u, p using f_EHD (pressure projection)
```

### Time step control

Global dt is determined as `CFL × min(step.max_dt())` across all steps. Fixed dt is also supported.
:::

## Performance Design

::: {lang=ja}
パフォーマンスは設計段階から考慮されている。以下の3つの軸で最適化する。
:::

::: {lang=en}
Performance is considered from the design stage. Optimization follows three axes.
:::

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
:::

::: {lang=en}
### Data layout

- All fields are contiguous `Vec<f64>` / `Vec<[f64; 3]>` arrays
- Mesh connectivity uses CSR compression (`Vec<u32>` offsets + `Vec<Id>` indices)
- `HashMap` stays outside hot loops (field lookup is once per timestep per field)

### Allocation avoidance

- `LinearSystem` sparsity pattern is built once; only values are overwritten each step
- CG solver scratch buffers are pre-allocated on the `SplittingStep` struct
- Face flux temporary buffers are reused

### Parallelism readiness

- Face and cell loops have rayon-parallelizable structure
- `[f64; 3]` vectors support auto-vectorization
- `SplittingStep: Send` guarantees thread safety
:::

## Dependency Graph

::: {lang=ja}
crate間の依存関係は厳密に一方向。循環依存は存在しない。

```
cfd-core (リーフ: thiserrorのみ)
  ↑
cfd-mesh ← cfd-fields ← cfd-linalg (sprs)
  ↑                        ↑
cfd-compute ← cfd-compute-cpu
  ↑
cfd-fvm ← cfd-time ← cfd-io (vtkio, toml, serde)
  ↑
ehd-physics
  ↑
ehd-cli (clap, tracing-subscriber)
```

`cfd-compute`は`cfd-mesh`に直接依存せず、`MeshData`という軽量構造体を自身の内部モジュール(`cfd_core_mesh_data`)で定義する。これにより、バックエンドcrateがメッシュの全実装を知る必要がなくなる。
:::

::: {lang=en}
Inter-crate dependencies are strictly unidirectional. No circular dependencies exist.

```
cfd-core (leaf: thiserror only)
  ↑
cfd-mesh ← cfd-fields ← cfd-linalg (sprs)
  ↑                        ↑
cfd-compute ← cfd-compute-cpu
  ↑
cfd-fvm ← cfd-time ← cfd-io (vtkio, toml, serde)
  ↑
ehd-physics
  ↑
ehd-cli (clap, tracing-subscriber)
```

`cfd-compute` does not directly depend on `cfd-mesh`. Instead, it defines a lightweight `MeshData` struct in its own internal module (`cfd_core_mesh_data`). This prevents backend crates from needing knowledge of the full mesh implementation.
:::

### 外部依存

| 用途 | crate | 選定理由 |
|---|---|---|
| 疎行列 | `sprs` | 成熟、pure Rust、CSR/COO対応 |
| Gmsh読込 | 自前パーサ | MSH 2.2 ASCII、依存最小化 |
| VTU出力 | `vtkio` | XML VTU非構造格子対応 |
| 設定 | `toml` + `serde` | Rust標準的手法 |
| CLI | `clap` | derive macro対応 |
| ログ | `tracing` | 構造化ログ、スパン計測 |
:::

::: {lang=en}
Inter-crate dependencies are strictly unidirectional. No circular dependencies exist.

```
cfd-core (leaf)
  ↑
cfd-mesh ← cfd-fields ← cfd-linalg
  ↑                        ↑
cfd-compute ← cfd-compute-cpu
  ↑
cfd-fvm ← cfd-time ← cfd-io
  ↑
ehd-physics
  ↑
ehd-cli
```

### External dependencies

| Purpose | Crate | Rationale |
|---|---|---|
| Sparse matrices | `sprs` | Mature, pure Rust, CSR/COO support |
| Gmsh import | Custom parser | MSH 2.2 ASCII, minimal dependencies |
| VTU output | `vtkio` | XML VTU unstructured grid support |
| Config | `toml` + `serde` | Standard Rust approach |
| CLI | `clap` | Derive macro support |
| Logging | `tracing` | Structured logging with timing spans |
:::
