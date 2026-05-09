
環境変数`RUST_LOG`でログレベルを制御:

```bash
RUST_LOG=debug cargo run --release --bin ehd-sim -- -c simulation.toml
RUST_LOG=info  cargo run --release --bin ehd-sim -- -c simulation.toml
```
:::

::: {lang=en}
```bash
# Generate mesh
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 \
     -o examples/wire_plate_2d/wire_plate.msh

# Run simulation
cargo run --release --bin ehd-sim -- --config examples/wire_plate_2d/simulation.toml

# Or use the pre-built binary directly
./target/release/ehd-sim --config examples/wire_plate_2d/simulation.toml
```
