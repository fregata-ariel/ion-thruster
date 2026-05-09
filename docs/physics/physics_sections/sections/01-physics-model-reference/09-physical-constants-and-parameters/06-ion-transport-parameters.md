
| Symbol | Name | Typical Value | Unit |
|---|---|---|---|
| $\mu_i$ | []{.term id=ion_mobility} | 2.0×10⁻⁴ | m²/(V·s) |
| $D_i$ | []{.term id=ion_diffusion} | 5.0×10⁻⁶ | m²/s |

These parameters are held in the `EhdConfig` struct and loaded from the `[physics]` section of the TOML configuration file:

```toml
[physics]
gas = "air"
rho_g = 1.225
mu_g = 1.81e-5
epsilon = 8.8541878128e-12
ion_mobility = 2.0e-4
ion_diffusion = 5.0e-6
```
:::
