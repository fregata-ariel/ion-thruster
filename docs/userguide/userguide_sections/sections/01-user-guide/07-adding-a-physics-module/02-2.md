
```rust
pub struct ThermalModule {
    pub config: ThermalConfig,
}

impl ThermalModule {
    pub fn new(config: ThermalConfig) -> Self { ... }

    pub fn register_fields(&self, registry: &mut FieldRegistry, mesh: &Mesh) {
        registry.register_scalar("temperature", mesh.n_cells, FieldLocation::Cell);
        registry.register_scalar("heat_flux", mesh.n_cells, FieldLocation::Cell);
    }

    pub fn initialize(&self, state: &mut SimState, mesh: &Mesh) { ... }

    pub fn splitting_steps(&self) -> Vec<Box<dyn SplittingStep>> {
        vec![Box::new(HeatDiffusionStep::new(self.config.clone()))]
    }
}
```
