
```rust
pub fn topology::build_mesh(
    node_coords: Vec<Vec3>,
    volume_elements: &[RawElement],
    boundary_faces: &[RawBoundaryFace],
) -> Result<Mesh, CfdError>;
```
:::
