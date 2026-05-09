//! Common error types for the CFD framework.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CfdError {
    #[error("Mesh error: {0}")]
    Mesh(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Solver did not converge: {message} (residual={residual:.2e}, iterations={iterations})")]
    SolverNotConverged {
        message: String,
        residual: f64,
        iterations: usize,
    },

    #[error("Field '{name}' not found in registry")]
    FieldNotFound { name: String },

    #[error("Field dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Boundary condition error: {0}")]
    Boundary(String),

    #[error("{0}")]
    Other(String),
}
