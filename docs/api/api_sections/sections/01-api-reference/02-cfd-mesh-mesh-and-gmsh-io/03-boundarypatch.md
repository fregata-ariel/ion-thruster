
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
