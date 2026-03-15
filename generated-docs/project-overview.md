Last updated: 2026-03-16

# Project Overview

## プロジェクト概要
- YM2151 FM音源の音色を編集するためのTUI（ターミナルユーザーインターフェース）アプリケーションです。
- Windows環境で動作し、直感的なキーボードやマウス操作でリアルタイムに音色パラメータを調整できます。
- 編集した音色は自動で保存・ロードされ、効率的なサウンドデザインとプレビュー機能を提供します。

## 技術スタック
- フロントエンド: **ratatui** (Rust製のターミナルUIフレームワークで、リッチなテキストベースのユーザーインターフェース構築に使用されます), **crossterm** (クロスプラットフォームなターミナル操作ライブラリで、キー入力やカーソル制御など、ターミナルとの低レベルなインタラクションを提供します)
- 音楽・オーディオ: **YM2151** (ヤマハFM音源チップのパラメータを扱います), **ym2151-log-play-server** (リアルタイム音声フィードバックを提供するためのサーバーライブラリで、編集中の音色を即座に再生します), **MML** / **SMF関連ライブラリ** (音色プレビュー時に使用される音楽記述言語および標準MIDIファイルフォーマット関連の処理を行います)
- 開発ツール: **Rust** (安全性、パフォーマンス、並行性に優れたシステムプログラミング言語で、本プロジェクトの主要言語です), **cargo** (Rustの公式ビルドシステムおよびパッケージマネージャーで、依存関係の管理、ビルド、テスト、ドキュメント生成などを担当します)
- テスト: Rust標準のテストフレームワーク (Rustの`cargo test`コマンドで実行される組み込みのテスト機能を使用して、アプリケーションの各コンポーネントの動作を検証します)
- ビルドツール: **cargo** (Rustプロジェクトのコンパイル、バイナリ生成など、ビルドプロセス全般を管理します)
- 言語機能: **Rust** (モダンな言語機能、強力な型システム、所有権モデルにより、メモリ安全性を保証しながら高性能なアプリケーションを開発します)
- 自動化・CI/CD: (情報が提供されていないため、特定のツールは記載しません)
- 開発標準: (情報が提供されていないため、特定のツールは記載しません)

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
  📖 176.md
  📖 177.md
  📖 210.md
  📖 212.md
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
- **.gitignore**: Gitのバージョン管理から除外すべきファイルやディレクトリのパターンを定義します。
- **Cargo.lock**: プロジェクトの依存関係ツリーと、使用されている各クレートの正確なバージョンを記録します。これにより、ビルドの一貫性が保証されます。
- **Cargo.toml**: Rustプロジェクトのメタデータ（名前、バージョン、作者など）と、ビルド設定、外部依存関係を定義するマニフェストファイルです。
- **LICENSE**: プロジェクトの配布条件と権利を定めるライセンス情報（例: MIT License）が含まれています。
- **README.ja.md**, **README.md**: プロジェクトの日本語版と英語版の概要、特徴、インストール方法、使用方法などを記述した主要なドキュメントファイルです。
- **README_generate_gm_templates.md**: General MIDIテンプレートの生成に関する詳細な手順や情報を提供します。
- **_config.yml**: GitHub Pagesのサイト設定を定義するファイルで、デモページなどの公開設定に使用されます。
- **build.rs**: ビルドプロセス中に実行されるカスタムビルドスクリプトです。特定のOS固有のリンク設定や、ビルド時の情報生成などに使用されることがあります。
- **core/**: アプリケーションの主要なビジネスロジックや共通機能を含むライブラリクレートです。
    - **core/Cargo.toml**: `core`クレート自体のメタデータと依存関係を定義します。
    - **core/src/lib.rs**: `core`クレートのメインソースファイルで、他のモジュールから利用される汎用的な関数やデータ構造が定義されます。
- **demo-library/**: ランダム音色関数ライブラリのデモンストレーション関連ファイルを格納するディレクトリです。
    - **demo-library/index.html**: ウェブブラウザでランダム音色関数ライブラリの動作を確認できるデモページです。
- **docs/**: プロジェクトに関する追加のドキュメントを格納するディレクトリです。
    - **docs/KEYBINDS.ja.md**: アプリケーションのキーバインド（キー操作）に関する日本語の詳細説明が含まれています。
- **generate_gm_templates.rs**: General MIDI標準に準拠した音色テンプレートを生成するためのRustスクリプトです。
- **generated-docs/**: Doxygenやrustdocなどのツールによって自動生成されたドキュメントを格納する場所です。
- **googled947dc864c270e07.html**: Googleサイト認証用のファイルで、ウェブサイトの所有権確認に使用されます。
- **issue-notes/**: 開発中の課題や検討事項に関するメモがファイルごとにまとめられています。（来訪者向けのため、詳細な内容は割愛します）
- **src/**: アプリケーションの主要なソースコードが格納されているディレクトリです。
    - **src/app/**: アプリケーションのコアロジックを構成するモジュール群です。
        - **src/app/mod.rs**: `app`モジュールのルートファイルで、他の`app`内のサブモジュールを管理します。
        - **src/app/shortcuts.rs**: キーバインドとそれに対応するアプリケーションの動作を定義および処理するロジックが含まれます。
    - **src/app_init.rs**: アプリケーションの起動時における初期設定やリソースの準備を行うロジックが含まれます。
    - **src/audio.rs**: リアルタイム音声フィードバックのための`ym2151-log-play-server`との通信やオーディオデータ処理に関するロジックです。
    - **src/config.rs**: アプリケーションの設定ファイル（例: `ym2151-tone-editor.toml`）の読み込み、解析、管理を行うロジックです。
    - **src/event_loop.rs**: ターミナルからのイベント（キー入力、マウスイベント、ウィンドウリサイズなど）を継続的に監視し、アプリケーションの状態を更新するメインループを管理します。
    - **src/favorites.rs**: ユーザーがお気に入りとして登録した音色データの管理（追加、削除、ロードなど）を行うロジックです。
    - **src/file_ops.rs**: 音色データのファイルからのロード、ファイルへの保存、自動セーブなどの一般的なファイル操作を処理するロジックです。
    - **src/history.rs**: ユーザーが行った音色パラメータ変更の履歴を記録し、元に戻す（Undo）ややり直し（Redo）操作を可能にするためのロジックです。
    - **src/history_selector.rs**: 編集履歴の中から特定の時点の状態を選択し、適用するためのユーザーインターフェースロジックです。
    - **src/logging.rs**: アプリケーションの実行状況やエラー情報を記録するためのロギング機能を提供します。
    - **src/main.rs**: アプリケーションのエントリポイント（開始点）となるファイルです。`cargo run`実行時に最初に呼び出されます。
    - **src/midi_conversion.rs**: MIDI関連のデータ形式（MML、SMFなど）とYM2151音色データ間の変換を行うロジックです。
    - **src/models.rs**: YM2151音色パラメータ、アプリケーションの状態、UI要素など、アプリケーション内で使用される主要なデータ構造を定義します。
    - **src/random_tone.rs**: YM2151の各パラメータに対してランダムな値を生成し、新しい音色を自動生成する機能を提供します。
    - **src/register.rs**: YM2151音源の各レジスタ（Detune, Multiplier, Total Levelなど）の値を表現し、操作するための低レベルなロジックです。
    - **src/register_list.rs**: ユーザーインターフェース上でYM2151のレジスタリストを表示し、操作するためのロジックです。
    - **src/tests/**: アプリケーションのユニットテストコードを格納するディレクトリです。
        - **app_adsr_mul_sm_tests.rs** など: 特定の機能（ADSRエンベロープ、Multiplier、Slot Maskなど）に関するテストケースを定義します。
        - **mod.rs**: `tests`モジュールのルートファイルです。
    - **src/ui/**: ユーザーインターフェースの描画と操作に関連するロジックを格納するモジュールです。
        - **src/ui/helpers.rs**: UIの描画やレイアウトを補助するユーティリティ関数群を提供します。
        - **src/ui/mod.rs**: `ui`モジュールのルートファイルで、UIコンポーネントの全体的な構造と管理を行います。
    - **src/updater.rs**: アプリケーションの新しいバージョンへの更新を管理するロジックです。
    - **src/variation_selector.rs**: 複数の音色バリエーションの中から選択するためのユーザーインターフェースロジックです。
- **tones/**: YM2151音色データファイルを格納するためのディレクトリです。
    - **tones/general_midi/**: General MIDI標準に準拠した音色テンプレートを格納します。
        - **000_AcousticGrand.json**: General MIDIの000番「Acoustic Grand Piano」に相当するYM2151音色データ（JSON形式）です。
        - **tone_names.json**: 音色の名前リストを格納するJSONファイルです。
- **wasm/**: WebAssembly (Wasm) 関連のコードを格納するクレートです。
    - **wasm/Cargo.toml**: `wasm`クレートのメタデータと依存関係を定義します。
    - **wasm/src/lib.rs**: WebAssemblyにコンパイルされるコードのメインソースファイルです。
- **ym2151-tone-editor.toml.example**: アプリケーションの設定ファイル（TOML形式）の例で、ユーザーが設定を変更する際の参考に供されます。

## 関数詳細説明
提供された情報からは、具体的な関数名、引数、戻り値、詳細な機能を抽出することができませんでした。しかし、プロジェクトのファイル構成から、以下のような主要な機能を提供する関数群が存在すると推測されます。

*   **UI描画・操作関数群**: `src/ui/`, `src/app/` モジュールで、ターミナルUIの描画、キー入力やマウス操作のイベント処理、カーソル移動、値の増減などを担当します。
*   **YM2151レジスタ操作関数群**: `src/register.rs` や `src/models.rs` で、YM2151音源の各パラメータ（DT, MUL, TLなど）の読み書き、範囲チェック、データ構造の管理を行います。
*   **オーディオフィードバック関数群**: `src/audio.rs` で、`ym2151-log-play-server` と連携し、編集中の音色をリアルタイムで再生する処理を制御します。
*   **ファイルI/O関数群**: `src/file_ops.rs` で、音色データの自動保存・ロード、JSON形式での音色データの読み書きなどを行います。
*   **ランダム音色生成関数群**: `src/random_tone.rs` で、YM2151パラメータの有効範囲内でランダムな値を生成し、新しい音色を作成します。
*   **設定・履歴管理関数群**: `src/config.rs`, `src/history.rs`, `src/favorites.rs` で、アプリケーションの設定、編集履歴、お気に入り音色の管理を行います。
*   **テスト関数群**: `src/tests/` ディレクトリ内の各ファイルで、アプリケーションの各機能が正しく動作するかを検証するためのテスト関数群が定義されています。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした
```

---
Generated at: 2026-03-16 07:11:45 JST
