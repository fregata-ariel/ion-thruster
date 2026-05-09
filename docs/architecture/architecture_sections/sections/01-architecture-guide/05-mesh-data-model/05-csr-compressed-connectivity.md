
Cell-to-face, cell-to-node, and face-to-node connectivity use CSR format (offsets array + indices array). This avoids heap-scattered `Vec<Vec<FaceId>>` and ensures cache efficiency and SIMD readiness.
