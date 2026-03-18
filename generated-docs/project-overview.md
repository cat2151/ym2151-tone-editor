Last updated: 2026-03-19

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色を編集するための、Windows向けターミナルユーザーインターフェース (TUI) エディタです。
- Rust言語で開発されており、リアルタイムで音色パラメータを調整しながらサウンドをプレビューできます。
- 直感的でシンプルな操作性により、YM2151音源のサウンドデザインを効率的に行えます。

## 技術スタック
- フロントエンド: ratatui (Rust製のターミナルユーザーインターフェースフレームワークで、リッチなUIをターミナル上に構築します), crossterm (クロスプラットフォーム対応のターミナル操作ライブラリで、キーボード入力、マウスイベント、画面描画などを制御します)
- 音楽・オーディオ: ym2151-log-play-server (音色エディタと連携し、YM2151音源のサウンドをリアルタイムで生成・再生するためのサーバーライブラリです)
- 開発ツール: Cargo (Rustの公式ビルドシステムおよびパッケージマネージャーで、依存関係の管理、ビルド、テスト、実行を効率的に行います)
- テスト: Rustの標準テストフレームワーク (各モジュールに付属する`_tests.rs`ファイル群で単体テストが記述されており、コードの品質と信頼性を確保しています)
- ビルドツール: Cargo (Rustプロジェクトのコンパイルと実行を管理します), build.rs (ビルド時にカスタムロジックを実行するためのスクリプトで、プロジェクト固有のビルド要件に対応します)
- 言語機能: Rust (安全性、速度、並行性に重点を置いたシステムプログラミング言語で、堅牢なアプリケーション開発を可能にします)

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
  📖 223.md
  📖 224.md
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
  📄 waveform.rs
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
- **`.gitignore`**: Gitのバージョン管理システムが無視すべきファイルやディレクトリのパターンを定義します。ビルド生成物や一時ファイルなどが含まれます。
- **`Cargo.lock`**: Rustプロジェクトの依存関係ツリーの正確なバージョンとチェックサムを記録し、再現可能なビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトの設定ファイルです。プロジェクト名、バージョン、著者、依存クレート、ビルドオプションなどを定義します。
- **`LICENSE`**: 本プロジェクトのソフトウェアライセンス情報が記述されています。
- **`README.ja.md`**: プロジェクトの目的、機能、使用方法などについて日本語で説明した概要ファイルです。
- **`README.md`**: プロジェクトの目的、機能、使用方法などについて英語で説明した概要ファイルです。
- **`README_generate_gm_templates.md`**: General MIDI (GM) 音色テンプレートの生成プロセスに関する詳細な説明が記載されています。
- **`_config.yml`**: GitHub Pagesのサイト設定ファイルです。ウェブサイトのテーマやプラグインなどの設定を定義します。
- **`build.rs`**: カスタムビルドロジックを実行するためのRustスクリプトです。コンパイル時に特定のタスク（例: コード生成）を実行するのに使用されます。
- **`core/`**: プロジェクトのコアロジックを含むサブクレートです。
    - **`core/Cargo.toml`**: `core`サブクレート固有の依存関係とメタデータを定義します。
    - **`core/src/lib.rs`**: `core`サブクレートのライブラリコードのエントリポイントです。再利用可能な共通ロジックが含まれます。
    - **`core/src/tests.rs`**: `core`サブクレートの単体テストコードが記述されています。
- **`demo-library/`**: ランダム音色生成ライブラリのデモンストレーション関連ファイルが格納されています。
    - **`demo-library/index.html`**: ランダム音色関数ライブラリの利用方法を示すウェブデモページです。
- **`docs/`**: プロジェクトのドキュメントが格納されています。
    - **`docs/KEYBINDS.ja.md`**: 日本語でキーバインド（キー操作）に関する詳細な説明が記載されています。
- **`generate_gm_templates.rs`**: General MIDI音色テンプレートを自動生成するためのRustスクリプトです。
- **`generated-docs/`**: 生成されたドキュメントや資料が格納されるディレクトリです。
- **`googled947dc864c270e07.html`**: Googleサイトの所有権確認用ファイルです。
- **`issue-notes/`**: 開発中に発生した課題や検討事項、設計メモなどを記録したMarkdownファイルが格納されています。
- **`src/`**: メインアプリケーションのソースコードが格納されています。
    - **`src/app/mod.rs`**: アプリケーションの主要な状態管理、データフロー、全体的なロジックをまとめるモジュールです。
    - **`src/app/shortcuts.rs`**: キーボードショートカットの定義と、それに対応するアプリケーションの動作を処理するモジュールです。
    - **`src/app_init.rs`**: アプリケーションの起動時における初期設定、リソースのロード、環境準備を行うモジュールです。
    - **`src/audio.rs`**: YM2151音源の制御やリアルタイム音声フィードバックに関連するオーディオ処理ロジックを扱います。
    - **`src/config.rs`**: アプリケーションの設定の読み込み、保存、管理を行うモジュールです。
    - **`src/event_loop.rs`**: ユーザーからの入力イベント（キーボード、マウス）を監視し、アプリケーションの状態を更新するメインのイベントループロジックを実装します。
    - **`src/favorites.rs`**: お気に入り音色の管理、保存、ロードに関連する機能を提供します。
    - **`src/file_ops.rs`**: 音色データや設定ファイルなど、各種ファイルの読み書きや操作を行うモジュールです。
    - **`src/history.rs`**: 音色編集の履歴を記録し、アンドゥ/リドゥ機能の基盤となるデータ管理を行います。
    - **`src/history_selector.rs`**: 編集履歴の中から特定のバージョンを選択するためのユーザーインターフェースロジックを扱います。
    - **`src/logging.rs`**: アプリケーションのデバッグ情報やイベントを記録するためのログ出力機能を提供します。
    - **`src/main.rs`**: アプリケーションのエントリポイントであり、全体のフローを orchestrate します。
    - **`src/midi_conversion.rs`**: MIDIデータとYM2151音色データ間の変換ロジックを扱います。
    - **`src/models.rs`**: アプリケーション内で使用される主要なデータ構造（例: 音色パラメータ、アプリケーション状態）を定義するモジュールです。
    - **`src/random_tone.rs`**: ランダムなYM2151音色パラメータを生成するアルゴリズムを実装します。
    - **`src/register.rs`**: YM2151音源の個々のレジスタに対する読み書き、検証などの低レベルな操作を扱います。
    - **`src/register_list.rs`**: YM2151の全レジスタまたは特定カテゴリのレジスタの一覧表示と管理ロジックを提供します。
    - **`src/tests/`**: メインアプリケーションの様々な機能に対する単体テストコードが格納されているディレクトリです。
        - **`src/tests/app_adsr_mul_sm_tests.rs`**: ADSR、Multiplier、Slot Maskなど、YM2151のエンベロープやアルゴリズム関連パラメータのテストです。
        - **`src/tests/app_ch_param_tests.rs`**: チャンネルパラメータに関するテストです。
        - **`src/tests/app_ks_ams_tests.rs`**: Key ScaleとAM Sensitivityに関するテストです。
        - **`src/tests/app_tests.rs`**: アプリケーション全体または主要な機能の統合テストです。
        - **`src/tests/app_tl_d1l_dt_dt2_tests.rs`**: Total Level、Decay 1 Level、Detune、Detune 2に関するテストです。
        - **`src/tests/app_value_by_tests.rs`**: 値の増減操作に関するテストです。
        - **`src/tests/event_loop_tests.rs`**: イベントループの挙動とイベント処理に関するテストです。
        - **`src/tests/favorites_tests.rs`**: お気に入り機能のロジックに関するテストです。
        - **`src/tests/file_ops_tests.rs`**: ファイル操作機能に関するテストです。
        - **`src/tests/history_tests.rs`**: 編集履歴機能に関するテストです。
        - **`src/tests/midi_conversion_tests.rs`**: MIDI変換ロジックに関するテストです。
        - **`src/tests/mod.rs`**: テストモジュール群をまとめるファイルです。
        - **`src/tests/random_tone_tests.rs`**: ランダム音色生成機能に関するテストです。
        - **`src/tests/register_roundtrip_tests.rs`**: レジスタ値の読み書きが正確に行われるかどうかの往復テストです。
        - **`src/tests/register_tests.rs`**: 個々のレジスタ操作に関するテストです。
        - **`src/tests/ui_tests.rs`**: ユーザーインターフェースの表示と操作に関するテストです。
        - **`src/tests/variation_selector_tests.rs`**: 音色バリエーション選択機能に関するテストです。
        - **`src/tests/verbose_logging_tests.rs`**: 詳細ログ出力機能に関するテストです。
    - **`src/ui/`**: ターミナルユーザーインターフェースの描画とインタラクションを管理するモジュール群です。
        - **`src/ui/help.rs`**: ヘルプ画面のコンテンツと表示ロジックを扱います。
        - **`src/ui/helpers.rs`**: UI描画や操作に役立つ汎用的なユーティリティ関数を提供します。
        - **`src/ui/mod.rs`**: UIコンポーネントや描画ロジックをまとめるメインモジュールです。
    - **`src/updater.rs`**: アプリケーションの新しいバージョンをチェックし、更新を行う機能を提供します。
    - **`src/variation_selector.rs`**: 保存された音色のバリエーションを一覧表示し、選択するUIロジックを実装します。
    - **`src/waveform.rs`**: 波形の表示ロジックや、波形データに関連する処理を扱います。
- **`tones/`**: 音色データが格納されているディレクトリです。
    - **`tones/general_midi/`**: General MIDI規格に準拠した音色データが格納されています。
        - **`tones/general_midi/000_AcousticGrand.json`**: General MIDIの「000 Acoustic Grand Piano」音色とそのバリエーションデータです。
        - **`tones/general_midi/tone_names.json`**: 音色名のリストを格納するJSONファイルです。
- **`wasm/`**: WebAssembly関連のコードを含むサブクレートです。
    - **`wasm/Cargo.lock`**: `wasm`サブクレートの依存関係ロックファイルです。
    - **`wasm/Cargo.toml`**: `wasm`サブクレート固有の依存関係とメタデータを定義します。
    - **`wasm/src/lib.rs`**: `wasm`サブクレートのライブラリコードのエントリポイントです。WebブラウザなどWebAssembly環境で利用される機能が含まれます。
- **`ym2151-tone-editor.toml.example`**: アプリケーションの設定ファイル（`Cargo.toml`とは異なる、ユーザー設定用）のサンプルです。

## 関数詳細説明
本プロジェクトのソースコードは、各ファイルが特定の役割を持つ関数群を提供することで、モジュール化された構造となっています。具体的な関数名とシグネチャはプロジェクト情報に明示されていませんが、各モジュールが担う主要な機能に基づき、期待される関数の役割を以下に示します。

- **`src/main.rs`**: アプリケーションのエントリポイントとして、初期設定、メインイベントループの起動、および終了処理を統括する主要な関数が含まれます。
- **`src/app/mod.rs`**: アプリケーションの全体的な状態を管理し、ユーザーの操作に応じて状態を更新するビジネスロジック関数を提供します。
- **`src/app/shortcuts.rs`**: キーボードショートカット入力と、それに対応する内部処理（例: 値の増減、モード切り替え）を関連付ける関数群を定義します。
- **`src/audio.rs`**: YM2151音源レジスタへのデータ送信、音色プレビューの開始・停止、リアルタイム音声フィードバックの制御など、オーディオ関連の操作を行う関数を含みます。
- **`src/config.rs`**: アプリケーションの設定値を読み込み (`load_config`)、保存 (`save_config`) するための関数を提供します。
- **`src/event_loop.rs`**: ターミナルからの入力イベント（キー入力、マウスイベントなど）をポーリングし、適切なハンドラーにディスパッチするイベントループ関連の関数 (`run_event_loop`) が含まれます。
- **`src/file_ops.rs`**: 音色データや設定ファイルなど、各種ファイルをディスクから読み込み (`load_file`)、ディスクに書き込む (`save_file`) ためのユーティリティ関数を提供します。
- **`src/history.rs`**: 音色編集の変更履歴を追加 (`add_history`), 巻き戻し (`undo`), やり直し (`redo`) など、履歴管理を行う関数群を含みます。
- **`src/random_tone.rs`**: YM2151のパラメータ範囲内でランダムな音色データを生成する関数 (`generate_random_tone`) を提供します。
- **`src/register.rs`**: YM2151の個々のレジスタ値を設定 (`set_register_value`) および取得 (`get_register_value`) するための低レベルな操作関数を提供します。
- **`src/ui/mod.rs`**: ターミナル画面全体、または個々のUI要素（例: パラメータ表示エリア、ヘルプメッセージ）を描画するための関数群 (`draw_ui`, `update_display`) を統括します。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2026-03-19 07:15:40 JST
