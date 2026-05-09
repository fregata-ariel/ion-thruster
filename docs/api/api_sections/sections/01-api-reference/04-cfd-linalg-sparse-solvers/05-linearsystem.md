
```rust
pub struct LinearSystem {
    pub matrix: CsMat<f64>,    // CSR coefficient matrix
    pub rhs: Vec<f64>,         // right-hand side vector
    pub solution: Vec<f64>,    // solution vector (pre-allocated)
}
impl LinearSystem {
    pub fn new(n: usize) -> Self;
    pub fn size(&self) -> usize;
    pub fn reset_vectors(&mut self);  // zero-reset RHS and solution (keeps matrix pattern)
}
```
