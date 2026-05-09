
::: {lang=ja}
[]{.term id=fvm}演算子のカーネルビルダー関数群。計算は行わず、カーネル記述のみを構築する。

```rust
pub fn laplacian_kernel(field: &str, gamma: impl Into<ParamRef>, target: &str) -> FaceKernel;
pub fn advection_kernel(field: &str, velocity: &str, scheme: AdvectionScheme, target: &str) -> FaceKernel;
pub fn scharfetter_gummel_kernel(concentration: &str, electric_field: &str,
    mobility: f64, diffusion: f64, target: &str) -> FaceKernel;
pub fn divergence_kernel(vector_field: &str, target: &str) -> FaceKernel;
pub fn fill_kernel(field: &str, value: f64) -> CellKernel;
pub fn axpy_kernel(a: f64, x: &str, y: &str) -> CellKernel;
pub fn clamp_kernel(field: &str, min: f64, max: f64) -> CellKernel;
```
:::

::: {lang=en}
Kernel builder functions for []{.term id=fvm} operators. These build kernel descriptions only — no computation is performed.

```rust
pub fn laplacian_kernel(field: &str, gamma: impl Into<ParamRef>, target: &str) -> FaceKernel;
pub fn advection_kernel(field: &str, velocity: &str, scheme: AdvectionScheme, target: &str) -> FaceKernel;
pub fn scharfetter_gummel_kernel(concentration: &str, electric_field: &str,
    mobility: f64, diffusion: f64, target: &str) -> FaceKernel;
pub fn divergence_kernel(vector_field: &str, target: &str) -> FaceKernel;
pub fn fill_kernel(field: &str, value: f64) -> CellKernel;
pub fn axpy_kernel(a: f64, x: &str, y: &str) -> CellKernel;
pub fn clamp_kernel(field: &str, min: f64, max: f64) -> CellKernel;
```
:::
