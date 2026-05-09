
::: {lang=ja}
非圧縮性[]{.term id=navier_stokes}に[]{.term id=ehd_body_force}を外力として加える:

$$
\rho_g \frac{\partial \mathbf{u}}{\partial t} + \rho_g (\mathbf{u} \cdot \nabla)\mathbf{u} = -\nabla p + \mu_g \nabla^2 \mathbf{u} + \mathbf{f}_{\text{EHD}}
$$

$$
\nabla \cdot \mathbf{u} = 0
$$

ここで $\rho_g$ = 1.225 kg/m³（[]{.term id=gas_density}）、$\mu_g$ = 1.81×10⁻⁵ Pa·s（[]{.term id=gas_viscosity}）（標準大気条件）。

[]{.term id=pressure_projection}で解く:

1. **予測ステップ**: 圧力項を除いた運動量方程式で仮速度 $\mathbf{u}^*$ を計算
2. **圧力Poisson方程式**: $\nabla^2 p = \frac{\rho_g}{\Delta t} \nabla \cdot \mathbf{u}^*$ を解く
3. **補正ステップ**: $\mathbf{u}^{n+1} = \mathbf{u}^* - \frac{\Delta t}{\rho_g} \nabla p$ で発散フリーに補正

移流項の離散化には風上差分（`AdvectionScheme::Upwind`）を使用する。将来的にTVDリミッター（Van Leer, MinMod, Superbee）を追加予定。

粘性項はラプラシアンカーネルで離散化:

```rust
let viscous = laplacian_kernel("velocity_x", mu_g, "momentum_rhs_x");
let advect  = advection_kernel("velocity_x", "velocity", AdvectionScheme::Upwind, "momentum_rhs_x");
```
:::

::: {lang=en}
The incompressible []{.term id=navier_stokes} with []{.term id=ehd_body_force} as an external force:

$$
\rho_g \frac{\partial \mathbf{u}}{\partial t} + \rho_g (\mathbf{u} \cdot \nabla)\mathbf{u} = -\nabla p + \mu_g \nabla^2 \mathbf{u} + \mathbf{f}_{\text{EHD}}
$$

$$
\nabla \cdot \mathbf{u} = 0
$$

where $\rho_g$ = 1.225 kg/m³ ([]{.term id=gas_density}), $\mu_g$ = 1.81×10⁻⁵ Pa·s ([]{.term id=gas_viscosity}) (standard atmospheric conditions).

Solved using the []{.term id=pressure_projection}:

1. **Predictor step**: compute tentative velocity $\mathbf{u}^*$ from the momentum equation without pressure
2. **Pressure Poisson equation**: solve $\nabla^2 p = \frac{\rho_g}{\Delta t} \nabla \cdot \mathbf{u}^*$
3. **Corrector step**: $\mathbf{u}^{n+1} = \mathbf{u}^* - \frac{\Delta t}{\rho_g} \nabla p$ to enforce divergence-free velocity

Advection discretization uses upwind differencing (`AdvectionScheme::Upwind`). TVD limiters (Van Leer, MinMod, Superbee) are planned for future addition.

The viscous term is discretized with the Laplacian kernel:

```rust
let viscous = laplacian_kernel("velocity_x", mu_g, "momentum_rhs_x");
let advect  = advection_kernel("velocity_x", "velocity", AdvectionScheme::Upwind, "momentum_rhs_x");
```
:::
