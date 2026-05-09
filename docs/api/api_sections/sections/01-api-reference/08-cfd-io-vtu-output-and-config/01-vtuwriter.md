
```rust
impl VtuWriter {
    pub fn new(output_dir: impl Into<PathBuf>,
               scalar_fields: Vec<String>, vector_fields: Vec<String>) -> Self;
    pub fn write_pvd(&self) -> Result<(), CfdError>;
}
impl FieldWriter for VtuWriter { ... }
```

出力形式はVTK XML Unstructured Grid（`.vtu`）。PVDコレクションファイルで時系列アニメーションに対応。`vtkio 0.6`を使用。
