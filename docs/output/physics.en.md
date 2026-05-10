# Physics Model Reference


Physics model reference for the ion-craft CFD framework. Describes the governing equations, discretization methods, boundary conditions, and coupling strategy used in EHD (electrohydrodynamic) simulation.

## Governing Equations


The EHD simulation consists of four coupled equation systems:

1. **Poisson equation** — determines the electric potential distribution from space charge density
2. **drift-diffusion equation** — ion transport via electric field drift and diffusion
3. **EHD body force** — product of space charge density and electric field
4. **Navier-Stokes equations** — incompressible flow with EHD body force as an external force

The physical causal chain is:

$$
\text{corona discharge} \xrightarrow{\text{ion generation}} n_i
\xrightarrow{q \cdot n_i} \rho_q
\xrightarrow{\nabla \cdot (\varepsilon \nabla \phi) = -\rho_q} \phi
\xrightarrow{-\nabla \phi} \mathbf{E}
\xrightarrow{\rho_q \mathbf{E}} \mathbf{f}_{\text{EHD}}
\xrightarrow{\text{NS eqs.}} \mathbf{u}
$$

ionic wind is the macroscopic gas flow produced when corona-generated ions transfer momentum to neutral molecules.

## Poisson Equation


The electric potential $\phi$ is determined by the Poisson equation:

$$
\nabla \cdot (\varepsilon_0 \nabla \phi) = -\rho_q
$$

where $\varepsilon_0$ = 8.8541878128×10⁻¹² F/m (permittivity of free space), and space charge density $\rho_q = q \cdot n_i$ ($q$ = 1.602176634×10⁻¹⁹ C, elementary charge).

In the Finite Volume Method discretization, for cell $P$:

$$
\sum_f \varepsilon_0 A_f \frac{\phi_N - \phi_P}{|\mathbf{d}|} = -\rho_{q,P} V_P
$$

where $A_f$ is the face area, $|\mathbf{d}|$ is the owner-neighbor centroid distance, and $V_P$ is the cell volume. This discretization is described by `laplacian_kernel()` in `cfd-fvm`:

```rust
let kernel = laplacian_kernel("phi", epsilon_0, "poisson_rhs");
```

The discretized system yields a symmetric positive-definite sparse matrix system $A\phi = b$, solved by the Jacobi-preconditioned Conjugate Gradient method.

The electric field is computed as the negative gradient of the potential:

$$
\mathbf{E} = -\nabla \phi
$$

Discretely, the Green-Gauss theorem is applied at cell centers, approximating the gradient from weighted face-value sums.

## Ion Transport


The time evolution of ion density $n_i$ is described by the drift-diffusion equation:

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot (\mu_i n_i \mathbf{E} - D_i \nabla n_i) = 0
$$

where $\mu_i$ = 2.0×10⁻⁴ m²/(V·s) (ion mobility), and $D_i$ = 5.0×10⁻⁶ m²/s (ion diffusion coefficient).

The first term $\mu_i n_i \mathbf{E}$ represents drift (transport by the electric field), and the second term $D_i \nabla n_i$ represents diffusion. Because these two effects compete, standard central or upwind differencing produces either numerical oscillation or excessive numerical diffusion. The Scharfetter-Gummel scheme resolves this problem.

## Scharfetter-Gummel Scheme


The Scharfetter-Gummel scheme applies exponential weighting based on the local face Peclet number, stably discretizing the competing drift and diffusion terms.

The face Peclet number at face $f$ (owner $O$, neighbor $N$):

$$
\text{Pe}_f = \frac{\mu_i \mathbf{E}_f \cdot \mathbf{d}}{D_i}
$$

where $\mathbf{d}$ is the owner→neighbor centroid vector. The face flux uses the Bernoulli function $B(x) = x/(e^x - 1)$:

$$
J_f = \frac{D_i A_f}{|\mathbf{d}|} \left[ B(\text{Pe}_f) \, n_O - B(-\text{Pe}_f) \, n_N \right]
$$

Numerical stability of the Bernoulli function is ensured across three regions:

| $|x|$ range | Approximation | Purpose |
|---|---|---|
| $|x| < 10^{-6}$ | Taylor expansion: $B(x) \approx 1 - x/2 + x^2/12$ | Avoid cancellation |
| $x > 500$ | $B(x) \approx 0$ | Avoid overflow |
| $x < -500$ | $B(x) \approx -x$ | Avoid overflow |
| otherwise | $B(x) = x/(e^x - 1)$ | Direct computation |

```rust
fn bernoulli(x: f64) -> f64 {
    if x.abs() < 1e-6 {
        1.0 - x * 0.5 + x * x / 12.0    // Taylor
    } else if x > 500.0 {
        0.0                               // exp(x) >> x
    } else if x < -500.0 {
        -x                                // B(x) ≈ -x for x << 0
    } else {
        x / (x.exp() - 1.0)              // exact
    }
}
```

The kernel is built in `cfd-fvm`:

```rust
let kernel = scharfetter_gummel_kernel(
    "ion_density",      // concentration field
    "electric_field",   // E field
    2.0e-4,             // ion mobility μ_i
    5.0e-6,             // diffusion D_i
    "ion_rhs",          // target accumulator
);
```

## EHD Body Force


The EHD body force is the product of space charge density and electric field:

$$
\mathbf{f}_{\text{EHD}} = \rho_q \mathbf{E}
$$

where $\rho_q = q \cdot n_i$. This body force drives the ionic wind as an external force term in the Navier-Stokes equations.

Discretely, it is computed as a per-cell element-wise product. `CellOp::Multiply` multiplies $\rho_q$ with each component of $\mathbf{E}$.

## Incompressible Navier-Stokes


The incompressible Navier-Stokes equations with EHD body force as an external force:

$$
\rho_g \frac{\partial \mathbf{u}}{\partial t} + \rho_g (\mathbf{u} \cdot \nabla)\mathbf{u} = -\nabla p + \mu_g \nabla^2 \mathbf{u} + \mathbf{f}_{\text{EHD}}
$$

$$
\nabla \cdot \mathbf{u} = 0
$$

where $\rho_g$ = 1.225 kg/m³ (gas density), $\mu_g$ = 1.81×10⁻⁵ Pa·s (gas dynamic viscosity) (standard atmospheric conditions).

Solved using the pressure projection method:

1. **Predictor step**: compute tentative velocity $\mathbf{u}^*$ from the momentum equation without pressure
2. **Pressure Poisson equation**: solve $\nabla^2 p = \frac{\rho_g}{\Delta t} \nabla \cdot \mathbf{u}^*$
3. **Corrector step**: $\mathbf{u}^{n+1} = \mathbf{u}^* - \frac{\Delta t}{\rho_g} \nabla p$ to enforce divergence-free velocity

Advection discretization uses upwind differencing (`AdvectionScheme::Upwind`). TVD limiters (Van Leer, MinMod, Superbee) are planned for future addition.

The viscous term is discretized with the Laplacian kernel:

```rust
let viscous = laplacian_kernel("velocity_x", mu_g, "momentum_rhs_x");
let advect  = advection_kernel("velocity_x", "velocity", AdvectionScheme::Upwind, "momentum_rhs_x");
```

## Boundary Conditions


Boundary conditions are defined by the `BoundaryCondition` enum and specified per boundary patch in the TOML file. Typical 3-patch configuration for EHD simulation:

### Emitter (discharge electrode)

| Variable | Condition | Value |
|---|---|---|
| $\phi$ | Dirichlet | 20,000 V |
| $n_i$ | Dirichlet | $1.0 \times 10^{15}$ m⁻³ |
| $\mathbf{u}$ | No-slip | $\mathbf{0}$ |

### Collector (collection electrode)

| Variable | Condition | Value |
|---|---|---|
| $\phi$ | Dirichlet | 0 V |
| $n_i$ | Absorbing | — |
| $\mathbf{u}$ | No-slip | $\mathbf{0}$ |

### Farfield (far-field boundary)

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

## Operator Splitting Coupling


The coupled equation system is solved using operator splitting method. Four sub-steps are executed sequentially each timestep:

```text
1. PoissonStep     : ρ_q → φ, E (elliptic, CG solve)
2. IonTransportStep: update n_i using E, u (SG scheme, explicit time integration)
3. EhdForceStep    : ρ_q × E → f_EHD (algebraic, dt-independent)
4. FluidStep       : update u, p using f_EHD (pressure projection)
```

The ordering matters:

- PoissonStep is elliptic and immediately determines $\phi, \mathbf{E}$ from $\rho_q$. Solving it first provides up-to-date electric field for subsequent steps.
- IonTransportStep performs drift-diffusion time evolution, requiring the latest $\mathbf{E}$ and $\mathbf{u}$.
- EhdForceStep is purely algebraic ($\mathbf{f} = \rho_q \mathbf{E}$) with no stability constraint.
- FluidStep runs last, correcting the velocity field with the updated body force.

The timestep $\Delta t$ is determined as the CFL coefficient times the minimum `max_dt()` across all steps:

$$
\Delta t = C_{\text{CFL}} \cdot \min_s \Delta t_s^{\max}
$$

Elliptic steps (Poisson) return `f64::INFINITY` (unconditionally stable). The IonTransportStep CFL constraint based on drift velocity is typically the most restrictive.

## Physical Constants and Parameters


Physical constants and parameters used in EHD simulation:

### Physical Constants

| Symbol | Name | Value | Unit |
|---|---|---|---|
| $\varepsilon_0$ | permittivity of free space | 8.8541878128×10⁻¹² | F/m |
| $q$ | elementary charge | 1.602176634×10⁻¹⁹ | C |

### Gas Parameters (standard atmospheric conditions)

| Symbol | Name | Typical Value | Unit |
|---|---|---|---|
| $\rho_g$ | gas density | 1.225 | kg/m³ |
| $\mu_g$ | gas dynamic viscosity | 1.81×10⁻⁵ | Pa·s |

### Ion Transport Parameters

| Symbol | Name | Typical Value | Unit |
|---|---|---|---|
| $\mu_i$ | ion mobility | 2.0×10⁻⁴ | m²/(V·s) |
| $D_i$ | ion diffusion coefficient | 5.0×10⁻⁶ | m²/s |

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
