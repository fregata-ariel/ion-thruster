# API Reference

::: {lang=ja}
ion-craft CFDフレームワークの公開APIリファレンス。各crateの主要な型・trait・関数を記述する。
:::

::: {lang=en}
Public API reference for the ion-craft CFD framework. Describes the key types, traits, and functions of each crate.
:::

## cfd-core: Types and Traits

::: {lang=ja}
リーフcrate。他の全crateが依存する共有基盤型を提供する。外部依存は`thiserror`のみ。

### Vec3

```rust
pub type Vec3 = [f64; 3];
```

3次元ベクトルの型エイリアス。構造体ではなく配列として定義することで、auto-vectorizationと`&[Vec3]` → `&[[f64; 3]]`の直接変換を実現。

ベクトル演算は`vec3::ops`モジュールのインライン自由関数群として提供:

```rust
use cfd_core::vec3::ops;
let a = [1.0, 2.0, 3.0];
let b = [4.0, 5.0, 6.0];
let c = ops::add(a, b);           // [5.0, 7.0, 9.0]
let d = ops::dot(a, b);           // 32.0
let n = ops::normalize(a);        // 単位ベクトル
let m = ops::magnitude(a);        // |a|
let x = ops::cross(a, b);         // 外積
ops::add_scaled(&mut buf, a, 2.0); // buf += 2.0 * a
```

### Index Newtypes

```rust
pub struct CellId(pub u32);  // セルインデックス
pub struct FaceId(pub u32);  // 面インデックス
pub struct NodeId(pub u32);  // ノードインデックス
```

`From<usize>`, `as_usize()`, `Debug`, `Display`を実装。u32により数十億要素まで対応しつつメモリ節約。

### BoundaryCondition

```rust
pub enum BoundaryCondition {
    Dirichlet(f64),  Neumann(f64),  ZeroGradient,  NoSlip,
    Outflow,  Absorbing,  FixedFlux(f64),  Custom(String),
}
```

### FieldLocation

```rust
pub enum FieldLocation { Cell, Face, Node }
```

### CfdError

```rust
pub enum CfdError {
    Mesh(String),  Io(#[from] std::io::Error),
    SolverNotConverged { message: String, residual: f64, iterations: usize },
    FieldNotFound { name: String },  DimensionMismatch { expected: usize, got: usize },
    Config(String),  Boundary(String),  Other(String),
}
```
:::

::: {lang=en}
Leaf crate. Provides shared foundational types depended on by all other crates. Only external dependency is `thiserror`.

### Vec3

```rust
pub type Vec3 = [f64; 3];
```

Type alias for 3D vectors. Defined as an array (not a struct) for auto-vectorization and direct `&[Vec3]` → `&[[f64; 3]]` conversion.

Vector operations are provided as inline free functions in `vec3::ops`:

```rust
use cfd_core::vec3::ops;
let a = [1.0, 2.0, 3.0];
let b = [4.0, 5.0, 6.0];
let c = ops::add(a, b);           // [5.0, 7.0, 9.0]
let d = ops::dot(a, b);           // 32.0
let n = ops::normalize(a);        // unit vector
let m = ops::magnitude(a);        // |a|
let x = ops::cross(a, b);         // cross product
ops::add_scaled(&mut buf, a, 2.0); // buf += 2.0 * a
```

### Index Newtypes

```rust
pub struct CellId(pub u32);  // cell index
pub struct FaceId(pub u32);  // face index
pub struct NodeId(pub u32);  // node index
```

Implements `From<usize>`, `as_usize()`, `Debug`, `Display`. Using u32 supports billions of elements while saving memory.

### BoundaryCondition

```rust
pub enum BoundaryCondition {
    Dirichlet(f64),  Neumann(f64),  ZeroGradient,  NoSlip,
    Outflow,  Absorbing,  FixedFlux(f64),  Custom(String),
}
```

### FieldLocation

```rust
pub enum FieldLocation { Cell, Face, Node }
```

### CfdError

```rust
pub enum CfdError {
    Mesh(String),  Io(#[from] std::io::Error),
    SolverNotConverged { message: String, residual: f64, iterations: usize },
    FieldNotFound { name: String },  DimensionMismatch { expected: usize, got: usize },
    Config(String),  Boundary(String),  Other(String),
}
```
:::

## cfd-mesh: Mesh and Gmsh I/O

::: {lang=ja}
非構造格子メッシュのデータ構造と入力。

### Mesh

`Mesh`構造体の主要フィールドと公開メソッド:

```rust
impl Mesh {
    pub fn cell_faces(&self, cell: CellId) -> &[FaceId];   // CSRルックアップ
    pub fn cell_nodes(&self, cell: CellId) -> &[NodeId];
    pub fn face_nodes(&self, face: FaceId) -> &[NodeId];
    pub fn is_boundary_face(&self, face: FaceId) -> bool;
    pub fn n_boundary_faces(&self) -> usize;
    pub fn find_patch(&self, name: &str) -> Option<&BoundaryPatch>;
    pub fn to_mesh_data(&self) -> MeshData;  // バックエンド用変換
}
```

### CellType

```rust
pub enum CellType { Triangle, Quad, Tetrahedron, Hexahedron, Wedge, Pyramid }
```

### BoundaryPatch

```rust
pub struct BoundaryPatch {
    pub name: String,          // Gmsh physical group名
    pub start_face: usize,     // 境界面領域内のオフセット
    pub n_faces: usize,
}
impl BoundaryPatch {
    pub fn face_range(&self, n_internal_faces: usize) -> Range<usize>;
}
```

### Gmsh I/O

```rust
pub fn gmsh::read_msh(path: &Path) -> Result<Mesh, CfdError>;
```

MSH 2.2 ASCIIフォーマットを読み込み、`topology::build_mesh()`でFVMトポロジを構築する。対応セクション: `$MeshFormat`, `$PhysicalNames`, `$Nodes`, `$Elements`。

### Topology Construction

```rust
pub fn topology::build_mesh(
    node_coords: Vec<Vec3>,
    volume_elements: &[RawElement],
    boundary_faces: &[RawBoundaryFace],
) -> Result<Mesh, CfdError>;
```
:::

::: {lang=en}
Unstructured mesh data structures and input.

### Mesh

Key fields and public methods of the `Mesh` struct:

```rust
impl Mesh {
    pub fn cell_faces(&self, cell: CellId) -> &[FaceId];   // CSR lookup
    pub fn cell_nodes(&self, cell: CellId) -> &[NodeId];
    pub fn face_nodes(&self, face: FaceId) -> &[NodeId];
    pub fn is_boundary_face(&self, face: FaceId) -> bool;
    pub fn n_boundary_faces(&self) -> usize;
    pub fn find_patch(&self, name: &str) -> Option<&BoundaryPatch>;
    pub fn to_mesh_data(&self) -> MeshData;  // conversion for backends
}
```

### CellType

```rust
pub enum CellType { Triangle, Quad, Tetrahedron, Hexahedron, Wedge, Pyramid }
```

### BoundaryPatch

```rust
pub struct BoundaryPatch {
    pub name: String,          // Gmsh physical group name
    pub start_face: usize,     // offset within boundary face region
    pub n_faces: usize,
}
impl BoundaryPatch {
    pub fn face_range(&self, n_internal_faces: usize) -> Range<usize>;
}
```

### Gmsh I/O

```rust
pub fn gmsh::read_msh(path: &Path) -> Result<Mesh, CfdError>;
```

Reads MSH 2.2 ASCII format and builds FVM topology via `topology::build_mesh()`. Supported sections: `$MeshFormat`, `$PhysicalNames`, `$Nodes`, `$Elements`.

### Topology Construction

```rust
pub fn topology::build_mesh(
    node_coords: Vec<Vec3>,
    volume_elements: &[RawElement],
    boundary_faces: &[RawBoundaryFace],
) -> Result<Mesh, CfdError>;
```
:::

## cfd-fields: Fields and Registry

::: {lang=ja}
場の格納、レジストリ、シミュレーション状態。

### ScalarField / VectorField

```rust
pub struct ScalarField { pub values: Vec<f64>, pub location: FieldLocation }
pub struct VectorField { pub values: Vec<Vec3>, pub location: FieldLocation }
pub enum Field { Scalar(ScalarField), Vector(VectorField) }
```

### FieldRegistry

```rust
impl FieldRegistry {
    pub fn new() -> Self;
    pub fn register_scalar(&mut self, name: impl Into<String>, size: usize, location: FieldLocation);
    pub fn register_vector(&mut self, name: impl Into<String>, size: usize, location: FieldLocation);
    pub fn get_scalar(&self, name: &str) -> Result<&ScalarField, CfdError>;
    pub fn get_scalar_mut(&mut self, name: &str) -> Result<&mut ScalarField, CfdError>;
    pub fn get_vector(&self, name: &str) -> Result<&VectorField, CfdError>;
    pub fn get_vector_mut(&mut self, name: &str) -> Result<&mut VectorField, CfdError>;
    pub fn get_scalar_pair_mut(&mut self, a: &str, b: &str)
        -> Result<(&mut ScalarField, &mut ScalarField), CfdError>;
    pub fn names(&self) -> Vec<&str>;
    pub fn contains(&self, name: &str) -> bool;
}
```

### SimState

```rust
pub struct SimState {
    pub fields: FieldRegistry,
    pub time: f64,
    pub step: usize,
    pub dt: f64,
}
```
:::

::: {lang=en}
Field storage, registry, and simulation state.

### ScalarField / VectorField

```rust
pub struct ScalarField { pub values: Vec<f64>, pub location: FieldLocation }
pub struct VectorField { pub values: Vec<Vec3>, pub location: FieldLocation }
pub enum Field { Scalar(ScalarField), Vector(VectorField) }
```

### FieldRegistry

```rust
impl FieldRegistry {
    pub fn new() -> Self;
    pub fn register_scalar(&mut self, name: impl Into<String>, size: usize, location: FieldLocation);
    pub fn register_vector(&mut self, name: impl Into<String>, size: usize, location: FieldLocation);
    pub fn get_scalar(&self, name: &str) -> Result<&ScalarField, CfdError>;
    pub fn get_scalar_mut(&mut self, name: &str) -> Result<&mut ScalarField, CfdError>;
    pub fn get_vector(&self, name: &str) -> Result<&VectorField, CfdError>;
    pub fn get_vector_mut(&mut self, name: &str) -> Result<&mut VectorField, CfdError>;
    pub fn get_scalar_pair_mut(&mut self, a: &str, b: &str)
        -> Result<(&mut ScalarField, &mut ScalarField), CfdError>;
    pub fn names(&self) -> Vec<&str>;
    pub fn contains(&self, name: &str) -> bool;
}
```

### SimState

```rust
pub struct SimState {
    pub fields: FieldRegistry,
    pub time: f64,
    pub step: usize,
    pub dt: f64,
}
```
:::

## cfd-linalg: Sparse Solvers

::: {lang=ja}
疎行列線形代数。`sprs` crateをバックエンドとしたCSR疎行列と反復ソルバ。

### LinearSystem

```rust
pub struct LinearSystem {
    pub matrix: CsMat<f64>,    // CSR係数行列
    pub rhs: Vec<f64>,         // 右辺ベクトル
    pub solution: Vec<f64>,    // 解ベクトル（事前確保）
}
impl LinearSystem {
    pub fn new(n: usize) -> Self;
    pub fn size(&self) -> usize;
    pub fn reset_vectors(&mut self);  // RHSと解をゼロリセット（行列パターン保持）
}
```

### LinearSolver trait

```rust
pub trait LinearSolver {
    fn solve(&mut self, system: &mut LinearSystem) -> Result<SolveStats, CfdError>;
}
pub struct SolveStats { pub iterations: usize, pub residual: f64, pub converged: bool }
```

### ConjugateGradient

```rust
pub struct ConjugateGradient {
    pub tol: f64,        // 相対残差閾値
    pub max_iter: usize, // 最大反復回数
}
impl ConjugateGradient {
    pub fn new(tol: f64, max_iter: usize) -> Self;
}
impl LinearSolver for ConjugateGradient { ... }
```

Jacobi前処理付き。スクラッチバッファ（r, z, p, ap）は内部で事前確保され再利用される。

### Preconditioner

```rust
pub trait Preconditioner { fn apply(&self, r: &[f64], z: &mut [f64]); }
pub struct JacobiPreconditioner;  // M = diag(A)
pub struct NoPreconditioner;      // 恒等写像
```
:::

::: {lang=en}
Sparse linear algebra. CSR sparse matrices and iterative solvers backed by the `sprs` crate.

### LinearSystem

```rust
pub struct LinearSystem {
    pub matrix: CsMat<f64>,    // CSR coefficient matrix
    pub rhs: Vec<f64>,         // right-hand side vector
    pub solution: Vec<f64>,    // solution vector (pre-allocated)
}
impl LinearSystem {
    pub fn new(n: usize) -> Self;
    pub fn size(&self) -> usize;
    pub fn reset_vectors(&mut self);  // zero-reset RHS and solution (keeps matrix pattern)
}
```

### LinearSolver trait

```rust
pub trait LinearSolver {
    fn solve(&mut self, system: &mut LinearSystem) -> Result<SolveStats, CfdError>;
}
pub struct SolveStats { pub iterations: usize, pub residual: f64, pub converged: bool }
```

### ConjugateGradient

```rust
pub struct ConjugateGradient {
    pub tol: f64,        // relative residual threshold
    pub max_iter: usize, // maximum iterations
}
impl ConjugateGradient {
    pub fn new(tol: f64, max_iter: usize) -> Self;
}
impl LinearSolver for ConjugateGradient { ... }
```

Jacobi-preconditioned. Scratch buffers (r, z, p, ap) are pre-allocated internally and reused.

### Preconditioner

```rust
pub trait Preconditioner { fn apply(&self, r: &[f64], z: &mut [f64]); }
pub struct JacobiPreconditioner;  // M = diag(A)
pub struct NoPreconditioner;      // identity
```
:::

## cfd-compute: Backend Abstraction

::: {lang=ja}
「何を計算するか」と「どう実行するか」を分離する抽象レイヤー。

### ComputeBackend trait

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
pub trait MeshHandle: Send + Sync {}
pub trait FieldStore: Send + Sync { ... }
```

### Kernel IR Types

```rust
// 面演算
pub enum FaceOp { Diffusion{..}, Advection{..}, ScharfetterGummel{..}, Divergence{..} }
pub struct FaceKernel { pub name: String, pub ops: Vec<FaceOp>, pub reads/writes: Vec<FieldRef> }

// セル演算
pub enum CellOp { Axpy{..}, Scale{..}, Clamp{..}, Multiply{..}, Fill{..}, Copy{..} }
pub struct CellKernel { pub name: String, pub ops: Vec<CellOp>, pub reads/writes: Vec<FieldRef> }

// パラメータ参照
pub struct FieldRef(pub String);
pub enum ParamRef { Constant(f64), Field(FieldRef) }
pub enum AdvectionScheme { Upwind, Central, TVD(Limiter) }
pub enum Limiter { VanLeer, MinMod, Superbee }
```
:::

::: {lang=en}
Abstraction layer separating "what to compute" from "how to execute."

### ComputeBackend trait

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
pub trait MeshHandle: Send + Sync {}
pub trait FieldStore: Send + Sync { ... }
```

### Kernel IR Types

```rust
// Face operations
pub enum FaceOp { Diffusion{..}, Advection{..}, ScharfetterGummel{..}, Divergence{..} }
pub struct FaceKernel { pub name: String, pub ops: Vec<FaceOp>, pub reads/writes: Vec<FieldRef> }

// Cell operations
pub enum CellOp { Axpy{..}, Scale{..}, Clamp{..}, Multiply{..}, Fill{..}, Copy{..} }
pub struct CellKernel { pub name: String, pub ops: Vec<CellOp>, pub reads/writes: Vec<FieldRef> }

// Parameter references
pub struct FieldRef(pub String);
pub enum ParamRef { Constant(f64), Field(FieldRef) }
pub enum AdvectionScheme { Upwind, Central, TVD(Limiter) }
pub enum Limiter { VanLeer, MinMod, Superbee }
```
:::

## cfd-fvm: FVM Operators

::: {lang=ja}
[]{.term id=fvm}演算子のカーネルビルダー関数群。計算は行わず、カーネル記述のみを構築する。

```rust
pub fn laplacian_kernel(field: &str, gamma: impl Into<ParamRef>, target: &str) -> FaceKernel;
pub fn advection_kernel(field: &str, velocity: &str, scheme: AdvectionScheme, target: &str) -> FaceKernel;
pub fn scharfetter_gummel_kernel(concentration: &str, electric_field: &str,
    mobility: f64, diffusion: f64, target: &str) -> FaceKernel;
pub fn divergence_kernel(vector_field: &str, target: &str) -> FaceKernel;
pub fn fill_kernel(field: &str, value: f64) -> CellKernel;
pub fn axpy_kernel(a: f64, x: &str, y: &str) -> CellKernel;
pub fn clamp_kernel(field: &str, min: f64, max: f64) -> CellKernel;
```
:::

::: {lang=en}
Kernel builder functions for []{.term id=fvm} operators. These build kernel descriptions only — no computation is performed.

```rust
pub fn laplacian_kernel(field: &str, gamma: impl Into<ParamRef>, target: &str) -> FaceKernel;
pub fn advection_kernel(field: &str, velocity: &str, scheme: AdvectionScheme, target: &str) -> FaceKernel;
pub fn scharfetter_gummel_kernel(concentration: &str, electric_field: &str,
    mobility: f64, diffusion: f64, target: &str) -> FaceKernel;
pub fn divergence_kernel(vector_field: &str, target: &str) -> FaceKernel;
pub fn fill_kernel(field: &str, value: f64) -> CellKernel;
pub fn axpy_kernel(a: f64, x: &str, y: &str) -> CellKernel;
pub fn clamp_kernel(field: &str, min: f64, max: f64) -> CellKernel;
```
:::

## cfd-time: Time Integration

::: {lang=ja}
時間積分と[]{.term id=operator_splitting}のオーケストレーション。

### SplittingStep trait

```rust
pub trait SplittingStep: Send {
    fn name(&self) -> &str;
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError>;
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}
```

### OperatorSplitting

```rust
impl OperatorSplitting {
    pub fn new(cfl: f64) -> Self;
    pub fn add_step(&mut self, step: Box<dyn SplittingStep>);
    pub fn compute_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
    pub fn advance(&mut self, mesh: &Mesh, state: &mut SimState) -> Result<(), CfdError>;
}
```

### SimulationDriver

```rust
pub struct SimulationDriver {
    pub splitting: OperatorSplitting,
    pub max_steps: usize,
    pub max_time: f64,
    pub output_interval: usize,
    pub fixed_dt: Option<f64>,
}
impl SimulationDriver {
    pub fn run(&mut self, mesh: &Mesh, state: &mut SimState,
               writer: &mut dyn FieldWriter) -> Result<(), CfdError>;
}
pub trait FieldWriter: Send {
    fn write_frame(&mut self, mesh: &Mesh, state: &SimState, step: usize) -> Result<(), CfdError>;
}
```
:::

::: {lang=en}
Time integration and []{.term id=operator_splitting} orchestration.

### SplittingStep trait

```rust
pub trait SplittingStep: Send {
    fn name(&self) -> &str;
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError>;
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}
```

### OperatorSplitting

```rust
impl OperatorSplitting {
    pub fn new(cfl: f64) -> Self;
    pub fn add_step(&mut self, step: Box<dyn SplittingStep>);
    pub fn compute_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
    pub fn advance(&mut self, mesh: &Mesh, state: &mut SimState) -> Result<(), CfdError>;
}
```

### SimulationDriver

```rust
pub struct SimulationDriver {
    pub splitting: OperatorSplitting,
    pub max_steps: usize,
    pub max_time: f64,
    pub output_interval: usize,
    pub fixed_dt: Option<f64>,
}
impl SimulationDriver {
    pub fn run(&mut self, mesh: &Mesh, state: &mut SimState,
               writer: &mut dyn FieldWriter) -> Result<(), CfdError>;
}
pub trait FieldWriter: Send {
    fn write_frame(&mut self, mesh: &Mesh, state: &SimState, step: usize) -> Result<(), CfdError>;
}
```
:::

## cfd-io: VTU Output and Config

::: {lang=ja}
入出力: VTU出力とTOML設定パーサ。

### VtuWriter

```rust
impl VtuWriter {
    pub fn new(output_dir: impl Into<PathBuf>,
               scalar_fields: Vec<String>, vector_fields: Vec<String>) -> Self;
    pub fn write_pvd(&self) -> Result<(), CfdError>;
}
impl FieldWriter for VtuWriter { ... }
```

出力形式はVTK XML Unstructured Grid（`.vtu`）。PVDコレクションファイルで時系列アニメーションに対応。`vtkio 0.6`を使用。

### SimConfig

```rust
pub struct SimConfig {
    pub mesh: MeshConfig,
    pub physics: PhysicsConfig,
    pub boundary: HashMap<String, BoundaryConfig>,
    pub fluid: FluidConfig,
    pub output: OutputConfig,
}
impl SimConfig {
    pub fn load(path: &Path) -> Result<Self, CfdError>;
}
```

TOML設定ファイルを`serde`でデシリアライズ。各フィールドにデフォルト値を提供（`rho_g = 1.225`, `mu_g = 1.81e-5`等）。
:::

::: {lang=en}
Input/output: VTU output and TOML config parser.

### VtuWriter

```rust
impl VtuWriter {
    pub fn new(output_dir: impl Into<PathBuf>,
               scalar_fields: Vec<String>, vector_fields: Vec<String>) -> Self;
    pub fn write_pvd(&self) -> Result<(), CfdError>;
}
impl FieldWriter for VtuWriter { ... }
```

Output format is VTK XML Unstructured Grid (`.vtu`). PVD collection files support time series animation. Uses `vtkio 0.6`.

### SimConfig

```rust
pub struct SimConfig {
    pub mesh: MeshConfig,
    pub physics: PhysicsConfig,
    pub boundary: HashMap<String, BoundaryConfig>,
    pub fluid: FluidConfig,
    pub output: OutputConfig,
}
impl SimConfig {
    pub fn load(path: &Path) -> Result<Self, CfdError>;
}
```

Deserializes TOML config files via `serde`. Provides defaults for each field (`rho_g = 1.225`, `mu_g = 1.81e-5`, etc.).
:::

## ehd-physics: EHD Module

::: {lang=ja}
EHD（電気流体力学）物理モジュール。

### EhdConfig

```rust
pub struct EhdConfig {
    pub permittivity: f64,      // ε₀ (F/m)
    pub ion_mobility: f64,      // μ_i (m²/V·s)
    pub ion_diffusion: f64,     // D_i (m²/s)
    pub gas_density: f64,       // ρ_g (kg/m³)
    pub gas_viscosity: f64,     // μ_g (Pa·s)
    pub elementary_charge: f64, // q (C)
}
impl Default for EhdConfig { ... }  // 標準大気条件のデフォルト値
```

### EhdModule

```rust
impl EhdModule {
    pub fn new(config: EhdConfig) -> Self;
    pub fn register_fields(&self, registry: &mut FieldRegistry, mesh: &Mesh);
    pub fn initialize(&self, state: &mut SimState, mesh: &Mesh);
    pub fn splitting_steps(&self) -> Vec<Box<dyn SplittingStep>>;
    pub fn output_fields(&self) -> (Vec<String>, Vec<String>);  // (scalars, vectors)
}
```

登録フィールド: `phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`
:::

::: {lang=en}
EHD (electrohydrodynamic) physics module.

### EhdConfig

```rust
pub struct EhdConfig {
    pub permittivity: f64,      // ε₀ (F/m)
    pub ion_mobility: f64,      // μ_i (m²/V·s)
    pub ion_diffusion: f64,     // D_i (m²/s)
    pub gas_density: f64,       // ρ_g (kg/m³)
    pub gas_viscosity: f64,     // μ_g (Pa·s)
    pub elementary_charge: f64, // q (C)
}
impl Default for EhdConfig { ... }  // standard atmospheric defaults
```

### EhdModule

```rust
impl EhdModule {
    pub fn new(config: EhdConfig) -> Self;
    pub fn register_fields(&self, registry: &mut FieldRegistry, mesh: &Mesh);
    pub fn initialize(&self, state: &mut SimState, mesh: &Mesh);
    pub fn splitting_steps(&self) -> Vec<Box<dyn SplittingStep>>;
    pub fn output_fields(&self) -> (Vec<String>, Vec<String>);  // (scalars, vectors)
}
```

Registered fields: `phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`
:::
