//! `cfd-mesh` — Mesh data structures and I/O for unstructured FVM meshes.
//!
//! The [`Mesh`] struct stores face-based FVM topology with contiguous arrays
//! (data-oriented layout). Internal faces come first, boundary faces after,
//! enabling branch-free inner loops over internal faces.
//!
//! # Mesh Layout Convention
//!
//! ```text
//! Face indices: [0 .. n_internal_faces) [n_internal_faces .. n_total_faces)
//!                ├── internal faces ──┤ ├── boundary faces ──────────────┤
//!                                       ├─ patch 0 ─┤ ├─ patch 1 ─┤ ...
//! ```
//!
//! This matches the OpenFOAM convention and ensures the hottest loops
//! (internal face flux computation) operate on contiguous memory.

pub mod geometry;
pub mod gmsh;
pub mod topology;

use cfd_compute::backend::cfd_core_mesh_data::{BoundaryPatchData, MeshData};
use cfd_core::{CellId, FaceId, NodeId, Vec3};

/// Cell element type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    /// 2D triangle (3 nodes, 3 faces).
    Triangle,
    /// 2D quadrilateral (4 nodes, 4 faces).
    Quad,
    /// 3D tetrahedron (4 nodes, 4 faces).
    Tetrahedron,
    /// 3D hexahedron (8 nodes, 6 faces).
    Hexahedron,
    /// 3D wedge/prism (6 nodes, 5 faces).
    Wedge,
    /// 3D pyramid (5 nodes, 5 faces).
    Pyramid,
}

/// A named boundary patch (group of boundary faces).
#[derive(Clone, Debug)]
pub struct BoundaryPatch {
    /// Name from Gmsh physical group.
    pub name: String,
    /// Start index in the boundary face region (offset from `n_internal_faces`).
    pub start_face: usize,
    /// Number of faces in this patch.
    pub n_faces: usize,
}

impl BoundaryPatch {
    /// Iterator over the global face indices belonging to this patch.
    pub fn face_range(&self, n_internal_faces: usize) -> std::ops::Range<usize> {
        let start = n_internal_faces + self.start_face;
        start..start + self.n_faces
    }
}

/// The core FVM mesh.
///
/// All arrays are contiguous, indexed by `CellId` / `FaceId` / `NodeId`.
/// No per-cell heap allocations. This is the data-oriented layout for
/// cache-friendly, SIMD-ready access patterns.
#[derive(Clone, Debug)]
pub struct Mesh {
    // --- Dimensions ---
    pub n_nodes: usize,
    pub n_cells: usize,
    pub n_faces: usize,
    /// Number of internal (inter-cell) faces. These occupy indices `[0..n_internal_faces)`.
    pub n_internal_faces: usize,

    // --- Node data ---
    pub node_coords: Vec<Vec3>,

    // --- Cell data ---
    pub cell_volumes: Vec<f64>,
    pub cell_centroids: Vec<Vec3>,
    pub cell_types: Vec<CellType>,

    // Cell-to-face connectivity (CSR compressed)
    pub cell_face_offsets: Vec<u32>,
    pub cell_face_indices: Vec<FaceId>,

    // Cell-to-node connectivity (CSR compressed, for output)
    pub cell_node_offsets: Vec<u32>,
    pub cell_node_indices: Vec<NodeId>,

    // --- Face data ---
    pub face_areas: Vec<f64>,
    /// Outward-pointing face normal vectors (unit normal × area = area vector).
    /// Direction: from owner toward neighbor for internal faces.
    pub face_normals: Vec<Vec3>,
    pub face_centroids: Vec<Vec3>,

    /// Owner cell for each face (always defined).
    pub face_owner: Vec<CellId>,
    /// Neighbor cell for internal faces only (length = `n_internal_faces`).
    pub face_neighbor: Vec<CellId>,

    // Face-to-node connectivity (CSR compressed)
    pub face_node_offsets: Vec<u32>,
    pub face_node_indices: Vec<NodeId>,

    // --- Boundary ---
    pub boundary_patches: Vec<BoundaryPatch>,

    // --- Precomputed geometry ---
    /// Delta vector from owner centroid to neighbor centroid (internal faces only).
    pub face_delta: Vec<Vec3>,
    /// |face_delta| for internal faces.
    pub face_delta_mag: Vec<f64>,
    /// Interpolation weight for face value: w such that
    /// φ_f ≈ w·φ_owner + (1-w)·φ_neighbor.
    pub face_weight: Vec<f64>,
}

impl Mesh {
    /// Get the face IDs for a given cell (CSR lookup).
    #[inline]
    pub fn cell_faces(&self, cell: CellId) -> &[FaceId] {
        let start = self.cell_face_offsets[cell.as_usize()] as usize;
        let end = self.cell_face_offsets[cell.as_usize() + 1] as usize;
        &self.cell_face_indices[start..end]
    }

    /// Get the node IDs for a given cell (CSR lookup).
    #[inline]
    pub fn cell_nodes(&self, cell: CellId) -> &[NodeId] {
        let start = self.cell_node_offsets[cell.as_usize()] as usize;
        let end = self.cell_node_offsets[cell.as_usize() + 1] as usize;
        &self.cell_node_indices[start..end]
    }

    /// Get the node IDs for a given face (CSR lookup).
    #[inline]
    pub fn face_nodes(&self, face: FaceId) -> &[NodeId] {
        let start = self.face_node_offsets[face.as_usize()] as usize;
        let end = self.face_node_offsets[face.as_usize() + 1] as usize;
        &self.face_node_indices[start..end]
    }

    /// Whether a face is on the boundary.
    #[inline]
    pub fn is_boundary_face(&self, face: FaceId) -> bool {
        face.as_usize() >= self.n_internal_faces
    }

    /// Number of boundary faces.
    pub fn n_boundary_faces(&self) -> usize {
        self.n_faces - self.n_internal_faces
    }

    /// Find a boundary patch by name.
    pub fn find_patch(&self, name: &str) -> Option<&BoundaryPatch> {
        self.boundary_patches.iter().find(|p| p.name == name)
    }

    /// Convert to backend-consumable `MeshData`.
    pub fn to_mesh_data(&self) -> MeshData {
        MeshData {
            n_cells: self.n_cells,
            n_internal_faces: self.n_internal_faces,
            n_boundary_faces: self.n_boundary_faces(),
            face_areas: self.face_areas.clone(),
            face_normals: self.face_normals.clone(),
            face_centroids: self.face_centroids.clone(),
            face_owner: self.face_owner.iter().map(|id| id.0).collect(),
            face_neighbor: self.face_neighbor.iter().map(|id| id.0).collect(),
            cell_volumes: self.cell_volumes.clone(),
            cell_centroids: self.cell_centroids.clone(),
            face_delta: self.face_delta.clone(),
            face_weights: self.face_weight.clone(),
            boundary_patches: self
                .boundary_patches
                .iter()
                .map(|p| BoundaryPatchData {
                    name: p.name.clone(),
                    start_face: p.start_face,
                    n_faces: p.n_faces,
                })
                .collect(),
        }
    }
}
