
```rust
impl EhdModule {
    pub fn new(config: EhdConfig) -> Self;
    pub fn register_fields(&self, registry: &mut FieldRegistry, mesh: &Mesh);
    pub fn initialize(&self, state: &mut SimState, mesh: &Mesh);
    pub fn splitting_steps(&self) -> Vec<Box<dyn SplittingStep>>;
    pub fn output_fields(&self) -> (Vec<String>, Vec<String>);  // (scalars, vectors)
}
```

登録フィールド: `phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`
:::

::: {lang=en}
EHD (electrohydrodynamic) physics module.
