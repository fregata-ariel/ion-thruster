//! Preconditioned Conjugate Gradient solver for symmetric positive-definite systems.

use sprs::CsMat;

use crate::precond::{JacobiPreconditioner, Preconditioner};
use crate::system::LinearSystem;
use crate::{LinearSolver, SolveStats};
use cfd_core::CfdError;

/// Preconditioned Conjugate Gradient solver.
pub struct ConjugateGradient {
    pub tol: f64,
    pub max_iter: usize,
    // Scratch buffers (pre-allocated to avoid per-solve allocations)
    r: Vec<f64>,
    z: Vec<f64>,
    p: Vec<f64>,
    ap: Vec<f64>,
}

impl ConjugateGradient {
    pub fn new(tol: f64, max_iter: usize) -> Self {
        Self {
            tol,
            max_iter,
            r: Vec::new(),
            z: Vec::new(),
            p: Vec::new(),
            ap: Vec::new(),
        }
    }

    /// Ensure scratch buffers are sized correctly.
    fn ensure_buffers(&mut self, n: usize) {
        if self.r.len() != n {
            self.r = vec![0.0; n];
            self.z = vec![0.0; n];
            self.p = vec![0.0; n];
            self.ap = vec![0.0; n];
        }
    }
}

impl LinearSolver for ConjugateGradient {
    fn solve(&mut self, system: &mut LinearSystem) -> Result<SolveStats, CfdError> {
        let n = system.size();
        self.ensure_buffers(n);

        let precond = JacobiPreconditioner::from_matrix(&system.matrix);

        // r = b - A*x
        spmv(&system.matrix, &system.solution, &mut self.r);
        for i in 0..n {
            self.r[i] = system.rhs[i] - self.r[i];
        }

        // z = M^{-1} r
        precond.apply(&self.r, &mut self.z);

        // p = z
        self.p.copy_from_slice(&self.z);

        let mut rz = dot(&self.r, &self.z);
        let b_norm = norm2(&system.rhs).max(1e-30);

        for iter in 0..self.max_iter {
            let r_norm = norm2(&self.r);
            if r_norm / b_norm < self.tol {
                return Ok(SolveStats {
                    iterations: iter,
                    residual: r_norm / b_norm,
                    converged: true,
                });
            }

            // ap = A * p
            spmv(&system.matrix, &self.p, &mut self.ap);

            let pap = dot(&self.p, &self.ap);
            if pap.abs() < 1e-30 {
                return Ok(SolveStats {
                    iterations: iter,
                    residual: r_norm / b_norm,
                    converged: true,
                });
            }

            let alpha = rz / pap;

            // x += alpha * p
            // r -= alpha * ap
            for i in 0..n {
                system.solution[i] += alpha * self.p[i];
                self.r[i] -= alpha * self.ap[i];
            }

            // z = M^{-1} r
            precond.apply(&self.r, &mut self.z);

            let rz_new = dot(&self.r, &self.z);
            let beta = rz_new / rz.max(1e-30);
            rz = rz_new;

            // p = z + beta * p
            for i in 0..n {
                self.p[i] = self.z[i] + beta * self.p[i];
            }
        }

        let final_residual = norm2(&self.r) / b_norm;
        Err(CfdError::SolverNotConverged {
            message: "CG did not converge".to_string(),
            residual: final_residual,
            iterations: self.max_iter,
        })
    }
}

/// Sparse matrix-vector multiply: y = A * x (CSR).
fn spmv(a: &CsMat<f64>, x: &[f64], y: &mut [f64]) {
    y.fill(0.0);
    for (row_i, row_vec) in a.outer_iterator().enumerate() {
        let mut sum = 0.0;
        for (col_j, &val) in row_vec.iter() {
            sum += val * x[col_j];
        }
        y[row_i] = sum;
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn norm2(v: &[f64]) -> f64 {
    dot(v, v).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sprs::TriMat;

    #[test]
    fn test_cg_simple_2x2() {
        // Solve: [4 1; 1 3] x = [1; 2]
        // Solution: x = [1/11; 7/11]
        let mut triplets = TriMat::new((2, 2));
        triplets.add_triplet(0, 0, 4.0);
        triplets.add_triplet(0, 1, 1.0);
        triplets.add_triplet(1, 0, 1.0);
        triplets.add_triplet(1, 1, 3.0);

        let mut system = LinearSystem {
            matrix: triplets.to_csr(),
            rhs: vec![1.0, 2.0],
            solution: vec![0.0, 0.0],
        };

        let mut cg = ConjugateGradient::new(1e-10, 100);
        let stats = cg.solve(&mut system).unwrap();

        assert!(stats.converged);
        assert!((system.solution[0] - 1.0 / 11.0).abs() < 1e-8);
        assert!((system.solution[1] - 7.0 / 11.0).abs() < 1e-8);
    }
}
