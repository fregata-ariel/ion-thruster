
::: {lang=ja}
シミュレーション全体のデータフローは以下の通り:

```text
Gmsh (.geo/.msh)
  → cfd-mesh: Mesh構造体（面ベースFVMトポロジ）
    → cfd-fields: FieldRegistry（場の登録・共有）
      → ehd-physics: PhysicsModule（場の初期化・ステップ定義）
        → cfd-time: OperatorSplitting（時間ループ制御）
          → cfd-fvm: カーネル記述（FaceKernel/CellKernel）
            → cfd-compute-cpu: カーネル実行
          → cfd-linalg: 線形系の組立・求解
        → cfd-io: VTU/PVD出力
          → ParaView可視化
```

具体的には、`ehd-cli`の`main()`で以下のように接続される:

```rust
// 1. 設定読込
let config = SimConfig::load(&path)?;

// 2. メッシュ読込・トポロジ構築
let mesh = cfd_mesh::gmsh::read_msh(&config.mesh.path)?;

// 3. 状態初期化 + 物理モジュールのフィールド登録
let mut state = SimState::new();
let ehd = EhdModule::new(ehd_config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

// 4. オペレータ分割の構成
let mut splitting = OperatorSplitting::new(cfl);
for step in ehd.splitting_steps() {
    splitting.add_step(step);
}

// 5. 出力ライターの設定
let mut writer = VtuWriter::new(&config.output.path, scalars, vectors);

// 6. シミュレーション実行
let mut driver = SimulationDriver { splitting, ... };
driver.run(&mesh, &mut state, &mut writer)?;
```

物理モジュール間のデータ共有は`FieldRegistry`を介して行う。各モジュールは名前付きフィールドを登録・参照するだけで、直接的なモジュール間参照は持たない。EHDモジュールは9個のフィールド（`phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`）を登録する。
:::

::: {lang=en}
The overall simulation data flow:

```text
Gmsh (.geo/.msh)
  → cfd-mesh: Mesh struct (face-based FVM topology)
    → cfd-fields: FieldRegistry (field registration & sharing)
      → ehd-physics: PhysicsModule (field initialization & step definition)
        → cfd-time: OperatorSplitting (time loop control)
          → cfd-fvm: kernel descriptions (FaceKernel/CellKernel)
            → cfd-compute-cpu: kernel execution
          → cfd-linalg: linear system assembly & solve
        → cfd-io: VTU/PVD output
          → ParaView visualization
```

Concretely, the wiring happens in `ehd-cli`'s `main()`:

```rust
// 1. Load config
let config = SimConfig::load(&path)?;

// 2. Read mesh and build topology
let mesh = cfd_mesh::gmsh::read_msh(&config.mesh.path)?;

// 3. Initialize state + register physics module fields
let mut state = SimState::new();
let ehd = EhdModule::new(ehd_config);
ehd.register_fields(&mut state.fields, &mesh);
ehd.initialize(&mut state, &mesh);

// 4. Configure operator splitting
let mut splitting = OperatorSplitting::new(cfl);
for step in ehd.splitting_steps() {
    splitting.add_step(step);
}

// 5. Set up output writer
let mut writer = VtuWriter::new(&config.output.path, scalars, vectors);

// 6. Run simulation
let mut driver = SimulationDriver { splitting, ... };
driver.run(&mesh, &mut state, &mut writer)?;
```

Data sharing between physics modules occurs through the `FieldRegistry`. Each module only registers and references named fields — no direct inter-module references exist. The EHD module registers 9 fields (`phi`, `electric_field`, `ion_density`, `charge_density`, `ehd_force`, `velocity`, `pressure`, `poisson_rhs`, `ion_rhs`).
:::
