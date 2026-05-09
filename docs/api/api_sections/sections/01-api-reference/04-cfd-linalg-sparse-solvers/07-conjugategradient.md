
```rust
pub struct ConjugateGradient {
    pub tol: f64,        // relative residual threshold
    pub max_iter: usize, // maximum iterations
}
impl ConjugateGradient {
    pub fn new(tol: f64, max_iter: usize) -> Self;
}
impl LinearSolver for ConjugateGradient { ... }
```

Jacobi-preconditioned. Scratch buffers (r, z, p, ap) are pre-allocated internally and reused.
