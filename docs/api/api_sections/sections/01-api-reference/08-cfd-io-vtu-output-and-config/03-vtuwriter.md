
```rust
impl VtuWriter {
    pub fn new(output_dir: impl Into<PathBuf>,
               scalar_fields: Vec<String>, vector_fields: Vec<String>) -> Self;
    pub fn write_pvd(&self) -> Result<(), CfdError>;
}
impl FieldWriter for VtuWriter { ... }
```

Output format is VTK XML Unstructured Grid (`.vtu`). PVD collection files support time series animation. Uses `vtkio 0.6`.
