
```
1. PoissonStep     : ρ_q → φ, E (elliptic, CG solve)
2. IonTransportStep: update n_i using E, u (SG scheme)
3. EhdForceStep    : ρ_q × E → f_EHD
4. FluidStep       : update u, p using f_EHD (pressure projection)
```
