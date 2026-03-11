Last updated: 2026-03-12

# Project Overview

## プロジェクト概要
- YM2151（OPM）FM音源の音色を編集するための、Windows向けターミナルユーザーインターフェース（TUI）エディタです。
- Rustで実装されており、リアルタイムな音声フィードバックを提供しながら、音色パラメータの直感的な編集が可能です。
- カーソル、キーボード、マウス操作に対応し、編集中の音色プレビュー、自動セーブ・ロード、キーバインド設定などの機能を備えています。

## 技術スタック
- フロントエンド: `ratatui` (ターミナルUIフレームワークにより、リッチなTUIを構築), `crossterm` (クロスプラットフォームなターミナル操作を抽象化し、キー入力や画面制御を実現)
- 音楽・オーディオ: `ym2151-log-play-server` (リアルタイム音声フィードバックを可能にするバックエンドサーバーライブラリ), YM2151音源 (編集対象となるFM音源チップ)
- 開発ツール: Rust (モダンで安全性の高いシステムプログラミング言語), `cargo` (Rustのビルドシステムとパッケージマネージャ)
- テスト: Rust標準のテストフレームワーク (テストディレクトリの存在から推測)
- ビルドツール: `cargo` (プロジェクトのビルド、実行、テスト、ドキュメント生成などを管理)
- 言語機能: Rust 1.70 以降 (プロジェクトの動作に必要なRust言語のバージョン)
- 自動化・CI/CD: (特段の記述なし。`cargo`コマンドによるビルド・実行は可能)
- 開発標準: (特段の記述なし)

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
  📖 176.md
  📖 177.md
  📖 182.md
  📖 95.md
  📖 96.md
📁 src/
  📁 app/
    📄 mod.rs
    📄 shortcuts.rs
  📄 app_init.rs
  📄 audio.rs
  📄 config.rs
  📄 file_ops.rs
  📄 main.rs
  📄 midi_conversion.rs
  📄 models.rs
  📄 register.rs
  📁 tests/
    📄 app_adsr_mul_sm_tests.rs
    📄 app_ch_param_tests.rs
    📄 app_ks_ams_tests.rs
    📄 app_tests.rs
    📄 app_tl_d1l_dt_dt2_tests.rs
    📄 app_value_by_tests.rs
    📄 file_ops_tests.rs
    📄 midi_conversion_tests.rs
    📄 mod.rs
    📄 register_roundtrip_tests.rs
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
- **.gitignore**: Gitが追跡しないファイルやディレクトリを指定する設定ファイルです。
- **Cargo.lock**: プロジェクトの依存関係が解決された正確なバージョンを記録します。ビルドの再現性を保証します。
- **Cargo.toml**: Rustプロジェクトのビルド設定ファイル（マニフェスト）です。プロジェクト名、バージョン、依存クレートなどが記述されています。
- **LICENSE**: プロジェクトの配布および使用に関するライセンス情報が記載されています。
- **README.ja.md**: プロジェクトの日本語での概要、機能、使い方、ビルド方法などを説明するドキュメントです。
- **README.md**: プロジェクトの英語での概要、機能、使い方、ビルド方法などを説明するドキュメントです。
- **README_generate_gm_templates.md**: `generate_gm_templates.rs`スクリプトに関する補足情報や説明を記述したドキュメントと推測されます。
- **_config.yml**: GitHub Pagesなどの静的サイトジェネレータの設定ファイルである可能性があります。
- **docs/KEYBINDS.ja.md**: アプリケーションのキーバインド（キー操作）の詳細を日本語で説明するドキュメントです。
- **generate_gm_templates.rs**: General MIDI用の音色テンプレートを生成するためのRustスクリプトまたはプログラムです。
- **generated-docs/**: 何らかのツールによって自動生成されたドキュメントが格納されるディレクトリです。
- **googled947dc864c270e07.html**: Googleサービスのサイト所有権確認などに使用される検証用ファイルです。
- **issue-notes/**: 開発中の課題や検討事項、特定のIssueに関連するメモが格納されているディレクトリです。
- **src/app/mod.rs**: `app`モジュールのエントリポイントであり、アプリケーションの主要なロジックや他のサブモジュールを統合します。
- **src/app/shortcuts.rs**: アプリケーション内で使用されるキーバインドやショートカットキーの処理ロジックを定義します。
- **src/app_init.rs**: アプリケーション起動時の初期化処理（設定の読み込み、UIのセットアップなど）を担当します。
- **src/audio.rs**: YM2151音声サーバーとの連携や、音色のプレビューなど、オーディオ関連の処理を扱います。
- **src/config.rs**: アプリケーションの設定（キーバインド設定ファイルなど）の読み込み、保存、管理を行います。
- **src/file_ops.rs**: 音色データのファイルへの保存や、既存の音色データの読み込みといったファイル入出力操作を処理します。
- **src/main.rs**: Rustアプリケーションの主要なエントリポイントであり、プログラムの実行を開始します。
- **src/midi_conversion.rs**: MIDIイベントとYM2151のレジスタデータ間の変換処理を実装します。
- **src/models.rs**: YM2151の音色データ構造、パラメータ、アプリケーションの状態など、各種データモデルを定義します。
- **src/register.rs**: YM2151のレジスタに関する操作、値の範囲チェック、データ変換など、低レベルなレジスタ処理を扱います。
- **src/tests/**: プロジェクトの各種機能に対するユニットテストや統合テストのコードが格納されているディレクトリです。
- **src/ui.rs**: Ratatuiクレートを使用して、ターミナルユーザーインターフェースの描画ロジックとイベント処理を実装します。
- **src/variation_selector.rs**: 複数の音色バリエーションの中から選択したり、管理したりするためのロジックを提供します。
- **tones/general_midi/000_AcousticGrand.json**: General MIDI規格の「Acoustic Grand Piano」に該当するYM2151音色データ（JSON形式）を格納しています。
- **tones/general_midi/tone_names.json**: 音色の名前リストや、GM音色テンプレートに関連するメタデータが格納されている可能性があります。
- **ym2151-tone-editor.toml.example**: アプリケーションの設定ファイル（キーバインドなど）の例を示すテンプレートファイルです。

## 関数詳細説明
提供されたプロジェクト情報からは、個々の関数の詳細な情報（名前、引数、戻り値、具体的な実装）は抽出できませんでした。しかし、プロジェクトの機能説明に基づき、一般的にこのようなアプリケーションで想定される主要な処理単位とその役割は以下の通りです。

-   `run_app()`: アプリケーションのメインループを実行し、UIの描画、イベント処理、状態更新を管理します。
-   `initialize_terminal()`: ターミナルの初期設定（Rawモードへの切り替え、代替スクリーンバッファの使用など）を行います。
-   `restore_terminal()`: アプリケーション終了時にターミナルを元の状態に戻します。
-   `handle_event(event)`: ユーザーからのキーボードやマウス入力イベントを処理し、アプリケーションの状態を更新します。
-   `update_parameter(parameter_type, value_change)`: 現在選択されているYM2151音色パラメータの値を増減させます。
-   `preview_current_tone()`: 現在編集中の音色をYM2151サーバーに送信し、リアルタイムで再生します。
-   `save_current_tone()`: 現在の音色データを設定された形式でファイルに保存します。
-   `load_last_tone()`: 前回終了時に保存された音色データを自動的にロードします。
-   `draw_ui(frame)`: 現在のアプリケーションの状態（音色パラメータ、カーソル位置など）に基づいて、ターミナルUIを描画します。
-   `ensure_server_ready()`: リアルタイム音声フィードバック用のサーバーが起動・準備完了状態であることを確認し、必要に応じて自動でセットアップ・起動します。
-   `send_register_commands(register_data)`: 変更されたレジスタデータのみをYM2151サーバーに送信し、効率的な音声フィードバックを実現します。
-   `convert_mml_to_log(mml_string, tone_data)`: MML（Music Macro Language）と音色データから、YM2151ログデータを生成します。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2026-03-12 07:12:28 JST
