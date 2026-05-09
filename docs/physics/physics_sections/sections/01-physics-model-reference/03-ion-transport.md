
::: {lang=ja}
[]{.term id=ion_density} $n_i$ の時間発展は[]{.term id=drift_diffusion}で記述される:

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot (\mu_i n_i \mathbf{E} - D_i \nabla n_i) = 0
$$

ここで $\mu_i$ = 2.0×10⁻⁴ m²/(V·s)（[]{.term id=ion_mobility}）、$D_i$ = 5.0×10⁻⁶ m²/s（[]{.term id=ion_diffusion}）。

第1項 $\mu_i n_i \mathbf{E}$ はドリフト（電場による輸送）、第2項 $D_i \nabla n_i$ は拡散を表す。この2つの効果が競合するため、通常の中心差分や風上差分では数値振動や過度な数値拡散が生じる。[]{.term id=scharfetter_gummel}がこの問題を解決する。
:::

::: {lang=en}
The time evolution of []{.term id=ion_density} $n_i$ is described by the []{.term id=drift_diffusion}:

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot (\mu_i n_i \mathbf{E} - D_i \nabla n_i) = 0
$$

where $\mu_i$ = 2.0×10⁻⁴ m²/(V·s) ([]{.term id=ion_mobility}), and $D_i$ = 5.0×10⁻⁶ m²/s ([]{.term id=ion_diffusion}).

The first term $\mu_i n_i \mathbf{E}$ represents drift (transport by the electric field), and the second term $D_i \nabla n_i$ represents diffusion. Because these two effects compete, standard central or upwind differencing produces either numerical oscillation or excessive numerical diffusion. The []{.term id=scharfetter_gummel} resolves this problem.
:::
