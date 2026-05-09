
```rust
impl SplittingStep for HeatDiffusionStep {
    fn name(&self) -> &str { "heat_diffusion" }
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError> {
        // Build kernel description and execute via backend
        let kernel = laplacian_kernel("temperature", self.conductivity, "heat_rhs");
        // ... assemble and solve linear system
        Ok(())
    }
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64 { f64::INFINITY }
}
```
