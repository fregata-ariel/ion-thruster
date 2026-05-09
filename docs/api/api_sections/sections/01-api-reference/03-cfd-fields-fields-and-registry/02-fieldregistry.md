
```rust
impl FieldRegistry {
    pub fn new() -> Self;
    pub fn register_scalar(&mut self, name: impl Into<String>, size: usize, location: FieldLocation);
    pub fn register_vector(&mut self, name: impl Into<String>, size: usize, location: FieldLocation);
    pub fn get_scalar(&self, name: &str) -> Result<&ScalarField, CfdError>;
    pub fn get_scalar_mut(&mut self, name: &str) -> Result<&mut ScalarField, CfdError>;
    pub fn get_vector(&self, name: &str) -> Result<&VectorField, CfdError>;
    pub fn get_vector_mut(&mut self, name: &str) -> Result<&mut VectorField, CfdError>;
    pub fn get_scalar_pair_mut(&mut self, a: &str, b: &str)
        -> Result<(&mut ScalarField, &mut ScalarField), CfdError>;
    pub fn names(&self) -> Vec<&str>;
    pub fn contains(&self, name: &str) -> bool;
}
```
