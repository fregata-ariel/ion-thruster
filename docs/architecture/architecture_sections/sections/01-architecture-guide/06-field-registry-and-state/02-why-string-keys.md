
- Physics modules don't need compile-time knowledge of each other
- Field names are directly readable in debug output and logs
- HashMap lookup cost is once per field per timestep — not in hot loops

`SimState` holds `FieldRegistry` + current time + step count + dt. All sub-steps within a timestep sequentially mutate the same `SimState`.
:::
