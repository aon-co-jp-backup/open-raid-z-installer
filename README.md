# open-raid-z-installer

**日本語**: [`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)用インストーラー(Tauri 2)。**画面をTypeScript/Viteから Rust(Leptos 0.8、WASM)へ書き直した**もので、機能・Tauriコマンド・9言語(英語既定・第二言語との併記・アラビア語/ペルシャ語はRTL)は元のまま。ディスク検出・構成助言・zpoolプレビューの本体は`open-raid-z`リポジトリ内の`open_runo_installer_core`をそのまま使う。

- `ui/` 画面(Leptos、`trunk`でビルド。`i18n_data.rs`は元の辞書から生成)
- `src-tauri/` Tauriの殻(コマンドは元のまま)
- 配置: 兄弟リポジトリとして`../open-raid-z`、`../open-cpu`、`../RS-JSON`が必要(`open-raid-z`と同じ流儀)。
- ビルド: `rustup target add wasm32-unknown-unknown`、`cargo install trunk tauri-cli`のあと、`src-tauri`で`cargo tauri build`(GPUは`-- --features gpu`)。
- 検証済み: `cargo check`(src-tauri・ui)、`trunk build`、Tauriを模したスタブでの画面操作(言語切替・RTL・ディスク選択と適用ボタンの有効化・対応状況・ベンチ・RAID10プレビュー)。**未検証**: 実機のTauriウィンドウでの通し、インストーラー(MSI)の作成と実機試験、複数解像度・文字サイズ。

**English**: Installer (Tauri 2) for [`open-raid-z`](https://github.com/aon-co-jp/open-raid-z). The **UI was rewritten from TypeScript/Vite to Rust (Leptos 0.8, WASM)**; features, Tauri commands and the 9 languages (English default, bilingual display, RTL for Arabic/Persian) are unchanged. Disk detection, advice and zpool preview still come from `open_runo_installer_core` in the `open-raid-z` repository.

- `ui/` the UI (Leptos, built with `trunk`; `i18n_data.rs` is generated from the original dictionaries)
- `src-tauri/` the Tauri shell (commands unchanged)
- Layout: sibling checkouts `../open-raid-z`, `../open-cpu`, `../RS-JSON` are required (same convention as `open-raid-z`).
- Build: `rustup target add wasm32-unknown-unknown`, `cargo install trunk tauri-cli`, then `cargo tauri build` in `src-tauri` (GPU: `-- --features gpu`).
- Verified: `cargo check` (src-tauri, ui), `trunk build`, and UI behaviour against a Tauri stub in a browser (language switching, RTL, disk selection enabling the apply button, status panel, benchmark, RAID10 preview). **Not verified**: a full run in a real Tauri window, building and testing the MSI installer, multiple resolutions/font sizes.

License: MPL-2.0.
