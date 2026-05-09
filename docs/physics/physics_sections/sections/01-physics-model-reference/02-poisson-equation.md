
::: {lang=ja}
[]{.term id=electric_potential} $\phi$ は[]{.term id=poisson_eq}で決定される:

$$
\nabla \cdot (\varepsilon_0 \nabla \phi) = -\rho_q
$$

ここで $\varepsilon_0$ = 8.8541878128×10⁻¹² F/m（[]{.term id=permittivity}）、[]{.term id=charge_density} $\rho_q = q \cdot n_i$（$q$ = 1.602176634×10⁻¹⁹ C、[]{.term id=elementary_charge}）。

[]{.term id=fvm}による離散化では、セル $P$ に対して:

$$
\sum_f \varepsilon_0 A_f \frac{\phi_N - \phi_P}{|\mathbf{d}|} = -\rho_{q,P} V_P
$$

ここで $A_f$ は面積、$|\mathbf{d}|$ はowner-neighbor重心間距離、$V_P$ はセル体積。この離散化は`cfd-fvm`の`laplacian_kernel()`で記述される:

```rust
let kernel = laplacian_kernel("phi", epsilon_0, "poisson_rhs");
```

離散化した結果は対称正定値の疎行列系 $A\phi = b$ となり、Jacobi前処理付き[]{.term id=cg_solver}で求解する。

[]{.term id=electric_field}は電位の負の勾配として計算する:

$$
\mathbf{E} = -\nabla \phi
$$

離散的にはセル中心でGreen-Gaussの定理を適用し、面値の重み付き和から勾配を近似する。
:::

::: {lang=en}
The []{.term id=electric_potential} $\phi$ is determined by the []{.term id=poisson_eq}:

$$
\nabla \cdot (\varepsilon_0 \nabla \phi) = -\rho_q
$$

where $\varepsilon_0$ = 8.8541878128×10⁻¹² F/m ([]{.term id=permittivity}), and []{.term id=charge_density} $\rho_q = q \cdot n_i$ ($q$ = 1.602176634×10⁻¹⁹ C, []{.term id=elementary_charge}).

In the []{.term id=fvm} discretization, for cell $P$:

$$
\sum_f \varepsilon_0 A_f \frac{\phi_N - \phi_P}{|\mathbf{d}|} = -\rho_{q,P} V_P
$$

where $A_f$ is the face area, $|\mathbf{d}|$ is the owner-neighbor centroid distance, and $V_P$ is the cell volume. This discretization is described by `laplacian_kernel()` in `cfd-fvm`:

```rust
let kernel = laplacian_kernel("phi", epsilon_0, "poisson_rhs");
```

The discretized system yields a symmetric positive-definite sparse matrix system $A\phi = b$, solved by the Jacobi-preconditioned []{.term id=cg_solver}.

The []{.term id=electric_field} is computed as the negative gradient of the potential:

$$
\mathbf{E} = -\nabla \phi
$$

Discretely, the Green-Gauss theorem is applied at cell centers, approximating the gradient from weighted face-value sums.
:::
