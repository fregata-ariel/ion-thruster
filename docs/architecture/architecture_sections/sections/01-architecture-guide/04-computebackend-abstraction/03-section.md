
```text
FaceKernel / CellKernel（データ記述）
    │
    ├── CpuBackend      → 直接Rustループ（現在）
    ├── CraneliftBackend → JITネイティブコード（将来）
    └── WgpuBackend      → GPU compute shader（将来）
```

:::

::: {lang=en}
