
::: {lang=ja}
ion-craftは、Rustで構築された汎用CFDシミュレーションフレームワークである。最初のユースケースはEHD（電気流体力学）[]{.term id=ionic_wind}シミュレーションだが、フレームワーク自体は物理非依存に設計されている。

設計の3本柱は以下の通り:

1. **汎用CFDコア + プラグイン物理モジュール**: メッシュ、場、離散化、線形代数、I/Oは物理に依存しない。EHDは差し替え可能な物理モジュールとして実装。将来的に熱伝達、反応流など異なる物理の追加が可能。
2. **ComputeBackend抽象レイヤー**: 「何を計算するか」と「どう実行するか」を分離。カーネルを`FaceOp`/`CellOp`という**検査可能なデータ記述**として構築し、バックエンドが解釈・コンパイルする。将来のJITコンパイラ(Cranelift)やGPU(WGPU)バックエンドに対応可能。
3. **データ指向設計**: 連続`Vec<f64>`配列、CSR圧縮接続、面ベース[]{.term id=fvm}。キャッシュ効率とSIMD auto-vectorization対応を重視。

フレームワークは11のRust crateで構成され、Cargo workspaceで管理される。各crateの責務は明確に分離され、依存関係は厳密に一方向（下流→上流）である。

ソルバの全体フローは:

1. Gmshメッシュファイルを読み込み、面ベースFVMトポロジを構築
2. 物理モジュールが`FieldRegistry`にフィールドを登録・初期化
3. []{.term id=operator_splitting}により連成方程式系を逐次サブステップに分割
4. 各タイムステップで`FaceKernel`/`CellKernel`記述を生成し、`ComputeBackend`で実行
5. VTU形式で結果を出力し、PVD時系列でParaView可視化に対応
:::

::: {lang=en}
ion-craft is a general-purpose CFD simulation framework built in Rust. While the first use case is EHD (electrohydrodynamic) []{.term id=ionic_wind} simulation, the framework itself is physics-agnostic by design.

Three design pillars:

1. **Generic CFD core + pluggable physics modules**: Mesh, fields, discretization, linear algebra, and I/O are physics-independent. EHD is implemented as a swappable physics module. Additional physics (heat transfer, reactive flows, etc.) can be added in the future.
2. **ComputeBackend abstraction layer**: Separates "what to compute" from "how to execute." Kernels are constructed as **inspectable data descriptions** (`FaceOp`/`CellOp` enums), which backends interpret or compile. Enables future JIT compiler (Cranelift) and GPU (WGPU) backends.
3. **Data-oriented design**: Contiguous `Vec<f64>` arrays, CSR-compressed connectivity, face-based []{.term id=fvm}. Prioritizes cache efficiency and SIMD auto-vectorization readiness.

The framework comprises 11 Rust crates managed as a Cargo workspace. Each crate has clearly separated responsibilities, and dependencies flow strictly in one direction (downstream → upstream).

The overall solver flow is:

1. Load a Gmsh mesh file and build face-based FVM topology
2. Physics modules register and initialize fields in the `FieldRegistry`
3. []{.term id=operator_splitting} decomposes the coupled equation system into sequential sub-steps
4. Each timestep generates `FaceKernel`/`CellKernel` descriptions, executed by the `ComputeBackend`
5. Output results in VTU format with PVD time series for ParaView visualization
:::
