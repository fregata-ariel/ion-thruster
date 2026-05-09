
```rust
pub trait LinearSolver {
    fn solve(&mut self, system: &mut LinearSystem) -> Result<SolveStats, CfdError>;
}
pub struct SolveStats { pub iterations: usize, pub residual: f64, pub converged: bool }
```
