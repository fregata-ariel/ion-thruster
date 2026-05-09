
Each sub-step implements the following trait:

```rust
pub trait SplittingStep: Send {
    fn name(&self) -> &str;
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<()>;
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}
```

`max_dt()` reports each step's stability constraint. Elliptic steps (Poisson) return `f64::INFINITY`.
