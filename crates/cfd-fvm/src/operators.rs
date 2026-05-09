//! FVM operator kernel builders.
//!
//! Each function constructs a kernel description. No computation happens here.

use cfd_compute::kernel::*;

/// Build a diffusion (Laplacian) kernel: ∇·(γ ∇φ)
///
/// Assembles face fluxes: γ_f · A_f · (φ_N - φ_O) / |d|
/// and accumulates into `target_field`.
pub fn laplacian_kernel(
    field: &str,
    gamma: impl Into<ParamRef>,
    target: &str,
) -> FaceKernel {
    FaceKernel::new("laplacian")
        .with_op(FaceOp::Diffusion {
            field: FieldRef::new(field),
            gamma: gamma.into(),
            target: FieldRef::new(target),
        })
        .with_read(field)
        .with_write(target)
}

/// Build an advection kernel: ∇·(φ u)
///
/// Computes face fluxes using the specified scheme and accumulates.
pub fn advection_kernel(
    field: &str,
    velocity: &str,
    scheme: AdvectionScheme,
    target: &str,
) -> FaceKernel {
    FaceKernel::new("advection")
        .with_op(FaceOp::Advection {
            field: FieldRef::new(field),
            velocity: FieldRef::new(velocity),
            scheme,
            target: FieldRef::new(target),
        })
        .with_read(field)
        .with_read(velocity)
        .with_write(target)
}

/// Build a Scharfetter-Gummel drift-diffusion kernel: ∇·(μ n E - D ∇n)
///
/// Used for ion transport in electric fields. Exponentially weights face
/// concentrations based on the local Peclet number for numerical stability.
pub fn scharfetter_gummel_kernel(
    concentration: &str,
    electric_field: &str,
    mobility: f64,
    diffusion: f64,
    target: &str,
) -> FaceKernel {
    FaceKernel::new("scharfetter_gummel")
        .with_op(FaceOp::ScharfetterGummel {
            concentration: FieldRef::new(concentration),
            electric_field: FieldRef::new(electric_field),
            mobility,
            diffusion,
            target: FieldRef::new(target),
        })
        .with_read(concentration)
        .with_read(electric_field)
        .with_write(target)
}

/// Build a divergence kernel: ∇·v
///
/// Computes the divergence of a vector field by summing face fluxes.
pub fn divergence_kernel(vector_field: &str, target: &str) -> FaceKernel {
    FaceKernel::new("divergence")
        .with_op(FaceOp::Divergence {
            vector_field: FieldRef::new(vector_field),
            target: FieldRef::new(target),
        })
        .with_read(vector_field)
        .with_write(target)
}

/// Build a cell fill kernel: field[i] = value
pub fn fill_kernel(field: &str, value: f64) -> CellKernel {
    CellKernel::new("fill")
        .with_op(CellOp::Fill {
            field: FieldRef::new(field),
            value,
        })
        .with_write(field)
}

/// Build an AXPY kernel: y[i] += a * x[i]
pub fn axpy_kernel(a: f64, x: &str, y: &str) -> CellKernel {
    CellKernel::new("axpy")
        .with_op(CellOp::Axpy {
            a: ParamRef::Constant(a),
            x: FieldRef::new(x),
            y: FieldRef::new(y),
        })
        .with_read(x)
        .with_write(y)
}

/// Build a clamp kernel: field[i] = clamp(field[i], min, max)
pub fn clamp_kernel(field: &str, min: f64, max: f64) -> CellKernel {
    CellKernel::new("clamp")
        .with_op(CellOp::Clamp {
            field: FieldRef::new(field),
            min_val: min,
            max_val: max,
        })
        .with_read(field)
        .with_write(field)
}
