//! Kernel description types — data representations of compute operations.
//!
//! These types describe *what* to compute, not *how*. A backend interprets
//! or compiles these descriptions into executable code.

/// Reference to a named field in the `FieldRegistry`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldRef(pub String);

impl FieldRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl From<&str> for FieldRef {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Reference to a parameter (constant or per-cell coefficient).
#[derive(Clone, Debug)]
pub enum ParamRef {
    /// A uniform scalar constant.
    Constant(f64),
    /// A per-cell scalar field.
    Field(FieldRef),
}

impl From<f64> for ParamRef {
    fn from(v: f64) -> Self {
        Self::Constant(v)
    }
}

impl From<FieldRef> for ParamRef {
    fn from(f: FieldRef) -> Self {
        Self::Field(f)
    }
}

/// Advection flux scheme.
#[derive(Clone, Debug)]
pub enum AdvectionScheme {
    Upwind,
    Central,
    TVD(Limiter),
}

/// TVD limiter functions.
#[derive(Clone, Debug)]
pub enum Limiter {
    VanLeer,
    MinMod,
    Superbee,
}

// ---------------------------------------------------------------------------
// Face kernel: operations over internal and boundary faces
// ---------------------------------------------------------------------------

/// An operation within a face loop.
///
/// Each variant is a self-contained instruction that a backend can interpret.
/// The operations are designed to cover common FVM flux computations.
#[derive(Clone, Debug)]
pub enum FaceOp {
    /// Compute diffusion flux: γ · A_f · (φ_N - φ_O) / |d|
    /// Accumulates into the target field.
    Diffusion {
        /// The scalar field being diffused.
        field: FieldRef,
        /// Diffusion coefficient (constant or per-cell).
        gamma: ParamRef,
        /// Target field for flux accumulation.
        target: FieldRef,
    },

    /// Compute advection flux using a face velocity field.
    Advection {
        /// The scalar field being advected.
        field: FieldRef,
        /// Face velocity field for flux computation.
        velocity: FieldRef,
        /// Advection scheme.
        scheme: AdvectionScheme,
        /// Target field for flux accumulation.
        target: FieldRef,
    },

    /// Compute Scharfetter-Gummel drift-diffusion flux.
    /// For equations of the form: ∇·(μ n E - D ∇n)
    ScharfetterGummel {
        /// Concentration field (n).
        concentration: FieldRef,
        /// Electric field (vector, used for drift).
        electric_field: FieldRef,
        /// Ion mobility coefficient.
        mobility: f64,
        /// Diffusion coefficient.
        diffusion: f64,
        /// Target field for flux accumulation.
        target: FieldRef,
    },

    /// Compute divergence of a vector field: Σ_f (v_f · n_f A_f).
    Divergence {
        /// Vector field whose divergence is computed.
        vector_field: FieldRef,
        /// Target scalar field for the divergence result.
        target: FieldRef,
    },
}

/// A face kernel: a named sequence of face operations with declared I/O.
#[derive(Clone, Debug)]
pub struct FaceKernel {
    /// Human-readable name (for logging/profiling).
    pub name: String,
    /// Sequence of operations to execute per face.
    pub ops: Vec<FaceOp>,
    /// Fields read by this kernel (for dependency tracking).
    pub reads: Vec<FieldRef>,
    /// Fields written by this kernel.
    pub writes: Vec<FieldRef>,
}

impl FaceKernel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ops: Vec::new(),
            reads: Vec::new(),
            writes: Vec::new(),
        }
    }

    pub fn with_op(mut self, op: FaceOp) -> Self {
        self.ops.push(op);
        self
    }

    pub fn with_read(mut self, field: impl Into<FieldRef>) -> Self {
        self.reads.push(field.into());
        self
    }

    pub fn with_write(mut self, field: impl Into<FieldRef>) -> Self {
        self.writes.push(field.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Cell kernel: per-cell operations (field updates, source terms)
// ---------------------------------------------------------------------------

/// An operation over all cells.
#[derive(Clone, Debug)]
pub enum CellOp {
    /// y[i] += a * x[i]  (AXPY / saxpy)
    Axpy {
        a: ParamRef,
        x: FieldRef,
        y: FieldRef,
    },

    /// field[i] *= factor
    Scale {
        field: FieldRef,
        factor: ParamRef,
    },

    /// field[i] = max(min(field[i], max_val), min_val)
    Clamp {
        field: FieldRef,
        min_val: f64,
        max_val: f64,
    },

    /// target[i] = a[i] * b[i]  (element-wise multiply of two scalar fields)
    Multiply {
        a: FieldRef,
        b: FieldRef,
        target: FieldRef,
    },

    /// Fill a field with a constant value.
    Fill {
        field: FieldRef,
        value: f64,
    },

    /// Copy one field to another.
    Copy {
        source: FieldRef,
        target: FieldRef,
    },
}

/// A cell kernel: a named sequence of per-cell operations.
#[derive(Clone, Debug)]
pub struct CellKernel {
    pub name: String,
    pub ops: Vec<CellOp>,
    pub reads: Vec<FieldRef>,
    pub writes: Vec<FieldRef>,
}

impl CellKernel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ops: Vec::new(),
            reads: Vec::new(),
            writes: Vec::new(),
        }
    }

    pub fn with_op(mut self, op: CellOp) -> Self {
        self.ops.push(op);
        self
    }

    pub fn with_read(mut self, field: impl Into<FieldRef>) -> Self {
        self.reads.push(field.into());
        self
    }

    pub fn with_write(mut self, field: impl Into<FieldRef>) -> Self {
        self.writes.push(field.into());
        self
    }
}
