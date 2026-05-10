
`examples/wire_plate_2d/wire_plate.geo` は2Dワイヤー-プレート電極構成を定義する:

- **ドメイン**: 60mm × 40mm
- **ワイヤー（emitter）**: 半径1mm、ギャップ20mm位置
- **プレート（collector）**: 底辺
- **メッシュサイズ**: ワイヤー近傍 0.5mm、プレート近傍 1mm、遠方 5mm

物理グループの定義が重要。TOML設定の`[boundary.*]`セクション名と一致させる:

```text
Physical Curve("collector") = {1};
Physical Curve("emitter") = {5, 6, 7, 8};
Physical Curve("farfield") = {2, 3, 4};
Physical Surface("fluid") = {1};
```

メッシュ生成コマンド:
```bash
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 -o examples/wire_plate_2d/wire_plate.msh
```

`-format msh2` が必要（ion-craftはMSH 2.2 ASCIIのみ対応）。
:::

::: {lang=en}
Define geometry in Gmsh `.geo` files and generate `.msh` files (MSH 2.2 ASCII).
