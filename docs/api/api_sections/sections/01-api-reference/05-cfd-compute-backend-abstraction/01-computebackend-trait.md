
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
