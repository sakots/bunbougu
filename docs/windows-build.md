# Windows でビルドする

Windows 10 / 11（x64）向けの手順です。WSL ではなく Windows の PowerShell を使います。

## 初回セットアップ

次のツールをインストールし、PowerShell を開き直してください。

- [Git for Windows](https://git-scm.com/download/win)
- [Node.js 24 LTS](https://nodejs.org/)
- [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)：「C++ によるデスクトップ開発」ワークロードを選択します。MSVC と Windows SDK が必要です。
- [Rustup](https://rustup.rs/)：Windows x64 の MSVC ツールチェーンを使います。
- [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/#download-section)：未導入の場合は Evergreen 版をインストールします。

## クローンとビルド

```powershell
rustup default stable-x86_64-pc-windows-msvc
npm.cmd install --global pnpm@12.15.0
git clone https://github.com/sakots/bunbougu.git
Set-Location bunbougu
pnpm.cmd install --frozen-lockfile
pnpm.cmd tauri build --no-bundle
.\src-tauri\target\release\bunbougu.exe
```

`--no-bundle` はインストーラーを作らず、起動確認用の `.exe` をビルドします。PowerShell では `npm.cmd` と `pnpm.cmd` を指定すると、実行ポリシーで `.ps1` がブロックされる環境でも起動できます。

開発中は `pnpm.cmd dev` で起動し、ブラウザーではなく Tauri のウィンドウを使ってください。
