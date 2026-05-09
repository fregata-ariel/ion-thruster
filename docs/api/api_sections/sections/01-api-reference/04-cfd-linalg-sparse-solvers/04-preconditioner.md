
```rust
pub trait Preconditioner { fn apply(&self, r: &[f64], z: &mut [f64]); }
pub struct JacobiPreconditioner;  // M = diag(A)
pub struct NoPreconditioner;      // 恒等写像
```
:::

::: {lang=en}
Sparse linear algebra. CSR sparse matrices and iterative solvers backed by the `sprs` crate.
