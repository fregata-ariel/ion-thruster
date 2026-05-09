
```toml
[fluid]
model = "incompressible"
advection = "upwind"
pressure_solver = "cg"
dt = 1.0e-6        # 時間刻み (s)
steps = 100         # 最大ステップ数
cfl = 0.5           # CFL係数（適応dt時）
max_time = 1.0e-4   # 最大シミュレーション時間 (s)
```
