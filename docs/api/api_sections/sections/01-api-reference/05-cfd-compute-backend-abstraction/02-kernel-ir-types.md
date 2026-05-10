
```rust
// 面演算
pub enum FaceOp { Diffusion{..}, Advection{..}, ScharfetterGummel{..}, Divergence{..} }
pub struct FaceKernel { pub name: String, pub ops: Vec<FaceOp>, pub reads: Vec<FieldRef>, pub writes: Vec<FieldRef> }

// セル演算
pub enum CellOp { Axpy{..}, Scale{..}, Clamp{..}, Multiply{..}, Fill{..}, Copy{..} }
pub struct CellKernel { pub name: String, pub ops: Vec<CellOp>, pub reads: Vec<FieldRef>, pub writes: Vec<FieldRef> }

// パラメータ参照
pub struct FieldRef(pub String);
pub enum ParamRef { Constant(f64), Field(FieldRef) }
pub enum AdvectionScheme { Upwind, Central, TVD(Limiter) }
pub enum Limiter { VanLeer, MinMod, Superbee }
```
:::

::: {lang=en}
Abstraction layer separating "what to compute" from "how to execute."
