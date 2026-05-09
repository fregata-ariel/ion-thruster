
- All fields are contiguous `Vec<f64>` / `Vec<[f64; 3]>` arrays
- Mesh connectivity uses CSR compression (`Vec<u32>` offsets + `Vec<Id>` indices)
- `HashMap` stays outside hot loops (field lookup is once per timestep per field)
