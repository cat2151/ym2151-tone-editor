Last updated: 2026-03-12

# Development Status

## 現在のIssues
- [Issue #177](../issue-notes/177.md) と [Issue #176](../issue-notes/176.md) は、sixelを用いて音色波形や各オペレータのエンベロープを描画し、ユーザーエクスペリエンスを検証することを目標としています。
- [Issue #174](../issue-notes/174.md) では、Issue 149の結果に基づき、ユーザーがローカルに音色テンプレートJSONファイルを生成する機能の実装が計画されています。
- [Issue #167](../issue-notes/167.md) ではプレビュー時のプチノイズ問題の解決が検討されていますが、現在は別リポジトリでのJSON編集GUIツール開発を待っている状態です。

## 次の一手候補
1. [Issue #177](../issue-notes/177.md) sixelで音色波形を描画するために`cat-play-mml`でWAVを生成する
   - 最初の小さな一歩: Rustアプリケーション内で`cat-play-mml`コマンドを外部プロセスとして実行し、指定されたMMLを元にWAVファイルを生成する基本的な関数を`src/audio.rs`に実装する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `src/audio.rs`, `src/config.rs`

     実行内容: `cat-play-mml`コマンドを外部プロセスとして呼び出し、MML文字列を元にWAVファイルを生成し、指定されたパスに保存するRust関数 `generate_wav_from_mml(mml: &str, output_path: &Path) -> Result<(), Box<dyn std::error::Error>>` を `src/audio.rs` に実装してください。`cat-play-mml`実行可能ファイルへのパスは`src/config.rs`で設定可能とします。

     確認事項: `cat-play-mml`コマンドがシステムパスにあるか、または設定ファイルで指定されたパスに存在するか。コマンド実行失敗、ファイル保存失敗などのエラーハンドリングが適切に行われているか。

     期待する出力: `src/audio.rs` に上記の関数が実装され、必要であれば`src/config.rs`に`cat_play_mml_path`設定が追加される。
     ```

2. [Issue #176](../issue-notes/176.md) sixelで各OPごとのエンベロープを描画するために必要なパラメータを抽出する
   - 最初の小さな一歩: 現在編集中の音色データ (`src/models.rs`の`Tone`構造体) から、選択されたオペレータのADSRパラメータ（AR, DR, SR, RR, TL, KS, AM, DT1, DT2, MUL）を抽出し、シンプルな文字列として表現する（例: "AR:15 DR:10 SR:5 RR:8 TL:30..."）。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/models.rs`, `src/app/mod.rs`

     実行内容: `src/models.rs`内の`Tone`構造体に、特定のオペレータ（`operator_index: usize`）のADSR関連パラメータを分かりやすい文字列として返すメソッド `get_operator_envelope_params_string(&self, op_idx: usize) -> String` を追加してください。この文字列はsixel描画の前段階としてパラメータを可視化するためのものです。`src/app/mod.rs`内でのこのメソッドの呼び出し例をコメントとして追加してください。

     確認事項: `Tone`構造体内のオペレータデータへのアクセス方法が適切であるか。パラメータが適切な形式で文字列に変換されているか。

     期待する出力: `src/models.rs`に上記メソッドが追加され、その利用例が`src/app/mod.rs`の関連する描画ロジックにコメントアウト形式で追加される。
     ```

3. [Issue #174](../issue-notes/174.md) ユーザーがローカルに音色template jsonファイルを生成する機能の第一歩として、音色データのJSON保存機能を実装する
   - 最初の小さな一歩: 現在編集中の音色データを表現するRustの構造体 (`src/models.rs`の`Tone`構造体) をJSON形式でシリアライズし、指定されたファイルパスに保存する関数を`src/file_ops.rs`に実装する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/file_ops.rs`, `src/models.rs`

     実行内容: `src/file_ops.rs`に、`src/models.rs`で定義されている`Tone`構造体のインスタンスを受け取り、それをJSON形式で指定されたファイルパスに保存するRust関数 `save_tone_to_json(tone: &Tone, path: &Path) -> Result<(), Box<dyn std::error::Error>>` を実装してください。

     確認事項: `Tone`構造体が`serde::Serialize`をderiveしていること。ファイル書き込み時のエラーハンドリング（例: ファイル作成失敗、書き込み失敗など）。ファイルパスの解決方法。

     期待する出力: `src/file_ops.rs` に`save_tone_to_json`関数が実装され、必要であれば`src/models.rs`に`#[derive(serde::Serialize)]`が追加される。
     ```

---
Generated at: 2026-03-12 07:12:21 JST
