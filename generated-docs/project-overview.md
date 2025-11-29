Last updated: 2025-11-30

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色を、リアルタイムでフィードバックを受けながら直感的に編集できるターミナルユーザーインターフェース（TUI）エディタです。
- Windows環境に特化し、簡単なキー操作やマウス操作でパラメータを増減させ、音色をプレビューできます。
- 編集した音色は自動的に保存され、再起動後も作業の続きから始められるよう設計されています。

## 技術スタック
- フロントエンド: **ratatui 0.28** (Rust製TUIフレームワーク、ターミナル上でリッチなユーザーインターフェースを構築), **crossterm 0.28** (クロスプラットフォームなターミナル操作ライブラリ、キー入力やカーソル移動などを制御)
- 音楽・オーディオ: **ym2151-log-play-server** (リアルタイム音声フィードバックを提供するサーバーライブラリ。音色エディタからYm2151レジスタデータを送信し、音を生成), **YM2151** (ヤマハのFM音源チップ、このプロジェクトの音色編集対象)
- 開発ツール: **Cargo** (Rustの公式ビルドシステムおよびパッケージマネージャ、プロジェクトのビルド、実行、テスト、依存関係管理に使用)
- テスト: Rustの標準テストフレームワーク (`cargo test`コマンドを通じて実行される) が利用されており、`src/tests/`ディレクトリ配下で各モジュールの単体テストが記述されています。
- ビルドツール: **Cargo** (Rustプロジェクトのコンパイル、最適化、バイナリ生成を行う)
- 言語機能: **Rust 1.70 以降** (安全性、パフォーマンス、並行処理に重点を置いたプログラミング言語)
- 自動化・CI/CD: ビルドと実行はCargoコマンド (`cargo build --release`, `cargo run`) で行われ、オーディオサーバーのセットアップは`ym2151-log-play-server`ライブラリの`ensure_server_ready()`関数によって自動化されています。
- 開発標準: **Cargo.toml** (プロジェクトのメタデータ、依存関係、ビルド設定を定義), **.gitignore** (Gitによるバージョン管理から除外するファイルを指定), **_config.yml** (GitHub Pagesの設定ファイルとして使用)

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
  📖 116.md
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
    📊 tone_names.json
📄 ym2151-tone-editor.toml.example
```

## ファイル詳細説明
-   `.gitignore`: Gitのバージョン管理から除外するファイルやディレクトリを指定します。
-   `Cargo.lock`: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現性のあるビルドを保証します。
-   `Cargo.toml`: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
-   `LICENSE`: プロジェクトの配布および使用に関するライセンス情報を提供します。
-   `README.ja.md`, `README.md`: プロジェクトの概要、機能、セットアップ方法、使い方などを説明するドキュメント（日本語と英語）。
-   `_config.yml`: GitHub Pagesのサイト設定ファイルとして使用されます。
-   `docs/KEYBINDS.ja.md`: キーバインドの詳細な説明を提供するドキュメント（日本語）。
-   `generated-docs/`: プロジェクトから自動生成されたドキュメントを格納するためのディレクトリです。
-   `issue-notes/`: 開発中の課題や検討事項に関するメモファイル群です。
-   `src/app.rs`: アプリケーションの主要なロジック、状態管理、イベント処理などを担当します。
-   `src/app_init.rs`: アプリケーションの初期化処理（ターミナル設定、初期データロードなど）を定義します。
-   `src/audio.rs`: リアルタイム音声フィードバックのためのYM2151オーディオサーバーとの通信処理を管理します。
-   `src/config.rs`: アプリケーションの設定（キーバインドなど）の読み込みと管理を行います。
-   `src/file_ops.rs`: 音色データのファイルへの保存やファイルからの読み込みといったファイルI/O操作を扱います。
-   `src/main.rs`: プログラムのエントリポイントであり、アプリケーションの起動とメインループを制御します。
-   `src/midi_conversion.rs`: MIDIノート番号とYM2151の周波数関連パラメータ（DT、MULなど）間の変換ロジックを提供します。
-   `src/models.rs`: YM2151のオペレータ、音色、アプリケーションの状態など、プロジェクトで使用される各種データ構造を定義します。
-   `src/register.rs`: YM2151の仮想レジスタの操作（値の読み書きや計算）に関するロジックをカプセル化します。
-   `src/tests/`: 各モジュールの単体テストコードを格納するディレクトリです。
    -   `src/tests/app_tests.rs`: `app`モジュールのテストコード。
    -   `src/tests/file_ops_tests.rs`: `file_ops`モジュールのテストコード。
    -   `src/tests/midi_conversion_tests.rs`: `midi_conversion`モジュールのテストコード。
    -   `src/tests/mod.rs`: `tests`モジュールの宣言ファイル。
    -   `src/tests/register_tests.rs`: `register`モジュールのテストコード。
    -   `src/tests/ui_tests.rs`: `ui`モジュールのテストコード。
    -   `src/tests/verbose_logging_tests.rs`: 詳細ロギング機能のテストコード。
-   `src/ui.rs`: ターミナルユーザーインターフェース (TUI) の描画ロジックを管理し、`ratatui`を使用して画面表示を構築します。
-   `tones/general_midi/000_AcousticGrand.json`: General MIDI準拠の「Acoustic Grand Piano」音色の設定データです。
-   `tones/general_midi/tone_names.json`: 音色名のリスト、または音色データを識別するためのインデックス情報を含みます。
-   `ym2151-tone-editor.toml.example`: アプリケーションの設定ファイル例。ユーザーがコピーしてカスタマイズできます。

## 関数詳細説明
-   **`main`** (src/main.rs):
    -   役割: アプリケーションのエントリポイント。
    -   機能: アプリケーションを起動し、初期化を行い、メインのイベントループを実行してユーザー入力や状態更新を処理し、最終的な終了処理を行います。
-   **`App`関連の関数** (src/app.rs):
    -   役割: アプリケーションのコアロジック、状態管理、イベント処理。
    -   機能: アプリケーションのインスタンスを生成し、TUIアプリケーションのメインループ内でイベント（キー入力、マウス操作、タイマーなど）を処理し、それに応じてアプリケーションの状態を更新します。
-   **`init_app`** (src/app_init.rs):
    -   役割: アプリケーションの初期設定。
    -   機能: ターミナル環境の準備、アプリケーションの設定ファイルの読み込み、初期音色データのロードなど、アプリケーションが開始する前に必要なセットアップを行います。
-   **`send_ym2151_data`**, **`ensure_server_ready`** (src/audio.rs):
    -   役割: YM2151オーディオサーバーとの通信。
    -   機能: 現在編集中のYM2151音色データをオーディオサーバーに送信してリアルタイムで音を生成させたり、オーディオサーバーが起動して音を鳴らせる状態にあることを確認・保証したりします。
-   **`load_config`**, **`save_config`** (src/config.rs):
    -   役割: アプリケーションの設定ファイル操作。
    -   機能: アプリケーションのキーバインドやその他の設定項目をファイルから読み込んだり、変更された設定をファイルに保存したりします。
-   **`load_tone`**, **`save_tone`** (src/file_ops.rs):
    -   役割: 音色データのファイルI/O処理。
    -   機能: 編集対象のYM2151音色データをJSONファイルなどの形式でストレージから読み込んだり、ユーザーが編集した音色データをファイルとして保存したりします。
-   **`midi_note_to_ym2151_params`** (src/midi_conversion.rs):
    -   役割: MIDIノート番号からYM2151の音高関連パラメータへの変換。
    -   機能: 標準的なMIDIノート番号を受け取り、その音高をYM2151音源で実現するために必要な周波数デチューン（DT）、周波数マルチプライヤー（MUL）などのレジスタ値を計算して返します。
-   **`Tone::new`**, **`Operator::new`**, **`Parameter::new`** (src/models.rs):
    -   役割: アプリケーションで使用されるデータ構造の構築。
    -   機能: YM2151の音色全体、個々のオペレータ（キャリア/モジュレータ）、および各オペレータのパラメータ（AR, DRなど）といったデータモデルのインスタンスを生成します。
-   **`read_register`**, **`write_register`** (src/register.rs):
    -   役割: YM2151の仮想レジスタ操作。
    -   機能: エディタ内部で管理しているYM2151の仮想レジスタから特定のアドレスの値を読み取ったり、指定したアドレスに新しい値を書き込んだりする処理を提供します。
-   **`draw_ui`** (src/ui.rs):
    -   役割: ユーザーインターフェースの描画。
    -   機能: `ratatui`ライブラリを使用して、YM2151のパラメータ一覧、現在の値、ステータスメッセージなどをターミナル画面に描画し、ユーザーが視覚的に情報を確認できるようにします。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2025-11-30 07:08:16 JST
