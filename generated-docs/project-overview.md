Last updated: 2026-03-18

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色を編集するためのWindows用ターミナルユーザーインターフェース (TUI) エディタです。
- マウスやキーボード操作で音色パラメータを直感的に調整し、リアルタイムでの音声フィードバックで確認できます。
- 終了時の音色自動保存と次回起動時の自動ロード、カスタマイズ可能なキーバインド、ランダム音色生成機能を備えています。

## 技術スタック
- フロントエンド: `ratatui` (0.28) - ターミナルUIの構築と表示を担うフレームワーク。
- 音楽・オーディオ: YM2151（OPM）FM音源 - 音源そのもの。`ym2151-log-play-server`ライブラリ - Windows環境でのリアルタイム音声フィードバックを自動的にセットアップ・提供します。
- 開発ツール: `Rust` - システムプログラミング言語。パフォーマンスと安全性に優れます。
- テスト: `cargo test` - Rust標準のテストランナーを利用し、アプリケーションの各モジュール（app, favorites, file_ops, history, midi_conversion, random_tone, register, uiなど）の動作を検証しています。
- ビルドツール: `cargo` - Rustプロジェクトのビルド、依存関係管理、テスト実行などを行う標準ツール。
- 言語機能: `Rust 1.70 以降` - アプリケーションの動作に必要とされるRustのバージョン。
- 自動化・CI/CD: GitHub - プロジェクトのバージョン管理、公開、インストールガイドの提供に利用しています。
- 開発標準: 特になし。Rustの標準的なコーディング規約に準拠しています。

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
- **`.gitignore`**: Gitによるバージョン管理から除外するファイルやディレクトリを指定します。
- **`Cargo.lock`**: `Cargo.toml`で定義された依存関係の具体的なバージョンを記録し、ビルドの再現性を保証します。
- **`Cargo.toml`**: Rustプロジェクトの設定ファイル。依存関係、パッケージ情報、ビルド設定などが記述されています。
- **`LICENSE`**: プロジェクトのライセンス情報（著作権、利用条件など）が記載されています。
- **`README.ja.md` / `README.md`**: プロジェクトの概要、機能、クイックスタートガイド、操作方法などを日本語と英語で説明する主要なドキュメントです。
- **`README_generate_gm_templates.md`**: GM音色テンプレート生成に関する補足説明。
- **`_config.yml`**: GitHub Pagesの設定ファイル。
- **`build.rs`**: ビルド時に実行されるカスタムビルドスクリプト。
- **`core/Cargo.toml` / `core/src/lib.rs` / `core/src/tests.rs`**: プロジェクトの中核となるライブラリコードとそのテストです。
- **`demo-library/index.html`**: ランダム音色関数ライブラリのデモページ。関数やインポートは含まれていません。
- **`docs/KEYBINDS.ja.md`**: キーバインドに関する詳細なドキュメント。
- **`generate_gm_templates.rs`**: General MIDIテンプレートを生成するためのスクリプト。
- **`generated-docs/`**: 自動生成されたドキュメントを格納するディレクトリ。
- **`googled947dc864c270e07.html`**: Googleサイト認証用ファイル。関数やインポートは含まれていません。
- **`issue-notes/`**: 開発中の検討事項や議論の記録（Issueのメモ）をMarkdown形式で格納するディレクトリです。
- **`src/app/mod.rs`**: アプリケーションの主要なロジック、状態管理、UI操作の処理を定義します。
- **`src/app/shortcuts.rs`**: ショートカットキーに関連するロジックを定義します。
- **`src/app_init.rs`**: アプリケーションの初期化処理を定義します。
- **`src/audio.rs`**: リアルタイム音声フィードバックに関する処理（`ym2151-log-play-server`との連携）を定義します。
- **`src/config.rs`**: アプリケーションの設定（キーバインドなど）の読み込み・保存を扱います。
- **`src/event_loop.rs`**: ターミナルイベント（キー入力、マウス操作など）の処理ループを管理します。
- **`src/favorites.rs`**: お気に入り音色の管理機能を提供します。
- **`src/file_ops.rs`**: 音色データのファイルからの読み込み、ファイルへの保存などの操作を定義します。
- **`src/history.rs`**: 音色変更履歴の管理機能を提供します。
- **`src/history_selector.rs`**: 変更履歴を選択・適用するためのUIロジックを定義します。
- **`src/logging.rs`**: アプリケーションのログ出力に関する処理を定義します。
- **`src/main.rs`**: アプリケーションのエントリーポイント。主要なコンポーネントを初期化し、イベントループを開始します。
- **`src/midi_conversion.rs`**: MIDIデータとYM2151レジスタ値の変換ロジックを扱います。
- **`src/models.rs`**: YM2151音色データ構造やアプリケーションの状態を表すデータモデルを定義します。
- **`src/random_tone.rs`**: ランダムなYM2151音色を生成するロジックを定義します。
- **`src/register.rs`**: YM2151音源の各レジスタ（DT, MUL, TLなど）の定義と値の操作ロジックを扱います。
- **`src/register_list.rs`**: UI上でレジスタリストを表示し、操作するためのロジックを定義します。
- **`src/tests/`**: アプリケーションの各機能ユニットに対するテストコードを格納するディレクトリです。
- **`src/ui/help.rs`**: ヘルプ画面の表示ロジックを定義します。
- **`src/ui/helpers.rs`**: UI描画のための補助関数を定義します。
- **`src/ui/mod.rs`**: ターミナルユーザーインターフェース（TUI）の主要なコンポーネントと描画ロジックを定義します。
- **`src/updater.rs`**: アプリケーションの更新機能に関するロジックを定義します。
- **`src/variation_selector.rs`**: 音色のバリエーションを選択するためのUIロジックを定義します。
- **`src/waveform.rs`**: 波形表示に関連する（今後の実装候補）ロジックを定義します。
- **`tones/general_midi/000_AcousticGrand.json`**: General MIDIの「Acoustic Grand Piano」に相当するYM2151音色のデータ例です。
- **`tones/general_midi/tone_names.json`**: 音色名のリストを格納するJSONファイルです。
- **`wasm/Cargo.lock` / `wasm/Cargo.toml` / `wasm/src/lib.rs`**: WebAssembly (Wasm) ターゲット向けのライブラリコードとその設定です。
- **`ym2151-tone-editor.toml.example`**: 設定ファイルの例。ユーザーがコピーしてカスタマイズできます。

## 関数詳細説明
関数呼び出し階層を分析できませんでした。個別の関数の詳細な説明は、現在の情報からは提供できません。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2026-03-18 07:16:01 JST
