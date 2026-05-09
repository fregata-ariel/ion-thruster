
```rust
impl SplittingStep for HeatDiffusionStep {
    fn name(&self) -> &str { "heat_diffusion" }
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError> {
        // カーネル記述を構築し、バックエンドで実行
        let kernel = laplacian_kernel("temperature", self.conductivity, "heat_rhs");
        // ... 線形系の組立・求解
        Ok(())
    }
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64 { f64::INFINITY }
}
```
