
::: {lang=ja}
crate間の依存関係は厳密に一方向。循環依存は存在しない。

```
cfd-core (リーフ: thiserrorのみ)
  ↑
cfd-mesh ← cfd-fields ← cfd-linalg (sprs)
  ↑                        ↑
cfd-compute ← cfd-compute-cpu
  ↑
cfd-fvm ← cfd-time ← cfd-io (vtkio, toml, serde)
  ↑
ehd-physics
  ↑
ehd-cli (clap, tracing-subscriber)
```

`cfd-compute`は`cfd-mesh`に直接依存せず、`MeshData`という軽量構造体を自身の内部モジュール(`cfd_core_mesh_data`)で定義する。これにより、バックエンドcrateがメッシュの全実装を知る必要がなくなる。
:::

::: {lang=en}
Inter-crate dependencies are strictly unidirectional. No circular dependencies exist.

```
cfd-core (leaf: thiserror only)
  ↑
cfd-mesh ← cfd-fields ← cfd-linalg (sprs)
  ↑                        ↑
cfd-compute ← cfd-compute-cpu
  ↑
cfd-fvm ← cfd-time ← cfd-io (vtkio, toml, serde)
  ↑
ehd-physics
  ↑
ehd-cli (clap, tracing-subscriber)
```

`cfd-compute` does not directly depend on `cfd-mesh`. Instead, it defines a lightweight `MeshData` struct in its own internal module (`cfd_core_mesh_data`). This prevents backend crates from needing knowledge of the full mesh implementation.
:::
