
```rust
pub enum BoundaryCondition {
    Dirichlet(f64),  Neumann(f64),  ZeroGradient,  NoSlip,
    Outflow,  Absorbing,  FixedFlux(f64),  Custom(String),
}
```
