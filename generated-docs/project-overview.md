Last updated: 2025-12-07

# Project Overview

## プロジェクト概要
- YM2151（OPM）FM音源の音色を編集するための、Windows向けターミナルユーザーインターフェース（TUI）アプリケーションです。
- Rustで実装されており、音色パラメータのリアルタイム調整と音声フィードバックを提供します。
- 直感的な操作と自動セーブ・ロード機能を備え、シンプルな音色作成・編集体験を目指しています。

## 技術スタック
- フロントエンド: `ratatui` (0.28) を使用し、ターミナル上でリッチなユーザーインターフェースを構築しています。`crossterm` (0.28) により、クロスプラットフォームでのキーボードやマウス操作、ターミナル制御を実現しています。
- 音楽・オーディオ: YM2151（OPM）FM音源をターゲットとし、リアルタイム音声フィードバックのために `ym2151-log-play-server` ライブラリを自動でセットアップ・起動します。将来的に `mmlabc-to-smf-rust` と `smf-to-ym2151log-rust` を利用し、MMLからのSMF生成およびYM2151ログ生成を計画しています。
- 開発ツール: Rust 1.70以降のバージョンで開発されており、その標準ツールチェインである `cargo` をビルド、実行、依存関係管理に利用しています。
- テスト: Rustの組み込みテストフレームワークを利用し、`src/tests/` ディレクトリ配下で各モジュールの単体テストや統合テストを実施しています。
- ビルドツール: Rustの標準ビルドツールである `cargo` を使用して、プロジェクトのコンパイルと実行を管理しています。
- 言語機能: 安全性、パフォーマンス、並行性に優れたシステムプログラミング言語であるRustで全面的に記述されています。
- 自動化・CI/CD: (情報が提供されていないため、特定できません。)
- 開発標準: (情報が提供されていないため、特定できません。)

## ファイル階層ツリー
```
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📄 _config.yml
📁 docs/
  📖 KEYBINDS.ja.md
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
- **.gitignore**: Gitがバージョン管理の対象としないファイルやディレクトリを指定します。
- **Cargo.lock**: プロジェクトのビルドに必要な依存クレートの正確なバージョンとハッシュを記録します。
- **Cargo.toml**: Rustプロジェクトの構成ファイル。プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
- **LICENSE**: 本プロジェクトのライセンス情報が記述されています。
- **README.ja.md**: プロジェクトの日本語による概要、機能、使用方法などを説明する主要なドキュメントです。
- **README.md**: プロジェクトの英語による概要、機能、使用方法などを説明する主要なドキュメントです。
- **_config.yml**: Jekyllなどの静的サイトジェネレーターの設定ファイルで、主にドキュメントサイトの構築に使用されます。
- **docs/**: プロジェクトに関する追加のドキュメントを格納するディレクトリです。
    - **KEYBINDS.ja.md**: アプリケーションのキーバインド（操作方法）に関する日本語の詳細説明です。
- **generated-docs/**: 自動生成されるドキュメントを格納するためのディレクトリです。
- **googled947dc864c270e07.html**: Googleのサイト所有権確認などのための静的なHTMLファイルです。
- **issue-notes/**: 開発中に検討された課題やアイデア、メモなどを記録するためのディレクトリです。
- **src/**: Rustのソースコードが格納されている主要なディレクトリです。
    - **app.rs**: アプリケーションのコアロジック、状態管理、ユーザーイベント処理、UI更新の調整などを担います。
    - **app_init.rs**: アプリケーション起動時の初期設定やリソースの準備に関するコードが含まれます。
    - **audio.rs**: YM2151音源へのオーディオ出力やリアルタイム音声フィードバックに関連する処理を管理します。
    - **config.rs**: アプリケーションの設定（例：キーバインド）の読み込み、解析、管理を行います。
    - **file_ops.rs**: 音色データなどのファイルの読み書き、保存、ロードといったファイルシステム操作を扱います。
    - **main.rs**: アプリケーションのエントリーポイントであり、初期化からメインループの実行までを制御します。
    - **midi_conversion.rs**: MIDIイベントやMMLデータとYM2151レジスタ値間の変換ロジックを提供します。
    - **models.rs**: アプリケーション内で使用される主要なデータ構造（音色パラメータ、UI状態など）を定義します。
    - **register.rs**: YM2151レジスタの値を表現し、その操作やバリデーションロジックを含みます。
    - **tests/**: 各モジュールの単体テストコードを格納するディレクトリです。
        - **app_tests.rs**: `app.rs`モジュールのテストコードです。
        - **file_ops_tests.rs**: `file_ops.rs`モジュールのテストコードです。
        - **midi_conversion_tests.rs**: `midi_conversion.rs`モジュールのテストコードです。
        - **mod.rs**: `tests`モジュール内のテストを構成するためのRustモジュールファイルです。
        - **register_tests.rs**: `register.rs`モジュールのテストコードです。
        - **ui_tests.rs**: `ui.rs`モジュールのテストコードです。
        - **variation_selector_tests.rs**: `variation_selector.rs`モジュールのテストコードです。
        - **verbose_logging_tests.rs**: 詳細ロギング機能に関するテストコードです。
    - **ui.rs**: ターミナルユーザーインターフェース (TUI) の描画ロジック、イベント処理、コンポーネント管理を実装します。
    - **variation_selector.rs**: 音色のバリエーションを選択、管理するためのロジックが含まれます。
- **tones/**: アプリケーションで使用される音色データ（プリセットなど）を格納するディレクトリです。
    - **general_midi/**: General MIDI互換の音色データが格納されるディレクトリです。
        - **000_AcousticGrand.json**: 「Acoustic Grand Piano」の音色データ（JSON形式）の例です。
        - **tone_names.json**: 音色名のリストまたは、音色データを管理するためのメタデータが記述されています。
- **ym2151-tone-editor.toml.example**: アプリケーションの設定ファイルの例で、ユーザーがこれをコピーしてカスタマイズできます。

## 関数詳細説明
プロジェクト情報には具体的な関数名やシグネチャが提供されていないため、詳細な関数説明は生成できません。一般的にRustアプリケーションには、`main`関数（アプリケーションのエントリーポイント）や、各モジュール（例: `app.rs`, `ui.rs`, `audio.rs`）内にそれぞれの機能に応じた関数が定義されています。

## 関数呼び出し階層ツリー
```
プロジェクト情報に関数呼び出し階層ツリーに関する詳細な情報が提供されていないため、生成できませんでした。

---
Generated at: 2025-12-07 07:07:57 JST
