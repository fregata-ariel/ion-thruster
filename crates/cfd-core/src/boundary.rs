//! Boundary condition types.
//!
//! These describe the mathematical condition applied at boundary faces.
//! Physics modules can define additional conditions via the `Custom` variant.

/// Mathematical boundary condition specification.
#[derive(Clone, Debug)]
pub enum BoundaryCondition {
    /// Fixed value: φ = value.
    Dirichlet(f64),
    /// Fixed gradient (flux): ∂φ/∂n = value.
    Neumann(f64),
    /// Zero-gradient (homogeneous Neumann): ∂φ/∂n = 0.
    ZeroGradient,
    /// No-slip wall: u = 0.
    NoSlip,
    /// Outflow / convective outlet.
    Outflow,
    /// Absorbing boundary: φ = 0 (used for ion density at collector).
    Absorbing,
    /// Fixed flux injection (particles/m²/s).
    FixedFlux(f64),
    /// Custom boundary condition with a string identifier.
    /// Physics modules can use this to define domain-specific BCs.
    Custom {
        kind: String,
        params: Vec<f64>,
    },
}

impl BoundaryCondition {
    /// Whether this BC fixes the value (Dirichlet-type).
    pub fn is_dirichlet(&self) -> bool {
        matches!(
            self,
            Self::Dirichlet(_) | Self::NoSlip | Self::Absorbing
        )
    }

    /// Whether this BC specifies a flux/gradient (Neumann-type).
    pub fn is_neumann(&self) -> bool {
        matches!(
            self,
            Self::Neumann(_) | Self::ZeroGradient | Self::Outflow | Self::FixedFlux(_)
        )
    }
}
