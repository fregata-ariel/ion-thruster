# Architecture Guide


Architecture guide for the ion-craft CFD framework. Describes the separation of generic CFD core and physics modules, the ComputeBackend abstraction layer, and the data-oriented design philosophy.

## Overview


ion-craft is a general-purpose CFD simulation framework built in Rust. While the first use case is EHD (electrohydrodynamic) ionic wind simulation, the framework itself is physics-agnostic by design.

Three design pillars:

1. **Generic CFD core + pluggable physics modules**: Mesh, fields, discretization, linear algebra, and I/O are physics-independent. EHD is implemented as a swappable physics module. Additional physics (heat transfer, reactive flows, etc.) can be added in the future.
2. **ComputeBackend abstraction layer**: Separates "what to compute" from "how to execute." Kernels are constructed as **inspectable data descriptions** (`FaceOp`/`CellOp` enums), which backends interpret or compile. Enables future JIT compiler (Cranelift) and GPU (WGPU) backends.
3. **Data-oriented design**: Contiguous `Vec<f64>` arrays, CSR-compressed connectivity, face-based Finite Volume Method. Prioritizes cache efficiency and SIMD auto-vectorization readiness.

The framework comprises 11 Rust crates managed as a Cargo workspace. Each crate has clearly separated responsibilities, and dependencies flow strictly in one direction (downstream → upstream).

The overall solver flow is:

1. Load a Gmsh mesh file and build face-based FVM topology
2. Physics modules register and initialize fields in the `FieldRegistry`
3. operator splitting method decomposes the coupled equation system into sequential sub-steps
4. Each timestep generates `FaceKernel`/`CellKernel` descriptions, executed by the `ComputeBackend`
5. Output results in VTU format with PVD time series for ParaView visualization

## Crate Structure


The framework comprises 11 Rust crates. These split into a generic CFD core (7 crates), physics-specific modules (2 crates), and a CLI binary. A Cargo workspace (`resolver = "2"`) centrally manages dependencies and builds.

### Generic CFD Core

| Crate | Role | Key Public Types |
|---|---|---|
| `cfd-core` | Shared foundational types. Leaf crate with no internal dependencies | `Vec3 = [f64; 3]`, `CellId(u32)`, `FaceId(u32)`, `NodeId(u32)`, `BoundaryCondition`, `CfdError`, `FieldLocation` |
| `cfd-mesh` | Unstructured mesh, Gmsh MSH 2.2 reader, face-based topology construction | `Mesh`, `BoundaryPatch`, `CellType` |
| `cfd-fields` | Field storage, registry, and simulation state | `ScalarField`, `VectorField`, `Field`, `FieldRegistry`, `SimState` |
| `cfd-linalg` | CSR sparse matrix ops (sprs), preconditioned Conjugate Gradient method | `LinearSystem`, `ConjugateGradient`, `JacobiPreconditioner` |
| `cfd-compute` | ComputeBackend trait, kernel IR description types | `ComputeBackend`, `FaceKernel`, `CellKernel`, `FaceOp`, `CellOp`, `FieldRef`, `ParamRef` |
| `cfd-compute-cpu` | CPU backend (interpreted Rust loop execution) | `CpuBackend`, `CpuMesh`, `CpuFieldStore` |
| `cfd-fvm` | Finite Volume Method operator kernel builder functions | `laplacian_kernel()`, `advection_kernel()`, `scharfetter_gummel_kernel()` etc. |
| `cfd-time` | Time integration control | `SplittingStep`, `OperatorSplitting`, `SimulationDriver`, `FieldWriter` |
| `cfd-io` | Input/output (VTU output, PVD time series, TOML config parser) | `VtuWriter`, `SimConfig` |

### Physics Modules

| Crate | Role | Key Public Types |
|---|---|---|
| `ehd-physics` | EHD physics module (Poisson equation, ion transport, EHD body force) | `EhdModule`, `EhdConfig` |
| `ehd-cli` | `ehd-sim` CLI binary (clap-based) | — |

### Design Principles

Crate dependencies flow in one direction (downstream → upstream) with no circular dependencies. `cfd-core` is a leaf crate with minimal external dependencies (only `thiserror`), and all other crates depend on it. Physics crates depend on the generic core, but the generic core has no knowledge of physics crates.

Notable type design choices in `cfd-core`:

- **Vec3 as type alias**: `pub type Vec3 = [f64; 3]` — defined as an array rather than a struct. This enables auto-vectorization and allows `&[Vec3]` to be used directly as `&[[f64; 3]]`. Vector operations are provided as inline free functions in the `vec3::ops` module.
- **Newtype indices**: `CellId(u32)`, `FaceId(u32)`, `NodeId(u32)` generated via macro. Convertible via `From<usize>` and `as_usize()`. Using u32 supports up to billions of cells while saving memory.
- **BoundaryCondition enum**: Supports 8 variants: `Dirichlet`, `Neumann`, `ZeroGradient`, `NoSlip`, `Outflow`, `Absorbing`, `FixedFlux`, `Custom`.
- **CfdError**: Derived via `thiserror`. 8 variants: `Mesh`, `Io`, `SolverNotConverged`, `FieldNotFound`, `DimensionMismatch`, `Config`, `Boundary`, `Other`.

External dependencies are centrally managed via `[workspace.dependencies]`:

| Purpose | Crate | Version |
|---|---|---|
| Sparse matrices | `sprs` | 0.11 |
| VTU output | `vtkio` | 0.6 |
| Serialization | `serde` | 1 |
| TOML config | `toml` | 0.8 |
| CLI | `clap` | 4 |
| Structured logging | `tracing` | 0.1 |
| Error type derive | `thiserror` | 2 |

## Data Flow


The overall simulation data flow:

```
Gmsh (.geo/.msh)
  → cfd-mesh: Mesh struct (face-based FVM topology)
    → cfd-fields: FieldRegistry (field registration & sharing)
      → ehd-physics: PhysicsModule (field initialization & step definition)
        → cfd-time: OperatorSplitting (time loop control)
          → cfd-fvm: kernel descriptions (FaceKernel/CellKernel)
            → cfd-compute-cpu: kernel execution
          → cfd-linalg: linear system assembly & solve
        → cfd-io: VTU/PVD output
          → ParaView visualization
```

Concretely, the wiring happens in `ehd-cli`'s `main()`:

```rust
// 1. Load config
let config = SimConfig::load(&path)?;

// 2. Read mesh and build topology
let mesh = cfd_mesh::gmsh::read_msh(&config.mesh.path)?;

// 3. Initialize state + register physics module fields
let mut state = SimState::new();
let ehd = EhdModule::new(ehd_config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

// 4. Configure operator splitting
let mut splitting = OperatorSplitting::new(cfl);
for step in ehd.splitting_steps() {
    splitting.add_step(step);
}

// 5. Set up output writer
let mut writer = VtuWriter::new(&config.output.path, scalars, vectors);

// 6. Run simulation
let mut driver = SimulationDriver { splitting, ... };
driver.run(&mesh, &mut state, &mut writer)?;
```

Data sharing between physics modules occurs through the `FieldRegistry`. Each module only registers and references named fields — no direct inter-module references exist. The EHD module registers 9 fields (`phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`).

## ComputeBackend Abstraction


The ComputeBackend abstraction layer separates "what to compute" from "how to execute." This is the framework's most distinctive design decision.

The `ComputeBackend` trait uses associated types `Mesh: MeshHandle` and `Fields: FieldStore`, allowing each backend its own data representation:

```rust
pub trait ComputeBackend: Send + Sync {
    type Mesh: MeshHandle;
    type Fields: FieldStore;

    fn prepare_mesh(&self, mesh: &MeshData) -> Result<Self::Mesh, CfdError>;
    fn execute_face_kernel(&self, mesh: &Self::Mesh, kernel: &FaceKernel,
                           fields: &mut Self::Fields) -> Result<(), CfdError>;
    fn execute_cell_kernel(&self, mesh: &Self::Mesh, kernel: &CellKernel,
                           fields: &mut Self::Fields) -> Result<(), CfdError>;
    fn spmv(&self, values: &[f64], col_idx: &[u32], row_ptr: &[u32], x: &[f64], y: &mut [f64]);
    fn dot(&self, x: &[f64], y: &[f64]) -> f64;
    fn norm2(&self, x: &[f64]) -> f64;
}
```

`MeshData` is a lightweight mesh struct defined within `cfd-compute` to avoid direct dependency on `cfd-mesh::Mesh`. Conversion is done via `Mesh::to_mesh_data()`.

The ComputeBackend abstraction layer separates "what to compute" from "how to execute." This is the framework's most distinctive design decision.

### Kernels are data, not code

FVM operators (Laplacian, advection, Scharfetter-Gummel scheme flux, etc.) return **data descriptions** as `FaceKernel`/`CellKernel`. They do not execute.

```rust
// cfd-fvm: build kernel "description" (no computation happens)
let kernel = laplacian_kernel("phi", 1.0, "residual");

// cfd-compute-cpu: "execute" the kernel (backend interprets the ops list)
backend.execute_face_kernel(mesh, &kernel, fields)?;
```

`FaceKernel` carries a name, a list of `FaceOp`s, and read/write field lists. Four face operation variants exist:

- `Diffusion` — diffusion flux: $\gamma_f \cdot A_f \cdot (\phi_N - \phi_O) / |d|$
- `Advection` — advection flux (Upwind/Central/TVD selectable)
- `ScharfetterGummel` — drift-diffusion flux (Bernoulli function weighted)
- `Divergence` — divergence of a vector field

`CellKernel` carries a list of `CellOp`s, providing 6 cell operations: `Axpy`, `Scale`, `Clamp`, `Multiply`, `Fill`, `Copy`.

### Why data descriptions instead of closures

Rust closures have types fixed at compile time. They cannot be passed to JIT compilers (Cranelift) or GPUs (WGPU). Data descriptions can be:

- Inspected and compiled by Cranelift
- Converted to compute shaders for WGPU
- Serialized and cached
- Optimized through transformation passes

### Backend hierarchy

```
FaceKernel / CellKernel (data descriptions)
    │
    ├── CpuBackend        → direct Rust loops (current)
    ├── CraneliftBackend   → JIT native code (future)
    └── WgpuBackend        → GPU compute shaders (future)
```

The `ComputeBackend` trait uses associated types for `MeshHandle` and `FieldStore`, allowing each backend to use its own data representation.

## Mesh Data Model


The `Mesh` struct stores face-based FVM topology in a data-oriented layout. All data is in contiguous arrays — no per-cell heap allocations.

```rust
pub struct Mesh {
    // Dimensions
    pub n_nodes: usize, pub n_cells: usize,
    pub n_faces: usize, pub n_internal_faces: usize,
    // Node coordinates
    pub node_coords: Vec<Vec3>,
    // Cell data
    pub cell_volumes: Vec<f64>, pub cell_centroids: Vec<Vec3>, pub cell_types: Vec<CellType>,
    // CSR-compressed connectivity (cell→face, cell→node, face→node)
    pub cell_face_offsets: Vec<u32>,  pub cell_face_indices: Vec<FaceId>,
    pub cell_node_offsets: Vec<u32>,  pub cell_node_indices: Vec<NodeId>,
    pub face_node_offsets: Vec<u32>,  pub face_node_indices: Vec<NodeId>,
    // Face data
    pub face_areas: Vec<f64>, pub face_normals: Vec<Vec3>, pub face_centroids: Vec<Vec3>,
    pub face_owner: Vec<CellId>,     // all faces
    pub face_neighbor: Vec<CellId>,  // internal faces only (length = n_internal_faces)
    // Boundary patches
    pub boundary_patches: Vec<BoundaryPatch>,
    // Precomputed geometry
    pub face_delta: Vec<Vec3>,       // owner → neighbor centroid vector
    pub face_delta_mag: Vec<f64>,    // |face_delta|
    pub face_weight: Vec<f64>,       // interpolation weight
}
```

`CellType` supports 6 variants: `Triangle`, `Quad`, `Tetrahedron`, `Hexahedron`, `Wedge`, `Pyramid`. Topology construction is done by `topology::build_mesh()`, which builds the face-based structure from raw element data read from Gmsh. Shared faces between two cells are identified as internal faces by hash-matching sorted node sets; faces belonging to only one cell are boundary faces.

The `Mesh` struct stores face-based FVM topology in a data-oriented layout.

### Face ordering convention

Internal faces first, boundary faces after:

```
Face indices: [0 .. n_internal) [n_internal .. n_total)
               ├─ internal ─────┤ ├─ boundary ──────────┤
                                   ├─ patch 0 ─┤ ├─ patch 1 ─┤ ...
```

This convention matches OpenFOAM and enables the hottest inner loop (internal face flux) to run branch-free with contiguous memory access.

### CSR-compressed connectivity

Cell-to-face, cell-to-node, and face-to-node connectivity use CSR format (offsets array + indices array). This avoids heap-scattered `Vec<Vec<FaceId>>` and ensures cache efficiency and SIMD readiness.

### Precomputed geometry

- Areas, normals, centroids (faces and cells)
- Cell volumes
- Face delta vectors (owner → neighbor centroid)
- Interpolation weights

These are computed once at mesh load time and never recomputed during solver execution.

## Field Registry and State


`FieldRegistry` manages fields through a string-keyed `HashMap<String, Field>`. Physics modules register fields here and share them with other modules.

```rust
// Register fields (inside EHD module)
registry.register_scalar("phi", n_cells, FieldLocation::Cell);
registry.register_vector("velocity", n_cells, FieldLocation::Cell);

// Access fields (inside SplittingStep)
let phi = state.fields.get_scalar("phi")?;
let vel = state.fields.get_vector_mut("velocity")?;
```

`Field` is an enum of `Scalar(ScalarField)` and `Vector(VectorField)`. `ScalarField` wraps `Vec<f64>`, `VectorField` wraps `Vec<Vec3>`. Both carry a `FieldLocation` (`Cell`/`Face`/`Node`).

When simultaneous mutable access to two scalar fields is needed (e.g., during Poisson solver RHS assembly), `get_scalar_pair_mut()` safely provides it via `unsafe`:

```rust
let (phi, rhs) = state.fields.get_scalar_pair_mut("phi", "poisson_rhs")?;
```

`SimState` holds `FieldRegistry` + current time (`time: f64`) + step count (`step: usize`) + timestep size (`dt: f64`). All sub-steps within a timestep sequentially mutate the same `SimState`.

`FieldRegistry` manages fields through a string-keyed HashMap. Physics modules register fields here and share them with other modules.

```rust
// Register fields (inside EHD module)
registry.register_scalar("phi", n_cells, FieldLocation::Cell);
registry.register_vector("velocity", n_cells, FieldLocation::Cell);

// Access fields (inside SplittingStep)
let phi = state.fields.get_scalar("phi")?;
let vel = state.fields.get_vector_mut("velocity")?;
```

### Why string keys

- Physics modules don't need compile-time knowledge of each other
- Field names are directly readable in debug output and logs
- HashMap lookup cost is once per field per timestep — not in hot loops

`SimState` holds `FieldRegistry` + current time + step count + dt. All sub-steps within a timestep sequentially mutate the same `SimState`.

## Physics Module System


Physics modules are implemented as concrete structs following the `PhysicsModule` pattern. A struct pattern is chosen over a trait because each module has different config types (`EhdConfig`, etc.) and returns different step types. Each module has three responsibilities:

1. **Field registration**: `register_fields(&self, registry, mesh)` — add required fields to the Registry
2. **Initialization**: `initialize(&self, state, mesh)` — set initial conditions
3. **Step provision**: `splitting_steps(&self) -> Vec<Box<dyn SplittingStep>>` — return a list of sub-steps for OperatorSplitting

```rust
let ehd = EhdModule::new(config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

for step in ehd.splitting_steps() {
    splitting.add_step(step);
}
```

The EHD module registers 9 fields: `phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`. It also provides `output_fields()` returning field name lists for VTU output.

Physics modules are implemented as concrete structs following the `PhysicsModule` pattern. Rather than a monolithic trait object, each module has three responsibilities:

1. **Field registration**: `register_fields()` — add required fields to the Registry
2. **Initialization**: `initialize()` — set initial conditions
3. **Step provision**: `splitting_steps()` — return a list of sub-steps for OperatorSplitting

```rust
let ehd = EhdModule::new(config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

for step in ehd.splitting_steps() {
    splitting.add_step(step);
}
```

### Future multi-physics coupling

```rust
let ehd = EhdModule::new(ehd_config);
let thermal = ThermalModule::new(thermal_config);

ehd.register_fields(&mut state.fields, &mesh);
thermal.register_fields(&mut state.fields, &mesh);

// Step ordering determines coupling strategy
for step in ehd.splitting_steps() { splitting.add_step(step); }
for step in thermal.splitting_steps() { splitting.add_step(step); }
```

Inter-module communication occurs solely through `FieldRegistry`. No direct references.

## Time Stepping and Operator Splitting


operator splitting method is used to solve the coupled equation system sequentially. `OperatorSplitting` holds `Vec<Box<dyn SplittingStep>>` and executes all steps in order each timestep. `SimulationDriver` controls the outermost time loop: initial state output, dt computation, step execution, periodic output, and termination checks.

```rust
// SimulationDriver::run() outline
writer.write_frame(mesh, state, 0)?;           // initial state
for step_num in 1..=max_steps {
    state.dt = fixed_dt.unwrap_or(splitting.compute_dt(mesh, state));
    splitting.advance(mesh, state)?;            // execute all sub-steps
    state.time += state.dt;
    if step_num % output_interval == 0 {
        writer.write_frame(mesh, state, step_num)?;
    }
    if state.time >= max_time { break; }
}
```

operator splitting method is used to solve the coupled equation system sequentially.

### SplittingStep trait

Each sub-step implements the following trait:

```rust
pub trait SplittingStep: Send {
    fn name(&self) -> &str;
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<()>;
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}
```

`max_dt()` reports each step's stability constraint. Elliptic steps (Poisson) return `f64::INFINITY`.

### EHD splitting order

```
1. PoissonStep     : ρ_q → φ, E (elliptic, CG solve)
2. IonTransportStep: update n_i using E, u (SG scheme)
3. EhdForceStep    : ρ_q × E → f_EHD
4. FluidStep       : update u, p using f_EHD (pressure projection)
```

### Time step control

Global dt is determined as `CFL × min(step.max_dt())` across all steps. Fixed dt is also supported.

## Performance Design


Performance is considered from the design stage. Optimization follows three axes.

### Data layout

- All fields are contiguous `Vec<f64>` / `Vec<[f64; 3]>` arrays
- Mesh connectivity uses CSR compression (`Vec<u32>` offsets + `Vec<Id>` indices)
- `HashMap` stays outside hot loops (field lookup is once per timestep per field)

### Allocation avoidance

- `LinearSystem` sparsity pattern is built once; only values are overwritten each step
- CG solver scratch buffers are pre-allocated on the `SplittingStep` struct
- Face flux temporary buffers are reused

### Parallelism readiness

- Face and cell loops have rayon-parallelizable structure
- `[f64; 3]` vectors support auto-vectorization
- `SplittingStep: Send` guarantees thread safety

## Dependency Graph


Inter-crate dependencies are strictly unidirectional. No circular dependencies exist.

```
cfd-core (leaf: thiserror only)
  ↑
cfd-mesh ← cfd-fields ← cfd-linalg (sprs)
  ↑                        ↑
cfd-compute ← cfd-compute-cpu
  ↑
cfd-fvm ← cfd-time ← cfd-io (vtkio, toml, serde)
  ↑
ehd-physics
  ↑
ehd-cli (clap, tracing-subscriber)
```

`cfd-compute` does not directly depend on `cfd-mesh`. Instead, it defines a lightweight `MeshData` struct in its own internal module (`cfd_core_mesh_data`). This prevents backend crates from needing knowledge of the full mesh implementation.

Inter-crate dependencies are strictly unidirectional. No circular dependencies exist.

```
cfd-core (leaf)
  ↑
cfd-mesh ← cfd-fields ← cfd-linalg
  ↑                        ↑
cfd-compute ← cfd-compute-cpu
  ↑
cfd-fvm ← cfd-time ← cfd-io
  ↑
ehd-physics
  ↑
ehd-cli
```

### External dependencies

| Purpose | Crate | Rationale |
|---|---|---|
| Sparse matrices | `sprs` | Mature, pure Rust, CSR/COO support |
| Gmsh import | Custom parser | MSH 2.2 ASCII, minimal dependencies |
| VTU output | `vtkio` | XML VTU unstructured grid support |
| Config | `toml` + `serde` | Standard Rust approach |
| CLI | `clap` | Derive macro support |
| Logging | `tracing` | Structured logging with timing spans |
