
```bash
cargo new crates/thermal-physics --lib
```

`Cargo.toml`の`[workspace.members]`に追加。依存先として`cfd-core`, `cfd-fields`, `cfd-mesh`, `cfd-time`を追加。
