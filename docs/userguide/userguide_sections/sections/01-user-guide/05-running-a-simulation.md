
::: {lang=ja}
```bash
# メッシュ生成
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 \
     -o examples/wire_plate_2d/wire_plate.msh

# シミュレーション実行
cargo run --release --bin ehd-sim -- --config examples/wire_plate_2d/simulation.toml

# またはビルド済みバイナリを直接使用
./target/release/ehd-sim --config examples/wire_plate_2d/simulation.toml
```
