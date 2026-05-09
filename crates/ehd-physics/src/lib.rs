//! `ehd-physics` — Electrohydrodynamic physics module.
//!
//! Implements the EHD simulation as a `PhysicsModule`:
//! - Poisson equation for electric potential
//! - Ion transport with Scharfetter-Gummel scheme
//! - EHD body force computation
//! - Coupling to incompressible Navier-Stokes
//!
//! # Equations
//!
//! ```text
//! ∇·(ε∇φ) = -ρ_q                           (Poisson)
//! E = -∇φ                                   (Electric field)
//! ∂n_i/∂t + ∇·(μ_i n_i E - D_i ∇n_i) = 0   (Ion transport)
//! ρ_q = q · n_i                              (Charge density)
//! f_EHD = ρ_q · E                            (EHD body force)
//! ```

use cfd_core::FieldLocation;
use cfd_fields::{FieldRegistry, SimState};
use cfd_mesh::Mesh;
use cfd_time::SplittingStep;

/// Configuration for the EHD physics module.
#[derive(Clone, Debug)]
pub struct EhdConfig {
    /// Permittivity of free space (F/m).
    pub permittivity: f64,
    /// Ion mobility (m²/V·s).
    pub ion_mobility: f64,
    /// Ion diffusion coefficient (m²/s).
    pub ion_diffusion: f64,
    /// Gas density (kg/m³).
    pub gas_density: f64,
    /// Gas dynamic viscosity (Pa·s).
    pub gas_viscosity: f64,
    /// Elementary charge (C).
    pub elementary_charge: f64,
}

impl Default for EhdConfig {
    fn default() -> Self {
        Self {
            permittivity: 8.8541878128e-12,
            ion_mobility: 2.0e-4,
            ion_diffusion: 5.0e-6,
            gas_density: 1.225,
            gas_viscosity: 1.81e-5,
            elementary_charge: 1.602176634e-19,
        }
    }
}

/// EHD physics module.
pub struct EhdModule {
    pub config: EhdConfig,
}

impl EhdModule {
    pub fn new(config: EhdConfig) -> Self {
        Self { config }
    }

    /// Register all fields needed by the EHD module.
    pub fn register_fields(&self, registry: &mut FieldRegistry, mesh: &Mesh) {
        let nc = mesh.n_cells;

        // Electric field
        registry.register_scalar("phi", nc, FieldLocation::Cell);
        registry.register_vector("electric_field", nc, FieldLocation::Cell);

        // Ion transport
        registry.register_scalar("ion_density", nc, FieldLocation::Cell);
        registry.register_scalar("charge_density", nc, FieldLocation::Cell);

        // EHD coupling
        registry.register_vector("ehd_force", nc, FieldLocation::Cell);

        // Fluid
        registry.register_vector("velocity", nc, FieldLocation::Cell);
        registry.register_scalar("pressure", nc, FieldLocation::Cell);

        // Scratch fields for solver
        registry.register_scalar("poisson_rhs", nc, FieldLocation::Cell);
        registry.register_scalar("ion_rhs", nc, FieldLocation::Cell);

        tracing::info!(n_cells = nc, "EHD fields registered");
    }

    /// Set initial conditions.
    pub fn initialize(&self, _state: &mut SimState, _mesh: &Mesh) {
        // Initial conditions are all zero (fields initialized to 0 by default).
        // Boundary conditions drive the solution.
        tracing::info!("EHD module initialized");
    }

    /// Return the ordered splitting steps for operator splitting.
    ///
    /// The order matters:
    /// 1. Poisson (compute φ, E from ρ_q)
    /// 2. Ion transport (advance n_i using E and u)
    /// 3. EHD force (compute f_EHD = ρ_q · E)
    /// 4. Fluid (advance u, p using f_EHD)
    pub fn splitting_steps(&self) -> Vec<Box<dyn SplittingStep>> {
        // For now, return an empty set — individual steps will be implemented
        // in Phase 1-3. This skeleton establishes the module structure.
        tracing::info!("EHD splitting steps created (skeleton)");
        Vec::new()
    }

    /// Field names to include in output.
    pub fn output_fields(&self) -> (Vec<String>, Vec<String>) {
        let scalars = vec![
            "phi".to_string(),
            "ion_density".to_string(),
            "charge_density".to_string(),
            "pressure".to_string(),
        ];
        let vectors = vec![
            "electric_field".to_string(),
            "velocity".to_string(),
            "ehd_force".to_string(),
        ];
        (scalars, vectors)
    }
}
