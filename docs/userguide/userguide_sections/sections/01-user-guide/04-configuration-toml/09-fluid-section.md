
```toml
[fluid]
model = "incompressible"
advection = "upwind"
pressure_solver = "cg"
dt = 1.0e-6        # timestep (s)
steps = 100         # maximum steps
cfl = 0.5           # CFL coefficient (for adaptive dt)
max_time = 1.0e-4   # maximum simulation time (s)
```
