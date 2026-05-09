
```rust
pub struct SimulationDriver {
    pub splitting: OperatorSplitting,
    pub max_steps: usize,
    pub max_time: f64,
    pub output_interval: usize,
    pub fixed_dt: Option<f64>,
}
impl SimulationDriver {
    pub fn run(&mut self, mesh: &Mesh, state: &mut SimState,
               writer: &mut dyn FieldWriter) -> Result<(), CfdError>;
}
pub trait FieldWriter: Send {
    fn write_frame(&mut self, mesh: &Mesh, state: &SimState, step: usize) -> Result<(), CfdError>;
}
```
:::
