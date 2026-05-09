
```rust
pub struct ScalarField { pub values: Vec<f64>, pub location: FieldLocation }
pub struct VectorField { pub values: Vec<Vec3>, pub location: FieldLocation }
pub enum Field { Scalar(ScalarField), Vector(VectorField) }
```
