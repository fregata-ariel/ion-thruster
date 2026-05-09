
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
