
```rust
pub struct CellId(pub u32);  // セルインデックス
pub struct FaceId(pub u32);  // 面インデックス
pub struct NodeId(pub u32);  // ノードインデックス
```

`From<usize>`, `as_usize()`, `Debug`, `Display`を実装。u32により数十億要素まで対応しつつメモリ節約。
