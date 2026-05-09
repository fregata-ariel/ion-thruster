
各境界パッチに対して変数ごとの境界条件を指定する。パッチ名はGmshの物理グループ名と一致させる:

```toml
[boundary.emitter]
phi = { type = "dirichlet", value = 20000.0 }
ion_density = { type = "dirichlet", value = 1.0e15 }
velocity = { type = "no_slip" }

[boundary.collector]
phi = { type = "dirichlet", value = 0.0 }
ion_density = { type = "absorbing" }
velocity = { type = "no_slip" }

[boundary.farfield]
phi = { type = "neumann", value = 0.0 }
ion_density = { type = "outflow" }
velocity = { type = "outflow" }
```

対応する境界条件タイプ: `dirichlet`, `neumann`, `no_slip`, `outflow`, `absorbing`
