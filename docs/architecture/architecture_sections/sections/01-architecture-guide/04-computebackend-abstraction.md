
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
