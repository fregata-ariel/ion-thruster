
::: {lang=ja}
連成方程式系は[]{.term id=operator_splitting}で解く。各タイムステップで以下の4つのサブステップを逐次実行する:

```
1. PoissonStep    : ρ_q → φ, E を計算（楕円型、CG求解）
2. IonTransportStep: E, u を使って n_i を更新（SGスキーム、陽的時間積分）
3. EhdForceStep   : ρ_q × E → f_EHD を計算（代数的、dtに依存しない）
4. FluidStep      : f_EHD を体積力として u, p を更新（圧力投影法）
```

この順序が重要な理由:

- PoissonStepは楕円型であり、$\rho_q$ から $\phi, \mathbf{E}$ を即座に決定する。先に解くことで、後続ステップが最新の電場を使える。
- IonTransportStepはドリフト拡散の時間発展であり、最新の $\mathbf{E}$ と $\mathbf{u}$ を必要とする。
- EhdForceStepは代数的な計算のみ（$\mathbf{f} = \rho_q \mathbf{E}$）であり、安定性制約がない。
- FluidStepは最後に実行し、更新された体積力で速度場を補正する。

時間刻み $\Delta t$ は全ステップの`max_dt()`の最小値にCFL係数を乗じて決定する:

$$
\Delta t = C_{\text{CFL}} \cdot \min_s \Delta t_s^{\max}
$$

楕円型ステップ（Poisson）は`f64::INFINITY`を返す（無条件安定）。IonTransportStepのCFL制約がドリフト速度に基づく最も厳しい制約となる。
:::

::: {lang=en}
The coupled equation system is solved using []{.term id=operator_splitting}. Four sub-steps are executed sequentially each timestep:

```
1. PoissonStep     : ρ_q → φ, E (elliptic, CG solve)
2. IonTransportStep: update n_i using E, u (SG scheme, explicit time integration)
3. EhdForceStep    : ρ_q × E → f_EHD (algebraic, dt-independent)
4. FluidStep       : update u, p using f_EHD (pressure projection)
```

The ordering matters:

- PoissonStep is elliptic and immediately determines $\phi, \mathbf{E}$ from $\rho_q$. Solving it first provides up-to-date electric field for subsequent steps.
- IonTransportStep performs drift-diffusion time evolution, requiring the latest $\mathbf{E}$ and $\mathbf{u}$.
- EhdForceStep is purely algebraic ($\mathbf{f} = \rho_q \mathbf{E}$) with no stability constraint.
- FluidStep runs last, correcting the velocity field with the updated body force.

The timestep $\Delta t$ is determined as the CFL coefficient times the minimum `max_dt()` across all steps:

$$
\Delta t = C_{\text{CFL}} \cdot \min_s \Delta t_s^{\max}
$$

Elliptic steps (Poisson) return `f64::INFINITY` (unconditionally stable). The IonTransportStep CFL constraint based on drift velocity is typically the most restrictive.
:::
