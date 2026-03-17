Last updated: 2026-03-18

# Development Status

## 現在のIssues
- `src/event_loop.rs`が500行を超えており、リファクタリングの優先度が高い（[Issue #226](../issue-notes/226.md)）。
- Windows環境での設定ファイル保存先が`AppData\Roaming`となっているため、`AppData\Local`への変更が求められています（[Issue #224](../issue-notes/224.md), [Issue #223](../issue-notes/223.md)）。
- UI/UXの改善として、ヘルプ表示の動的生成（[Issue #219](../issue-notes/219.md)）やエンベロープ折れ線グラフの視認性向上の課題が残っています（[Issue #218](../issue-notes/218.md)）。

## 次の一手候補
1. `src/event_loop.rs` のマウスイベントハンドリングロジックを抽出する [Issue #226](../issue-notes/226.md)
   - 最初の小さな一歩: `src/event_loop.rs` 内で、マウスイベントの処理ブロックを特定し、関連するロジック（ペンタトニック鍵盤ホバー座標の更新、値の更新など）を新しい関数として`src/ui/helpers.rs`に抽出し、`event_loop.rs`からその関数を呼び出すように変更します。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/event_loop.rs, src/ui/helpers.rs, src/app/mod.rs

     実行内容: src/event_loop.rs 内の `Event::Mouse` に関する処理ブロックを特定し、そのロジックを `src/ui/helpers.rs` 内の新しい関数 `handle_mouse_event` として抽出してください。この関数は `App` の可変参照とマウスイベントデータを受け取り、`App`の状態を適切に更新できるように設計してください。抽出後、`src/event_loop.rs`から新しい関数を呼び出すように変更してください。

     確認事項:
     - 抽出後もマウスイベント処理（ホバー表示、値の更新、スクロールによる値変更）が正しく機能すること。
     - `src/event_loop.rs` のイベントハンドリングロジックが簡素化され、行数が減少すること。
     - 既存の他のイベント処理（キーイベントなど）に影響がないこと。
     - 必要に応じて `src/app/mod.rs` に新しいメソッドを追加または修正してください。

     期待する出力:
     - 修正された `src/event_loop.rs` と `src/ui/helpers.rs` のコード。
     - 変更内容の概要をMarkdown形式で記述してください。
     ```

2. Windowsでの設定・音色ファイル保存パスを`AppData\Local`に変更する [Issue #224](../issue-notes/224.md) [Issue #223](../issue-notes/223.md)
   - 最初の小さな一歩: `src/file_ops.rs` で、アプリケーションのデータディレクトリパスを解決している箇所を特定し、`directories-rs`クレートの`ProjectDirs::config_dir()`や`ProjectDirs::data_local_dir()`メソッドを使用して、パスが`AppData\Local`を指すように修正します。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/file_ops.rs, src/config.rs, Cargo.toml

     実行内容:
     1. `src/file_ops.rs` 内で、Windows環境におけるアプリケーション設定ファイルや音色ファイルのパスを生成しているロジックを特定してください。
     2. これを `C:\Users\<your name>\AppData\Local\ym2151-tone-editor` を指すように修正してください。修正には `directories-rs` クレートの `ProjectDirs::from` メソッドを利用し、設定ファイルとデータファイルで適切なローカルパスを解決するようにしてください。
     3. `src/config.rs` でパス解決に関連するロジックがあれば、同様に修正してください。
     4. `Cargo.toml` に `directories-rs` クレートがまだ追加されていない場合は、追加してください。

     確認事項:
     - Windows以外のOS（Linux, macOSなど）でのファイルパス解決に影響がないこと。
     - アプリケーションが設定ファイルや音色ファイルを正しく読み書きできること。
     - 既存のテスト（特に`src/tests/file_ops_tests.rs`）が引き続きパスすること、または必要に応じてテストを更新すること。

     期待する出力:
     - 修正された `src/file_ops.rs`、`src/config.rs`、および `Cargo.toml` のコード。
     - 変更内容の概要をMarkdown形式で記述してください。
     ```

3. ヘルプ画面を現在のキーバインド設定に基づいて動的に生成する [Issue #219](../issue-notes/219.md)
   - 最初の小さな一歩: `src/ui/help.rs` 内の `draw_help_dialog` 関数で、ハードコードされたキーバインド説明を削除し、`src/config.rs` から読み込んだ `Config` オブジェクトを引数として受け取り、その内容を基にヘルプテキストを生成するように変更します。また、ヘルプダイアログから`hjkl`と`wasd`に関する記述を削除し、代わりに矢印キーに関する記述を含めます。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/ui/help.rs, src/ui/mod.rs, src/config.rs, src/app/mod.rs

     実行内容:
     1. `src/ui/help.rs` の `draw_help_dialog` 関数を修正し、`src/app/mod.rs` の `App` 構造体を通じて `src/config.rs` から取得したキーバインド設定 (`Config` オブジェクト) を受け取り、その情報に基づいてヘルプダイアログの内容を動的に生成するように変更してください。
     2. ヘルプダイアログ内の `hjkl` および `wasd` キーに関する古い記述を削除し、現在の矢印キー (`Left`, `Right`, `Up`, `Down`) を中心とした移動キーバインドの説明に置き換えてください。
     3. `src/ui/mod.rs` 内の `ui` 関数を修正し、常に画面の左下隅に「?:help」というテキストが表示されるようにしてください。

     確認事項:
     - ヘルプダイアログの内容が `src/config.rs` の最新のキーバインド設定と一致すること。
     - ヘルプダイアログが表示/非表示切り替え時に正しくレンダリングされ、レイアウトが崩れないこと。
     - 画面左下の「?:help」表示が他のUI要素と重ならないこと。
     - 既存のキーバインド動作に影響がないこと。

     期待する出力:
     - 修正された `src/ui/help.rs`、`src/ui/mod.rs`、および `src/app/mod.rs` のコード。
     - 変更内容の概要をMarkdown形式で記述してください。
     ```

---
Generated at: 2026-03-18 07:16:07 JST
