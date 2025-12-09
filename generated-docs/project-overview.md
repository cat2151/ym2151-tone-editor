Last updated: 2025-12-10

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色をWindows上で直感的に編集できるTUI（ターミナルユーザーインターフェース）エディタです。
- Rustで開発され、リアルタイム音声フィードバックと自動保存機能を備えています。
- シンプルで応答性の高い操作性により、YM2151の音作りを手軽に楽しむことができます。

## 技術スタック
- フロントエンド: 
    - `ratatui` 0.28: Rust製のターミナルUIフレームワークで、リッチなテキストベースのユーザーインターフェースを構築するために使用されています。
    - `crossterm` 0.28: クロスプラットフォームでターミナルの操作（キー入力、マウスイベント、カーソル制御など）を可能にするライブラリです。
- 音楽・オーディオ: 
    - YM2151 (OPM): ヤマハのFM音源チップであり、本プロジェクトで音色を編集する対象です。
    - `ym2151-log-play-server`: リアルタイム音声フィードバックを提供するためのバックエンドサーバーで、エディタが自動的に準備・起動します。
    - `mmlabc-to-smf-rust`: MML (Music Macro Language) をSMF (Standard MIDI File) に変換するライブラリです。プレビューMMLの変換に利用されます。
    - `smf-to-ym2151log-rust`: SMFをYM2151のレジスタログ形式に変換するライブラリで、音色データとSMFからログを生成するために使用されます。
- 開発ツール: 
    - `Rust`: 安全性、並行性、パフォーマンスに重点を置いたプログラミング言語で、プロジェクト全体がこの言語で記述されています。
    - `cargo`: Rustの公式ビルドシステムおよびパッケージマネージャーで、プロジェクトのビルド、実行、依存関係管理に使用されます。
- テスト: 
    - Rust標準のテストフレームワーク: `src/tests/` ディレクトリ配下のテストコードは、Rustに組み込まれたテスト機能を利用しています。
- ビルドツール: 
    - `cargo`: 上記の通り、プロジェクトのビルドとコンパイルを管理します。
- 言語機能: 
    - `Rust 1.70 以降`: プロジェクトの動作に必要なRustのバージョン指定です。
- 自動化・CI/CD: 
    - (特筆すべき情報はありません)
- 開発標準: 
    - (特筆すべき情報はありません)

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
- **`.gitignore`**: Gitがバージョン管理から無視するファイルやディレクトリを指定する設定ファイルです。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンを記録し、ビルドの一貫性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、著者、依存クレート、ビルド設定などが定義されています。
- **`LICENSE`**: プロジェクトの配布および使用に関するライセンス情報が記述されています。
- **`README.ja.md` / `README.md`**: それぞれ日本語版と英語版のプロジェクト概要、機能、ビルド方法、操作ガイドなどが含まれる主要なドキュメントファイルです。
- **`README_generate_gm_templates.md`**: General MIDIテンプレートを生成するプロセスに関する説明が記述されたドキュメントです。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレーターで使用される設定ファイルです。
- **`docs/KEYBINDS.ja.md`**: 日本語でキーバインドに関する詳細な情報を提供するドキュメントです。
- **`generate_gm_templates.rs`**: General MIDI音色テンプレートを生成するためのRustスクリプトファイルです。
- **`generated-docs/`**: 自動生成されたドキュメントを格納するためのディレクトリです。
- **`googled947dc864c270e07.html`**: Googleサイトの所有権確認に使用されるファイルです。
- **`issue-notes/`**: 開発中の課題や検討事項を記録するためのディレクトリです。（来訪者向けには詳細な内容は含まれません）
- **`src/`**: プロジェクトの主要なソースコードが格納されているディレクトリです。
    - **`app.rs`**: アプリケーションの全体的な状態管理、メインロジック、イベント処理などを担当するモジュールです。
    - **`app_init.rs`**: アプリケーションの起動時における初期設定や準備処理を行うモジュールです。
    - **`audio.rs`**: YM2151の音色プレビューやリアルタイム音声フィードバック、オーディオサーバーとの連携を担当するモジュールです。
    - **`config.rs`**: アプリケーションの設定（例：キーバインド設定）の読み込み、保存、管理を行うモジュールです。
    - **`file_ops.rs`**: 音色データなどのファイルの読み書き、保存、ロードといったファイルシステム操作を担うモジュールです。
    - **`main.rs`**: プログラムのエントリーポイント（開始点）となるファイルです。`cargo run`で最初に実行されます。
    - **`midi_conversion.rs`**: MIDI関連のデータ形式（MML、SMF）とYM2151レジスタデータ間の変換ロジックを提供するモジュールです。
    - **`models.rs`**: アプリケーション内で使用される主要なデータ構造（例：YM2151音色データ、設定データ）を定義するモジュールです。
    - **`register.rs`**: YM2151音源のレジスタ値の操作や、そのパラメータに関するロジックをカプセル化したモジュールです。
    - **`tests/`**: 各モジュールの単体テストや統合テストのコードを格納するディレクトリです。
        - **`app_tests.rs`**: `app.rs`モジュールのテストコードです。
        - **`file_ops_tests.rs`**: `file_ops.rs`モジュールのテストコードです。
        - **`midi_conversion_tests.rs`**: `midi_conversion.rs`モジュールのテストコードです。
        - **`mod.rs`**: `tests`モジュールの定義ファイルです。
        - **`register_tests.rs`**: `register.rs`モジュールのテストコードです。
        - **`ui_tests.rs`**: ユーザーインターフェース(`ui.rs`)関連のテストコードです。
        - **`variation_selector_tests.rs`**: `variation_selector.rs`モジュールのテストコードです。
        - **`verbose_logging_tests.rs`**: 詳細ロギング機能に関するテストコードです。
    - **`ui.rs`**: ターミナルユーザーインターフェース (TUI) の描画ロジックや、画面要素の構成を担当するモジュールです。
    - **`variation_selector.rs`**: 音色バリエーションの選択や管理に関するロジックを提供するモジュールです。
- **`tones/`**: 編集・管理される音色データファイルを格納するためのディレクトリです。
    - **`general_midi/`**: General MIDI規格に準拠した音色データが格納されるサブディレクトリです。
        - **`000_AcousticGrand.json`**: アコースティックグランドピアノの音色データ（JSON形式）。
        - **`tone_names.json`**: 音色名とそのIDなどを定義するデータファイル（JSON形式）。
- **`ym2151-tone-editor.toml.example`**: 設定ファイルの例。ユーザーが設定をカスタマイズする際のテンプレートとして提供されます。

## 関数詳細説明
提供されたプロジェクト情報からは、具体的な関数名、引数、戻り値、詳細な機能に関する情報を特定できませんでした。
このため、個々の関数の詳細な説明は提供できません。

## 関数呼び出し階層ツリー
```
提供されたプロジェクト情報からは、関数間の呼び出し階層を分析できませんでした。

---
Generated at: 2025-12-10 07:09:16 JST
