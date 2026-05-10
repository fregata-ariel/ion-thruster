
`examples/wire_plate_2d/wire_plate.geo` defines a 2D wire-to-plate electrode configuration:

- **Domain**: 60mm × 40mm
- **Wire (emitter)**: radius 1mm, positioned at 20mm gap
- **Plate (collector)**: bottom edge
- **Mesh sizes**: 0.5mm near wire, 1mm near plate, 5mm far field

Physical group definitions are critical — they must match the `[boundary.*]` section names in the TOML config:

```text
Physical Curve("collector") = {1};
Physical Curve("emitter") = {5, 6, 7, 8};
Physical Curve("farfield") = {2, 3, 4};
Physical Surface("fluid") = {1};
```

Mesh generation command:
```bash
gmsh -2 examples/wire_plate_2d/wire_plate.geo -format msh2 -o examples/wire_plate_2d/wire_plate.msh
```

`-format msh2` is required (ion-craft supports MSH 2.2 ASCII only).
:::
