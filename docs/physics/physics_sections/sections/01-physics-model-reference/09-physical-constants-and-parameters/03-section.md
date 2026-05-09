
| 記号 | 名称 | 典型値 | 単位 |
|---|---|---|---|
| $\mu_i$ | []{.term id=ion_mobility} | 2.0×10⁻⁴ | m²/(V·s) |
| $D_i$ | []{.term id=ion_diffusion} | 5.0×10⁻⁶ | m²/s |

これらのパラメータは`EhdConfig`構造体で保持され、TOML設定ファイルの`[physics]`セクションから読み込まれる:

```toml
[physics]
gas = "air"
rho_g = 1.225
mu_g = 1.81e-5
epsilon = 8.8541878128e-12
ion_mobility = 2.0e-4
ion_diffusion = 5.0e-6
```
:::

::: {lang=en}
Physical constants and parameters used in EHD simulation:
