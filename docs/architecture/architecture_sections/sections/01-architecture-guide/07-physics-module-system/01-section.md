
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
