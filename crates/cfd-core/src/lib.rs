//! `cfd-core` — Shared foundational types for the CFD framework.
//!
//! This is the leaf crate with no dependencies on other `cfd-*` crates.
//! It provides index newtypes, vector math, boundary condition types,
//! field location descriptors, and common error types.

pub mod boundary;
pub mod error;
pub mod index;
pub mod vec3;

pub use boundary::BoundaryCondition;
pub use error::CfdError;
pub use index::{CellId, FaceId, NodeId};
pub use vec3::Vec3;

/// Where a field quantity is stored on the mesh.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FieldLocation {
    /// Cell-centered (finite volume primary location).
    Cell,
    /// Face-centered (flux quantities).
    Face,
    /// Node / vertex (for interpolation or output).
    Node,
}
