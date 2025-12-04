Last updated: 2025-12-05

# Project Overview

## プロジェクト概要
- YM2151（OPM）FM音源の音色を編集するための、Windows向けのターミナルユーザーインターフェース（TUI）エディタです。
- 音色パラメータの直感的な編集、マウスやキーボードによる操作、リアルタイムの音声フィードバック、および音色の自動セーブ/ロード機能を提供します。
- Rust言語で開発されており、シンプルで高速な起動と操作性を目指し、将来的な音色管理機能の拡充も計画されています。

## 技術スタック
- フロントエンド: `ratatui` (0.28) - ターミナルユーザーインターフェース（TUI）構築のためのRustフレームワークです。
- 音楽・オーディオ:
    - `ym2151-log-play-server` - リアルタイム音声フィードバックを可能にするためのサーバーライブラリで、音色エディタからレジスタ書き込みコマンドや音色データを送信します。
    - `mmlabc-to-smf-rust` - MML（Music Macro Language）形式の音楽データをStandard MIDI File (SMF) 形式に変換するためのライブラリです。
    - `smf-to-ym2151log-rust` - SMFデータとYM2151音色情報からYM2151ログデータを生成するためのライブラリです。
- 開発ツール: `Rust` - システムプログラミング言語であり、安全性、速度、並行性を重視しています。
- テスト: `Rust標準テストフレームワーク` - Rust言語に標準で組み込まれているテストツールを利用して、コードの品質と信頼性を確保しています。
- ビルドツール: `cargo` - Rustプロジェクトのビルド、パッケージ管理、テスト実行などを統合的に行う公式ツールです。
- 言語機能: `Rust 1.70 以降` - プロジェクトはRustの特定のバージョン以降の言語機能およびランタイムに依存しています。

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
- **.gitignore**: Gitがバージョン管理から無視するファイルやディレクトリを指定します。
- **Cargo.lock**: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証します。
- **Cargo.toml**: Rustプロジェクトのメタデータ（名前、バージョンなど）と外部ライブラリへの依存関係を定義するマニフェストファイルです。
- **LICENSE**: 本プロジェクトのソフトウェアライセンス情報が記述されています。
- **README.ja.md**: プロジェクトの日本語での概要、機能、セットアップ方法、使い方などが記載された主要なドキュメントです。
- **README.md**: プロジェクトの英語での概要、機能、セットアップ方法、使い方などが記載された主要なドキュメントです。
- **_config.yml**: GitHub Pagesなどで使用されるJekyllなどの静的サイトジェネレータの設定ファイルです。
- **docs/KEYBINDS.ja.md**: アプリケーションのキーバインド（キー操作）に関する日本語の詳細説明ドキュメントです。
- **generated-docs/**: ドキュメンテーションツールによって自動生成されたファイルが格納されるディレクトリです。
- **googled947dc864c270e07.html**: Googleサイト所有権の確認などの目的で配置されることが多いファイルです。
- **issue-notes/**: 開発中に発生した課題や検討事項に関するメモが格納されています。
- **src/app.rs**: アプリケーション全体の状態管理とメインロジックを定義します。UIの状態、音色データ、イベント処理などを統括します。
- **src/app_init.rs**: アプリケーションの起動時に必要な初期化処理（設定のロード、オーディオサーバーの準備など）を定義します。
- **src/audio.rs**: リアルタイム音声フィードバックのためのロジックを含みます。YM2151サーバーとの通信や音色データの送信などを担当します。
- **src/config.rs**: アプリケーションの設定（例: キーバインド、各種オプション）の読み込みと保存を処理します。
- **src/file_ops.rs**: ファイルシステムに関連する操作、特に音色データのロードとセーブのロジックを実装しています。
- **src/main.rs**: プログラムのエントリーポイント（開始点）であり、アプリケーションの実行フロー全体を調整します。
- **src/midi_conversion.rs**: MMLからSMF、SMFからYM2151ログといったMIDI関連データの変換ロジックを定義します。
- **src/models.rs**: アプリケーション内で使用されるデータ構造（YM2151の音色パラメータ、エディタの状態など）を定義します。
- **src/register.rs**: YM2151音源のレジスタ（各種パラメータを制御するメモリ領域）への読み書きや操作に関連するロジックを実装しています。
- **src/tests/**: アプリケーションの様々なコンポーネントに対する単体テストコードを格納するディレクトリです。
- **src/ui.rs**: ターミナルユーザーインターフェース（TUI）の描画ロジックを担当します。`ratatui`ライブラリを使用して画面要素をレンダリングします。
- **src/variation_selector.rs**: 複数の音色バリエーションを管理し、それらの選択・切り替えを行うためのロジックを提供します。
- **tones/general_midi/000_AcousticGrand.json**: General MIDI互換の「Acoustic Grand Piano」の音色データ定義を含むJSONファイルです。複数のバリエーションを含めることができます。
- **tones/general_midi/tone_names.json**: 音色名とそのIDなどを定義したJSONファイルで、音色管理に利用されます。
- **ym2151-tone-editor.toml.example**: アプリケーションの設定ファイルである`ym2151-tone-editor.toml`の例（サンプル）です。

## 関数詳細説明
提供された情報では具体的な関数名やシグネチャが特定できないため、各ファイルの役割から推測される一般的な機能とその説明を記述します。

- `src/app.rs`
    - `new()`: アプリケーションの初期状態を構築し、新しいインスタンスを生成します。
    - `run()`: アプリケーションのメインループを実行し、ユーザー入力の処理とUIの更新を継続的に行います。
    - `update()`: キー入力やマウス操作などのイベントを受けて、アプリケーションの状態を更新します。
    - `handle_event()`: 特定のユーザー入力イベントを解釈し、対応する内部アクションをトリガーします。
- `src/app_init.rs`
    - `init_app_state()`: アプリケーションの起動時に必要な設定のロード、オーディオサーバーの準備、初期音色データのロードなど、全体的な初期化処理を行います。
- `src/audio.rs`
    - `ensure_server_ready()`: リアルタイム音声フィードバックを提供するサーバー（`ym2151-log-play-server`）が実行中であることを確認し、必要であれば起動します。
    - `send_tone_data()`: 現在編集中のYM2151音色データをオーディオサーバーに送信し、その音色での再生を指示します。
    - `send_register_write()`: 特定のYM2151レジスタに対する書き込みコマンドをサーバーに送信し、効率的なリアルタイム変更を実現します（インタラクティブモード用）。
    - `play_preview_note()`: 指定された音符と現在の音色で短い音を再生し、音色のプレビューを提供します。
- `src/config.rs`
    - `load_config()`: アプリケーションの設定ファイル（例: `ym2151-tone-editor.toml`）を読み込み、アプリケーション全体に設定を適用します。
    - `save_config()`: 現在のアプリケーション設定をファイルに保存します。
- `src/file_ops.rs`
    - `load_tone()`: 指定されたファイルパスからYM2151音色データを読み込み、アプリケーションの状態に反映させます。
    - `save_tone()`: 現在編集中のYM2151音色データをファイルに保存します。アプリケーション終了時の自動セーブ機能に利用されます。
- `src/main.rs`
    - `main()`: プログラムの実行開始点であり、アプリケーションの初期化、メインループの実行、および終了時のクリーンアップ処理を調整します。
- `src/midi_conversion.rs`
    - `mml_to_smf()`: MML形式の文字列をStandard MIDI File (SMF) 形式のバイナリデータに変換します。
    - `smf_to_ym2151_log()`: SMFデータとYM2151音色情報に基づいて、YM2151のレジスタログ（再生指示）データを生成します。
- `src/models.rs`
    - `get_parameter_range()`: YM2151の特定の音色パラメータが取りうる有効な値の範囲（最小値と最大値）を返します。
    - `update_parameter()`: 指定されたパラメータの値を更新し、その値が有効な範囲内にあるか検証します。
- `src/register.rs`
    - `set_register_value()`: YM2151の特定のレジスタアドレスに指定された値を設定します。
    - `get_register_value()`: YM2151の特定のレジスタアドレスから現在の値を取得します。
    - `tone_to_registers()`: 内部の音色データ構造を、YM2151レジスタに書き込むためのバイト列またはマップ形式に変換します。
    - `registers_to_tone()`: YM2151レジスタのバイト列またはマップから、アプリケーションで扱える音色データ構造を再構築します。
- `src/ui.rs`
    - `draw_ui()`: アプリケーションの現在の状態に基づいて、ターミナル上にユーザーインターフェース（各種パネル、ステータスバーなど）を描画します。
    - `render_parameter_panel()`: YM2151の各音色パラメータを表示するパネルを描画し、現在の値やカーソル位置を視覚化します。
    - `render_status_bar()`: アプリケーションの現在のモード、ヒント、エラーメッセージなどを表示するステータスバーを描画します。
- `src/variation_selector.rs`
    - `select_next_variation()`: 現在の音色ファイルの次のバリエーションに選択を移動します。
    - `select_previous_variation()`: 現在の音色ファイルの前のバリエーションに選択を移動します。
    - `get_current_variation_info()`: 現在選択されている音色バリエーションに関する詳細情報（説明、MMLなど）を取得します。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-12-05 07:07:51 JST
