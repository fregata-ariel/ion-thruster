
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
