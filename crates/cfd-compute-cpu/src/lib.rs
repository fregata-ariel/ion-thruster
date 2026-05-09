//! `cfd-compute-cpu` — CPU compute backend.
//!
//! Interprets `FaceKernel` and `CellKernel` descriptions as direct Rust loops.
//! This is the initial backend — simple, correct, and reasonably fast with
//! auto-vectorization. Future backends (Cranelift, WGPU) will compile kernels
//! to optimized native/GPU code.

use cfd_compute::backend::cfd_core_mesh_data::MeshData;
use cfd_compute::backend::{ComputeBackend, FieldStore, MeshHandle};
use cfd_compute::kernel::*;
use cfd_core::vec3::ops as v;
use cfd_core::CfdError;
use cfd_fields::FieldRegistry;

/// CPU backend mesh handle — just holds the MeshData directly.
pub struct CpuMesh {
    pub data: MeshData,
}

impl MeshHandle for CpuMesh {}

/// CPU backend field store — wraps a FieldRegistry.
pub struct CpuFieldStore {
    pub registry: FieldRegistry,
}

impl FieldStore for CpuFieldStore {
    fn scalar_data(&self, name: &str) -> Result<&[f64], CfdError> {
        Ok(&self.registry.get_scalar(name)?.values)
    }

    fn scalar_data_mut(&mut self, name: &str) -> Result<&mut [f64], CfdError> {
        Ok(&mut self.registry.get_scalar_mut(name)?.values)
    }

    fn vector_data(&self, name: &str) -> Result<&[[f64; 3]], CfdError> {
        Ok(&self.registry.get_vector(name)?.values)
    }

    fn vector_data_mut(&mut self, name: &str) -> Result<&mut [[f64; 3]], CfdError> {
        Ok(&mut self.registry.get_vector_mut(name)?.values)
    }
}

impl CpuFieldStore {
    pub fn new(registry: FieldRegistry) -> Self {
        Self { registry }
    }
}

/// The CPU compute backend.
pub struct CpuBackend;

impl ComputeBackend for CpuBackend {
    type Mesh = CpuMesh;
    type Fields = CpuFieldStore;

    fn prepare_mesh(&self, mesh: &MeshData) -> Result<CpuMesh, CfdError> {
        // CPU backend just clones the data — no special preparation needed.
        Ok(CpuMesh {
            data: MeshData {
                n_cells: mesh.n_cells,
                n_internal_faces: mesh.n_internal_faces,
                n_boundary_faces: mesh.n_boundary_faces,
                face_areas: mesh.face_areas.clone(),
                face_normals: mesh.face_normals.clone(),
                face_centroids: mesh.face_centroids.clone(),
                face_owner: mesh.face_owner.clone(),
                face_neighbor: mesh.face_neighbor.clone(),
                cell_volumes: mesh.cell_volumes.clone(),
                cell_centroids: mesh.cell_centroids.clone(),
                face_delta: mesh.face_delta.clone(),
                face_weights: mesh.face_weights.clone(),
                boundary_patches: mesh
                    .boundary_patches
                    .iter()
                    .map(|p| cfd_compute::backend::cfd_core_mesh_data::BoundaryPatchData {
                        name: p.name.clone(),
                        start_face: p.start_face,
                        n_faces: p.n_faces,
                    })
                    .collect(),
            },
        })
    }

    fn execute_face_kernel(
        &self,
        mesh: &CpuMesh,
        kernel: &FaceKernel,
        fields: &mut CpuFieldStore,
    ) -> Result<(), CfdError> {
        for op in &kernel.ops {
            execute_face_op(&mesh.data, op, &mut fields.registry)?;
        }
        Ok(())
    }

    fn execute_cell_kernel(
        &self,
        mesh: &CpuMesh,
        kernel: &CellKernel,
        fields: &mut CpuFieldStore,
    ) -> Result<(), CfdError> {
        for op in &kernel.ops {
            execute_cell_op(mesh.data.n_cells, op, &mut fields.registry)?;
        }
        Ok(())
    }

    fn spmv(&self, values: &[f64], col_idx: &[u32], row_ptr: &[u32], x: &[f64], y: &mut [f64]) {
        let n = row_ptr.len() - 1;
        for i in 0..n {
            let start = row_ptr[i] as usize;
            let end = row_ptr[i + 1] as usize;
            let mut sum = 0.0;
            for k in start..end {
                sum += values[k] * x[col_idx[k] as usize];
            }
            y[i] = sum;
        }
    }

    fn dot(&self, x: &[f64], y: &[f64]) -> f64 {
        x.iter().zip(y.iter()).map(|(a, b)| a * b).sum()
    }

    fn norm2(&self, x: &[f64]) -> f64 {
        self.dot(x, x).sqrt()
    }
}

/// Execute a single face operation over all internal faces.
fn execute_face_op(
    mesh: &MeshData,
    op: &FaceOp,
    fields: &mut FieldRegistry,
) -> Result<(), CfdError> {
    match op {
        FaceOp::Diffusion {
            field,
            gamma,
            target,
        } => {
            let phi = fields.get_scalar(&field.0)?.values.clone();
            let gamma_values = resolve_param(gamma, fields)?;
            let target = fields.get_scalar_mut(&target.0)?;

            for fi in 0..mesh.n_internal_faces {
                let owner = mesh.face_owner[fi] as usize;
                let neighbor = mesh.face_neighbor[fi] as usize;
                let area = mesh.face_areas[fi];
                let delta_mag = v::magnitude(mesh.face_delta[fi]);

                if delta_mag < 1e-30 {
                    continue;
                }

                let gamma_f = match &gamma_values {
                    ParamValue::Constant(c) => *c,
                    ParamValue::PerCell(vals) => 0.5 * (vals[owner] + vals[neighbor]),
                };

                let flux = gamma_f * area * (phi[neighbor] - phi[owner]) / delta_mag;
                target.values[owner] += flux;
                target.values[neighbor] -= flux;
            }
            Ok(())
        }

        FaceOp::Advection {
            field,
            velocity,
            scheme,
            target,
        } => {
            let phi = fields.get_scalar(&field.0)?.values.clone();
            let vel = fields.get_vector(&velocity.0)?.values.clone();
            let target = fields.get_scalar_mut(&target.0)?;

            for fi in 0..mesh.n_internal_faces {
                let owner = mesh.face_owner[fi] as usize;
                let neighbor = mesh.face_neighbor[fi] as usize;
                let normal = mesh.face_normals[fi];
                let area = mesh.face_areas[fi];

                // Face velocity (interpolated)
                let vel_f = [
                    0.5 * (vel[owner][0] + vel[neighbor][0]),
                    0.5 * (vel[owner][1] + vel[neighbor][1]),
                    0.5 * (vel[owner][2] + vel[neighbor][2]),
                ];
                let flux_vol = v::dot(vel_f, normal) * area;

                let phi_f = match scheme {
                    AdvectionScheme::Upwind => {
                        if flux_vol >= 0.0 {
                            phi[owner]
                        } else {
                            phi[neighbor]
                        }
                    }
                    AdvectionScheme::Central => 0.5 * (phi[owner] + phi[neighbor]),
                    AdvectionScheme::TVD(_limiter) => {
                        // TODO: implement TVD limiters
                        if flux_vol >= 0.0 {
                            phi[owner]
                        } else {
                            phi[neighbor]
                        }
                    }
                };

                let flux = flux_vol * phi_f;
                target.values[owner] -= flux;
                target.values[neighbor] += flux;
            }
            Ok(())
        }

        FaceOp::ScharfetterGummel {
            concentration,
            electric_field,
            mobility,
            diffusion,
            target,
        } => {
            let n = fields.get_scalar(&concentration.0)?.values.clone();
            let e_field = fields.get_vector(&electric_field.0)?.values.clone();
            let target = fields.get_scalar_mut(&target.0)?;

            for fi in 0..mesh.n_internal_faces {
                let owner = mesh.face_owner[fi] as usize;
                let neighbor = mesh.face_neighbor[fi] as usize;
                let _normal = mesh.face_normals[fi];
                let area = mesh.face_areas[fi];
                let delta = mesh.face_delta[fi];
                let delta_mag = v::magnitude(delta);

                if delta_mag < 1e-30 {
                    continue;
                }

                // E field at face (average)
                let e_f = [
                    0.5 * (e_field[owner][0] + e_field[neighbor][0]),
                    0.5 * (e_field[owner][1] + e_field[neighbor][1]),
                    0.5 * (e_field[owner][2] + e_field[neighbor][2]),
                ];

                // Peclet number: Pe = mu * E · d / D
                let e_dot_d = v::dot(e_f, delta);
                let pe = mobility * e_dot_d / diffusion;

                // Bernoulli function: B(x) = x / (exp(x) - 1)
                let flux = if pe.abs() < 1e-6 {
                    // Taylor expansion for small Pe
                    diffusion * area / delta_mag * (n[owner] - n[neighbor])
                } else {
                    let b_pos = bernoulli(pe);
                    let b_neg = bernoulli(-pe);
                    diffusion * area / delta_mag * (b_pos * n[owner] - b_neg * n[neighbor])
                };

                target.values[owner] -= flux;
                target.values[neighbor] += flux;
            }
            Ok(())
        }

        FaceOp::Divergence {
            vector_field,
            target,
        } => {
            let vec_field = fields.get_vector(&vector_field.0)?.values.clone();
            let target = fields.get_scalar_mut(&target.0)?;

            for fi in 0..mesh.n_internal_faces {
                let owner = mesh.face_owner[fi] as usize;
                let neighbor = mesh.face_neighbor[fi] as usize;
                let normal = mesh.face_normals[fi];
                let area = mesh.face_areas[fi];

                let v_f = [
                    0.5 * (vec_field[owner][0] + vec_field[neighbor][0]),
                    0.5 * (vec_field[owner][1] + vec_field[neighbor][1]),
                    0.5 * (vec_field[owner][2] + vec_field[neighbor][2]),
                ];

                let flux = v::dot(v_f, normal) * area;
                target.values[owner] += flux;
                target.values[neighbor] -= flux;
            }
            Ok(())
        }
    }
}

/// Execute a single cell operation.
fn execute_cell_op(
    n_cells: usize,
    op: &CellOp,
    fields: &mut FieldRegistry,
) -> Result<(), CfdError> {
    match op {
        CellOp::Fill { field, value } => {
            let f = fields.get_scalar_mut(&field.0)?;
            f.values.fill(*value);
            Ok(())
        }
        CellOp::Scale { field, factor } => {
            let factor_val = match factor {
                ParamRef::Constant(c) => *c,
                ParamRef::Field(_) => {
                    return Err(CfdError::Other(
                        "Per-cell scale factor not yet supported in CellOp::Scale".into(),
                    ));
                }
            };
            let f = fields.get_scalar_mut(&field.0)?;
            for v in &mut f.values {
                *v *= factor_val;
            }
            Ok(())
        }
        CellOp::Clamp {
            field,
            min_val,
            max_val,
        } => {
            let f = fields.get_scalar_mut(&field.0)?;
            for v in &mut f.values {
                *v = v.clamp(*min_val, *max_val);
            }
            Ok(())
        }
        CellOp::Axpy { a, x, y } => {
            let a_val = match a {
                ParamRef::Constant(c) => *c,
                ParamRef::Field(_) => {
                    return Err(CfdError::Other(
                        "Per-cell Axpy factor not yet supported".into(),
                    ));
                }
            };
            let x_vals = fields.get_scalar(&x.0)?.values.clone();
            let y_field = fields.get_scalar_mut(&y.0)?;
            for i in 0..n_cells.min(y_field.values.len()) {
                y_field.values[i] += a_val * x_vals[i];
            }
            Ok(())
        }
        CellOp::Multiply { a, b, target } => {
            let a_vals = fields.get_scalar(&a.0)?.values.clone();
            let b_vals = fields.get_scalar(&b.0)?.values.clone();
            let t = fields.get_scalar_mut(&target.0)?;
            for i in 0..n_cells.min(t.values.len()) {
                t.values[i] = a_vals[i] * b_vals[i];
            }
            Ok(())
        }
        CellOp::Copy { source, target } => {
            let src = fields.get_scalar(&source.0)?.values.clone();
            let t = fields.get_scalar_mut(&target.0)?;
            t.values.copy_from_slice(&src);
            Ok(())
        }
    }
}

/// Bernoulli function: B(x) = x / (exp(x) - 1)
///
/// Used in the Scharfetter-Gummel scheme. Numerically stable for all x.
#[inline]
fn bernoulli(x: f64) -> f64 {
    if x.abs() < 1e-6 {
        // Taylor: B(x) ≈ 1 - x/2 + x²/12
        1.0 - x * 0.5 + x * x / 12.0
    } else if x > 500.0 {
        // For large positive x: B(x) ≈ x * exp(-x) → 0
        0.0
    } else if x < -500.0 {
        // For large negative x: B(x) ≈ -x
        -x
    } else {
        x / (x.exp() - 1.0)
    }
}

/// Resolve a ParamRef to either a constant or per-cell values.
enum ParamValue {
    Constant(f64),
    PerCell(Vec<f64>),
}

fn resolve_param(param: &ParamRef, fields: &FieldRegistry) -> Result<ParamValue, CfdError> {
    match param {
        ParamRef::Constant(c) => Ok(ParamValue::Constant(*c)),
        ParamRef::Field(f) => {
            let field = fields.get_scalar(&f.0)?;
            Ok(ParamValue::PerCell(field.values.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bernoulli_small() {
        let b = bernoulli(0.0);
        assert!((b - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_bernoulli_large_positive() {
        let b = bernoulli(100.0);
        assert!(b < 1e-10);
    }

    #[test]
    fn test_bernoulli_large_negative() {
        let b = bernoulli(-100.0);
        assert!((b - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_bernoulli_moderate() {
        // B(1) = 1 / (e - 1) ≈ 0.58198
        let b = bernoulli(1.0);
        assert!((b - 1.0 / (1.0_f64.exp() - 1.0)).abs() < 1e-12);
    }
}
