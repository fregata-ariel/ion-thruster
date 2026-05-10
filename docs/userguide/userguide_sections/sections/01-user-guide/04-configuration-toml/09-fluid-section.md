
```toml
[fluid]
model = "incompressible"
advection = "upwind"
pressure_solver = "cg"
dt = 1.0e-6        # timestep (s)
steps = 100         # maximum steps
cfl = 0.5           # CFL coefficient (optional, used for adaptive dt)
max_time = 1.0e-4   # maximum simulation time (optional, unlimited if omitted)
```

`pressure_solver = "cg"` uses the []{.term id=cg_solver}. Time integration uses []{.term id=operator_splitting} to solve the coupled equation system sequentially.
