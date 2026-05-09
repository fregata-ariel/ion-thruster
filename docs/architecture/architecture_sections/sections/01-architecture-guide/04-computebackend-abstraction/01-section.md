
FVM演算子（ラプラシアン、移流、[]{.term id=scharfetter_gummel}フラックス等）は`FaceKernel`/`CellKernel`という**データ記述**を返す。実行はしない。

```rust
// cfd-fvm: カーネル「記述」を構築（計算は発生しない）
let kernel = laplacian_kernel("phi", 1.0, "residual");

// cfd-compute-cpu: カーネルを「実行」（バックエンドがops列を解釈）
backend.execute_face_kernel(mesh, &kernel, fields)?;
```

`FaceKernel`は名前、`FaceOp`列、読み書きフィールドリストを持つ。`FaceOp`には4種の面演算がある:

- `Diffusion` — 拡散フラックス: $\gamma_f \cdot A_f \cdot (\phi_N - \phi_O) / |d|$
- `Advection` — 移流フラックス（Upwind/Central/TVD選択）
- `ScharfetterGummel` — ドリフト拡散フラックス（[]{.term id=bernoulli_function}重み付け）
- `Divergence` — ベクトル場の発散

`CellKernel`は`CellOp`列を持ち、6種のセル演算を提供: `Axpy`, `Scale`, `Clamp`, `Multiply`, `Fill`, `Copy`。
