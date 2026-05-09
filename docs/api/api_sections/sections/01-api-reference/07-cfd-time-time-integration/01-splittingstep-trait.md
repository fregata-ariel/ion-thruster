
```rust
pub trait SplittingStep: Send {
    fn name(&self) -> &str;
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError>;
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}
```
