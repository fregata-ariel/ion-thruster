
```rust
pub fn gmsh::read_msh(path: &Path) -> Result<Mesh, CfdError>;
```

Reads MSH 2.2 ASCII format and builds FVM topology via `topology::build_mesh()`. Supported sections: `$MeshFormat`, `$PhysicalNames`, `$Nodes`, `$Elements`.
