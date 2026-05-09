
| 変数 | 条件 | 値 |
|---|---|---|
| $\phi$ | Neumann | 0（零勾配） |
| $n_i$ | Outflow | — |
| $\mathbf{u}$ | Outflow | — |

TOML設定例:

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

`Absorbing`境界条件はイオンが電極に到達して消滅する物理を表す。`Outflow`は対流による自然流出を許容する。
:::

::: {lang=en}
Boundary conditions are defined by the `BoundaryCondition` enum and specified per boundary patch in the TOML file. Typical 3-patch configuration for EHD simulation:
