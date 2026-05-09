//! `cfd-compute` — Compute backend abstraction layer.
//!
//! This crate defines the interface between "what to compute" (FVM operators)
//! and "how to execute" (CPU loops, JIT, GPU). The key types are:
//!
//! - [`FaceKernel`] / [`CellKernel`]: Data descriptions of compute operations.
//! - [`ComputeBackend`]: Trait that backends implement to execute kernels.
//!
//! The kernel descriptions are **data, not code** — they can be inspected,
//! optimized, and compiled to different targets (Cranelift, WGPU, etc.).
//!
//! # Architecture
//!
//! ```text
//! cfd-fvm operators → build FaceKernel/CellKernel descriptions
//!                          │
//!                          ▼
//!                   ComputeBackend::execute()
//!                          │
//!              ┌───────────┼───────────┐
//!              ▼           ▼           ▼
//!          CpuBackend  CraneliftJIT  WgpuBackend
//!          (Phase 0)   (future)      (future)
//! ```

pub mod backend;
pub mod kernel;

pub use backend::ComputeBackend;
pub use kernel::{
    AdvectionScheme, CellKernel, CellOp, FaceKernel, FaceOp, FieldRef, Limiter, ParamRef,
};
