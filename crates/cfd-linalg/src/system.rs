//! Linear system representation: Ax = b.

use sprs::CsMat;

/// A sparse linear system Ax = b.
///
/// The matrix is stored in CSR format. The `solution` vector is pre-allocated
/// and reused across timesteps to avoid allocations in hot loops.
pub struct LinearSystem {
    /// Coefficient matrix (CSR format).
    pub matrix: CsMat<f64>,
    /// Right-hand side vector.
    pub rhs: Vec<f64>,
    /// Solution vector (pre-allocated, filled by solver).
    pub solution: Vec<f64>,
}

impl LinearSystem {
    /// Create a new system of the given size with zero matrix and vectors.
    pub fn new(n: usize) -> Self {
        Self {
            matrix: CsMat::zero((n, n)),
            rhs: vec![0.0; n],
            solution: vec![0.0; n],
        }
    }

    /// System dimension.
    pub fn size(&self) -> usize {
        self.rhs.len()
    }

    /// Reset RHS and solution to zero (keeps matrix sparsity pattern).
    pub fn reset_vectors(&mut self) {
        self.rhs.fill(0.0);
        self.solution.fill(0.0);
    }
}
