
```rust
pub struct EhdConfig {
    pub permittivity: f64,      // ε₀ (F/m)
    pub ion_mobility: f64,      // μ_i (m²/V·s)
    pub ion_diffusion: f64,     // D_i (m²/s)
    pub gas_density: f64,       // ρ_g (kg/m³)
    pub gas_viscosity: f64,     // μ_g (Pa·s)
    pub elementary_charge: f64, // q (C)
}
impl Default for EhdConfig { ... }  // standard atmospheric defaults
```
