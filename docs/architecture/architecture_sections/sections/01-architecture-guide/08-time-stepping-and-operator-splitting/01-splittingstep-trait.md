
各サブステップは以下のtraitを実装する:

```rust
pub trait SplittingStep: Send {
    fn name(&self) -> &str;
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<()>;
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}
```

`max_dt()`は各ステップの安定性制約を報告する。楕円型（Poisson）は`f64::INFINITY`を返す。
