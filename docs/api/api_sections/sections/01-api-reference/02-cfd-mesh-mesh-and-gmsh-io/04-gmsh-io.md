
```rust
pub fn gmsh::read_msh(path: &Path) -> Result<Mesh, CfdError>;
```

MSH 2.2 ASCIIフォーマットを読み込み、`topology::build_mesh()`でFVMトポロジを構築する。対応セクション: `$MeshFormat`, `$PhysicalNames`, `$Nodes`, `$Elements`。
