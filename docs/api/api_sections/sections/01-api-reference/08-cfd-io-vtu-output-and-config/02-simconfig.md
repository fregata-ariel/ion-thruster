
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

TOML設定ファイルを`serde`でデシリアライズ。各フィールドにデフォルト値を提供（`rho_g = 1.225`, `mu_g = 1.81e-5`等）。
:::

::: {lang=en}
Input/output: VTU output and TOML config parser.
