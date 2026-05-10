
::: {lang=ja}
crate間の依存関係は厳密に一方向。循環依存は存在しない。依存グラフはDAG（有向非巡回グラフ）を形成する。

```text
crate                 依存先
─────────────────────────────────────────────────────
cfd-core              (なし — thiserrorのみ)
cfd-compute           cfd-core
cfd-fields            cfd-core
cfd-linalg            cfd-core (+sprs)
cfd-mesh              cfd-core, cfd-compute
cfd-fvm               cfd-core, cfd-compute
cfd-compute-cpu       cfd-core, cfd-compute, cfd-fields
cfd-time              cfd-core, cfd-mesh, cfd-fields
cfd-io                cfd-core, cfd-mesh, cfd-fields, cfd-time (+vtkio, toml, serde)
ehd-physics           cfd-core, cfd-mesh, cfd-fields, cfd-linalg, cfd-compute, cfd-fvm, cfd-time
ehd-cli               全crate (+clap, tracing-subscriber, anyhow)
```

`cfd-compute`は`cfd-mesh`に直接依存せず、`MeshData`という軽量構造体を自身の内部モジュール(`cfd_core_mesh_data`)で定義する。これにより、バックエンドcrateがメッシュの全実装を知る必要がなくなる。
