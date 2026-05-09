
::: {lang=ja}
EHDシミュレーションは4つの連成方程式系で構成される:

1. **[]{.term id=poisson_eq}** — 空間電荷密度から電位分布を決定
2. **[]{.term id=drift_diffusion}** — 電場によるドリフトと拡散によるイオン輸送
3. **EHD体積力** — 空間電荷と電場の積
4. **[]{.term id=navier_stokes}** — EHD体積力を外力とする非圧縮性流れ

物理的な因果連鎖は以下の通り:

$$
\text{[]{.term id=corona_discharge}} \xrightarrow{\text{イオン生成}} n_i
\xrightarrow{q \cdot n_i} \rho_q
\xrightarrow{\nabla \cdot (\varepsilon \nabla \phi) = -\rho_q} \phi
\xrightarrow{-\nabla \phi} \mathbf{E}
\xrightarrow{\rho_q \mathbf{E}} \mathbf{f}_{\text{EHD}}
\xrightarrow{\text{NS方程式}} \mathbf{u}
$$

[]{.term id=ionic_wind}は、コロナ放電で生成されたイオンが中性分子に運動量を渡して生じる巨視的な気流である。
:::

::: {lang=en}
The EHD simulation consists of four coupled equation systems:

1. **[]{.term id=poisson_eq}** — determines the electric potential distribution from space charge density
2. **[]{.term id=drift_diffusion}** — ion transport via electric field drift and diffusion
3. **EHD body force** — product of space charge density and electric field
4. **[]{.term id=navier_stokes}** — incompressible flow with EHD body force as an external force

The physical causal chain is:

$$
\text{[]{.term id=corona_discharge}} \xrightarrow{\text{ion generation}} n_i
\xrightarrow{q \cdot n_i} \rho_q
\xrightarrow{\nabla \cdot (\varepsilon \nabla \phi) = -\rho_q} \phi
\xrightarrow{-\nabla \phi} \mathbf{E}
\xrightarrow{\rho_q \mathbf{E}} \mathbf{f}_{\text{EHD}}
\xrightarrow{\text{NS eqs.}} \mathbf{u}
$$

[]{.term id=ionic_wind} is the macroscopic gas flow produced when corona-generated ions transfer momentum to neutral molecules.
:::
