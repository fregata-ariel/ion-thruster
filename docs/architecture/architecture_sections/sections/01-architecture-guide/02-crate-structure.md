
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
