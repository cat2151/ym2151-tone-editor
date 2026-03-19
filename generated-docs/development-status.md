Last updated: 2026-03-20

# Development Status

## 現在のIssues
- [Issue #174](../issue-notes/174.md) は、issue 149で得られた知見を活用し、ユーザーがローカルで音色テンプレートJSONファイルを生成する機能の実装を目指しています。
- [Issue #167](../issue-notes/167.md) は、音色プレビュー時のプチノイズ発生問題で、その原因究明のため軽量GUIでのJSON編集ツールの開発を別リポジトリで進行中であり、本Issueは現在待ち状態です。
- [Issue #155](../issue-notes/155.md) は、プロジェクトの品質向上と実用性確認のための、開発者自身によるドッグフーディングを促すタスクです。

## 次の一手候補
1. [Issue #174](../issue-notes/174.md) - 音色テンプレート生成機能のデータ構造分析
   - 最初の小さな一歩: 既存の音色データがどのように表現されているか、`src/models.rs` や `tones/general_midi/*.json` を調査し、テンプレート化に必要な要素を特定します。
   - Agent実行プロンプ:
     ```
     対象ファイル: src/models.rs, tones/general_midi/tone_names.json, tones/general_midi/000_AcousticGrand.json

     実行内容: 既存の音色データ構造とGeneral MIDI JSONファイルの構造を分析し、ユーザーがローカルに生成する音色テンプレートJSONファイルに必要なフィールド（オペレータ設定、エンベロープ、LFO等）と、それらのデフォルト値や許容範囲を定義してください。

     確認事項: `src/models.rs` で定義されているYM2151のレジスタ構造と、既存のGeneral MIDI JSONファイルにおけるパラメータ表現の整合性を確認してください。

     期待する出力: 提案する音色テンプレートJSONのスキーマ定義（フィールド名、型、説明、デフォルト値があればそれも）をMarkdown形式で出力してください。
     ```

2. [Issue #174](../issue-notes/174.md) - 音色テンプレート生成処理のロジック検討
   - 最初の小さな一歩: 既存の音色データをJSON形式で出力するための基本的なRust関数スケルトンを検討します。`src/file_ops.rs` を参考に、ファイル書き込み処理の既存パターンを理解します。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/file_ops.rs, src/models.rs

     実行内容: `src/file_ops.rs` に新しい関数 `generate_tone_template_json` を追加することを想定し、現在のアプリケーションでロードされている音色データ（`src/models.rs` の構造体を参照）をJSON形式で指定されたパスに保存するためのRustコードのスケルトンを生成してください。既存のファイル保存処理のパターンに沿ってください。

     確認事項: `src/file_ops.rs` の既存のファイル書き込み処理（例: `save_favorites_to_file`）との整合性、および`src/models.rs` の`Tone`構造体からJSONへのシリアライズ方法を確認してください。

     期待する出力: `src/file_ops.rs` に追加する関数のRustコードスニペットと、その関数の呼び出し例をMarkdown形式で出力してください。
     ```

3. [Issue #174](../issue-notes/174.md) - コマンドラインからのテンプレート生成の検討
   - 最初の小さな一歩: アプリケーションに新しいコマンドライン引数を追加し、テンプレート生成をトリガーするエントリポイントを検討します。`src/main.rs` や `src/app_init.rs` の引数処理部分を調査します。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/main.rs, src/app_init.rs, src/file_ops.rs

     実行内容: アプリケーションが起動時に新しいコマンドライン引数 `--generate-template <output_path>` を受け取るように変更し、`output_path` に音色テンプレートJSONファイルを生成する処理（候補2で作成する関数を呼び出す想定）を追加するための変更点を分析してください。

     確認事項: 既存のコマンドライン引数処理との競合がないか、また、ファイル生成処理を呼び出す際に必要なアプリケーションの状態（例: デフォルトの音色データ）が利用可能であるかを確認してください。

     期待する出力: `src/main.rs` および `src/app_init.rs` に加えるべき変更の概要と、引数解析およびテンプレート生成関数呼び出しの簡単なRustコードスニペットをMarkdown形式で出力してください。

---
Generated at: 2026-03-20 07:11:14 JST
