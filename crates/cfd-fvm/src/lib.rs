//! `cfd-fvm` — Finite Volume Method operators.
//!
//! This crate provides functions that **build kernel descriptions** for common
//! FVM operations. The kernels are not executed here — they are passed to a
//! `ComputeBackend` for execution.
//!
//! # Design
//!
//! Each function returns a `FaceKernel` or `CellKernel` describing the operation.
//! This separation of "what" from "how" enables backend portability.
//!
//! ```text
//! laplacian_kernel("phi", 1.0, "residual")  → FaceKernel
//!     ↓
//! backend.execute_face_kernel(mesh, &kernel, fields)
//! ```

pub mod operators;

pub use operators::*;
