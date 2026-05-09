
```
FaceKernel / CellKernel (data descriptions)
    │
    ├── CpuBackend        → direct Rust loops (current)
    ├── CraneliftBackend   → JIT native code (future)
    └── WgpuBackend        → GPU compute shaders (future)
```

The `ComputeBackend` trait uses associated types for `MeshHandle` and `FieldStore`, allowing each backend to use its own data representation.
:::
