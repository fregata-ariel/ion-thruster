
Internal faces first, boundary faces after:

```text
Face indices: [0 .. n_internal) [n_internal .. n_total)
               ├─ internal ─────┤ ├─ boundary ──────────┤
                                   ├─ patch 0 ─┤ ├─ patch 1 ─┤ ...
```

This convention matches OpenFOAM and enables the hottest inner loop (internal face flux) to run branch-free with contiguous memory access.
