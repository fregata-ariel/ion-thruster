
```rust
pub trait Preconditioner { fn apply(&self, r: &[f64], z: &mut [f64]); }
pub struct JacobiPreconditioner;  // M = diag(A)
pub struct NoPreconditioner;      // identity
```
:::
