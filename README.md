# ぶんぼうぐやさん

ぶんぼうぐやさんは、obsidianとscrapBoxをあわせたようなアプリを目指します。

現在の版では、ノートの作成・編集・端末内保存と、`[[ページ名]]` での移動ができます。端末間同期は未対応です。

## 開発

Node.js、pnpm、Rust、[Tauri 2 のプラットフォーム別依存関係](https://v2.tauri.app/start/prerequisites/)を用意してください。

```sh
pnpm install
pnpm dev
```

ノートの編集は、起動した Tauri のウィンドウで行ってください。ブラウザーで開発用 URL を開いても、端末内のノートにはアクセスできません。

Windows（x64）で試す場合は、[クローンから実行ファイルを起動するまでの手順](docs/windows-build.md)を参照してください。

Android は Android SDK/NDK のセットアップ後に `pnpm tauri android init`、`pnpm tauri android dev` を実行してください。

仕様は [ローカルノート](docs/local-notes.md)、全履歴は [changelog.md](changelog.md) を参照してください。

## 更新履歴

### [2026/11/24] v0.1.0

- ノートを作成・編集し、端末内に保存する機能を追加。
- `[[ページ名]]` でノート間を移動する機能を追加。
- `pnpm dev` でアプリが起動し、ブラウザーで開いた場合は起動方法を案内するよう修正。
- Windows のクローン・ビルド手順と、GitHub Actions による Windows ビルドチェックを追加。
