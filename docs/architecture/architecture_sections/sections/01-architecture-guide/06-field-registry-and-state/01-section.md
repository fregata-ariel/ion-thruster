
- 物理モジュール間がコンパイル時に互いを知らなくてよい
- デバッグ・ログで場の名前が直接読める
- HashMap lookupのコストはタイムステップあたり1回/フィールドであり、ホットループ内ではない

`SimState`は`FieldRegistry` + 時刻 + ステップ数 + dtを保持する。タイムステップ中の全サブステップが同一の`SimState`を逐次的に変更する。
:::

::: {lang=en}
`FieldRegistry` manages fields through a string-keyed HashMap. Physics modules register fields here and share them with other modules.

```rust
// Register fields (inside EHD module)
registry.register_scalar("phi", n_cells, FieldLocation::Cell);
registry.register_vector("velocity", n_cells, FieldLocation::Cell);

// Access fields (inside SplittingStep)
let phi = state.fields.get_scalar("phi")?;
let vel = state.fields.get_vector_mut("velocity")?;
```
