> 私:  
ある雰囲気（空気や、低密度酸素雰囲気など）中に電極対を配置して、電極間に高電圧をかけた際の空間中の分子の電離をシミュレートするための基礎理論を知りたいです。

***

> ChatGPT:  

やりたいことは、ざっくり言うと **気体放電・低温プラズマの初期発生シミュレーション** です。
電極間に高電圧をかけたとき、空間中の中性分子がどう電離し、電子雪崩・ストリーマ・グロー放電・アークへ進むかを見る問題ですね。

最初に全体像を言うと、基礎理論はこの4層です。

1. **静電場・電磁場**：電極形状と電圧から電場を求める
2. **電子・イオンの輸送**：電場で荷電粒子が移動・拡散する
3. **電子衝突反応**：電子が分子に衝突して励起・電離・付着・解離する
4. **空間電荷効果**：生成された電子・イオン自身が電場を歪める

このうち、低温・低密度・空気/酸素中の放電開始を考えるなら、まずは **Poisson 方程式 + ドリフト拡散方程式 + 反応率モデル** が基本になります。

---

## 1. まずは電場：Poisson 方程式

電極に電圧を与えると、空間に電位 φ ができ、電場は

$$
\mathbf{E} = -\nabla \phi
$$

で与えられます。

空間中に電荷密度 ρ がある場合、電位は Poisson 方程式で決まります。

$$
\nabla \cdot \left(\varepsilon \nabla \phi\right) = -\rho
$$

ここで

$$
\rho = e(n_i - n_e + \cdots)
$$

です。
$n_e$ は電子密度、$n_i$ は正イオン密度です。負イオンを入れるなら $n_-$ も入ります。

重要なのは、初期段階では電場はほぼ電極形状で決まりますが、電離が進むと電子・イオンによる **空間電荷** が増えて、電場分布そのものが変わることです。この空間電荷による電場歪みがストリーマ形成の本体です。

---

## 2. 電子の運動：粒子として見るか、流体として見るか

シミュレーションには大きく2系統あります。

### A. 粒子モデル：PIC-MCC

電子やイオンを多数の粒子、正確には超粒子として追跡します。

$$
m\frac{d\mathbf{v}}{dt} = q\mathbf{E}
$$

で加速し、一定確率で中性分子と衝突させます。衝突は Monte Carlo Collision, MCC として扱います。

この方法では、電子エネルギー分布や非平衡性を比較的自然に扱えます。低圧・希薄気体・非局所効果が強い場合に向いています。ただし計算コストが高いです。

### B. 流体モデル：ドリフト拡散近似

電子密度やイオン密度を連続場として扱います。放電開始やストリーマの連続体モデルではこちらがよく使われます。

電子密度 (n_e) については、典型的には

$$
\frac{\partial n_e}{\partial t} + \nabla \cdot \boldsymbol{\Gamma}_e = S_e
$$

と書きます。

電子フラックスは

$$
\boldsymbol{\Gamma}_e
=
-\mu_e n_e \mathbf{E}
-D_e \nabla n_e
$$

のように置きます。
$\mu_e$ は電子移動度、$D_e$ は拡散係数、$S_e$ は電離・付着・再結合などによる生成消滅項です。

イオンについても同様に

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot \boldsymbol{\Gamma}_i = S_i
$$

を書きます。ただしイオンは電子よりずっと重いので、応答は遅く、拡散も小さいことが多いです。

---

## 3. 電離の本体：電子衝突反応

高電圧で直接「分子が電場に引き裂かれる」というより、多くの場合はまず少数の自由電子が電場で加速され、中性分子に衝突します。

代表的な反応はこうです。


$$
e + \mathrm{O_2} \rightarrow 2e + \mathrm{O_2^+}
$$

$$
e + \mathrm{N_2} \rightarrow 2e + \mathrm{N_2^+}
$$

$$
e + \mathrm{O_2} \rightarrow \mathrm{O_2^-}
$$

最後は酸素で重要な **電子付着** です。空気や酸素では、電離だけでなく電子付着で自由電子が減るため、窒素・アルゴンなどより放電開始の条件が変わります。

反応率は電子エネルギー分布関数 $f(\epsilon)$ と衝突断面積 $\sigma(\epsilon)$ から決まります。

$$
k = \int_0^\infty \sigma(\epsilon),v(\epsilon),f(\epsilon),d\epsilon
$$

ここで $k$ が反応速度係数です。
このため、真面目にやるなら **電子衝突断面積データ** が必要になります。LXCat は低温非平衡プラズマモデリング向けに電子—中性粒子散乱断面積や swarm parameter を集めたオープンなデータベースとして使われます。NIST も電子衝突電離・励起断面積のデータベースを提供しています。([LxCat][1])

---

## 4. 最初の近似：Townsend 電離

もっとも基礎的な放電開始モデルは **Townsend avalanche** です。

自由電子が電場で加速され、衝突電離で電子数が指数的に増えます。距離 (x) 進むごとに電子数が

$$
n_e(x) = n_e(0)e^{\alpha x}
$$

のように増える、というモデルです。
$\alpha$ は **第一 Townsend 電離係数** で、「電子が単位長さ進む間に何回電離を起こすか」を表します。Townsend 放電は、自由電子が電場で加速され、気体分子と衝突して追加電子を生み、それが雪崩的に増える過程として説明されます。([ウィキペディア][2])

この $\alpha$ は多くの場合、換算電場

$$
E/N
$$

の関数として扱います。
$E$ は電場、$N$ は中性粒子密度です。

ここがとても大事です。
気体放電では単なる $E$ ではなく、**換算電場 $E/N$** が効きます。低圧・低密度になると、同じ電場でも分子間の平均自由行程が長くなり、電子が衝突までにより大きなエネルギーを得られるからです。

---

## 5. Paschen 則：絶縁破壊電圧の第一近似

平行平板電極、均一気体、定常条件という理想化では、絶縁破壊電圧は圧力 $p$ と電極間距離 $d$ の積 $pd$ に主に依存します。これが **Paschen 則** です。

ざっくり言えば、

* 圧力が高すぎると、電子はすぐ衝突して十分加速できない
* 圧力が低すぎると、そもそも衝突相手が少ない
* その中間に、最小破壊電圧がある

という関係です。

ただし Paschen 則は初期見積もり用です。電極が針状、表面粗さがある、酸素付着が強い、湿度がある、空間電荷が効く、ストリーマになる、などの条件では簡単に外れます。

---

## 6. 酸素・空気で特に重要な反応

空気や低密度酸素雰囲気なら、最低限こういう化学種を考えます。

### 空気の簡略モデル

中性種：

$$
\mathrm{N_2}, \mathrm{O_2}
$$

荷電種：

$$
e,\ \mathrm{N_2^+},\ \mathrm{O_2^+},\ \mathrm{O_2^-}
$$

主な過程：

* 電子衝突電離
  $$
  e + \mathrm{N_2} \rightarrow 2e + \mathrm{N_2^+}
  $$
  $$
  e + \mathrm{O_2} \rightarrow 2e + \mathrm{O_2^+}
  $$

* 電子付着
  $$
  e + \mathrm{O_2} \rightarrow \mathrm{O_2^-}
  $$

* 再結合
  $$
  e + \mathrm{O_2^+} \rightarrow \text{neutral products}
  $$

* 光電離
  空気中の正ストリーマではかなり重要になることがあります。

酸素を含む気体では電子付着があるため、電子密度が単純に増え続けるわけではありません。電離係数 $\alpha$ と付着係数 $\eta$ を用いて、有効電離係数を

$$
\alpha_{\mathrm{eff}} = \alpha - \eta
$$

のように扱うことがあります。

$$
\alpha_{\mathrm{eff}} > 0
$$

なら電子雪崩が成長し、

$$
\alpha_{\mathrm{eff}} < 0
$$

なら電子はむしろ失われます。

---

## 7. Boltzmann 方程式：輸送係数を得る上位理論

ドリフト拡散モデルで使う

$$
\mu_e,\ D_e,\ k_{\mathrm{ion}},\ k_{\mathrm{att}}
$$

などは、電子エネルギー分布から決まります。その電子エネルギー分布は本来、Boltzmann 方程式から求めます。

$$
\frac{\partial f}{\partial t} + \mathbf{v}\cdot\nabla_{\mathbf{x}} f + \frac{q\mathbf{E}}{m}\cdot\nabla_{\mathbf{v}} f = C[f]
$$

右辺 $C[f]$ が衝突項です。

ただしこれをフルに解くのは大変なので、実用上は

* BOLSIG+
* LoKI-B
* LXCat の swarm data
* PIC-MCC
* 既存文献の transport coefficient table

などから (E/N) に対する係数表を作り、流体シミュレーションに入れることが多いです。LXCat では BOLSIG+ のような Boltzmann equation solver と組み合わせた swarm parameter 計算が使われています。([nl.lxcat.net][3])

---

## 8. 最小モデル：放電開始だけを見るなら

最初の自作シミュレーションなら、いきなり空気の完全反応系に行かず、以下の「最小流体モデル」から始めるのがよいです。

対象：

$$
e,\ i^+
$$

だけ。

方程式：

$$
\nabla \cdot (\varepsilon \nabla \phi) = -e(n_i - n_e)
$$

$$
\frac{\partial n_e}{\partial t}
=

\nabla \cdot \left(\mu_e n_e \nabla \phi + D_e \nabla n_e \right)
+
S_{\mathrm{ion}}
$$

$$
\frac{\partial n_i}{\partial t}
=

-\nabla \cdot \boldsymbol{\Gamma}_i
+
S_{\mathrm{ion}}
$$

電離源項：

$$
S_{\mathrm{ion}} = \alpha(E) |\boldsymbol{\Gamma}_e|
$$

または

$$
S_{\mathrm{ion}} = k_{\mathrm{ion}}(E/N) n_e N
$$

このモデルだけでも、電場が強い場所で電子密度が指数的に増え、空間電荷が電場を変形し始める様子は見えます。

---

## 9. 低密度酸素雰囲気なら何が変わるか

低密度酸素の場合、普通の空気放電より次の要素が効きやすいです。

### 平均自由行程が長い

中性粒子密度 $N$ が小さいので、電子は衝突までに長く飛びます。
局所的な $E/N$ モデルが破綻しやすく、電子エネルギーが「その場所の電場」だけで決まらない場合があります。

その場合は流体モデルより **PIC-MCC** の方が自然です。

### 酸素付着が効く

酸素は電子を捕獲して負イオンを作りやすいので、

$$
\mathrm{O_2^-}
$$

を入れる必要があります。
これを入れないと、電子雪崩を過大評価しがちです。

### 表面過程が効く

電極表面からの二次電子放出が放電維持に効きます。

$$
i^+ + \text{cathode} \rightarrow e + \cdots
$$

この係数を第二 Townsend 係数 $\gamma$ として扱います。Townsend 理論では、気体中の雪崩電離に加え、陽イオン衝突などによる陰極からの二次電子放出が自己持続放電の条件に入ります。([ウィキペディア][2])

---

## 10. シミュレーション手法の選び方

目的別にはこんな感じです。

| 目的              | 適したモデル                                      |
| --------------- | ------------------------------------------- |
| 破壊電圧のざっくり見積もり   | Townsend / Paschen                          |
| 電極形状による電場集中を見る  | 静電場 FEM                                     |
| 電子雪崩の成長を見る      | 流体モデル                                       |
| ストリーマ形成を見る      | Poisson + drift-diffusion + photoionization |
| 低圧・低密度で非局所効果を見る | PIC-MCC                                     |
| 詳細な空気プラズマ化学を見る  | 多種流体 + 反応ネットワーク                             |
| アーク・熱・流体まで見る    | MHD / 熱流体結合                                 |

最初のステップとしては、

1. 電極形状を決める
2. Poisson 方程式で初期電場を求める
3. $E/N$ から $\alpha, \mu_e, D_e$ を与える
4. 電子密度とイオン密度の時間発展を解く
5. 電荷密度で Poisson 方程式を再計算する
6. これを時間ステップで回す

という流れがよいです。

---

## 11. 実装上の注意点

この手の問題は数値的にかなり硬いです。

理由は、

* 電子の時間スケールが非常に短い
* イオンは遅い
* 電場は楕円型方程式で瞬時に全体へ効く
* 電子密度が指数的に増える
* 反応係数が $E/N$ に強く依存する
* 電極近傍で電場が発散的に強くなる

からです。

そのため、有限差分で雑に陽解法を組むとすぐ壊れます。
使うなら、

* 有限体積法
* Scharfetter–Gummel スキーム
* upwind flux
* adaptive mesh refinement
* implicit / semi-implicit time integration
* positivity-preserving scheme

あたりがキーワードになります。

特にドリフト拡散方程式は、半導体デバイスシミュレーションの drift-diffusion とかなり似ています。電極間放電版の半導体デバイス、みたいな気持ちで見ると理解しやすいです。

---

## 12. 何を勉強すればよいか

順番としては、次が良いと思います。

### 第1段階：気体放電の古典理論

* Townsend avalanche
* 第一 Townsend 係数 (\alpha)
* 第二 Townsend 係数 (\gamma)
* Paschen law
* electron attachment
* breakdown condition

ここで「なぜ電圧・圧力・ギャップ長が効くのか」を掴みます。

### 第2段階：低温プラズマの輸送

* electron energy distribution function, EEDF
* Boltzmann equation
* mobility
* diffusion coefficient
* reaction rate coefficient
* reduced electric field (E/N)

ここで「電子衝突データから反応率を作る」考え方を掴みます。

### 第3段階：数値モデル

* Poisson equation
* continuity equation
* drift-diffusion model
* source terms
* boundary conditions
* photoionization
* secondary emission

### 第4段階：粒子法

* PIC
* MCC
* null-collision method
* electron-neutral collision cross sections
* particle weighting
* charge deposition

---

## 13. かなり実用的な最初のモデル案

坂本さんの目的が「電極対 + 高電圧 + 空気/低密度酸素中の電離」をまず掴むことなら、最初はこれで十分です。

### 2D軸対称または2D平面

針対平板、または平行平板。

### 変数

$$
\phi,\ n_e,\ n_p,\ n_n
$$

ここで

* (n_e)：電子
* (n_p)：正イオン、例えば (\mathrm{O_2^+})
* (n_n)：負イオン、例えば (\mathrm{O_2^-})

### 反応

$$
e + \mathrm{O_2} \rightarrow 2e + \mathrm{O_2^+}
$$

$$
e + \mathrm{O_2} \rightarrow \mathrm{O_2^-}
$$

$$
e + \mathrm{O_2^+} \rightarrow \text{neutral}
$$

$$
\mathrm{O_2^-} + \mathrm{O_2^+} \rightarrow \text{neutral}
$$

### 電場

$$
\mathbf{E} = -\nabla \phi
$$

$$
\nabla \cdot (\varepsilon \nabla \phi)
=
-e(n_p - n_e - n_n)
$$

### 係数

$$
\mu_e(E/N),\quad D_e(E/N),\quad k_{\mathrm{ion}}(E/N),\quad k_{\mathrm{att}}(E/N)
$$

をテーブルで持つ。

これが「低密度酸素中の電離を見る」ための最初のちゃんとしたモデルになります。

---

## 14. 注意：高電圧・酸素雰囲気はかなり危険

実験に進むなら、ここは本当に注意です。高電圧は感電だけでなく、オゾン生成、NOx、紫外線、火災、酸素富化による可燃物の発火リスクがあります。低密度酸素や酸素雰囲気は、普通の空気より燃焼リスクが跳ねます。シミュレーション段階でも、実験設計に使う場合は安全規格・絶縁距離・インターロック・排気を前提にしたほうがよいです。

---

## まとめ

基礎理論の芯はこれです。

$$
\text{電極電圧}
\rightarrow
\mathbf{E}
\rightarrow
\text{電子加速}
\rightarrow
\text{衝突電離}
\rightarrow
\text{電子雪崩}
\rightarrow
\text{空間電荷}
\rightarrow
\text{電場変形}
\rightarrow
\text{放電成長}
$$

シミュレーションとしては、まず

**Poisson 方程式 + 電子/イオン連続方程式 + ドリフト拡散 + Townsend/反応率モデル**

から始めるのが良いです。
低密度酸素まで本気で見るなら、次に **Boltzmann solver 由来の輸送係数**、さらに必要なら **PIC-MCC** へ進む、という流れになります。

[1]: https://us.lxcat.net/notes/index.php?download=lxcat_team5&utm_source=chatgpt.com "The LXCat project: electron scattering cross sections and ..."
[2]: https://en.wikipedia.org/wiki/Townsend_discharge?utm_source=chatgpt.com "Townsend discharge"
[3]: https://nl.lxcat.net/notes/index.php?download=lxcat_team1&utm_source=chatgpt.com "Electron scattering cross sections and swarm parameters ..."

***

> 私:  
テーマとしてはリフターやイオンクラフトとして知られているものが良いですね。簡単な作りの割に複雑な相互作用が見られて面白いです。なんとなくですが、周囲のイオン化していない空気もろとも動作させているきがします。

***

> ChatGPT:  

はい、その感覚はかなり本質的です。
**リフター / イオンクラフトの主役は「イオンそのもの」だけではなく、むしろ大量の中性空気を巻き込んで作る流れ**です。

つまり、

* 先端電極近傍で空気が一部だけ電離する
* 生成されたイオンが電場で加速される
* そのイオンが周囲の**電離していない中性分子**に衝突して運動量を渡す
* 結果として、空気全体の流れ（**イオン風 / ionic wind / EHD flow**）が生じる
* その反作用として推力が出る

というのが基本像です。

---

# 1. リフターの理解の中心は「コロナ放電 + 電気流体力学」

リフターは、物理的にはたいてい

* 細いワイヤ電極（高電界を作る）
* 広い集電極（アルミ箔など）

の組です。

細い電極の周囲では電場が非常に強くなり、そこで**コロナ放電**が起きます。
しかし空間全体が一様にプラズマになるわけではありません。

むしろ空間は大きく分けて：

## (A) 電離領域（ionization region）

* ワイヤ近傍の非常に狭い領域
* ここでは電場が強く、電子が加速されて分子を電離
* 新しいイオン・電子がどんどん作られる

## (B) ドリフト領域（drift region）

* 電極間の大部分
* ここではもはや強い電離はそこまで起きず、
* すでにできたイオンが電場で移動し、
* 中性分子と衝突して流れを作る

という構造になります。

ここが重要で、**リフターの推力の主な発生場所は、狭い電離領域そのものというより、広いドリフト領域でのイオン-中性分子の運動量交換**です。

---

# 2. 「中性空気もろとも動いている」はその通り

ユーザーの直感通りです。
リフターは「イオンだけを飛ばしている」のではなく、実際には**ほとんど中性の空気を引きずって流している**と考えたほうがよいです。

なぜかというと、大気圧近傍ではイオンの平均自由行程は短く、すぐに中性分子へ衝突します。
したがってイオンは単独で高速に飛び続けるというより、

* 電場で少し加速
* すぐ衝突
* 運動量を中性分子へ渡す
* また加速
* また衝突

を繰り返します。

この繰り返しによって、周囲の空気全体に「体積力」がかかるような形になり、ジェット流ができます。

---

# 3. 一番簡単な理論図式

リフターを理論的に見ると、最低限次の3つが結合しています。

## 3.1 電場

まず電極形状と印加電圧から電場を決めます。

$$
\mathbf{E} = -\nabla \phi
$$

$$
\nabla \cdot (\varepsilon \nabla \phi) = -\rho_e
$$

* (\phi): 電位
* (\mathbf{E}): 電場
* (\rho_e): 空間電荷密度

リフターでは空間電荷が無視できないことが多いので、単なる真空中の静電場ではなく、**イオンが作る空間電荷が電場を変形する**のが重要です。

---

## 3.2 荷電粒子の輸送

たとえば正イオン密度 (n_i) は、連続体近似では

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot \Gamma_i = S_i
$$

$$
\Gamma_i = \mu_i n_i \mathbf{E} - D_i \nabla n_i + n_i \mathbf{u}
$$

で書けます。

* $\mu_i$: イオン移動度
* $D_i$: 拡散係数
* $\mathbf{u}$: 気体流速
* $S_i$: 電離・再結合などの生成消滅項

ここでフラックス (\Gamma_i) には

1. 電場によるドリフト
2. 拡散
3. 気流に乗る対流

が入っています。

---

## 3.3 気体の運動

そして中性気体は、Navier–Stokes 方程式に**電気的な体積力**を入れて扱います。

$$
\rho_g\left(
\frac{\partial \mathbf{u}}{\partial t}

+ \mathbf{u}\cdot\nabla\mathbf{u}
  \right)
  =
  -\nabla p
+ \mu_g \nabla^2 \mathbf{u}
+ \mathbf{f}_{EHD}
$$

ここで重要なのが

$$
\mathbf{f}_{EHD} \approx \rho_e \mathbf{E}
$$

です。
これが **EHD body force**（電気流体力学的体積力）です。

この $\rho_e \mathbf{E}$ が、イオンを介して実際には中性流体を押している、と解釈できます。

---

# 4. リフターの推力は何が本質か

よく「静電引力で浮いているのでは？」と思われがちですが、通常のリフターはそうではなく、**下向きに吹くイオン風の反作用**で持ち上がります。

つまり本質は

* 電場がイオンを押す
* イオンが空気を押す
* 空気が下向きに流れる
* 反作用で機体が上向きに力を受ける

です。

真空中で普通のリフターがうまく動かないのは、この「周囲の中性分子に運動量を渡す」相手がいなくなるからです。
これは「中性空気もろとも動いている」という見方の非常に強い裏付けです。

---

# 5. なぜ簡単な構造なのに複雑か

おっしゃる通り、作りは単純なのに相互作用はかなり複雑です。理由は、

## 5.1 電場が幾何形状に敏感

細線の半径、集電極の曲率、距離、端部形状で電場集中が大きく変わります。

## 5.2 電離が非線形

少し電場が強くなると電離率が急増します。
閾値を超えると急に電流が流れ始めます。

## 5.3 空間電荷が電場を変える

できたイオンが新しい電場分布を作り、元の静電場を変えてしまいます。

## 5.4 気流が荷電粒子輸送を変える

発生した流れが、今度はイオンの分布も運ぶため、双方向結合になります。

## 5.5 化学反応もある

空気中では

* $N_2^+$
* $O_2^+$
* $O_2^-$
* オゾン $O_3$

などが絡みます。特に酸素は電子付着があるため、単純な正イオン流だけでは済まないことがあります。

---

# 6. リフターの最小モデル

もし「理論の芯だけ押さえたい」なら、最初は以下の簡略化がよいです。

## モデル

* 2次元断面
* 細線陽極 + 平板陰極
* 正コロナ放電
* 主な荷電種は正イオン1種類
* 非圧縮性空気

## 方程式

### 電場

$$
\nabla \cdot (\varepsilon \nabla \phi) = -q n_i
$$

### イオン輸送

$$
\frac{\partial n_i}{\partial t} + \nabla \cdot (\mu_i n_i \mathbf{E} + n_i \mathbf{u}) = S_i
$$

### 流体

$$
\rho_g\left(
\frac{\partial \mathbf{u}}{\partial t}

+ \mathbf{u}\cdot\nabla \mathbf{u}
  \right)
  =
  -\nabla p + \mu_g \nabla^2 \mathbf{u} + q n_i \mathbf{E}
$$

これだけでも、

* ワイヤ近傍で電荷生成
* 電極間に空間電荷雲ができる
* 下向きのイオン風が生じる
* 推力が発生する

という現象のかなり本質部分が見えてきます。

---

# 7. さらに一段深い理解：正コロナと負コロナ

リフターでは通常、**細いワイヤを正極**にすることが多いです。
これは正コロナの方が比較的安定に扱いやすいからです。

## 正コロナ

* ワイヤ近傍で電離
* 正イオンが外へ流れる
* 比較的安定なグロー状/コロナ状になりやすい

## 負コロナ

* 電子放出やパルス的挙動が強くなりやすい
* トリッキーで不安定になりやすい
* オゾン生成や放電形態も変わる

そのため理論的にも、まずは**正コロナ型イオンクラフト**を基準に考えるのが理解しやすいです。

---

# 8. 推力の簡略評価式

詳細式ではありませんが、EHD推進では経験的・近似的に

$$
T \sim \frac{I,d}{\mu_i}
$$

のような形で推力を見積もることがあります。

* $T$: 推力
* $I$: コロナ電流
* $d$: 電極間距離の代表長
* $\mu_i$: イオン移動度

これは「電流が多いほど、また電荷が長い距離で力を受けるほど、より多くの運動量を気体へ渡す」という感覚に対応しています。
もちろん実際には幾何形状・空間電荷・流れの損失が入るので、厳密式というよりスケーリング則です。

---

# 9. 理論を勉強する順番

リフターを理解する目的なら、次の順番がかなり良いです。

## 第1段階

* クーロン力
* 電場と電位
* Poisson 方程式
* 電極周辺の電場集中

## 第2段階

* コロナ放電
* Townsend 電離
* Paschen 則
* 空間電荷

## 第3段階

* イオン移動度
* ドリフト・拡散
* 連続方程式
* EHD body force (\rho_e E)

## 第4段階

* Navier–Stokes 方程式
* ジェット・噴流
* 境界層
* 電気流体力学連成

---

# 10. リフターの「見かけ以上の本体」

直感的にまとめると、リフターは

**「プラズマ装置」というより、むしろ“極端に薄いプラズマ層で駆動される送風機”**
として理解すると分かりやすいです。

* 電離している領域はごく一部
* でもその少数のイオンが
* 大量の中性空気に力を伝え
* 全体の噴流を作る

という構図です。

なので、観測される現象が複雑なのは当然で、

* 放電現象
* 荷電粒子輸送
* 流体力学
* 表面現象
* 気体化学

が全部少しずつ入っています。

---

# 11. もし次に進めるなら

次にやると面白いのは、以下のどちらかです。

## A. 理論寄り

「リフター用の最小方程式系」を、変数・仮定・境界条件つきで整理する

## B. シミュレーション寄り

2次元断面の
**細線-平板電極のイオンクラフトの簡易モデル**
を、有限差分/FEMでどう組むかを段階的に整理する

---

必要なら次に、

1. **リフターに特化した支配方程式の最小セット**
2. **正コロナ放電の境界条件の置き方**
3. **Python / FEM での簡易シミュレーション構成案**

のどれかを、もう少し具体的に書けます。

***

> 私:  
いっそシミュレーションによって流体の動作を計算、保存してそれを流体が動く様子として表示してみたいですね。シミュレータ本体はRustで書いてみたいですね。電極の配置を3dモデルで与えられると良いですね。見た目と精度の観点から見ると流体の小さな渦が表現できるくらいの精度でセルを切る必要があります。電極周辺でより細かくセルを構成する仕組みも併せて作るか、ライブラリがあればそれを使いましょう。

***

> ChatGPT:  

かなり良い方向です。
リフター/イオンクラフトは、**見た目は薄い三角フレームなのに、中身は「コロナ放電 + イオン輸送 + 非圧縮性流体 + メッシュ生成 + 可視化」の総合格闘技**になります。沼の香りがします。いい沼です。

結論から言うと、最初の実装方針はこうするのが現実的です。

> **Rust で EHD-CFD ソルバ本体を書く。
> 電極形状は STEP/STL から Gmsh で3Dメッシュ化する。
> 電極近傍は Gmsh の mesh size field で細かく切る。
> 結果は VTK/XDMF/HDF5 に保存して ParaView か自作ビューアで流れを可視化する。**

本当の動的AMRまで最初から作るより、まずは **静的な局所細分化メッシュ**で始めるのが良いです。

---

## 1. 物理モデルの最初の到達点

リフターを対象にするなら、最初から電子衝突まで全部解くより、まずは **単極性イオン流 + 中性流体** の EHD モデルが良いです。

つまり、ワイヤ近傍の詳細な電離は境界条件・注入条件として扱い、空間中では

* 電位 $\phi$
* 電場 $\mathbf{E}$
* 空間電荷密度 $\rho_q$
* 正イオン密度 $n_i$
* 気体速度 $\mathbf{u}$
* 圧力 $p$

を解きます。

イオン風の基本は、細い高電圧電極近傍で空気が電離し、正イオンが陰極側へドリフトし、その移動中に中性分子へ運動量を渡す、という描像です。二本線電極の実験論文でも、細いワイヤ周辺の高電場で電離し、正イオンが移動中に中性分子へ運動量を渡して “ionic wind” が生じる、と説明されています。

---

## 2. 支配方程式の最小セット

まずはこの4本です。

### Poisson 方程式

$$
\nabla \cdot (\epsilon \nabla \phi) = -\rho_q
$$

$$
\mathbf{E} = -\nabla \phi
$$

### イオン輸送

$$
\frac{\partial n_i}{\partial t}
+
\nabla \cdot \Gamma_i
=

S_i
$$

$$
\Gamma_i
=
\mu_i n_i \mathbf{E}
-D_i \nabla n_i
+n_i \mathbf{u}
$$

ここで $\mu_i n_i \mathbf{E}$ が電場によるドリフト、$-D_i \nabla n_i$ が拡散、$n_i \mathbf{u}$ が流体に乗る対流です。

### 電気流体力

$$
\mathbf{f}_{EHD} = \rho_q \mathbf{E}
$$

### 非圧縮性 Navier–Stokes

$$
\nabla \cdot \mathbf{u} = 0
$$

$$
\rho_g
\left(
\frac{\partial \mathbf{u}}{\partial t}
+
\mathbf{u} \cdot \nabla \mathbf{u}
\right)
=
-\nabla p
+
\mu_g \nabla^2 \mathbf{u}
+
\rho_q \mathbf{E}
$$

この形なら、**イオンが中性空気を押して渦や噴流を作る**ところが直接見えます。

コロナ放電誘起イオン風の既存数値モデルでも、Navier–Stokes、Poisson、電荷連続式、場合によっては熱輸送を含む強結合PDE系として扱われています。OpenFOAM上の実装例では、半導体の電荷輸送で使われる Gummel 型の staggered algorithm を一般化した解法が使われています。([arXiv][4])

---

## 3. 最初は「完全な電離シミュレーション」ではなく「コロナ電極からのイオン注入」にする

ここはかなり重要です。

本気で電離まで解くと、

* 電子密度
* 正イオン
* 負イオン
* 電子エネルギー分布
* 電離
* 付着
* 再結合
* 光電離
* 二次電子放出

が入ります。これは最初のRust実装としてはデカすぎます。

リフター可視化目的なら、まずはこう割り切るのが良いです。

### 細線陽極の近傍

「ここでコロナ放電が起きて、一定量の正イオンが注入される」とする。

境界条件例：

$$
n_i = n_{i,emit}
$$

または

$$
\Gamma_i \cdot \mathbf{n} = J_i/q
$$

### 空間中

イオンは電場でドリフトし、空気に運動量を渡す。

### 集電極

イオンを吸収する。

$$
n_i = 0
$$

または outflow / absorbing boundary。

これなら「リフターの見た目の流体挙動」をかなり早く出せます。
電離物理は後で追加できます。

---

## 4. メッシュ生成：Rust内製より Gmsh 連携がよい

3D電極配置をモデルで与えるなら、**Gmshを外部メッシュ生成器として使う**のが一番現実的です。

Gmsh は STEP を OpenCASCADE カーネルで読み込めますし、STL のリメッシュも扱えます。Gmsh のチュートリアルでは、OpenCASCADE カーネルによる STEP ファイル読み込み、STL のリメッシュが説明されています。([gmsh.info][5])

### 形状入力

おすすめはこの順です。

1. **STEP**
   CAD由来の電極形状。境界面や物理グループを扱いやすい。

2. **STL**
   Blenderや簡易モデラから出しやすい。ただし境界面の意味づけが弱い。

3. **Gmsh .geo**
   針電極、ワイヤ、リング、板などをパラメトリックに生成するには便利。

最初は `.geo` でワイヤ・集電極・外部流体領域を作り、後から STEP/STL に対応すると良いです。

---

## 5. 電極周辺の局所細分化

これは Gmsh の **mesh size field** を使うのが良いです。

Gmsh は各メッシュ生成ステップで mesh size field によって要素サイズを制御でき、距離境界・曲率・背景フィールドなどに基づいて局所的な要素サイズを指定できます。電極近傍だけ細かく、遠方は粗く、という設計に向いています。([gmsh.info][5])

たとえば考え方としては：

* ワイヤ電極表面から 0.1〜1 mm は非常に細かい
* 電極間の主流路は中程度
* 外部境界付近は粗い
* 曲率が大きいところは自動的に細かくする
* 電極先端・エッジ・角は最小セルサイズを強制する

という分布にします。

擬似的にはこうです。

```text
distance_to_emitter < r1  -> h_min
distance_to_emitter < r2  -> interpolate(h_min, h_mid)
else                      -> h_far
```

本当の動的AMR、つまり流れや空間電荷の発展を見ながら途中で再メッシュする仕組みは、最初から入れると相当大変です。まずは **静的な距離ベース細分化**で十分です。

---

## 6. 「小さな渦」を見るには、メッシュより時間刻みも効く

小さな渦を表現したい場合、単にセルを細かくすればよいわけではありません。

必要になるのは：

* 渦の長さスケールより十分小さいセル
* CFL条件を満たす時間刻み
* 数値粘性の小さい移流スキーム
* 圧力投影の精度
* 境界層近傍の解像度
* 必要なら LES / VLES / DNS 的な扱い

です。

リフターのイオン風は、条件によって層流から遷移的な流れになり得ます。近年の needle-ring positive corona の研究でも、正コロナ誘起イオン風は層流から乱流遷移的な流れになる可能性が高い、と扱われています。([Hal Science][6])

最初の目標としては、DNS で乱流微細構造まで解くより、

> **渦が見える程度の非定常2D/3D流れを、数値粘性を抑えた有限体積法で出す**

くらいが良いです。

---

## 7. Rust 実装の全体構成案

こういう分割が良いです。

```text
ehdcraft/
  crates/
    ehd-core/          # 物理量、単位、場、メッシュ、セル隣接
    ehd-mesh/          # Gmsh/VTK/独自mesh読み込み
    ehd-solver/        # Poisson, charge transport, fluid solver
    ehd-io/            # HDF5, XDMF, VTK 出力
    ehd-vis/           # 後でBevy/WGPU可視化
    ehd-cli/           # 実行用CLI
  examples/
    wire_plate_2d/
    wire_collector_3d/
    triangle_lifter/
```

Rust本体では、最初からCADやメッシュ生成まで抱え込まないほうが良いです。
**Gmsh → メッシュファイル → Rustソルバ → VTK/XDMF → ParaView** のパイプラインにするのが安全です。

---

## 8. 数値解法の構成

最初の解法は **staggered / operator splitting** が扱いやすいです。

1 timestep で：

```text
1. Poisson:
   rho_q から phi を解く
   E = -grad(phi)

2. Ion transport:
   E と u を使って n_i を更新

3. EHD force:
   f = rho_q E を計算

4. Fluid:
   Navier-Stokes を1ステップ進める
   圧力投影で div(u)=0 にする

5. Output:
   u, p, phi, E, rho_q, vorticity を保存
```

この形なら、電荷輸送と流体を完全同時に解くより実装がずっと楽です。

---

## 9. メッシュ形式：まずは非構造格子

3D電極形状を扱うなら、最初は非構造格子が自然です。

Gmsh の 3D メッシュなら主に tetra 要素になります。
ParaView/VTK も、点・セル・接続情報からなるメッシュ構造を基本にしていて、tetra/hexa などの有限要素・有限体積メッシュを扱えます。([docs.paraview.org][7])

Rust側の内部表現は、最初はこんな感じで十分です。

```rust
struct Mesh {
    points: Vec<Vec3>,
    cells: Vec<Cell>,
    faces: Vec<Face>,
    cell_faces: Vec<Vec<FaceId>>,
    face_cells: Vec<[Option<CellId>; 2]>,
    boundary_faces: Vec<BoundaryFace>,
}
```

有限体積法なら、特に face-based な情報が重要です。

```rust
struct FaceGeom {
    center: Vec3,
    normal: Vec3,
    area: f64,
    owner: CellId,
    neighbor: Option<CellId>,
}
```

---

## 10. 出力・可視化

ここは最初から割り切って、**ParaViewで見る**のが良いです。

Rustから直接アニメーションビューアを作るより、まずは可視化用ファイルを吐くべきです。

候補は：

### VTK / VTU

* ParaViewでそのまま読める
* 非構造格子と相性が良い
* Rust では `vtkio` が使える候補

`vtkio` は Legacy VTK と XML VTK 形式の parser/writer を目指す Rust crate です。([crates.io][8])

### XDMF + HDF5

* 大規模時系列に向く
* HDF5に実データ、XDMFにメタデータ
* ParaView/VisIt で読みやすい

Rust の `xdmf` crate は、ParaView/VisIt で可視化するための XDMF ファイルを書けるライブラリで、XDMF はメタデータをXMLに、実データを一般にHDF5へ置く形式だと説明されています。([Docs.rs][9])

最初は VTU 連番で良いです。

```text
output/
  frame_000000.vtu
  frame_000001.vtu
  frame_000002.vtu
  ...
```

保存する場：

```text
cell data:
  pressure
  charge_density
  ion_density
  ehd_force

point or cell vector data:
  velocity
  electric_field

derived:
  vorticity
  speed
  E_magnitude
```

ParaViewなら stream tracer, glyph, volume rendering, slice, contour, vorticity coloring がすぐできます。これは自作ビューアより圧倒的に早いです。

---

## 11. 線形ソルバ：ここがRust単体では一番悩ましい

Poisson方程式と圧力Poissonを解くので、疎行列ソルバが必要です。

Rust純正で全部やるなら：

* `sprs`
* `nalgebra-sparse`
* `faer`
* `russell_sparse`

あたりが候補です。

ただし本格的な非構造格子・大規模3D・反復解法・前処理まで考えるなら、PETSc連携を視野に入れた方が良いです。PETSc はKrylov法、直接法、前処理などの疎線形ソルバ群を持ち、PETSc公式ドキュメントにも sparse linear solvers の一覧があります。Rust binding の `petsc-rs` は作業中のプロジェクトですが、PETScの並列線形/非線形ソルバやODE積分器をRustから使う方向のものです。([PETSc][10])

おすすめは段階を分けることです。

### Phase 1

構造格子または小規模非構造格子で、自前CG/BiCGSTAB + 簡単なJacobi/ILUもどき。

### Phase 2

`sprs` / `russell_sparse` / `faer` などで疎行列まわりを整理。

### Phase 3

PETSc連携。大規模化・MPI化・ちゃんとした前処理。

---

## 12. 既存Rust CFDライブラリは、研究用の土台としてはまだ薄い

RustでCFDソルバを書く例は出てきていますが、この用途にそのまま使える成熟ライブラリはまだ少ないです。

たとえば ORC は Rust 製の3D非構造格子CFDソルバですが、README上では arbitrary unstructured grids を扱える一方、現状は laminar and steady と説明されています。つまり、リフターの非定常EHD流をそのまま載せるには不足があります。([GitHub][11])

なので方針としては、

> 既存Rust CFDライブラリを「使う」というより、
> メッシュI/O、線形代数、出力形式、並列化だけライブラリを使い、
> EHD用ソルバは自作する

が良いと思います。

---

## 13. 最初の実装ロードマップ

### Step 0：可視化パイプラインだけ作る

まず物理を解かず、Gmshのメッシュを読み、適当なベクトル場をVTK/XDMFで出す。

例：

```text
u = vortex_field(x, y, z)
phi = analytic_potential
rho_q = gaussian_near_wire
```

これをParaViewで表示する。

ここで「流体が動く様子を保存して見る」流れを先に固めます。

---

### Step 1：2D軸対称または2D断面

いきなり3Dリフターに行かず、

* wire-to-plate
* needle-to-ring
* two-wire

から始めるのが良いです。

二本線コロナ放電の実験では、陽極近傍のコロナ、電流、電極距離、電極径、極性とイオン風速度の関係が測定されており、正放電では定常電流、負放電ではTrichel pulseが観測されています。こうした単純形状は検証ケースとして良いです。

---

### Step 2：Poisson + イオン輸送

流体を止めたまま、

```text
phi
E
n_i
rho_q
```

だけを解く。

ここで空間電荷が電場分布を歪めるところを見る。

---

### Step 3：Navier–Stokes を追加

[
\mathbf{f}_{EHD} = \rho_q \mathbf{E}
]

を体積力として入れて、流速場を出す。

この段階で初めて「中性空気もろとも動く」様子が出ます。

---

### Step 4：3D電極形状

Gmshで

* 細線電極
* アルミ箔集電極
* 外部流体領域
* 開放境界

をメッシュ化する。

最初は三角リフター全体ではなく、短い断面か、1本の emitter-collector pair が良いです。

---

### Step 5：渦・非定常流の改善

ここでようやく、

* higher-order advection
* bounded TVD scheme
* vorticity output
* LES的なフィルタ
* 局所メッシュ細分化
* 時間刻み自動調整

を入れる。

---

## 14. リポジトリ設計案

`Cargo.toml` workspace はこんな感じがよさそうです。

```toml
[workspace]
members = [
  "crates/ehd-core",
  "crates/ehd-mesh",
  "crates/ehd-solver",
  "crates/ehd-io",
  "crates/ehd-cli",
]
```

### `ehd-core`

```rust
pub struct ScalarField<T> {
    pub values: Vec<T>,
    pub location: FieldLocation, // Cell, Face, Point
}

pub struct VectorField<T> {
    pub values: Vec<[T; 3]>,
    pub location: FieldLocation,
}
```

### `ehd-mesh`

```rust
pub trait MeshReader {
    fn read(path: &Path) -> Result<Mesh>;
}

pub trait MeshRefinementMetric {
    fn target_size(&self, x: Vec3) -> f64;
}
```

### `ehd-solver`

```rust
pub trait Equation {
    fn assemble(&self, mesh: &Mesh, state: &State) -> LinearSystem;
}

pub trait TimeStepper {
    fn step(&mut self, state: &mut State, dt: f64) -> Result<()>;
}
```

### `ehd-io`

```rust
pub trait FieldWriter {
    fn write_frame(&mut self, mesh: &Mesh, state: &State, step: usize) -> Result<()>;
}
```

---

## 15. 最初の設定ファイル例

TOMLでこういう設定にすると扱いやすいです。

```toml
[mesh]
path = "mesh/wire_plate.msh"
format = "gmsh"

[physics]
gas = "air"
rho_g = 1.225
mu_g = 1.81e-5
epsilon = 8.8541878128e-12
ion_mobility = 2.0e-4
ion_diffusion = 5.0e-6

[electrodes.emitter]
boundary = "emitter"
voltage = 20000.0
ion_flux = 1.0e15

[electrodes.collector]
boundary = "collector"
voltage = 0.0
ion_absorbing = true

[fluid]
model = "incompressible"
advection = "upwind"
pressure_solver = "cg"
dt = 1.0e-6
steps = 10000

[output]
format = "vtu"
every = 20
path = "output/"
fields = ["velocity", "pressure", "phi", "electric_field", "ion_density", "charge_density", "vorticity"]
```

---

## 16. メッシュ生成はこういう考え方

`.geo` 側では、

```text
- emitter を Physical Surface / Curve として名前づけ
- collector を Physical Surface として名前づけ
- farfield/open boundary を Physical Surface として名前づけ
- fluid volume を Physical Volume として名前づけ
- emitter からの距離に基づく mesh size field を設定
```

Rust側は、Gmshの物理名を読んで境界条件に割り当てます。

```text
"emitter"   -> high voltage + ion injection
"collector" -> ground + ion absorption
"farfield"  -> open boundary
"solid"     -> no slip wall
```

ここをちゃんとやると、CADモデルを差し替えてもソルバ側をほぼ変更せずに済みます。

---

## 17. AMRについての現実的な方針

電極近傍を細かくしたいだけなら、**Gmshの静的細分化で十分**です。

動的AMRが必要になるのは、

* 電荷密度の鋭い前線が移動する
* ストリーマヘッドを追跡する
* 渦が移動・崩壊する
* 局所的なせん断層だけ追いたい

という段階です。

ただし、非構造格子で動的AMRをやると、

* 再メッシュ
* 場の補間
* 保存量の保守
* 境界条件の再設定
* 時系列出力
* 並列分割

が全部絡みます。

なので最初は：

1. Gmshの距離ベース細分化
2. 解いた結果から誤差指標・渦度・電荷勾配を見る
3. その結果をもとに再度Gmshでメッシュ生成
4. 再計算

という **offline adaptive loop** が良いです。

---

## 18. 最小プロトタイプのゴール

最初のマイルストーンはこれです。

> **2Dまたは疑似3Dの wire-to-plate モデルで、電極間に生じる EHD body force によって中性空気のジェットが発生し、ParaViewで流線・渦度・速度分布をアニメーション表示できる。**

この時点でかなり楽しいはずです。

保存する可視化は：

* 速度ベクトル
* 流線
* 渦度
* 圧力
* 電位
* 電場強度
* 空間電荷密度
* イオン密度
* EHD force

です。

特に見た目としては、

```text
vorticity magnitude
stream tracer
glyph arrows
slice plane
isosurface of charge density
```

を出すと、「イオン風が空気を巻き込んでいる」感じが見えやすいと思います。

---

## 19. いちばん良い設計判断

このプロジェクトは、最初から全部Rustで閉じようとしないほうが良いです。

おすすめ構成は：

```text
CAD / geometry
  STEP, STL, .geo

mesh generation
  Gmsh

solver
  Rust

linear algebra
  initially custom / sprs-like
  later PETSc

output
  VTU first
  XDMF + HDF5 later

visualization
  ParaView first
  Bevy/WGPU viewer later
```

これなら、Rustで書きたい「シミュレータ本体」の純度を保ちながら、メッシュ生成と可視化という巨大な周辺機能を外部の強い道具に逃がせます。

---

## 20. 次に作るなら、この順番が堅いです

最初の実装タスクはこうです。

1. Gmsh `.msh` 読み込み
2. cell / face / boundary 構造の構築
3. VTU または XDMF 出力
4. Poisson 方程式ソルバ
5. イオン輸送ソルバ
6. EHD force 計算
7. 非圧縮性流体ソルバ
8. 時系列保存
9. ParaViewで可視化
10. STEP/STLモデル入力対応
11. 電極近傍 mesh size field 自動生成
12. offline adaptive loop

まずは **「流体が動いて見える」まで最短で行く**のが良いです。
電離の詳細化、酸素化学、動的AMR、乱流モデルはその後で十分です。

[4]: https://arxiv.org/abs/1306.6578?utm_source=chatgpt.com "Multiphysics simulation of corona discharge induced ionic wind"
[5]: https://gmsh.info/doc/texinfo/ "Gmsh 4.15.2"
[6]: https://hal.science/hal-04637760/file/2024_needlering.pdf?utm_source=chatgpt.com "self-consistent modeling and turbulence effect"
[7]: https://docs.paraview.org/en/latest/UsersGuide/understandingData.html "4. Understanding Data — ParaView Documentation 6.1.0 documentation"
[8]: https://crates.io/crates/vtkio?utm_source=chatgpt.com "vtkio - crates.io: Rust Package Registry"
[9]: https://docs.rs/xdmf "xdmf - Rust"
[10]: https://petsc.org/release/overview/linear_solve_table/ "Summary of Sparse Linear Solvers Available In PETSc — PETSc 3.25.1 documentation"
[11]: https://github.com/reidprichard/ORC?utm_source=chatgpt.com "reidprichard/ORC: ORC is a computational fluid dynamics ..."
