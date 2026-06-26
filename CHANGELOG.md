# 変更履歴 (Changelog)

このプロジェクトのすべての重要な変更は、このファイルに記録されます。

このファイルのフォーマットは [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) に基づいており、
このプロジェクトは [セマンティック バージョニング](https://semver.org/spec/v2.0.0.html) に準拠しています。

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
