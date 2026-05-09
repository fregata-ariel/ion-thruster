
- 面積、法線、重心（面・セル）
- セル体積
- 面delta（owner→neighbor重心間ベクトル）
- 補間重み

これらはメッシュ読込時に一度計算し、ソルバ実行中は再計算しない。
:::

::: {lang=en}
The `Mesh` struct stores face-based FVM topology in a data-oriented layout.
