Last updated: 2026-03-17

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色を、ターミナル上で直感的に編集するためのWindows向けエディタです。
- Rustで開発されており、リアルタイムでの音色プレビュー機能と自動保存機能を提供します。
- シンプルな操作性と可視化により、YM2151音色作成の効率化を目指しています。

## 技術スタック
- フロントエンド: `ratatui` (ターミナルUIフレームワークにより、リッチなテキストベースのユーザーインターフェースを実現します), `crossterm` (クロスプラットフォームのターミナル操作ライブラリで、キーボードやマウスイベントの処理、画面描画を制御します)
- 音楽・オーディオ: `ym2151-log-play-server` (リアルタイム音声フィードバックを提供するサーバーとの連携を自動化します), `mmlabc-to-smf-rust` (MML (Music Macro Language) をSMF (Standard MIDI File) に変換するためのライブラリです), `smf-to-ym2151log-rust` (SMFをYM2151レジスタ書き込みログに変換するためのライブラリです)
- 開発ツール: `Rust` (安全性、並行性、パフォーマンスに重点を置いたシステムプログラミング言語です), `cargo` (Rustのビルドシステムおよびパッケージマネージャーで、プロジェクトのビルド、テスト、依存関係管理を行います)
- テスト: Rustの組み込みテスト機能 (`cargo test`コマンドで実行され、各モジュールの機能が期待通りに動作するか検証します)
- ビルドツール: `cargo` (Rustプロジェクトのコンパイルと実行を管理します)
- 言語機能: `Rust` (強力な型システム、所有権モデルにより、メモリ安全性を保証しつつ高性能なアプリケーション開発を可能にします)
- 自動化・CI/CD: (現在、具体的な自動化・CI/CDツールは明記されていません)
- 開発標準: (現在、特定の開発標準ツールは明記されていません)

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
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンを記録し、ビルドの再現性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ、依存関係、ビルド設定などを定義するマニフェストファイルです。
- **`LICENSE`**: プロジェクトのライセンス情報が記載されています。
- **`README.ja.md`**: プロジェクトの日本語版説明書です。概要、機能、使い方、開発状況などが記されています。
- **`README.md`**: プロジェクトの英語版説明書です。日本語版と同様にプロジェクトの情報を提供します。
- **`README_generate_gm_templates.md`**: General MIDIテンプレートの生成に関する詳細な説明ドキュメントです。
- **`_config.yml`**: GitHub Pagesサイトの設定ファイルです。
- **`build.rs`**: Rustプロジェクトのビルド時に実行されるカスタムビルドスクリプトです。
- **`core/Cargo.toml`**: `core`クレートのマニフェストファイルです。
- **`core/src/lib.rs`**: `core`クレートのライブラリコードが格納されており、プロジェクトの中核的なロジックやデータ構造を提供する可能性があります。
- **`core/src/tests.rs`**: `core`クレートの単体テストコードです。
- **`demo-library/index.html`**: ランダム音色関数ライブラリのデモページであり、Webブラウザで表示されます。
- **`docs/KEYBINDS.ja.md`**: アプリケーションのキーバインド設定に関する日本語のドキュメントです。
- **`generate_gm_templates.rs`**: General MIDIテンプレートを生成するためのRustスクリプトファイルです。
- **`googled947dc864c270e07.html`**: Googleサイトの所有権確認に使用されるHTMLファイルです。
- **`issue-notes/`**: 開発中のIssueに関するメモや詳細がMarkdown形式で格納されているディレクトリです。
- **`src/app/mod.rs`**: アプリケーションの主要なロジックをまとめるモジュールです。
- **`src/app/shortcuts.rs`**: アプリケーション内で使用されるショートカットキーに関連する処理を定義します。
- **`src/app_init.rs`**: アプリケーション起動時の初期化処理を担当するモジュールです。
- **`src/audio.rs`**: リアルタイム音声フィードバックの処理（`ym2151-log-play-server`との連携など）を実装しています。
- **`src/config.rs`**: アプリケーションの設定（キーバインドなど）の読み込み、保存、管理を行います。
- **`src/event_loop.rs`**: ターミナルからのイベント（キー入力、マウス操作）を捕捉し、アプリケーションの状態を更新するイベントループを実装します。
- **`src/favorites.rs`**: お気に入りの音色を管理する機能を提供します。
- **`src/file_ops.rs`**: 音色データの読み書き、設定ファイルの管理など、ファイルシステムに関する操作を扱います。
- **`src/history.rs`**: 編集履歴の記録と管理を行います。
- **`src/history_selector.rs`**: 編集履歴から特定の音色を選択するユーザーインターフェースコンポーネントを実装します。
- **`src/logging.rs`**: アプリケーション内のイベントやデバッグ情報を出力するロギング機能を提供します。
- **`src/main.rs`**: アプリケーションのエントリポイントであり、プログラムの実行を開始します。
- **`src/midi_conversion.rs`**: MIDIデータ形式とYM2151音源のデータ形式間の変換処理を扱います。
- **`src/models.rs`**: アプリケーション全体で共有されるデータ構造やモデルを定義します。
- **`src/random_tone.rs`**: YM2151のランダムな音色を生成する機能を提供します。
- **`src/register.rs`**: YM2151のレジスタにアクセスし、値を読み書きするための低レベルな操作を定義します。
- **`src/register_list.rs`**: YM2151のレジスタ値をリスト形式で表示し、操作するロジックを実装します。
- **`src/tests/`**: アプリケーションの各モジュールに対する単体テストコードが格納されています。
- **`src/ui/mod.rs`**: ターミナルユーザーインターフェース (TUI) の主要な描画ロジックとコンポーネントをまとめます。
- **`src/ui/helpers.rs`**: UIの描画や表示に関する汎用的なヘルパー関数を提供します。
- **`src/updater.rs`**: アプリケーションのアップデートを確認し、必要に応じて更新を行う機能を提供します。
- **`src/variation_selector.rs`**: 音色ファイルの複数のバリエーションから選択するUIコンポーネントを実装します。
- **`tones/general_midi/000_AcousticGrand.json`**: General MIDIの「Acoustic Grand Piano」に相当するYM2151音色データがJSON形式で保存されています。
- **`tones/general_midi/tone_names.json`**: General MIDI音色の名前リストがJSON形式で格納されています。
- **`wasm/Cargo.lock`**: `wasm`クレートの依存関係の正確なバージョンを記録します。
- **`wasm/Cargo.toml`**: `wasm`クレートのマニフェストファイルです。WebAssembly関連の機能やライブラリを定義する可能性があります。
- **`wasm/src/lib.rs`**: `wasm`クレートのライブラリコードが格納されており、WebAssembly対応の機能を提供する可能性があります。
- **`ym2151-tone-editor.toml.example`**: アプリケーションの設定ファイル例です。

## 関数詳細説明
このプロジェクト情報からは、特定の関数の引数、戻り値、機能までを詳細に特定することは困難です。
ただし、リアルタイム音声フィードバックのセクションで `ensure_server_ready()` 関数が言及されています。

- `ensure_server_ready()`
    - **役割**: `ym2151-log-play-server` ライブラリの一部であり、リアルタイム音声フィードバックのためのサーバーが準備できていることを保証します。
    - **機能**: サーバーのインストール、起動、準備状況チェックを自動的に処理します。これにより、ユーザーは手動でサーバーをセットアップすることなく、音色エディタを実行するだけで音声フィードバックを得られます。
    - **引数**: (プロジェクト情報からは不明確ですが、サーバーの状態を確認・制御するための情報を受け取る可能性があります。)
    - **戻り値**: (プロジェクト情報からは不明確ですが、サーバーの準備状況を示す結果を返す可能性があります。)

プロジェクト全体に関わる主要な関数は、YM2151音色パラメータの編集、ターミナルUIの描画、キーボード/マウスイベントの処理、音色データの読み書き、リアルタイム音声フィードバックの確立、MML/SMFデータ変換、音色履歴管理、ランダム音色生成など多岐にわたりますが、具体的な関数名、引数、戻り値については詳細なソースコード分析が必要です。

## 関数呼び出し階層ツリー
```
関数呼び出し階層の分析は行われていません。

---
Generated at: 2026-03-17 07:16:42 JST
