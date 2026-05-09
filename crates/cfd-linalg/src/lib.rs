//! `cfd-linalg` — Sparse linear algebra for the CFD framework.
//!
//! Provides CSR sparse matrices (via `sprs`), iterative solvers (CG, BiCGSTAB),
//! and preconditioners (Jacobi). The `LinearSolver` trait allows backend swaps.

pub mod cg;
pub mod precond;
pub mod system;

pub use cg::ConjugateGradient;
pub use precond::{JacobiPreconditioner, Preconditioner};
pub use system::LinearSystem;

/// Statistics from a linear solve.
#[derive(Clone, Debug)]
pub struct SolveStats {
    pub iterations: usize,
    pub residual: f64,
    pub converged: bool,
}

/// Trait for pluggable linear solver backends.
pub trait LinearSolver {
    fn solve(&mut self, system: &mut LinearSystem) -> Result<SolveStats, cfd_core::CfdError>;
}
