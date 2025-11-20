Last updated: 2025-11-21

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色を編集するための、Windows向けテキストユーザーインターフェース (TUI) エディタです。
- リアルタイムでの音色パラメータ変更と音声フィードバックを提供し、直感的で素早い音作りを支援します。
- 編集した音色は専用フォーマットで保存・管理でき、今後の開発でより効率的な音色ライブラリ構築を目指します。

## 技術スタック
使用している技術をカテゴリ別に整理して説明
- フロントエンド: **Ratatui** (0.28) - Rust製のターミナルユーザーインターフェース (TUI) フレームワークで、テキストベースのグラフィカルな表示を実現しています。**Crossterm** (0.28) - クロスプラットフォームなターミナル操作ライブラリで、キー入力や画面描画を制御します。
- 音楽・オーディオ: **ym2151-log-play-server** - Windows環境限定で、リアルタイムの音声フィードバックを提供するためのライブラリです。エディタと連携し、パラメータ変更を即座に音で確認できます。
- 開発ツール: **Rust** (1.70 以降) - 高速かつ安全なアプリケーション開発に適したプログラミング言語です。
- テスト: Rustの標準テスト機能を用いて、各モジュールの機能が期待通りに動作するかを検証しています。
- ビルドツール: **Cargo** - Rustの公式パッケージマネージャー兼ビルドシステムです。依存関係の管理とプロジェクトのビルドを効率的に行います。
- 言語機能: **Rust** - メモリ安全性、並行処理、高性能を特徴とする現代的なプログラミング言語です。
- 自動化・CI/CD: 明示的なCI/CDパイプラインの記述はありませんが、`cargo build --release`コマンドによるリリースビルドが可能です。
- 開発標準: 特記なし。Rustの慣習に則った開発が行われています。

## ファイル階層ツリー
```
.
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.ja.md
├── README.md
├── _config.yml
├── docs/
│   └── KEYBINDS.ja.md
├── generated-docs/
├── issue-notes/
│   ├── 55.md
│   ├── 57.md
│   ├── 59.md
│   ├── 61.md
│   ├── 62.md
│   ├── 65.md
│   ├── 66.md
│   ├── 68.md
│   ├── 70.md
│   ├── 72.md
│   ├── 75.md
│   ├── 77.md
│   ├── 79.md
│   ├── 81.md
│   ├── 83.md
│   ├── 85.md
├── src/
│   ├── app.rs
│   ├── audio.rs
│   ├── config.rs
│   ├── file_ops.rs
│   ├── main.rs
│   ├── midi_conversion.rs
│   ├── models.rs
│   ├── register.rs
│   ├── tests/
│   │   ├── app_tests.rs
│   │   ├── file_ops_tests.rs
│   │   ├── midi_conversion_tests.rs
│   │   ├── mod.rs
│   │   ├── register_tests.rs
│   │   ├── ui_tests.rs
│   │   └── verbose_logging_tests.rs
│   └── ui.rs
├── tones/
│   └── general_midi/
│       └── 000_AcousticGrand.json
└── ym2151-tone-editor.toml.example
```

## ファイル詳細説明
- **.gitignore**: Gitのバージョン管理から除外するファイルやディレクトリを指定します。
- **Cargo.lock**: プロジェクトの依存関係の正確なバージョンを記録し、ビルドの再現性を保証します。
- **Cargo.toml**: Rustプロジェクトのマニフェストファイルで、プロジェクトのメタデータ、依存関係、ビルド設定などが定義されています。
- **LICENSE**: プロジェクトの利用条件を定めるライセンス情報が記述されています。
- **README.ja.md**: プロジェクトの概要、機能、ビルド方法、操作方法などを日本語で説明する主要なドキュメントです。
- **README.md**: プロジェクトの概要、機能、ビルド方法、操作方法などを英語で説明する主要なドキュメントです。
- **_config.yml**: GitHub Pagesなどのウェブサイトの構成設定ファイルです。
- **docs/KEYBINDS.ja.md**: エディタのキーバインド（操作キー）に関する詳細な情報を日本語で提供します。
- **generated-docs/**: 自動生成されたドキュメントやコードが格納される可能性のあるディレクトリです。
- **issue-notes/*.md**: 開発中に発生した課題や検討事項、決定事項などを記録したメモファイル群です。
- **src/app.rs**: アプリケーションのコアロジックを担います。UIの描画、ユーザーイベントの処理、アプリケーション状態の管理など、エディタ全体の挙動を制御します。
- **src/audio.rs**: リアルタイム音声フィードバックに関する処理を担当します。特に、`ym2151-log-play-server`ライブラリとの連携や、音色データ/レジスタ変更コマンドのサーバーへの送信ロジックを実装しています。
- **src/config.rs**: アプリケーションの設定（キーバインドの定義など）を読み込み、管理するためのロジックを格納しています。
- **src/file_ops.rs**: ファイル操作に関する機能を提供します。音色データのJSONファイルへの保存や、アプリケーション起動時の最新音色データの読み込みなどを担当します。
- **src/main.rs**: Rustアプリケーションのエントリポイントです。アプリケーションの初期化を行い、メインのアプリケーションループ（`app.rs`で定義）を起動します。
- **src/midi_conversion.rs**: MIDIノート番号から周波数への変換など、MIDI関連の補助的な変換ロジックを提供する可能性があります。
- **src/models.rs**: アプリケーション内で使用されるデータ構造（YM2151の音色パラメータ、音色データ全体の構造など）を定義しています。
- **src/register.rs**: YM2151音源チップのレジスタ操作に関するロジックをカプセル化しています。パラメータ値とレジスタ値の変換、レジスタの有効範囲管理などを行います。
- **src/tests/**: 各モジュールの単体テストや統合テストを含むディレクトリです。
    - **src/tests/app_tests.rs**: `app.rs`で定義されたロジックに対するテストです。
    - **src/tests/file_ops_tests.rs**: `file_ops.rs`のファイル操作機能に対するテストです。
    - **src/tests/midi_conversion_tests.rs**: `midi_conversion.rs`の変換ロジックに対するテストです。
    - **src/tests/mod.rs**: `src/tests`モジュールのルートファイルで、他のテストファイルを宣言します。
    - **src/tests/register_tests.rs**: `register.rs`のYM2151レジスタ関連ロジックに対するテストです。
    - **src/tests/ui_tests.rs**: `ui.rs`で定義されたユーザーインターフェースの描画や挙動に対するテストです。
    - **src/tests/verbose_logging_tests.rs**: 詳細なロギング機能に関するテストです。
- **src/ui.rs**: ターミナルユーザーインターフェース (TUI) の描画ロジックを管理します。Ratatuiライブラリを使用して、パラメータテーブルやステータスバーなど、画面の各要素を描画します。
- **tones/general_midi/000_AcousticGrand.json**: General MIDIの「Acoustic Grand Piano」に相当するYM2151音色データの例です。
- **ym2151-tone-editor.toml.example**: アプリケーションの設定ファイル`ym2151-tone-editor.toml`の例であり、設定のカスタマイズ方法を示しています。

## 関数詳細説明
このプロジェクトはRustで実装されており、具体的な関数名やシグネチャはソースコードに依存しますが、ファイルごとの役割から以下の主要な機能が提供されていると推測できます。

- **`app.rs`**:
    - `run()`: アプリケーションのメインループを実行し、イベント処理とUIの更新を統括します。
    - `handle_event(event)`: ユーザーからのキー入力やマウスイベントを検知し、それに応じてアプリケーションの状態を更新します。
    - `update_state(parameter_changes)`: 音色パラメータの変更をアプリケーションの状態に反映し、必要に応じて音声フィードバックをトリガーします。
- **`audio.rs`**:
    - `ensure_server_ready()`: リアルタイム音声フィードバックのための`ym2151-log-play-server`が起動していることを確認し、必要であれば自動でセットアップ・起動します。
    - `send_json(tone_data)`: レガシーモードにおいて、現在の音色データ全体をJSON形式で音声サーバーに送信します。
    - `start_interactive()`: インタラクティブモードを開始し、サーバー上での継続的なオーディオストリーミングを開始させます。
    - `write_register(address, value)`: インタラクティブモードにおいて、YM2151チップの特定のレジスタアドレスに新しい値を書き込むコマンドを音声サーバーに送信します。
    - `stop_interactive()`: インタラクティブモードを終了し、音声ストリーミングを停止します。
- **`config.rs`**:
    - `load_config()`: 設定ファイルからキーバインドなどのアプリケーション設定を読み込みます。
    - `get_keybind(action)`: 特定のアプリケーションアクションに対応するキーバインドを返します。
- **`file_ops.rs`**:
    - `load_latest_tone()`: アプリケーション終了時に保存された、最新の音色データをファイルから読み込みます。
    - `save_tone_as_json(tone_data)`: 現在編集中の音色データを、指定されたJSONファイル形式で保存します。
- **`main.rs`**:
    - `main()`: プログラムの実行開始点です。アプリケーション全体の初期化と`app::run()`関数の呼び出しを行います。
- **`midi_conversion.rs`**:
    - `note_to_frequency(midi_note_number)`: MIDIノート番号をYM2151が使用する周波数やピッチ関連の値に変換する可能性があります。
- **`models.rs`**:
    - `ToneData::new()`: 新しいYM2151音色データの構造体インスタンスを作成します。
    - `ToneData::from_json(json_string)`: JSON形式の文字列をパースし、`ToneData`構造体として変換します。
    - `ToneData::to_json()`: `ToneData`構造体の内容をJSON形式の文字列に変換します。
- **`register.rs`**:
    - `YM2151Parameter::to_register_value(param_value)`: エディタで表示される抽象化されたパラメータ値を、YM2151チップの実際のレジスタ値に変換します。
    - `YM2151Parameter::from_register_value(register_value)`: YM2151チップのレジスタ値を、エディタで表示・編集するためのパラメータ値に変換します。
    - `get_parameter_range(param_name)`: 指定されたYM2151パラメータの有効な値の最小値と最大値を提供します。
- **`ui.rs`**:
    - `draw_tui(frame, app_state)`: 現在のアプリケーション状態に基づいて、ターミナルユーザーインターフェースの全体を描画します。
    - `render_parameter_table(frame, parameters)`: YM2151の各音色パラメータを表示するテーブル部分を描画します。
    - `render_status_bar(frame, status_message)`: 画面下部に、現在のモードやヒントなどのステータスメッセージを表示するバーを描画します。

## 関数呼び出し階層ツリー
```

---
Generated at: 2025-11-21 07:08:41 JST
