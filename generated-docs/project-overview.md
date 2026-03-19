Last updated: 2026-03-20

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色をリアルタイムで編集・プレビューできるWindows用ターミナルUIエディタです。
- 直感的なマウス・キーボード操作に対応し、音色パラメータの変更を即座に音声でフィードバックします。
- 音色データの自動保存・ロード機能を備え、シンプルな操作性と視覚的な分かりやすさを重視して開発されています。

## 技術スタック
- フロントエンド: **ratatui 0.28** (Rust製のターミナルユーザーインターフェース (TUI) フレームワーク)、**crossterm 0.28** (クロスプラットフォーム対応のターミナル操作ライブラリ)
- 音楽・オーディオ: **ym2151-log-play-server** (リアルタイム音声フィードバックを提供するためのサーバーライブラリ。エディタが自動でセットアップ・起動します)
- 開発ツール: **cargo** (Rustプロジェクトのビルド、依存関係管理、テスト実行などを行う標準ツール)
- ビルドツール: **cargo** (Rustのソースコードをコンパイルし、実行可能なバイナリを生成します)
- 言語機能: **Rust 1.70 以降** (安全性、パフォーマンス、並行性に優れたシステムプログラミング言語)
- 自動化・CI/CD: (特になし)
- 開発標準: (特になし)

## ファイル階層ツリー
```
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📖 README_generate_gm_templates.md
📄 _config.yml
📄 build.rs
📁 core/
  📄 Cargo.toml
  📁 src/
    📄 lib.rs
    📄 tests.rs
📁 demo-library/
  🌐 index.html
📁 docs/
  📖 KEYBINDS.ja.md
📄 generate_gm_templates.rs
📁 generated-docs/
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 113.md
  📖 115.md
  📖 139.md
  📖 141.md
  📖 148.md
  📖 155.md
  📖 156.md
  📖 164.md
  📖 167.md
  📖 174.md
  📖 177.md
  📖 218.md
  📖 219.md
  📖 220.md
  📖 224.md
  📖 232.md
  📖 234.md
  📖 95.md
  📖 96.md
📁 src/
  📁 app/
    📄 mod.rs
    📄 shortcuts.rs
  📄 app_init.rs
  📄 audio.rs
  📄 config.rs
  📄 event_loop.rs
  📄 favorites.rs
  📄 file_ops.rs
  📄 history.rs
  📄 history_selector.rs
  📄 logging.rs
  📄 main.rs
  📄 midi_conversion.rs
  📄 models.rs
  📄 random_tone.rs
  📄 register.rs
  📄 register_list.rs
  📁 tests/
    📄 app_adsr_mul_sm_tests.rs
    📄 app_ch_param_tests.rs
    📄 app_ks_ams_tests.rs
    📄 app_tests.rs
    📄 app_tl_d1l_dt_dt2_tests.rs
    📄 app_value_by_tests.rs
    📄 event_loop_tests.rs
    📄 favorites_tests.rs
    📄 file_ops_tests.rs
    📄 history_tests.rs
    📄 midi_conversion_tests.rs
    📄 mod.rs
    📄 random_tone_tests.rs
    📄 register_roundtrip_tests.rs
    📄 register_tests.rs
    📄 ui_tests.rs
    📄 variation_selector_tests.rs
    📄 verbose_logging_tests.rs
  📁 ui/
    📄 help.rs
    📄 helpers.rs
    📄 mod.rs
  📄 updater.rs
  📄 variation_selector.rs
📁 tones/
  📁 general_midi/
    📊 000_AcousticGrand.json
    📊 tone_names.json
📁 wasm/
  📄 Cargo.lock
  📄 Cargo.toml
  📁 src/
    📄 lib.rs
📄 ym2151-tone-editor.toml.example
```

## ファイル詳細説明
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定します。
- **`Cargo.lock`**: Rustプロジェクトの依存関係の正確なバージョンを記録します。
- **`Cargo.toml`**: Rustプロジェクトの設定ファイルで、プロジェクト名、バージョン、依存関係などが定義されています。
- **`LICENSE`**: プロジェクトのライセンス情報が記載されています。
- **`README.ja.md` / `README.md`**: プロジェクトの概要、機能、使い方などを記述した日本語版と英語版のドキュメントです。
- **`README_generate_gm_templates.md`**: General MIDIテンプレートの生成に関する追加の説明ドキュメントです。
- **`_config.yml`**: GitHub Pagesのサイト設定ファイルです。
- **`build.rs`**: ビルド時に実行されるカスタムビルドスクリプトです。
- **`core/Cargo.toml` / `core/src/lib.rs` / `core/src/tests.rs`**: プロジェクトのコアロジックをカプセル化した独立したRustクレートです。`lib.rs`に主要な機能が、`tests.rs`にそのテストが含まれます。
- **`demo-library/index.html`**: ランダム音色関数ライブラリの利用方法を紹介するウェブページのデモです。
- **`docs/KEYBINDS.ja.md`**: エディタのキーバインド（ショートカットキー）に関する詳細な日本語ドキュメントです。
- **`generate_gm_templates.rs`**: General MIDIテンプレートを生成するためのRustスクリプトです。
- **`googled947dc864c270e07.html`**: Googleサイトの所有権確認用のファイルです。
- **`issue-notes/`**: 開発中の課題や検討事項を記録したメモファイル群です。（来訪者向けのため詳細は割愛）
- **`src/app/mod.rs`**: アプリケーションの主要なロジックを構成するモジュール群のルートファイルです。
- **`src/app/shortcuts.rs`**: ショートカットキーの処理に関連するロジックを定義します。
- **`src/app_init.rs`**: アプリケーションの初期化処理を定義します。
- **`src/audio.rs`**: YM2151音源へのオーディオ信号送信やリアルタイムフィードバックに関連するロジックを扱います。
- **`src/config.rs`**: アプリケーションの設定管理に関連するロジックを扱います。
- **`src/event_loop.rs`**: ユーザー入力やその他のイベントを処理するイベントループのロジックを定義します。
- **`src/favorites.rs`**: お気に入りの音色管理機能に関連するロジックです。
- **`src/file_ops.rs`**: ファイルの読み書きなど、ファイル操作に関するロジックを定義します。
- **`src/history.rs`**: 編集履歴の管理に関するロジックを扱います。
- **`src/history_selector.rs`**: 履歴の選択UIに関連するロジックです。
- **`src/logging.rs`**: アプリケーション内のログ出力機能に関連するロジックを扱います。
- **`src/main.rs`**: プロジェクトのメインエントリポイントとなるファイルです。
- **`src/midi_conversion.rs`**: MIDIデータと音色データの変換に関連するロジックを扱います。
- **`src/models.rs`**: アプリケーション内で使用されるデータ構造（モデル）を定義します。
- **`src/random_tone.rs`**: ランダムな音色を生成する機能に関連するロジックです。
- **`src/register.rs`**: YM2151レジスタの操作やデータ構造に関するロジックを定義します。
- **`src/register_list.rs`**: YM2151レジスタのリスト表示や管理に関連するロジックです。
- **`src/tests/`**: アプリケーションの各モジュールに対する単体テストコードが格納されています。
- **`src/ui/help.rs`**: ヘルプ表示UIに関連するロジックを定義します。
- **`src/ui/helpers.rs`**: UI構築で共通して利用されるヘルパー関数群です。
- **`src/ui/mod.rs`**: ユーザーインターフェース (TUI) の主要な部分を構成するモジュール群のルートファイルです。
- **`src/updater.rs`**: アプリケーションの更新機能に関連するロジックです。
- **`src/variation_selector.rs`**: 音色のバリエーションを選択するUIに関連するロジックです。
- **`tones/general_midi/000_AcousticGrand.json`**: General MIDIのアコースティックグランドピアノの音色データ例です。
- **`tones/general_midi/tone_names.json`**: General MIDIの音色名リストです。
- **`wasm/Cargo.lock` / `wasm/Cargo.toml` / `wasm/src/lib.rs`**: WebAssembly (Wasm) へのコンパイルを目的とした独立したRustクレートです。`lib.rs`にWasm向け機能が含まれます。
- **`ym2151-tone-editor.toml.example`**: 設定ファイルの例で、キーバインドなどが定義されています。

## 関数詳細説明
提供された情報からは関数の具体的な役割、引数、戻り値を特定できませんでした。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2026-03-20 07:11:12 JST
