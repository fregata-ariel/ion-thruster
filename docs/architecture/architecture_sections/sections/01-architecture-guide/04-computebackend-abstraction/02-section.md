
Rustのクロージャはコンパイル時に型が確定する。JITコンパイラ(Cranelift)やGPU(WGPU)には渡せない。データ記述なら:

- Craneliftで検査・コンパイルできる
- WGPUでcompute shaderに変換できる
- シリアライズしてキャッシュできる
- 最適化パスを走らせられる
