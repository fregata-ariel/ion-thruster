
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
