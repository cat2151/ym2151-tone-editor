Last updated: 2025-12-12

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色パラメータを直感的に編集できる、Windows向けのターミナルユーザーインターフェース (TUI) アプリケーションです。
- Rustで開発されており、リアルタイムで編集中の音色をプレビュー再生する機能を持ちます。
- シンプルな操作性と高速な応答を目指し、開発中の音色データを自動保存・ロードする機能を備えています。

## 技術スタック
- フロントエンド: **ratatui 0.28** (Rust製ターミナルUIフレームワーク。テキストベースのユーザーインターフェース構築に使用されます。), **crossterm 0.28** (クロスプラットフォーム対応のターミナル操作ライブラリ。キー入力、マウスイベント、画面制御などに利用されます。)
- 音楽・オーディオ: **ym2151-log-play-server** (YM2151音源のリアルタイム音声フィードバックを提供するライブラリ。エディタが自動でサーバーの準備を行います。), **mmlabc-to-smf-rust**, **smf-to-ym2151log-rust** (MML (Music Macro Language) とSMF (Standard MIDI File)、YM2151ログデータ間の変換に使用されるライブラリ群。)
- 開発ツール: **Rust 1.70 以降** (システムプログラミング言語。パフォーマンス、安全性、並行性を重視しています。), **Cargo** (Rustの公式ビルドシステム兼パッケージマネージャー。依存関係の管理、プロジェクトのビルド、テスト、ドキュメント生成などを行います。)
- テスト: **Rustの標準テストフレームワーク** (Rust言語に組み込まれているテスト機能を利用し、各モジュールの機能が期待通りに動作するかを確認します。)
- ビルドツール: **Cargo** (前述の通り、Rustプロジェクトのビルドと管理の中核を担います。)
- 言語機能: **Rust言語とそのエコシステム** (メモリ安全性、エラーハンドリング、強力な型システムなど、Rustが提供する言語機能全般を利用して堅牢なアプリケーションを構築します。)
- 自動化・CI/CD: なし (プロジェクト情報からは特定の自動化・CI/CDツールに関する記述はありません。)
- 開発標準: **Rustの慣習とフォーマッター (rustfmt)** (Rustコミュニティで推奨されるコーディングスタイルとツールにより、コードの一貫性と可読性を保っています。)

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
📁 docs/
  📖 KEYBINDS.ja.md
📄 generate_gm_templates.rs
📁 generated-docs/
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 100.md
  📖 101.md
  📖 102.md
  📖 103.md
  📖 104.md
  📖 105.md
  📖 106.md
  📖 107.md
  📖 108.md
  📖 109.md
  📖 110.md
  📖 111.md
  📖 112.md
  📖 113.md
  📖 114.md
  📖 115.md
  📖 116.md
  📖 130.md
  📖 134.md
  📖 136.md
  📖 138.md
  📖 139.md
  📖 141.md
  📖 144.md
  📖 146.md
  📖 147.md
  📖 148.md
  📖 149.md
  📖 150.md
  📖 151.md
  📖 155.md
  📖 156.md
  📖 158.md
  📖 164.md
  📖 165.md
  📖 166.md
  📖 167.md
  📖 95.md
  📖 96.md
  📖 97.md
  📖 99.md
📁 src/
  📄 app.rs
  📄 app_init.rs
  📄 audio.rs
  📄 config.rs
  📄 file_ops.rs
  📄 main.rs
  📄 midi_conversion.rs
  📄 models.rs
  📄 register.rs
  📁 tests/
    📄 app_tests.rs
    📄 file_ops_tests.rs
    📄 midi_conversion_tests.rs
    📄 mod.rs
    📄 register_tests.rs
    📄 ui_tests.rs
    📄 variation_selector_tests.rs
    📄 verbose_logging_tests.rs
  📄 ui.rs
  📄 variation_selector.rs
📁 tones/
  📁 general_midi/
    📊 000_AcousticGrand.json
    📊 tone_names.json
📄 ym2151-tone-editor.toml.example
```

## ファイル詳細説明
-   **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定する設定ファイルです。
-   **`Cargo.lock`**: `Cargo.toml`に基づいて、プロジェクトが実際に使用するすべての依存クレートの正確なバージョンとハッシュ値を記録するファイルです。これにより、ビルドの一貫性が保証されます。
-   **`Cargo.toml`**: Rustプロジェクトのビルド設定ファイルです。プロジェクト名、バージョン、著者、依存関係、各種設定などが記述されています。
-   **`LICENSE`**: プロジェクトのライセンス情報が記載されています。
-   **`README.ja.md`**: プロジェクトの概要、機能、使い方などを日本語で説明するメインのドキュメントファイルです。
-   **`README.md`**: プロジェクトの概要、機能、使い方などを英語で説明するメインのドキュメントファイルです。
-   **`README_generate_gm_templates.md`**: GM（General MIDI）テンプレートの生成に関する説明が記載されたドキュメントファイルです。
-   **`_config.yml`**: GitHub Pagesなどのサイト設定に使用されるYAML形式の設定ファイルです。
-   **`docs/KEYBINDS.ja.md`**: アプリケーションのキーバインドに関する詳細な情報が日本語で記載されたドキュメントです。
-   **`generate_gm_templates.rs`**: General MIDI音色のテンプレートを生成するためのRustスクリプトファイルです。
-   **`generated-docs/`**: 自動生成されたドキュメントが格納されるディレクトリです。
-   **`googled947dc864c270e07.html`**: Googleサービスによるサイト所有権確認のための認証ファイルです。
-   **`issue-notes/`**: 開発中の課題や検討事項に関するメモがMarkdown形式で格納されるディレクトリです（来訪者向けの詳細説明は省略）。
-   **`src/app.rs`**: アプリケーションの主要なロジックと状態管理を定義するモジュールです。TUIの各要素の振る舞いやデータフローを制御します。
-   **`src/app_init.rs`**: アプリケーションの起動時の初期化処理（設定のロード、UIの準備など）を担うモジュールです。
-   **`src/audio.rs`**: YM2151音源のリアルタイム音声フィードバックに関する処理を扱うモジュールです。`ym2151-log-play-server`ライブラリとの連携を行います。
-   **`src/config.rs`**: アプリケーションの設定（キーバインド、保存パスなど）の読み込み、解析、管理を行うモジュールです。
-   **`src/file_ops.rs`**: 音色データの保存（セーブ）や読み込み（ロード）といったファイル操作に関する機能を提供するモジュールです。
-   **`src/main.rs`**: アプリケーションのエントリポイントとなるファイルです。`main`関数が含まれ、アプリケーションの初期化、イベントループの開始、終了処理などを担当します。
-   **`src/midi_conversion.rs`**: MIDI関連のデータ（MML, SMFなど）とYM2151のレジスタデータとの間の変換ロジックを扱うモジュールです。
-   **`src/models.rs`**: アプリケーション内で使用される主要なデータ構造（YM2151の音色パラメータ、UIの状態など）を定義するモジュールです。
-   **`src/register.rs`**: YM2151音源のレジスタ操作や、レジスタ値の変換・検証に関するロジックを提供するモジュールです。
-   **`src/tests/`**: アプリケーションの各モジュールに対する単体テストコードを格納するディレクトリです。
    -   **`src/tests/app_tests.rs`**: `app.rs`モジュールのテストコード。
    -   **`src/tests/file_ops_tests.rs`**: `file_ops.rs`モジュールのテストコード。
    -   **`src/tests/midi_conversion_tests.rs`**: `midi_conversion.rs`モジュールのテストコード。
    -   **`src/tests/mod.rs`**: `tests`モジュールのルートファイルで、他のテストファイルをまとめます。
    -   **`src/tests/register_tests.rs`**: `register.rs`モジュールのテストコード。
    -   **`src/tests/ui_tests.rs`**: `ui.rs`モジュールのテストコード。
    -   **`src/tests/variation_selector_tests.rs`**: `variation_selector.rs`モジュールのテストコード。
    -   **`src/tests/verbose_logging_tests.rs`**: 詳細ロギング機能に関するテストコード。
-   **`src/ui.rs`**: ターミナルUIの描画ロジック、ウィジェットの構成、イベントハンドリングなど、ユーザーインターフェース関連の処理を担うモジュールです。
-   **`src/variation_selector.rs`**: 音色のバリエーションを選択・管理するための機能を提供するモジュールです。
-   **`tones/`**: 音色データファイルを格納するディレクトリです。
    -   **`tones/general_midi/`**: General MIDI互換の音色データを格納するディレクトリです。
        -   **`tones/general_midi/000_AcousticGrand.json`**: アコースティックグランドピアノの音色データがJSON形式で保存されています。
        -   **`tones/general_midi/tone_names.json`**: 音色名のリストなどが含まれるJSONファイルです。
-   **`ym2151-tone-editor.toml.example`**: アプリケーションの設定ファイル (`ym2151-tone-editor.toml`) の例を示すファイルです。

## 関数詳細説明
プロジェクト情報からは具体的な関数名、引数、戻り値、機能に関する詳細な情報を抽出できませんでした。

## 関数呼び出し階層ツリー
```
プロジェクト情報からは関数呼び出し階層を分析できませんでした

---
Generated at: 2025-12-12 07:09:02 JST
