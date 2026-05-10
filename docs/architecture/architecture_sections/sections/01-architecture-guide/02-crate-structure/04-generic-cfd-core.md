
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
