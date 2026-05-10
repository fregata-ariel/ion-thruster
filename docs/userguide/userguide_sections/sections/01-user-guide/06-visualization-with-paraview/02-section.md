
```text
output/
├── output.pvd           # PVDコレクション（時系列インデックス）
├── frame_000000.vtu     # 初期状態
├── frame_000010.vtu     # ステップ10
├── frame_000020.vtu     # ステップ20
└── ...
```

PVDファイルはXML形式で各フレームのタイムスタンプとファイル名を記録。ParaViewが自動的に時系列として認識する。
:::

::: {lang=en}
Simulation results are output in VTU format to the `output/` directory.
