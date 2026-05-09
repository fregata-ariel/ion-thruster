
```
1. PoissonStep    : ρ_q → φ, E を計算（楕円型、CG求解）
2. IonTransportStep: E, u を使って n_i を更新（SG scheme）
3. EhdForceStep   : ρ_q × E → f_EHD を計算
4. FluidStep      : f_EHD を体積力として u, p を更新（圧力投影）
```
