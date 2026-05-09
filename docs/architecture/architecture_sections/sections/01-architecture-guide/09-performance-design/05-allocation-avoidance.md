
- `LinearSystem` sparsity pattern is built once; only values are overwritten each step
- CG solver scratch buffers are pre-allocated on the `SplittingStep` struct
- Face flux temporary buffers are reused
