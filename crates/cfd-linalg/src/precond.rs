//! Preconditioners for iterative solvers.

use sprs::CsMat;

/// Preconditioner trait: transforms residual r into preconditioned direction z.
pub trait Preconditioner {
    /// Apply preconditioner: z = M^{-1} r
    fn apply(&self, r: &[f64], z: &mut [f64]);
}

/// Jacobi (diagonal) preconditioner.
///
/// M = diag(A), z_i = r_i / A_{ii}
pub struct JacobiPreconditioner {
    diagonal_inv: Vec<f64>,
}

impl JacobiPreconditioner {
    /// Build from a CSR matrix. Extracts diagonal and inverts.
    pub fn from_matrix(matrix: &CsMat<f64>) -> Self {
        let n = matrix.rows();
        let mut diagonal_inv = vec![1.0; n];

        for i in 0..n {
            if let Some(&val) = matrix.get(i, i) {
                if val.abs() > 1e-30 {
                    diagonal_inv[i] = 1.0 / val;
                }
            }
        }

        Self { diagonal_inv }
    }
}

impl Preconditioner for JacobiPreconditioner {
    fn apply(&self, r: &[f64], z: &mut [f64]) {
        for i in 0..r.len() {
            z[i] = r[i] * self.diagonal_inv[i];
        }
    }
}

/// Identity preconditioner (no preconditioning).
pub struct NoPreconditioner;

impl Preconditioner for NoPreconditioner {
    fn apply(&self, r: &[f64], z: &mut [f64]) {
        z.copy_from_slice(r);
    }
}
