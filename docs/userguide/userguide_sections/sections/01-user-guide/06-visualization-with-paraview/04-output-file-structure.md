
```text
output/
├── output.pvd           # PVD collection (time series index)
├── frame_000000.vtu     # initial state
├── frame_000010.vtu     # step 10
├── frame_000020.vtu     # step 20
└── ...
```

The PVD file is XML recording each frame's timestamp and filename. ParaView automatically recognizes it as a time series.
:::
