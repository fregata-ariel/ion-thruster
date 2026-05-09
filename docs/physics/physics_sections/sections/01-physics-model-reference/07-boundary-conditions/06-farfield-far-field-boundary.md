
| Variable | Condition | Value |
|---|---|---|
| $\phi$ | Neumann | 0 (zero gradient) |
| $n_i$ | Outflow | — |
| $\mathbf{u}$ | Outflow | — |

TOML configuration example:

```toml
[boundary.emitter]
phi = { type = "dirichlet", value = 20000.0 }
ion_density = { type = "dirichlet", value = 1.0e15 }
velocity = { type = "no_slip" }

[boundary.collector]
phi = { type = "dirichlet", value = 0.0 }
ion_density = { type = "absorbing" }
velocity = { type = "no_slip" }
```

The `Absorbing` boundary condition represents the physics of ions reaching the electrode and being neutralized. `Outflow` permits natural convective outflow.
:::
