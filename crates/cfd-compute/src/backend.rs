//! Compute backend trait — the abstraction boundary between
//! "what to compute" and "how to execute".

use cfd_core::CfdError;

use crate::kernel::{CellKernel, FaceKernel};

/// Opaque handle to mesh data prepared for a specific backend.
///
/// Each backend can store its own optimized representation of the mesh
/// (e.g., GPU buffers, pre-computed connectivity, SIMD-aligned arrays).
/// The framework passes this handle to the backend during kernel execution.
pub trait MeshHandle: Send + Sync {}

/// Opaque handle to field data prepared for a specific backend.
///
/// Allows backends to manage field storage in their own way
/// (e.g., GPU device memory, pinned host memory).
pub trait FieldStore: Send + Sync {
    /// Get a scalar field's data as a slice (for CPU-side access).
    fn scalar_data(&self, name: &str) -> Result<&[f64], CfdError>;
    /// Get a mutable scalar field's data.
    fn scalar_data_mut(&mut self, name: &str) -> Result<&mut [f64], CfdError>;
    /// Get a vector field's data as a slice of [f64; 3].
    fn vector_data(&self, name: &str) -> Result<&[[f64; 3]], CfdError>;
    /// Get a mutable vector field's data.
    fn vector_data_mut(&mut self, name: &str) -> Result<&mut [[f64; 3]], CfdError>;
}

/// The core backend trait.
///
/// Implementations execute face/cell kernels on the target hardware.
/// The initial `CpuBackend` interprets kernels as direct Rust loops.
/// Future backends (Cranelift, WGPU) will compile kernels to optimized code.
pub trait ComputeBackend: Send + Sync {
    /// The backend's mesh representation.
    type Mesh: MeshHandle;
    /// The backend's field storage.
    type Fields: FieldStore;

    /// Prepare mesh data for this backend (called once at setup).
    fn prepare_mesh(&self, mesh: &cfd_core_mesh_data::MeshData) -> Result<Self::Mesh, CfdError>;

    /// Execute a face kernel.
    fn execute_face_kernel(
        &self,
        mesh: &Self::Mesh,
        kernel: &FaceKernel,
        fields: &mut Self::Fields,
    ) -> Result<(), CfdError>;

    /// Execute a cell kernel.
    fn execute_cell_kernel(
        &self,
        mesh: &Self::Mesh,
        kernel: &CellKernel,
        fields: &mut Self::Fields,
    ) -> Result<(), CfdError>;

    /// Sparse matrix-vector multiply: y = A * x.
    fn spmv(&self, values: &[f64], col_idx: &[u32], row_ptr: &[u32], x: &[f64], y: &mut [f64]);

    /// Dot product.
    fn dot(&self, x: &[f64], y: &[f64]) -> f64;

    /// L2 norm.
    fn norm2(&self, x: &[f64]) -> f64;
}

/// Lightweight struct holding the minimal mesh geometry data needed by backends.
///
/// This avoids coupling `cfd-compute` to the full `cfd-mesh` crate.
/// `cfd-mesh::Mesh` can produce this via a conversion method.
pub mod cfd_core_mesh_data {
    use cfd_core::Vec3;

    /// Minimal mesh data for backend consumption.
    pub struct MeshData {
        pub n_cells: usize,
        pub n_internal_faces: usize,
        pub n_boundary_faces: usize,

        // Face geometry
        pub face_areas: Vec<f64>,
        pub face_normals: Vec<Vec3>,
        pub face_centroids: Vec<Vec3>,

        // Face connectivity
        pub face_owner: Vec<u32>,
        pub face_neighbor: Vec<u32>, // length = n_internal_faces

        // Cell geometry
        pub cell_volumes: Vec<f64>,
        pub cell_centroids: Vec<Vec3>,

        // Face-to-cell vectors (for gradient computation)
        pub face_delta: Vec<Vec3>,   // centroid_neighbor - centroid_owner
        pub face_weights: Vec<f64>,  // interpolation weight

        // Boundary patches
        pub boundary_patches: Vec<BoundaryPatchData>,
    }

    pub struct BoundaryPatchData {
        pub name: String,
        pub start_face: usize, // offset from n_internal_faces
        pub n_faces: usize,
    }
}
