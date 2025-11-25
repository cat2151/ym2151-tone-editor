Last updated: 2025-11-26

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色を、リアルタイム音声フィードバック付きで編集できるWindows用ターミナルエディタです。
- 直感的なキーボード・マウス操作とシンプルなTUI (Text-based User Interface) により、音色作成を支援します。
- Rust言語で開発されており、高速起動と即時演奏が可能な設計思想に基づいて構築されています。

## 技術スタック
- フロントエンド: **ratatui** (0.28) - Rust製のターミナルUIフレームワーク。テキストベースの豊かなユーザーインターフェースを構築するために使用されます。
- 音楽・オーディオ: **ym2151-log-play-server** - リアルタイムでのYM2151音源の音声フィードバックを可能にするライブラリ。Windows環境でのみ動作し、エディタからのパラメータ変更を即座に音として反映します。
- 開発ツール: **Rust** (1.70 以降) - 高速かつ安全なシステムプログラミング言語。本プロジェクトの主要な開発言語です。
- テスト: `src/tests` ディレクトリ内のモジュール別テストファイル群 - アプリケーション、ファイル操作、MIDI変換、レジスタ操作、UIといった各機能の単体テストや統合テストを実施します。
- ビルドツール: **cargo** - Rustプロジェクトのビルド、依存関係管理、テスト実行などを統合的に行う公式のパッケージマネージャー兼ビルドシステムです。
- 言語機能: **Rust 1.70 以降** - プロジェクトの動作に推奨されるRust言語のバージョンであり、特定の言語機能やライブラリの安定版を利用します。
- 自動化・CI/CD: (特になし) - ビルドおよび実行は`cargo`コマンドを用いて手動で行います。
- 開発標準: **crossterm** (0.28) - クロスプラットフォームでターミナル操作（カーソル移動、キー入力イベントなど）を扱うためのライブラリです。

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
    📄 verbose_logging_tests.rs
  📄 ui.rs
📁 tones/
  📁 general_midi/
    📊 000_AcousticGrand.json
📄 ym2151-tone-editor.toml.example
```

## ファイル詳細説明
- **README.ja.md / README.md**: プロジェクトの目的、機能、操作方法、ビルド手順などを記述した主要なドキュメント。日本語版と英語版があります。
- **docs/KEYBINDS.ja.md**: アプリケーションのキーバインド（キー操作割り当て）に関する詳細な説明を提供します。
- **issue-notes/**: 開発過程で発生した課題や検討事項に関するメモが保存されています。
- **tones/general_midi/000_AcousticGrand.json**: YM2151音色データの保存例。JSON形式で音色パラメータと関連情報が記述されています。
- **ym2151-tone-editor.toml.example**: アプリケーションの設定ファイル（例: キーバインド）のサンプルです。
- **src/main.rs**: アプリケーションのエントリーポイント。起動時の初期設定、イベントループの開始など、プログラム全体のライフサイクルを制御します。
- **src/app.rs**: アプリケーションのコアロジックを実装。ユーザー入力の処理、アプリケーション状態の更新、UI描画のトリガーなど、全体の振る舞いを管理します。
- **src/app_init.rs**: アプリケーションの起動時に必要な初期化処理（設定の読み込み、ターミナル設定など）を行います。
- **src/audio.rs**: `ym2151-log-play-server`との連携を担い、YM2151音源へのコマンド送信、リアルタイム音声フィードバックの管理を行います。
- **src/config.rs**: アプリケーションの設定（キーバインドなど）をファイルから読み込み、メモリ上で管理し、必要に応じて保存する機能を提供します。
- **src/file_ops.rs**: 音色データのファイルへの保存（自動セーブ）やファイルからの読み込み（自動ロード）など、永続化に関わるファイルシステム操作を処理します。
- **src/midi_conversion.rs**: MIDIノート番号とYM2151音源の周波数パラメータ間の変換など、MIDI関連のデータ処理ロジックを扱います。
- **src/models.rs**: YM2151の各パラメータ（DT, MUL, TLなど）のデータ構造、音色全体の構造、アプリケーションの状態など、プログラム内で使用されるデータモデルを定義します。
- **src/register.rs**: YM2151音源のレジスタ（特定のアドレスに特定の値を書き込むことで音色を制御）に関する抽象化と操作ロジックを提供します。
- **src/ui.rs**: `ratatui`ライブラリを用いて、ターミナル上に音色パラメータ、カーソル、メッセージなどのUI要素を描画するロジックを実装します。
- **src/tests/**: 各モジュールの機能が期待通りに動作するかを検証するためのテストコード群が格納されています。

## 関数詳細説明
- **main()**: アプリケーションの起動点。初期設定を行い、メインアプリケーションループを開始します。
- **run_app()**: アプリケーションの主要なイベントループを実行。ユーザー入力の監視、状態更新、UI描画を繰り返し処理します。
- **handle_event()**: キーボードやマウスからのユーザー入力イベントを受け取り、それに応じてアプリケーションの状態を変更します。
- **draw_ui()**: 現在のアプリケーションの状態に基づいて、ターミナル画面にエディタのインターフェース（パラメータ値、カーソル、メッセージなど）を描画します。
- **update_parameter()**: ユーザー操作に応じて、選択されているYM2151音色パラメータの値を増減または直接設定します。
- **preview_tone()**: 現在編集中の音色データに基づいて、YM2151音源サーバーを通して音色を再生し、リアルタイムでフィードバックを提供します。
- **save_tone()**: 現在のエディタの音色データを指定されたフォーマット（例: JSON）でファイルに保存します。終了時の自動セーブも含まれます。
- **load_tone()**: ファイルから音色データを読み込み、エディタにロードして編集を再開できるようにします。起動時の自動ロードも可能です。
- **ensure_server_ready()**: リアルタイム音声フィードバックのために必要な`ym2151-log-play-server`が起動していることを確認し、必要であれば自動的に起動します。
- **send_register_command()**: インタラクティブモードで使用され、YM2151サーバーに対して個々のレジスタ書き込みコマンドを効率的に送信します。
- **send_full_tone_data()**: レガシーモードで使用され、YM2151サーバーに対して完全な音色データをJSON形式で送信します。
- **set_keybind()**: 設定ファイルから読み込んだキーバインドをアプリケーションに適用し、ユーザー操作への応答方法をカスタマイズします。
- **get_ym2151_parameter_value()**: 指定されたYM2151パラメータの現在の値を取得し、表示や処理に利用します。
- **convert_midi_note_to_frequency()**: 標準的なMIDIノート番号を、YM2151音源で利用可能な周波数関連のパラメータ値に変換します。

## 関数呼び出し階層ツリー
```
main()
└── run_app()
    ├── handle_event()
    │   ├── update_parameter()
    │   ├── preview_tone()
    │   │   └── audio::ensure_server_ready()
    │   │   └── audio::send_register_command() / audio::send_full_tone_data()
    │   ├── save_tone()
    │   │   └── file_ops::save_tone_data_to_file()
    │   ├── load_tone()
    │   │   └── file_ops::load_tone_data_from_file()
    │   └── config::set_keybind()
    └── draw_ui()
        ├── ui::render_tone_editor()
        ├── ui::render_status_bar()
        └── models::get_current_tone_state()

---
Generated at: 2025-11-26 07:08:44 JST
