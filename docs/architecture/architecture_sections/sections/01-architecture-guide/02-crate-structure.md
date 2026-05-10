
::: {lang=ja}
フレームワークは11のRust crateで構成される。汎用CFDコア（7 crate）と物理固有部（2 crate）、およびCLIバイナリに分かれる。Cargo workspace（`resolver = "2"`）で依存関係とビルドを一括管理する。
