
```
FaceKernel / CellKernel（データ記述）
    │
    ├── CpuBackend      → 直接Rustループ（現在）
    ├── CraneliftBackend → JITネイティブコード（将来）
    └── WgpuBackend      → GPU compute shader（将来）
```

`ComputeBackend` traitは`MeshHandle`と`FieldStore`を関連型として持ち、各バックエンドが独自のデータ表現を使える。
:::

::: {lang=en}
The ComputeBackend abstraction layer separates "what to compute" from "how to execute." This is the framework's most distinctive design decision.
