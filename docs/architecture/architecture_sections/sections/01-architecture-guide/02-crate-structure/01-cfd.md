
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
