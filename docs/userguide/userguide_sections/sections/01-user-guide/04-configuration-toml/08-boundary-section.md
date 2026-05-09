
Specify boundary conditions per variable for each boundary patch. Patch names must match Gmsh physical group names:

```toml
[boundary.emitter]
phi = { type = "dirichlet", value = 20000.0 }
ion_density = { type = "dirichlet", value = 1.0e15 }
velocity = { type = "no_slip" }

[boundary.collector]
phi = { type = "dirichlet", value = 0.0 }
ion_density = { type = "absorbing" }
velocity = { type = "no_slip" }

[boundary.farfield]
phi = { type = "neumann", value = 0.0 }
ion_density = { type = "outflow" }
velocity = { type = "outflow" }
```

Supported boundary condition types: `dirichlet`, `neumann`, `no_slip`, `outflow`, `absorbing`
