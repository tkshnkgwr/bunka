# 変更履歴 (Changelog)

このプロジェクトのすべての重要な変更は、このファイルに記録されます。

このファイルのフォーマットは [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) に基づいており、
このプロジェクトは [セマンティック バージョニング](https://semver.org/spec/v2.0.0.html) に準拠しています。

## [0.4.3] - 2026-07-01

### 追加 (Added)
- **Rustおよびプラットフォームのステータスバッジの追加**:
  - `README.md` および `README.ja.md` のタイトル直下に、Rustのエディション（2024）および動作対象プラットフォーム（Windows | macOS | Linux）のバッジを追加。

### 修正 (Fixed)
- **ライセンスステータスバッジの表示不具合の修正**:
  - `README.md` および `README.ja.md` のライセンスバッジ（MIT）の画像URLが大文字小文字の誤り（`License` ではなく `license`）により表示されていなかった問題を、正しいURL（`license-MIT-yellow.svg`）に修正することで解消。

## [0.4.2] - 2026-06-30


### 修正 (Fixed)
- **GitHub Actions ワークフロー定義の修正**:
  - `ci.yml` および `release.yml` において、無効なバージョンが指定されていた `actions/checkout@v7` を `actions/checkout@v4` に修正。
  - `release.yml` において、無効なバージョンが指定されていた `softprops/action-gh-release@v3` を `softprops/action-gh-release@v2` に修正。
  - これにより GitHub Actions の解析エラーが解消され、README のビルドステータスバッジが正常に表示されるように対応。

## [0.4.1] - 2026-06-30

### 追加 (Added)
- **開発環境のアップデートと検証**:
  - 開発およびテストの実行環境を Rust 1.96.0 にアップデートし、CLI版・GUI版のビルド健全性および全単体テストが正常にパスすることを確認。
  - Rust 1.96.0 環境でのバイナリフットプリント（ファイルサイズ）を再測定し、[docs/FOOTPRINTS.md](docs/FOOTPRINTS.md) および [docs/TEST_REPORT.md](docs/TEST_REPORT.md) を更新。
- **READMEでのステータスバッジ追加**:
  - `README.md` および `README.ja.md` のタイトル直下に、GitHub Actions の CI ビルドステータスバッジと MIT ライセンスバッジを追加。
- **開発ガイドラインの更新**:
  - `.agents/AGENTS.md` に、READMEの更新時にステータスバッジや多言語リンクの設置状況を維持・確認することを義務付けるルールを追加。

### 変更 (Changed)
- **GUI版の不具合修正とUIの再調整**:
  - 日本語環境を持たないデフォルトフォントでの文字化け（トーフ表示）を防ぐため、UI上のテキストをすべて英語（Decimal, Max Denom, Tolerance, RESULT, APPROXIMATION）に統一。
  - 特殊な記号（✕、📋）も文字化けを防ぐため、ASCII文字（X、Copy）に変更。
  - 画面レイアウトの見切れを防ぐため、ウィンドウの初期サイズを `300x195` から `320x220` に拡張。
  - 枠なしウィンドウのヘッダー領域に対して明示的に `Sense::drag` を割り当て、ウィンドウ移動ができない不具合を修正。
- **README.mdとREADME.ja.mdの相互リンク**:
  - `README.md` に日本語版（`README.ja.md`）へのリンク、`README.ja.md` に英語版（`README.md`）へのリンクを相互に追加。
- **VS Code 設定警告の解決**:
  - `.vscode/settings.json` における `editor.defaultFormatter` (`rust-lang.rust-analyzer`) の無効値警告を解決するため、該当設定項目を削除。推奨拡張機能ファイル `.vscode/extensions.json` を新規作成して補完。
- **Clippy警告・エラーの修正**:
  - `src/lib.rs` で発生していた Clippy 警告（ドキュメント後の不要な空行）を修正。
  - テスト関数内の円周率近似リテラルによる Clippy エラーを回避するため、`#[allow(clippy::approx_constant)]` 属性を追加。
- **ソースコードのフォーマット整形**:
  - プロジェクト全体で `cargo fmt` を実行し、コードのインデントやスタイル規則を統一。

## [0.4.0] - 2026-06-26

### 追加 (Added)
- **GUI版のUX/UI向上と新機能**:
  - 仕様通りの「最前面表示（always on top）」を実際に有効化。
  - ウィンドウサイズを `320x220` に固定し、要素の見切れが発生しないように拡張。
  - 枠なしウィンドウの右上に「X（閉じる）ボタン」を追加し、GUI上からのアプリ終了に対応。
  - 結果表示エリアの横に「Copy」ボタンを新設し、ワンクリックで変換結果の分数をクリップボードにコピー可能に変更。
  - eguiのカスタムテーマと角丸・半透明デザインを適用し、透過オーバーレイとして美しいビジュアルを実現。
  - 日本語環境を持たないデフォルトフォントでの文字化け（トーフ表示）を防ぐため、UI上のテキスト（Decimal, Max Denom, Tolerance, RESULT, APPROXIMATION）をすべて英語化。
  - 枠なしウィンドウのヘッダー部分をドラッグして移動できるように `Sense::drag` によるインタラクション領域を割り当て（移動できない不具合を修正）。

### 変更 (Changed)
- **ソースコードのモジュール分割（リファクタリング）**:
  - `src/main.rs` から共通アルゴリズムとテストを `src/lib.rs` へ、CLI用のパースと実行を `src/cli.rs` へ、GUI用の描画と実行を `src/gui.rs` へそれぞれ分離。`src/main.rs` はそれらを呼び分けるエントリーポイントのみに変更。
- **システムドキュメントの更新**:
  - 実装に合わせて `docs/SPEC.md` および `docs/DIAGRAM.md` の記述・構成図を最新化（サイズ仕様なども `320x220` に修正）。

## [0.3.0] - 2026-06-26

### 変更 (Changed)
- **GUI依存ライブラリのメジャーアップデートとAPI追従**:
  - `eframe` クレートを `0.22.0` から `0.35.0` へアップデート。それに伴い `eframe::App` の `update` メソッドから `ui` メソッドへの変更に対応。
  - `NativeOptions` での `decorated` / `transparent` 設定を最新の `ViewportBuilder` 経由に修正。
  - `_frame.drag_window()` によるウィンドウドラッグ処理を `ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag)` へ変更。
  - `run_native` アプリ生成クロージャが `Result` を返すように変更されたことに伴い、戻り値を `Ok(...)` で包み込むように変更。
- **Windows API依存関係のアップデート**:
  - `windows` クレートを `0.48.0` から `0.62.0` へアップデート。`windows::core::w` からインポート不可になった問題に対し、マクロ `windows::core::w!` を直接修飾して呼び出す形式に修正。
- **開発テンプレートの最新化**:
  - [docs/project_template_guide.md](docs/project_template_guide.md) に最新の `Cargo.toml` 依存ライブラリ構成テンプレート（eframe 0.35.0 / windows 0.62.0）を追記。

## [0.2.2] - 2026-06-26

### 変更 (Changed)
- **CI/CDワークフローの最新化と高速化**:
  - [.github/workflows/ci.yml](.github/workflows/ci.yml) において、`actions/checkout` を `v4` にアップグレードし、ビルドを高速化するために `Swatinem/rust-cache@v2` アクションを追加。
  - [.github/workflows/release.yml](.github/workflows/release.yml) において、`actions/checkout` を `v4`、`softprops/action-gh-release` を `v2` にアップグレード。
- **プロジェクトテンプレートの汎用化**:
  - [docs/project_template_guide.md](docs/project_template_guide.md) 内の各種 Actions のバージョンを最新版に更新し、CI にキャッシュアクションを追加。
  - テンプレート内のバイナリ名等の表記をプレースホルダー `<YOUR_APP_NAME>` に変更し、他プロジェクトへ導入しやすいように汎用化。
- **開発ガイドラインの調整**:
  - [.agents/AGENTS.md](.agents/AGENTS.md) の「自動化設定の維持ルール」に、最新のアクションバージョンやキャッシュ設定の維持についての文言を追加。

## [0.2.1] - 2026-06-26

### 追加 (Added)
- **Dependabot による依存ライブラリの自動アップデート設定**:
  - [.github/dependabot.yml](.github/dependabot.yml) を新規追加し、毎週 Cargo 依存関係と GitHub Actions ワークフローの更新をチェックするように構成。
- **GitHub Releases への自動デプロイワークフロー**:
  - [.github/workflows/release.yml](.github/workflows/release.yml) を新規追加。`v*` タグプッシュ時に Windows 向け CLI バイナリおよび GUI バイナリをビルドし、自動的に GitHub Release へ zip アーカイブをアップロードするワークフローを構築。
- **開発用エディタ設定の統一**:
  - プロジェクト全体で一貫したコーディングスタイルを維持するための [.editorconfig](.editorconfig) を追加。
  - VS Code 利用者向けの設定を共通化する [.vscode/settings.json](.vscode/settings.json) を追加。

## [0.2.0] - 2026-06-26

### 追加 (Added)
- **CLI用パラメータ調整オプションの追加**:
  - 近似パラメータ（最大分母 `--max-den` / `-d` および許容誤差 `--tolerance` / `-t`）を指定可能にする手動引数パース処理を [src/main.rs](src/main.rs) に実装。
- **GUI版の Cargo features ビルド対応**:
  - `eframe`, `windows`, `winapi` を用いた GUI 機能を、オプションフィーチャー `gui` として [Cargo.toml](Cargo.toml) に追加。
  - GUI アプリ起動処理と [AGENTS.md](.agents/AGENTS.md) の基準（影なし、枠なし、最前面、ドラッグによるウィンドウ移動、Named Mutex による二重起動チェック）を満たすウィンドウ構築ロジックを [src/main.rs](src/main.rs) に実装。
- **.gitignore の整備**:
  - Rust, OS, 主要エディタが生成する不要ファイルを無視するルールを [.gitignore](.gitignore) に追記。
- **GitHub ActionsによるCI環境の構築**:
  - プッシュおよびプルリクエスト発生時に自動でテストとビルドを実行する [.github/workflows/ci.yml](.github/workflows/ci.yml) を新規追加。

## [0.1.3] - 2026-06-26

### 追加 (Added)
- **自動単体テストコードの組み込み**:
  - `approximate_fraction` 関数に対して、正常値（正数・ゼロ・負数）を用いた自動検証（アサーション）を行う単体テストモジュール `tests` を [src/main.rs](src/main.rs) に実装。
  - テスト実行環境と検証レポートである [docs/TEST_REPORT.md](docs/TEST_REPORT.md) の内容を、実際の自動テスト導入に合わせて更新。
- **開発ガイドラインの更新**:
  - [.agents/AGENTS.md](.agents/AGENTS.md) を見直し、多言語対応ポリシー（README以外は日本語）、ドキュメント内リンクの相対パス徹底（ポータビリティの確保）、および自動テスト品質維持ルール（テストケース追加・回帰テスト実行）を明文化。

## [0.1.2] - 2026-06-26

### 追加 (Added)
- **ヘルプ表示オプションの追加**:
  - `--help`、`-h` オプションを指定した際に、詳細な使用方法やオプション説明を表示し、正常終了（終了コード `0`）する機能を [src/main.rs](src/main.rs) に実装。

## [0.1.1] - 2026-06-26

### 追加 (Added)
- **バージョン表示オプションの追加**:
  - `--version`、`-v`、`-V` オプションを指定した際に、`bunka <バージョン番号>` 形式でバージョンを表示し、正常終了（終了コード `0`）する機能を [src/main.rs](src/main.rs) に実装。

## [0.1.0] - 2026-06-26

### 追加 (Added)
- **CLI版の初期実装**:
  - 連分数展開アルゴリズムによる実数から分数への変換ロジックを [src/main.rs](src/main.rs) に実装。
  - コマライン引数に対する入力値検証（有効な浮動小数点数かどうかの判定）を追加。
  - エラー発生時の標準エラー出力（stderr）への警告表示および終了コード `1` での終了処理を実装。
- **GUI版のプロトタイプコード（コメントアウト）**:
  - `eframe`/`egui` を使用した極小最前面デスクトップオーバーレイアプリのレイアウト設計コードをソースコードに内包。
- **リリースビルドの最適化設定**:
  - [Cargo.toml](Cargo.toml) にバイナリサイズ削減のための最適化設定（`opt-level = 'z'`, `lto = true`, `codegen-units = 1`, `panic = 'abort'`, `strip = true`）を導入。
- **MITライセンスの設定**:
  - MITライセンス本文を含む [LICENSE](LICENSE) ファイルを追加。
  - [Cargo.toml](Cargo.toml) に `license = "MIT"` を追加。
  - [README.md](README.md) および [README.ja.md](README.ja.md) にライセンス表記を追記。
- **ドキュメントの整備**:
  - クイックスタート用の [README.md](README.md)（英語）および [README.ja.md](README.ja.md)（日本語）を作成。
  - システム仕様書 [docs/SPEC.md](docs/SPEC.md) の作成。
  - システム構成図 [docs/DIAGRAM.md](docs/DIAGRAM.md) の作成。
  - 性能およびバイナリサイズを記録する [docs/FOOTPRINTS.md](docs/FOOTPRINTS.md) の作成。
  - テストケースと検証結果をまとめた [docs/TEST_REPORT.md](docs/TEST_REPORT.md) の作成。
