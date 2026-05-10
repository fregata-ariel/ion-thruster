
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
Inter-crate dependencies are strictly unidirectional. No circular dependencies exist. The dependency graph forms a DAG (directed acyclic graph).

```text
crate                 depends on
─────────────────────────────────────────────────────
cfd-core              (none — thiserror only)
cfd-compute           cfd-core
cfd-fields            cfd-core
cfd-linalg            cfd-core (+sprs)
cfd-mesh              cfd-core, cfd-compute
cfd-fvm               cfd-core, cfd-compute
cfd-compute-cpu       cfd-core, cfd-compute, cfd-fields
cfd-time              cfd-core, cfd-mesh, cfd-fields
cfd-io                cfd-core, cfd-mesh, cfd-fields, cfd-time (+vtkio, toml, serde)
ehd-physics           cfd-core, cfd-mesh, cfd-fields, cfd-linalg, cfd-compute, cfd-fvm, cfd-time
ehd-cli               all crates (+clap, tracing-subscriber, anyhow)
```

`cfd-compute` does not directly depend on `cfd-mesh`. Instead, it defines a lightweight `MeshData` struct in its own internal module (`cfd_core_mesh_data`). This prevents backend crates from needing knowledge of the full mesh implementation.
