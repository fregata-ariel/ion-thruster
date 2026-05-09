
::: {lang=ja}
`FieldRegistry`は文字列キーの`HashMap<String, Field>`で場を管理する。物理モジュールはここにフィールドを登録し、他のモジュールと共有する。

```rust
// 場の登録（EHDモジュール内）
registry.register_scalar("phi", n_cells, FieldLocation::Cell);
registry.register_vector("velocity", n_cells, FieldLocation::Cell);

// 場へのアクセス（SplittingStep内）
let phi = state.fields.get_scalar("phi")?;
let vel = state.fields.get_vector_mut("velocity")?;
```

`Field`は`Scalar(ScalarField)`と`Vector(VectorField)`のenum。`ScalarField`は`Vec<f64>`、`VectorField`は`Vec<Vec3>`を内部に持つ。いずれも`FieldLocation`（`Cell`/`Face`/`Node`）を保持する。

2つのスカラーフィールドへの同時可変参照が必要な場合（例: PoissonソルバのRHS組立時）、`get_scalar_pair_mut()`が`unsafe`で安全に提供する:

```rust
let (phi, rhs) = state.fields.get_scalar_pair_mut("phi", "poisson_rhs")?;
```

`SimState`は`FieldRegistry` + 時刻(`time: f64`) + ステップ数(`step: usize`) + 時間刻み(`dt: f64`)を保持する。タイムステップ中の全サブステップが同一の`SimState`を逐次的に変更する。
:::

::: {lang=en}
`FieldRegistry` manages fields through a string-keyed `HashMap<String, Field>`. Physics modules register fields here and share them with other modules.

```rust
// Register fields (inside EHD module)
registry.register_scalar("phi", n_cells, FieldLocation::Cell);
registry.register_vector("velocity", n_cells, FieldLocation::Cell);

// Access fields (inside SplittingStep)
let phi = state.fields.get_scalar("phi")?;
let vel = state.fields.get_vector_mut("velocity")?;
```

`Field` is an enum of `Scalar(ScalarField)` and `Vector(VectorField)`. `ScalarField` wraps `Vec<f64>`, `VectorField` wraps `Vec<Vec3>`. Both carry a `FieldLocation` (`Cell`/`Face`/`Node`).

When simultaneous mutable access to two scalar fields is needed (e.g., during Poisson solver RHS assembly), `get_scalar_pair_mut()` safely provides it via `unsafe`:

```rust
let (phi, rhs) = state.fields.get_scalar_pair_mut("phi", "poisson_rhs")?;
```

`SimState` holds `FieldRegistry` + current time (`time: f64`) + step count (`step: usize`) + timestep size (`dt: f64`). All sub-steps within a timestep sequentially mutate the same `SimState`.
:::
