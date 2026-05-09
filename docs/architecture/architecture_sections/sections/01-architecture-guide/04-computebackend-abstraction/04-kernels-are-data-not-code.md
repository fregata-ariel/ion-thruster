
FVM operators (Laplacian, advection, []{.term id=scharfetter_gummel} flux, etc.) return **data descriptions** as `FaceKernel`/`CellKernel`. They do not execute.

```rust
// cfd-fvm: build kernel "description" (no computation happens)
let kernel = laplacian_kernel("phi", 1.0, "residual");

// cfd-compute-cpu: "execute" the kernel (backend interprets the ops list)
backend.execute_face_kernel(mesh, &kernel, fields)?;
```

`FaceKernel` carries a name, a list of `FaceOp`s, and read/write field lists. Four face operation variants exist:

- `Diffusion` — diffusion flux: $\gamma_f \cdot A_f \cdot (\phi_N - \phi_O) / |d|$
- `Advection` — advection flux (Upwind/Central/TVD selectable)
- `ScharfetterGummel` — drift-diffusion flux ([]{.term id=bernoulli_function} weighted)
- `Divergence` — divergence of a vector field

`CellKernel` carries a list of `CellOp`s, providing 6 cell operations: `Axpy`, `Scale`, `Clamp`, `Multiply`, `Fill`, `Copy`.
