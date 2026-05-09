
```toml
[output]
format = "vtu"
every = 10          # 出力間隔（ステップ数）
path = "output/"
fields = ["phi", "electric_field", "ion_density", "charge_density",
          "velocity", "pressure", "ehd_force"]
```
:::

::: {lang=en}
Simulation settings are defined in TOML files. Refer to `examples/wire_plate_2d/simulation.toml`:
