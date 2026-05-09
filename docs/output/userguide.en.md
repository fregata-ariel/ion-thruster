# User Guide


User guide for the ion-craft CFD framework. Covers installation, mesh generation, configuration, simulation execution, and visualization.

## Prerequisites


The following tools are required:

| Tool | Version | Purpose |
|---|---|---|
| Rust (rustc + cargo) | 1.85+ (edition 2024) | Build the framework |
| Gmsh | 4.x | Mesh generation (`.geo` → `.msh`) |
| ParaView | 5.x | Result visualization (`.vtu` / `.pvd`) |

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation


```bash
git clone <repository-url> ion-craft
cd ion-craft
cargo build --release
```

After a successful build, the `target/release/ehd-sim` binary is created.

Run tests:
```bash
cargo test --workspace
```

## Mesh Generation with Gmsh


Define geometry in Gmsh `.geo` files and generate `.msh` files (MSH 2.2 ASCII).

### wire_plate_2d sample

`examples/wire_plate_2d/wire_plate.geo` defines a 2D wire-to-plate electrode configuration:

- **Domain**: 60mm × 40mm
- **Wire (emitter)**: radius 1mm, positioned at 20mm gap
- **Plate (collector)**: bottom edge
- **Mesh sizes**: 0.5mm near wire, 1mm near plate, 5mm far field

Physical group definitions are critical — they must match the `[boundary.*]` section names in the TOML config:

```
Physical Curve("collector") = {1};
Physical Curve("emitter") = {5, 6, 7, 8};
Physical Curve("farfield") = {2, 3, 4};
Physical Surface("fluid") = {1};
```

Mesh generation command:
```bash
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 -o examples/wire_plate_2d/wire_plate.msh
```

`-format msh2` is required (ion-craft supports MSH 2.2 ASCII only).

## Configuration (TOML)


Simulation settings are defined in TOML files. Refer to `examples/wire_plate_2d/simulation.toml`:

### [mesh] section

```toml
[mesh]
path = "examples/wire_plate_2d/wire_plate.msh"
format = "gmsh"
```

### [physics] section

```toml
[physics]
gas = "air"
rho_g = 1.225          # gas density (kg/m³)
mu_g = 1.81e-5         # gas dynamic viscosity (Pa·s)
epsilon = 8.8541878128e-12  # permittivity (F/m)
ion_mobility = 2.0e-4  # ion mobility (m²/V·s)
ion_diffusion = 5.0e-6 # ion diffusion coefficient (m²/s)
```

### [boundary.*] section

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

### [fluid] section

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

### [output] section

```toml
[output]
format = "vtu"
every = 10          # output interval (steps)
path = "output/"
fields = ["phi", "electric_field", "ion_density", "charge_density",
          "velocity", "pressure", "ehd_force"]
```

## Running a Simulation


```bash
# Generate mesh
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 \
     -o examples/wire_plate_2d/wire_plate.msh

# Run simulation
cargo run --release --bin ehd-sim -- --config examples/wire_plate_2d/simulation.toml

# Or use the pre-built binary directly
./target/release/ehd-sim --config examples/wire_plate_2d/simulation.toml
```

### CLI options

```
ehd-sim [OPTIONS]

Options:
  -c, --config <PATH>   Config file path [default: simulation.toml]
  -m, --mesh <PATH>     Override mesh file path
  -o, --output <PATH>   Override output directory
  -h, --help            Show help
```

### Log control

Control log level with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run --release --bin ehd-sim -- -c simulation.toml
RUST_LOG=info  cargo run --release --bin ehd-sim -- -c simulation.toml
```

## Visualization with ParaView


Simulation results are output in VTU format to the `output/` directory.

### Steps

1. Open `output/output.pvd` in ParaView (File → Open)
2. Click "Apply"
3. Select the field to display (phi, velocity, ion_density, etc.)
4. Use the play button for time series animation

### Output file structure

```
output/
├── output.pvd           # PVD collection (time series index)
├── frame_000000.vtu     # initial state
├── frame_000010.vtu     # step 10
├── frame_000020.vtu     # step 20
└── ...
```

The PVD file is XML recording each frame's timestamp and filename. ParaView automatically recognizes it as a time series.

## Adding a Physics Module


Steps to add a new physics module:

### 1. Create the crate

```bash
cargo new crates/thermal-physics --lib
```

Add to `[workspace.members]` in `Cargo.toml`. Add `cfd-core`, `cfd-fields`, `cfd-mesh`, `cfd-time` as dependencies.

### 2. Implement the module struct

```rust
pub struct ThermalModule {
    pub config: ThermalConfig,
}

impl ThermalModule {
    pub fn new(config: ThermalConfig) -> Self { ... }

    pub fn register_fields(&self, registry: &mut FieldRegistry, mesh: &Mesh) {
        registry.register_scalar("temperature", mesh.n_cells, FieldLocation::Cell);
        registry.register_scalar("heat_flux", mesh.n_cells, FieldLocation::Cell);
    }

    pub fn initialize(&self, state: &mut SimState, mesh: &Mesh) { ... }

    pub fn splitting_steps(&self) -> Vec<Box<dyn SplittingStep>> {
        vec![Box::new(HeatDiffusionStep::new(self.config.clone()))]
    }
}
```

### 3. Implement SplittingStep

```rust
impl SplittingStep for HeatDiffusionStep {
    fn name(&self) -> &str { "heat_diffusion" }
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError> {
        // Build kernel description and execute via backend
        let kernel = laplacian_kernel("temperature", self.conductivity, "heat_rhs");
        // ... assemble and solve linear system
        Ok(())
    }
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64 { f64::INFINITY }
}
```

### 4. Wire into the CLI

Same pattern as the EHD module in `main()`:

```rust
let thermal = ThermalModule::new(thermal_config);
thermal.register_fields(&mut state.fields, &mesh);
for step in thermal.splitting_steps() {
    splitting.add_step(step);
}
```

Inter-module communication is through `FieldRegistry` only. No direct references needed.
