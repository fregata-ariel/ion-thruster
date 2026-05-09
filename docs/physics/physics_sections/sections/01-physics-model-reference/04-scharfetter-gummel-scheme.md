
::: {lang=ja}
[]{.term id=scharfetter_gummel}は、面のローカルPeclet数に基づいて指数的な重み付けを行い、ドリフトと拡散の競合を安定に離散化する。

面 $f$（owner $O$、neighbor $N$）における面Peclet数:

$$
\text{Pe}_f = \frac{\mu_i \mathbf{E}_f \cdot \mathbf{d}}{D_i}
$$

ここで $\mathbf{d}$ はowner→neighbor重心間ベクトル。面フラックスは[]{.term id=bernoulli_function} $B(x) = x/(e^x - 1)$ を用いて:

$$
J_f = \frac{D_i A_f}{|\mathbf{d}|} \left[ B(\text{Pe}_f) \, n_O - B(-\text{Pe}_f) \, n_N \right]
$$

Bernoulli関数の数値的安定性は3領域で確保する:

| $|x|$ の範囲 | 近似 | 実装 |
|---|---|---|
| $|x| < 10^{-6}$ | Taylor展開: $B(x) \approx 1 - x/2 + x^2/12$ | 桁落ち回避 |
| $x > 500$ | $B(x) \approx 0$ | オーバーフロー回避 |
| $x < -500$ | $B(x) \approx -x$ | オーバーフロー回避 |
| その他 | $B(x) = x/(e^x - 1)$ | 直接計算 |

```rust
fn bernoulli(x: f64) -> f64 {
    if x.abs() < 1e-6 {
        1.0 - x * 0.5 + x * x / 12.0    // Taylor
    } else if x > 500.0 {
        0.0                               // exp(x) >> x
    } else if x < -500.0 {
        -x                                // B(x) ≈ -x for x << 0
    } else {
        x / (x.exp() - 1.0)              // exact
    }
}
```

カーネルは`cfd-fvm`で構築される:

```rust
let kernel = scharfetter_gummel_kernel(
    "ion_density",      // concentration field
    "electric_field",   // E field
    2.0e-4,             // ion mobility μ_i
    5.0e-6,             // diffusion D_i
    "ion_rhs",          // target accumulator
);
```
:::

::: {lang=en}
The []{.term id=scharfetter_gummel} applies exponential weighting based on the local face Peclet number, stably discretizing the competing drift and diffusion terms.

The face Peclet number at face $f$ (owner $O$, neighbor $N$):

$$
\text{Pe}_f = \frac{\mu_i \mathbf{E}_f \cdot \mathbf{d}}{D_i}
$$

where $\mathbf{d}$ is the owner→neighbor centroid vector. The face flux uses the []{.term id=bernoulli_function} $B(x) = x/(e^x - 1)$:

$$
J_f = \frac{D_i A_f}{|\mathbf{d}|} \left[ B(\text{Pe}_f) \, n_O - B(-\text{Pe}_f) \, n_N \right]
$$

Numerical stability of the Bernoulli function is ensured across three regions:

| $|x|$ range | Approximation | Purpose |
|---|---|---|
| $|x| < 10^{-6}$ | Taylor expansion: $B(x) \approx 1 - x/2 + x^2/12$ | Avoid cancellation |
| $x > 500$ | $B(x) \approx 0$ | Avoid overflow |
| $x < -500$ | $B(x) \approx -x$ | Avoid overflow |
| otherwise | $B(x) = x/(e^x - 1)$ | Direct computation |

```rust
fn bernoulli(x: f64) -> f64 {
    if x.abs() < 1e-6 {
        1.0 - x * 0.5 + x * x / 12.0    // Taylor
    } else if x > 500.0 {
        0.0                               // exp(x) >> x
    } else if x < -500.0 {
        -x                                // B(x) ≈ -x for x << 0
    } else {
        x / (x.exp() - 1.0)              // exact
    }
}
```

The kernel is built in `cfd-fvm`:

```rust
let kernel = scharfetter_gummel_kernel(
    "ion_density",      // concentration field
    "electric_field",   // E field
    2.0e-4,             // ion mobility μ_i
    5.0e-6,             // diffusion D_i
    "ion_rhs",          // target accumulator
);
```
:::
