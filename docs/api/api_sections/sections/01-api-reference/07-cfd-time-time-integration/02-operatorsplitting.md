
```rust
impl OperatorSplitting {
    pub fn new(cfl: f64) -> Self;
    pub fn add_step(&mut self, step: Box<dyn SplittingStep>);
    pub fn compute_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
    pub fn advance(&mut self, mesh: &Mesh, state: &mut SimState) -> Result<(), CfdError>;
}
```
