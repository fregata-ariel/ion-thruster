
```rust
pub struct SimConfig {
    pub mesh: MeshConfig,
    pub physics: PhysicsConfig,
    pub boundary: HashMap<String, BoundaryConfig>,
    pub fluid: FluidConfig,
    pub output: OutputConfig,
}
impl SimConfig {
    pub fn load(path: &Path) -> Result<Self, CfdError>;
}
```

Deserializes TOML config files via `serde`. Provides defaults for each field (`rho_g = 1.225`, `mu_g = 1.81e-5`, etc.).
:::
