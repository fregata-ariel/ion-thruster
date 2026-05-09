
| 用途 | crate | 選定理由 |
|---|---|---|
| 疎行列 | `sprs` | 成熟、pure Rust、CSR/COO対応 |
| Gmsh読込 | 自前パーサ | MSH 2.2 ASCII、依存最小化 |
| VTU出力 | `vtkio` | XML VTU非構造格子対応 |
| 設定 | `toml` + `serde` | Rust標準的手法 |
| CLI | `clap` | derive macro対応 |
| ログ | `tracing` | 構造化ログ、スパン計測 |
:::

::: {lang=en}
Inter-crate dependencies are strictly unidirectional. No circular dependencies exist.

```
cfd-core (leaf)
  ↑
cfd-mesh ← cfd-fields ← cfd-linalg
  ↑                        ↑
cfd-compute ← cfd-compute-cpu
  ↑
cfd-fvm ← cfd-time ← cfd-io
  ↑
ehd-physics
  ↑
ehd-cli
```
