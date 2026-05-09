
Rust closures have types fixed at compile time. They cannot be passed to JIT compilers (Cranelift) or GPUs (WGPU). Data descriptions can be:

- Inspected and compiled by Cranelift
- Converted to compute shaders for WGPU
- Serialized and cached
- Optimized through transformation passes
