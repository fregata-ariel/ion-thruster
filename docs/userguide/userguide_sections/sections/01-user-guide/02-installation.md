
::: {lang=ja}
```bash
git clone <repository-url> ion-craft
cd ion-craft
cargo build --release
```

ビルド成功後、`target/release/ehd-sim` バイナリが生成される。

テストの実行:
```bash
cargo test --workspace
```
:::

::: {lang=en}
```bash
git clone <repository-url> ion-craft
cd ion-craft
cargo build --release
```

After a successful build, the `target/release/ehd-sim` binary is created.

Run tests:
```bash
cargo test --workspace
```
:::
