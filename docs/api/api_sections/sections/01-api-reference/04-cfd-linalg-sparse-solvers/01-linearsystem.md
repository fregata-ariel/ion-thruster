
```rust
pub struct LinearSystem {
    pub matrix: CsMat<f64>,    // CSR係数行列
    pub rhs: Vec<f64>,         // 右辺ベクトル
    pub solution: Vec<f64>,    // 解ベクトル（事前確保）
}
impl LinearSystem {
    pub fn new(n: usize) -> Self;
    pub fn size(&self) -> usize;
    pub fn reset_vectors(&mut self);  // RHSと解をゼロリセット（行列パターン保持）
}
```
