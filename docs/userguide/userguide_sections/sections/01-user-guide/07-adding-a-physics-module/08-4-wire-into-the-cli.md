
Same pattern as the EHD module in `main()`:

```rust
let thermal = ThermalModule::new(thermal_config);
thermal.register_fields(&mut state.fields, &mesh);
for step in thermal.splitting_steps() {
    splitting.add_step(step);
}
```

Inter-module communication is through `FieldRegistry` only. No direct references needed.
:::
