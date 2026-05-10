
crate依存の方向は一方向（下流→上流）であり、循環依存は存在しない。`cfd-core`はリーフcrateとして外部依存を最小限に保ち（`thiserror`のみ）、他の全crateがこれに依存する。物理crateは汎用コアに依存するが、汎用コアは物理crateを知らない。

`cfd-core`の型設計には特筆すべき点がある:

- **Vec3は型エイリアス**: `pub type Vec3 = [f64; 3]` とし、構造体ではなく配列として定義。auto-vectorization対応と、`&[Vec3]`を`&[[f64; 3]]`として直接扱える利便性を両立。ベクトル演算は`vec3::ops`モジュールのインライン自由関数群として提供。
- **newtype index**: `CellId(u32)`, `FaceId(u32)`, `NodeId(u32)` はマクロで生成されたnewtype。`From<usize>`, `as_usize()`で変換可能。u32にすることで、数十億セルまで対応しつつメモリ節約。
- **BoundaryCondition enum**: `Dirichlet`, `Neumann`, `ZeroGradient`, `NoSlip`, `Outflow`, `Absorbing`, `FixedFlux`, `Custom` の8種をサポート。
- **CfdError**: `thiserror`によるderive。`Mesh`, `Io`, `SolverNotConverged`, `FieldNotFound`, `DimensionMismatch`, `Config`, `Boundary`, `Other` の8バリアント。

外部依存は`[workspace.dependencies]`で集中管理される:

| 用途 | crate | バージョン |
|---|---|---|
| 疎行列 | `sprs` | 0.11 |
| VTU出力 | `vtkio` | 0.6 |
| シリアライゼーション | `serde` | 1 |
| TOML設定 | `toml` | 0.8 |
| CLI | `clap` | 4 |
| 構造化ログ | `tracing` | 0.1 |
| エラー型derive | `thiserror` | 2 |
:::

::: {lang=en}
The framework comprises 11 Rust crates. These split into a generic CFD core (7 crates), physics-specific modules (2 crates), and a CLI binary. A Cargo workspace (`resolver = "2"`) centrally manages dependencies and builds.
