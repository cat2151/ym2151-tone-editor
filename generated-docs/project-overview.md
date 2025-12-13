Last updated: 2025-12-14

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色をターミナル上で編集するためのアプリケーションです。
- Windows環境で動作し、リアルタイムで音色の変化を音声で確認できます。
- 直感的なTUI（Terminal User Interface）とキーボード・マウス操作で音色作成を楽しめます。

## 技術スタック
- フロントエンド: **Ratatui 0.28** (ターミナルUIの描画と管理)、**Crossterm 0.28** (クロスプラットフォームなターミナル操作を可能にするライブラリ)
- 音楽・オーディオ: **ym2151-log-play-server** (YM2151ログを再生するバックエンドサーバーとの連携によりリアルタイム音声フィードバックを実現)、YM2151 FM音源のレジスタ操作
- 開発ツール: **Rust 1.70以降** (プログラミング言語)、**Cargo** (Rustのプロジェクト管理およびビルドツール)
- テスト: Rustの標準テストフレームワーク (`src/tests/` ディレクトリ配下で単体テストや結合テストを実装)
- ビルドツール: **Cargo** (プロジェクトのコンパイル、テスト、実行を管理)
- 言語機能: **Rust** (安全性、並行性、パフォーマンスに優れたシステムプログラミング言語)
- 自動化・CI/CD: (プロジェクト情報からは具体的なツールは特定できませんが、CargoコマンドがCI/CDパイプラインに組み込まれることを想定)
- 開発標準: (プロジェクト情報からは具体的なツールやルールは特定できません)

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
  📖 172.md
  📖 174.md
  📖 175.md
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
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定します。
- **`Cargo.lock`**: プロジェクトの依存関係の正確なバージョンを記録し、ビルドの一貫性を保証します。
- **`Cargo.toml`**: Rustプロジェクトの設定ファイル。依存クレート、メタデータ、ビルド設定などが記述されています。
- **`LICENSE`**: プロジェクトのライセンス情報が記載されています。
- **`README.ja.md`**, **`README.md`**: プロジェクトの概要、機能、使い方などを説明するドキュメント（日本語版と英語版）。
- **`README_generate_gm_templates.md`**: General MIDIテンプレートの生成に関する説明ドキュメント。
- **`_config.yml`**: GitHub Pagesなどのウェブサイト設定ファイル。
- **`docs/KEYBINDS.ja.md`**: アプリケーションのキーバインド（キー操作）に関する日本語の説明ドキュメント。
- **`generate_gm_templates.rs`**: General MIDI音色テンプレートを生成するためのRustスクリプト。
- **`googled947dc864c270e07.html`**: Googleのサイト所有権確認用のファイル。
- **`issue-notes/`**: 開発中の課題や検討事項をメモしたファイル群が格納されています。（来訪者向けのため、これらの内容は開発者向けのものです）
- **`src/main.rs`**: アプリケーションのエントリーポイント。プログラムの起動時に最初に実行されるコードを含みます。
- **`src/app.rs`**: アプリケーションのコアロジックを管理します。UIの状態、ユーザー入力処理、音色パラメータの更新などが含まれます。
- **`src/app_init.rs`**: アプリケーションの起動時における初期化処理を担います。
- **`src/audio.rs`**: YM2151音源のリアルタイム音声フィードバックに関する処理を担当します。`ym2151-log-play-server`との通信を管理します。
- **`src/config.rs`**: アプリケーションの設定（例: キーバインド）の読み込み、解析、管理を行います。
- **`src/file_ops.rs`**: 音色データの保存（自動セーブ）や読み込みなど、ファイルシステムに関する操作を処理します。
- **`src/midi_conversion.rs`**: MIDI関連のデータ（MMLなど）をYM2151が解釈できる形式に変換する処理を含みます。
- **`src/models.rs`**: アプリケーション全体で使用されるデータ構造（YM2151のパラメータ、音色構造、UIの状態など）を定義します。
- **`src/register.rs`**: YM2151音源のレジスタに関するデータ構造と、その操作ロジックを定義します。
- **`src/ui.rs`**: ターミナルユーザーインターフェース (TUI) の描画ロジックを管理します。画面のレイアウトや要素の表示を担当します。
- **`src/variation_selector.rs`**: 複数の音色バリエーションを切り替えたり選択したりするUIロジックを実装します。
- **`src/tests/`**: 各ソースコードモジュール（`app`, `file_ops`, `midi_conversion`など）のテストコードが格納されています。
- **`tones/general_midi/000_AcousticGrand.json`**: General MIDIの「Acoustic Grand Piano」ファミリーの音色データがJSON形式で保存されています。
- **`tones/general_midi/tone_names.json`**: General MIDIの音色名のリストなど、音色管理に利用されるデータ。
- **`ym2151-tone-editor.toml.example`**: 設定ファイルの記述例を提供し、ユーザーがカスタム設定を作成する際の参考となります。

## 関数詳細説明
提供されたプロジェクト情報（READMEとファイル一覧）からは、個々の関数の具体的な役割、引数、戻り値、機能までを詳細に抽出することはできませんでした。通常、これらの情報はソースコードの解析によって得られます。

```
関数情報はソースコード解析によって詳細化されます。
```

## 関数呼び出し階層ツリー
提供されたプロジェクト情報からは、関数間の具体的な呼び出し関係をツリー形式で表現することはできませんでした。この情報は通常、ソースコードの静的解析や実行時プロファイリングによって生成されます。

```
関数呼び出し階層はソースコード解析によって詳細化されます。

---
Generated at: 2025-12-14 07:08:10 JST
