
::: {lang=ja}
物理モジュールは`PhysicsModule`パターンに従う具体的な構造体として実装する。traitではなく構造体パターンを採用する理由は、各モジュールが異なる設定型（`EhdConfig`等）を持ち、返すステップの型も異なるためである。3つの責務を持つ:

1. **場の登録**: `register_fields(&self, registry, mesh)` — 必要なフィールドをRegistryに追加
2. **初期化**: `initialize(&self, state, mesh)` — 初期条件の設定
3. **ステップ提供**: `splitting_steps(&self) -> Vec<Box<dyn SplittingStep>>` — OperatorSplittingに追加するサブステップのリストを返す

```rust
let ehd = EhdModule::new(config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

for step in ehd.splitting_steps() {
    splitting.add_step(step);
}
```

EHDモジュールは9個のフィールドを登録する: `phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`。`output_fields()`メソッドでVTU出力すべきフィールドの名前リストも返す。
:::

::: {lang=en}
Physics modules are implemented as concrete structs following the `PhysicsModule` pattern. A struct pattern is chosen over a trait because each module has different config types (`EhdConfig`, etc.) and returns different step types. Each module has three responsibilities:

1. **Field registration**: `register_fields(&self, registry, mesh)` — add required fields to the Registry
2. **Initialization**: `initialize(&self, state, mesh)` — set initial conditions
3. **Step provision**: `splitting_steps(&self) -> Vec<Box<dyn SplittingStep>>` — return a list of sub-steps for OperatorSplitting

```rust
let ehd = EhdModule::new(config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

for step in ehd.splitting_steps() {
    splitting.add_step(step);
}
```

The EHD module registers 9 fields: `phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`. It also provides `output_fields()` returning field name lists for VTU output.
:::
::: {lang=ja}
