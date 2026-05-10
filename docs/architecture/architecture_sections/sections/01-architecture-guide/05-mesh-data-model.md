
::: {lang=ja}
`Mesh`構造体は面ベースFVMトポロジをデータ指向レイアウトで保持する。全てのデータは連続配列であり、セルごとのヒープ確保は一切行わない。

```rust
pub struct Mesh {
    // 次元
    pub n_nodes: usize, pub n_cells: usize,
    pub n_faces: usize, pub n_internal_faces: usize,
    // ノード座標
    pub node_coords: Vec<Vec3>,
    // セルデータ
    pub cell_volumes: Vec<f64>, pub cell_centroids: Vec<Vec3>, pub cell_types: Vec<CellType>,
    // CSR圧縮接続 (セル→面、セル→ノード、面→ノード)
    pub cell_face_offsets: Vec<u32>,  pub cell_face_indices: Vec<FaceId>,
    pub cell_node_offsets: Vec<u32>,  pub cell_node_indices: Vec<NodeId>,
    pub face_node_offsets: Vec<u32>,  pub face_node_indices: Vec<NodeId>,
    // 面データ
    pub face_areas: Vec<f64>, pub face_normals: Vec<Vec3>, pub face_centroids: Vec<Vec3>,
    pub face_owner: Vec<CellId>,     // 全面
    pub face_neighbor: Vec<CellId>,  // 内部面のみ (長さ = n_internal_faces)
    // 境界パッチ
    pub boundary_patches: Vec<BoundaryPatch>,
    // 事前計算幾何量
    pub face_delta: Vec<Vec3>,       // owner→neighbor重心間ベクトル
    pub face_delta_mag: Vec<f64>,    // |face_delta|
    pub face_weight: Vec<f64>,       // 補間重み
}
```

`CellType`は`Triangle`, `Quad`, `Tetrahedron`, `Hexahedron`, `Wedge`, `Pyramid`の6種をサポートする。トポロジ構築は`topology::build_mesh()`で、Gmshから読んだ生要素データから面ベース構造を構築する。ソートされたノード集合でハッシュマッチングし、2セルが共有する面を内部面、1セルのみの面を境界面として識別する。
:::

::: {lang=en}
The `Mesh` struct stores face-based FVM topology in a data-oriented layout. All data is in contiguous arrays — no per-cell heap allocations.

```rust
pub struct Mesh {
    // Dimensions
    pub n_nodes: usize, pub n_cells: usize,
    pub n_faces: usize, pub n_internal_faces: usize,
    // Node coordinates
    pub node_coords: Vec<Vec3>,
    // Cell data
    pub cell_volumes: Vec<f64>, pub cell_centroids: Vec<Vec3>, pub cell_types: Vec<CellType>,
    // CSR-compressed connectivity (cell→face, cell→node, face→node)
    pub cell_face_offsets: Vec<u32>,  pub cell_face_indices: Vec<FaceId>,
    pub cell_node_offsets: Vec<u32>,  pub cell_node_indices: Vec<NodeId>,
    pub face_node_offsets: Vec<u32>,  pub face_node_indices: Vec<NodeId>,
    // Face data
    pub face_areas: Vec<f64>, pub face_normals: Vec<Vec3>, pub face_centroids: Vec<Vec3>,
    pub face_owner: Vec<CellId>,     // all faces
    pub face_neighbor: Vec<CellId>,  // internal faces only (length = n_internal_faces)
    // Boundary patches
    pub boundary_patches: Vec<BoundaryPatch>,
    // Precomputed geometry
    pub face_delta: Vec<Vec3>,       // owner → neighbor centroid vector
    pub face_delta_mag: Vec<f64>,    // |face_delta|
    pub face_weight: Vec<f64>,       // interpolation weight
}
```

`CellType` supports 6 variants: `Triangle`, `Quad`, `Tetrahedron`, `Hexahedron`, `Wedge`, `Pyramid`. Topology construction is done by `topology::build_mesh()`, which builds the face-based structure from raw element data read from Gmsh. Shared faces between two cells are identified as internal faces by hash-matching sorted node sets; faces belonging to only one cell are boundary faces.
:::
::: {lang=ja}
