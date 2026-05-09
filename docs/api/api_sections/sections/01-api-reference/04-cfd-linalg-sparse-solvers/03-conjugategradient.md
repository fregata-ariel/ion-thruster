
```rust
pub struct ConjugateGradient {
    pub tol: f64,        // 相対残差閾値
    pub max_iter: usize, // 最大反復回数
}
impl ConjugateGradient {
    pub fn new(tol: f64, max_iter: usize) -> Self;
}
impl LinearSolver for ConjugateGradient { ... }
```

Jacobi前処理付き。スクラッチバッファ（r, z, p, ap）は内部で事前確保され再利用される。
