
```rust
pub struct CellId(pub u32);  // cell index
pub struct FaceId(pub u32);  // face index
pub struct NodeId(pub u32);  // node index
```

Implements `From<usize>`, `as_usize()`, `Debug`, `Display`. Using u32 supports billions of elements while saving memory.
