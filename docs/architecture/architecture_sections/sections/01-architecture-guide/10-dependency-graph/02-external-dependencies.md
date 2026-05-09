
| Purpose | Crate | Rationale |
|---|---|---|
| Sparse matrices | `sprs` | Mature, pure Rust, CSR/COO support |
| Gmsh import | Custom parser | MSH 2.2 ASCII, minimal dependencies |
| VTU output | `vtkio` | XML VTU unstructured grid support |
| Config | `toml` + `serde` | Standard Rust approach |
| CLI | `clap` | Derive macro support |
| Logging | `tracing` | Structured logging with timing spans |
:::
