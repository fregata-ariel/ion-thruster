
```toml
[fluid]
model = "incompressible"
advection = "upwind"
pressure_solver = "cg"
dt = 1.0e-6        # 時間刻み (s)
steps = 100         # 最大ステップ数
cfl = 0.5           # CFL係数（省略可、適応dt時に使用）
max_time = 1.0e-4   # 最大シミュレーション時間（省略可、省略時は無制限）
```

`pressure_solver = "cg"` は[]{.term id=cg_solver}を使用。時間積分は[]{.term id=operator_splitting}で連成方程式系を逐次的に解く。
