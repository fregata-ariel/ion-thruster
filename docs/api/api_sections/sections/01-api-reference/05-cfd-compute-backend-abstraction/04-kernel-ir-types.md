
```rust
// Face operations
pub enum FaceOp { Diffusion{..}, Advection{..}, ScharfetterGummel{..}, Divergence{..} }
pub struct FaceKernel { pub name: String, pub ops: Vec<FaceOp>, pub reads/writes: Vec<FieldRef> }

// Cell operations
pub enum CellOp { Axpy{..}, Scale{..}, Clamp{..}, Multiply{..}, Fill{..}, Copy{..} }
pub struct CellKernel { pub name: String, pub ops: Vec<CellOp>, pub reads/writes: Vec<FieldRef> }

// Parameter references
pub struct FieldRef(pub String);
pub enum ParamRef { Constant(f64), Field(FieldRef) }
pub enum AdvectionScheme { Upwind, Central, TVD(Limiter) }
pub enum Limiter { VanLeer, MinMod, Superbee }
```
:::
