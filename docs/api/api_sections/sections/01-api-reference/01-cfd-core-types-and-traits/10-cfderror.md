
```rust
pub enum CfdError {
    Mesh(String),  Io(#[from] std::io::Error),
    SolverNotConverged { message: String, residual: f64, iterations: usize },
    FieldNotFound { name: String },  DimensionMismatch { expected: usize, got: usize },
    Config(String),  Boundary(String),  Other(String),
}
```
:::
