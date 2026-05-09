
::: {lang=ja}
以下のツールが必要:

| ツール | バージョン | 用途 |
|---|---|---|
| Rust (rustc + cargo) | 1.85+ (edition 2024) | フレームワークのビルド |
| Gmsh | 4.x | メッシュ生成 (`.geo` → `.msh`) |
| ParaView | 5.x | 結果の可視化 (`.vtu` / `.pvd`) |

Rustのインストール:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
:::

::: {lang=en}
The following tools are required:

| Tool | Version | Purpose |
|---|---|---|
| Rust (rustc + cargo) | 1.85+ (edition 2024) | Build the framework |
| Gmsh | 4.x | Mesh generation (`.geo` → `.msh`) |
| ParaView | 5.x | Result visualization (`.vtu` / `.pvd`) |

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
:::
