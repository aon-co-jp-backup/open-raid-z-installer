# PORTING

## 到達点(2026-11-15)
- 画面をTypeScript/ViteからRust(Leptos 0.8、WASM)へ書き直し、`cargo check`・`trunk build`・`cargo tauri build`(MSI 3.08MiB / NSISセットアップ 2.04MiB)まで成功。
- 実機: ビルドしたexeを起動し、200% DPIで1600x1200と700x1100の2サイズを確認。`open_runo_installer_core`の実データ(CpuFallback、非管理者のため「ディスクが検出されません」警告)を表示。Tauri命令をスタブしたブラウザ確認で、言語切替・RTL・ディスク選択→適用ボタン有効化・対応状況・ベンチマーク・RAID10プレビューが動作。
- **未確認**: 実ウィンドウでの「対応状況」ボタンのクリック(クリック後のキャプチャが取れず)、1600px幅での横スクロールバー(CSSは元のTS版と同一なので新規の不具合かは未切り分け)、下部固定ボタン、複数文字サイズ、MSI/NSISのインストール/アンインストール。初回起動で「対応状況」パネルが開いて見えた場面が1回あったが、再起動後は閉じており原因は未特定。
- `src-tauri/tauri.conf.json`の`beforeBuildCommand`を`cd ui && trunk build --release`に修正済み(trunkは`ui/`で実行する必要がある)。

## Status (2026-11-15)
- The UI was rewritten from TypeScript/Vite to Rust (Leptos 0.8, WASM); `cargo check`, `trunk build` and `cargo tauri build` (MSI 3.08 MiB / NSIS setup 2.04 MiB) all succeed.
- Real device: the built exe was launched and checked at 200% DPI in two sizes (1600x1200, 700x1100), showing real `open_runo_installer_core` data (CpuFallback; the "no disks detected" warning because it ran non-elevated). With the Tauri commands stubbed in a browser, language switching, RTL, disk selection enabling the apply button, the status panel, the benchmark and the RAID10 preview all worked.
- **Not verified**: clicking the "Compatibility" button in the real window (the post-click capture failed), the horizontal scrollbar at 1600 px width (the CSS is identical to the TS version, so whether it is new is not isolated), pinned bottom buttons, several font sizes, MSI/NSIS install/uninstall. The Compatibility panel looked open once on a first launch but was closed after a restart; cause not identified.
- `beforeBuildCommand` in `src-tauri/tauri.conf.json` was fixed to `cd ui && trunk build --release` (trunk must run inside `ui/`).

## 再開用メッセージ(別アカウント向け・2026-11-15)
次をそのままClaude Codeに貼り付けてください。

> `F:\open-raid-z-installer`(GitHub: aon-co-jp/open-raid-z-installer)の続きです。`CLAUDE.md`と`PORTING.md`の「再開用メッセージ」を読んでください。やること: (1) `src-tauri`で`cargo tauri build`(PATHに`%USERPROFILE%\.cargoin`、`CARGO_TARGET_DIR`は`F:db-target\oritauri`など)し、生成された`bundle/msi`と`bundle/nsis`を使って、実ウィンドウの通し試験(「対応状況」ボタンの開閉、1600px幅で出る横スクロールバーの原因、複数解像度・文字サイズ、主要ボタンの下部固定)とインストール/アンインストール試験を行う。(2) 全て通ったら、`F:\open-raid-z`の`open_runo_zfs_source/open_runo_installer`(旧TypeScript版)を削除してpushし、通った項目/通らなかった項目を正直に報告する。失敗が残るなら削除しない。兄弟配置(`../open-raid-z`・`../open-cpu`・`../RS-JSON`)が必要。日本語を含むファイルはPythonかEdit/Writeで編集し、PowerShellのSet-Contentは使わない。

## Resume message (for the other account, 2026-11-15)
Paste the following into Claude Code as is.

> Continue `F:\open-raid-z-installer` (GitHub: aon-co-jp/open-raid-z-installer). Read the "Resume message" in `CLAUDE.md` and `PORTING.md`. To do: (1) run `cargo tauri build` in `src-tauri` (put `%USERPROFILE%\.cargoin` on PATH; use a `CARGO_TARGET_DIR` such as `F:db-target\oritauri`), then use the generated `bundle/msi` and `bundle/nsis` to run the end-to-end tests: real window (open/close the "Compatibility" panel, find the cause of the horizontal scrollbar seen at 1600 px width, several resolutions and font sizes, primary buttons pinned to the bottom) and install/uninstall tests. (2) If everything passes, delete the old TypeScript `open_runo_zfs_source/open_runo_installer` in `F:\open-raid-z`, push, and report honestly what passed and what did not. If failures remain, do not delete. Sibling checkouts (`../open-raid-z`, `../open-cpu`, `../RS-JSON`) are required. Edit files containing Japanese with Python or Edit/Write, never PowerShell Set-Content.
