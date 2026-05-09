
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
