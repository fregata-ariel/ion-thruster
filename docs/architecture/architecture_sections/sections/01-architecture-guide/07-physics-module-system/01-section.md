
```rust
let ehd = EhdModule::new(ehd_config);
let thermal = ThermalModule::new(thermal_config);

ehd.register_fields(&mut state.fields, &mesh);
thermal.register_fields(&mut state.fields, &mesh);

// ステップの順序が連成の仕方を決める
for step in ehd.splitting_steps() { splitting.add_step(step); }
for step in thermal.splitting_steps() { splitting.add_step(step); }
```

モジュール間の通信は`FieldRegistry`のみを介する。直接参照なし。
:::

::: {lang=en}
Physics modules are implemented as concrete structs following the `PhysicsModule` pattern. Rather than a monolithic trait object, each module has three responsibilities:

1. **Field registration**: `register_fields()` — add required fields to the Registry
2. **Initialization**: `initialize()` — set initial conditions
3. **Step provision**: `splitting_steps()` — return a list of sub-steps for OperatorSplitting

```rust
let ehd = EhdModule::new(config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

for step in ehd.splitting_steps() {
    splitting.add_step(step);
}
```
