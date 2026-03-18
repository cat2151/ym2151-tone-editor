Last updated: 2026-03-19

# Development Status

## 現在のIssues
- カーソル移動キーの廃止に伴い、ヘルプ表示のキーバインドを`config`から自動生成し、hjkl/wasdを削除して矢印キーを表示する必要があることが課題となっています。([Issue #231](../issue-notes/231.md), [Issue #219](../issue-notes/219.md))
- また、ヘルプのヒント「?:help」を常に画面左下に表示する機能も求められています。([Issue #231](../issue-notes/231.md), [Issue #219](../issue-notes/219.md))
- その他の主要な課題として、音色テンプレートJSONのローカル生成機能の検討([Issue #174](../issue-notes/174.md))や、プレビュー時のノイズ問題が別リポジトリの進捗待ちとなっています。([Issue #167](../issue-notes/167.md))

## 次の一手候補
1. ヘルプ表示をキーバインド設定から自動生成し、UIを更新する [Issue #231](../issue-notes/231.md), [Issue #219](../issue-notes/219.md)
   - 最初の小さな一歩: `src/ui/help.rs` の `draw_keybind_hints` 関数が `Config` を参照できるように、`src/ui/mod.rs` の `ui` 関数と `src/event_loop.rs` の `ui` 呼び出しに `Config` 引数を追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/ui/help.rs`, `src/ui/mod.rs`, `src/event_loop.rs`

     実行内容: `src/ui/help.rs` の `draw_keybind_hints` 関数がアプリケーションのキーバインド設定 `Config` を参照してヘルプテキストを生成できるように、以下の変更を提案してください。
     1. `src/ui/mod.rs` の `ui` 関数のシグネチャに `config: &Config` 引数を追加する。
     2. `src/ui/help.rs` の `draw_keybind_hints` 関数のシグネチャに `config: &Config` 引数を追加する。
     3. `src/event_loop.rs` 内の `crate::ui::ui(f, app);` の呼び出し箇所を `crate::ui::ui(f, app, config);` に変更する。
     4. `src/ui/mod.rs` 内の `help::draw_keybind_hints(f, app, inner);` の呼び出し箇所を `help::draw_keybind_hints(f, app, config, inner);` に変更する。

     確認事項: シグネチャ変更に伴う全ての呼び出し箇所が適切に更新され、既存のUI描画ロジックに影響がないことを確認してください。

     期待する出力: 上記の変更を反映したコード差分と、変更内容を説明するmarkdown形式のテキスト。
     ```

2. 音色テンプレートJSONのローカル生成機能の検討と既存ファイルの分析 [Issue #174](../issue-notes/174.md)
   - 最初の小さな一歩: `generate_gm_templates.rs` の現在の実装と `src/file_ops.rs` を分析し、`generate_gm_templates.rs` の機能をアプリケーションから呼び出し、生成されたJSONファイルを`file_ops`で管理されるディレクトリに保存するための統合プロセスを検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `generate_gm_templates.rs`, `src/file_ops.rs`

     実行内容: `generate_gm_templates.rs` の現在の機能と `src/file_ops.rs` のファイル操作ユーティリティを分析してください。その上で、ユーザーがアプリケーション内でGMテンプレートJSONファイルをローカルに生成し、それを `file_ops` モジュールが管理する適切なディレクトリ（例: `tones/general_midi/`）に保存するための統合案をmarkdown形式で出力してください。具体的には、以下の点を明確にしてください。
     1. `generate_gm_templates.rs` をアプリケーションから呼び出すためのエントリポイントや必要な引数。
     2. 生成されたJSONファイルを `file_ops::gm_file_path()` など既存のファイルI/O関数を使って保存する方法。
     3. アプリケーションの既存のワークフロー（例: メニュー項目や新しいキーバインド）にこの機能を統合するためのUI/ロジックの概要。

     確認事項: 提案される統合案が既存のファイル構造や `file_ops` の設計と整合性があることを確認してください。ハルシネーションを避け、具体的なファイルパスや関数呼び出しに焦点を当ててください。

     期待する出力: 統合案をmarkdown形式で出力し、具体的なコードスニペットや擬似コードを含むこと。
     ```

3. 新しいファイルI/Oパスでの保存・ロードに関するテストの拡充 [Issue #155](../issue-notes/155.md)
   - 最初の小さな一歩: `src/tests/file_ops_tests.rs` を分析し、最近 `src/file_ops.rs` に加えられた `AppData\Local` への移行およびGMフォーマット対応に関するテストケースが存在するかを確認し、不足している場合はそれらを追加するためのテスト計画を立案する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/file_ops.rs`, `src/tests/file_ops_tests.rs`

     実行内容: `src/file_ops.rs` と `src/tests/file_ops_tests.rs` のコードを分析し、最近 `src/file_ops.rs` に加えられた以下の変更をカバーするための新しいテストケースを `src/tests/file_ops_tests.rs` に追加する変更案を記述してください。
     1. `AppData\Local` ディレクトリへの設定ファイル（`ym2151-tone-editor.json` や履歴ファイル）の保存と読み込み。
     2. 新しいGMフォーマットでの音色ファイル（`tones/general_midi/` 内の `.json`）の保存と読み込み、特に `save_to_gm_file` および `append_to_gm_file` の機能検証。

     確認事項: 既存のテストを重複させず、新しいファイルパスとフォーマットのロジックが確実に検証されることを確認してください。テストは隔離されており、システム環境に副作用を与えないように配慮してください。

     期待する出力: `src/tests/file_ops_tests.rs` に追加すべき新しいテスト関数のコードスニペットと、それぞれのテストが検証する内容を説明するmarkdown形式のテキスト。

---
Generated at: 2026-03-19 07:15:15 JST
