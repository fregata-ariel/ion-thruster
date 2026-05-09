
::: {lang=ja}
[]{.term id=operator_splitting}を使用して、連成方程式系を逐次的に解く。`OperatorSplitting`は`Vec<Box<dyn SplittingStep>>`を保持し、毎タイムステップで全ステップを順番に実行する。`SimulationDriver`が最外側の時間ループを制御し、初期状態出力、dt計算、ステップ実行、定期出力、終了判定を行う。

```rust
// SimulationDriver::run() の概略
writer.write_frame(mesh, state, 0)?;           // 初期状態
for step_num in 1..=max_steps {
    state.dt = fixed_dt.unwrap_or(splitting.compute_dt(mesh, state));
    splitting.advance(mesh, state)?;            // 全サブステップ実行
    state.time += state.dt;
    if step_num % output_interval == 0 {
        writer.write_frame(mesh, state, step_num)?;
    }
    if state.time >= max_time { break; }
}
```
:::

::: {lang=en}
[]{.term id=operator_splitting} is used to solve the coupled equation system sequentially. `OperatorSplitting` holds `Vec<Box<dyn SplittingStep>>` and executes all steps in order each timestep. `SimulationDriver` controls the outermost time loop: initial state output, dt computation, step execution, periodic output, and termination checks.

```rust
// SimulationDriver::run() outline
writer.write_frame(mesh, state, 0)?;           // initial state
for step_num in 1..=max_steps {
    state.dt = fixed_dt.unwrap_or(splitting.compute_dt(mesh, state));
    splitting.advance(mesh, state)?;            // execute all sub-steps
    state.time += state.dt;
    if step_num % output_interval == 0 {
        writer.write_frame(mesh, state, step_num)?;
    }
    if state.time >= max_time { break; }
}
```
:::
