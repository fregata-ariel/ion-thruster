
::: {lang=ja}
[]{.term id=ehd_body_force}は[]{.term id=charge_density}と[]{.term id=electric_field}の積:

$$
\mathbf{f}_{\text{EHD}} = \rho_q \mathbf{E}
$$

ここで $\rho_q = q \cdot n_i$。この体積力がNavier-Stokes方程式の外力項として[]{.term id=ionic_wind}を駆動する。

離散的にはセルごとの要素積として計算する。`CellOp::Multiply`で $\rho_q$ と $\mathbf{E}$ の各成分を掛け合わせる。
:::

::: {lang=en}
The []{.term id=ehd_body_force} is the product of []{.term id=charge_density} and []{.term id=electric_field}:

$$
\mathbf{f}_{\text{EHD}} = \rho_q \mathbf{E}
$$

where $\rho_q = q \cdot n_i$. This body force drives the []{.term id=ionic_wind} as an external force term in the Navier-Stokes equations.

Discretely, it is computed as a per-cell element-wise product. `CellOp::Multiply` multiplies $\rho_q$ with each component of $\mathbf{E}$.
:::
