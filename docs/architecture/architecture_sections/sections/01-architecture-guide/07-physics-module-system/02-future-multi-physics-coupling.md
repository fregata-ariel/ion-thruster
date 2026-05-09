
```rust
let ehd = EhdModule::new(ehd_config);
let thermal = ThermalModule::new(thermal_config);

ehd.register_fields(&mut state.fields, &mesh);
thermal.register_fields(&mut state.fields, &mesh);

// Step ordering determines coupling strategy
for step in ehd.splitting_steps() { splitting.add_step(step); }
for step in thermal.splitting_steps() { splitting.add_step(step); }
```

Inter-module communication occurs solely through `FieldRegistry`. No direct references.
:::
