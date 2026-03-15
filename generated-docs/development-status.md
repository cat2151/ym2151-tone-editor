Last updated: 2026-03-16

# Development Status

## 現在のIssues
- [Issue #213](../issue-notes/213.md) は、`core/src/lib.rs` が500行を超えているため、リファクタリングによるコード品質向上が推奨されています。
- [Issue #212](../issue-notes/212.md) では、WASM版のランダム音色機能が `bluesky-text-to-audio` リポジトリで利用できない状態であることが報告されています。
- [Issue #177](../issue-notes/177.md) は、sixelを利用して音色波形を描画し、変更後5秒でバックグラウンドでwavを生成・表示することでUXを検証することを目的としています。

## 次の一手候補
1. `core/src/lib.rs` のモジュール分割によるリファクタリング ([Issue #213](../issue-notes/213.md))
   - 最初の小さな一歩: `core/src/lib.rs` の内容を分析し、論理的に関連性の高い機能群（例: `SimpleRng`、`midi_to_kc_kf`、`editor_rows_to_registers`）を個別のモジュールに分割するプランを立てる。
   - Agent実行プロンプト:
     ```
     対象ファイル: `core/src/lib.rs`

     実行内容: `core/src/lib.rs`の内容を分析し、以下の機能群をそれぞれ別のモジュールとして切り出す場合の抽象的な分割方針と、各モジュールに移動する関数・定数をリストアップしてください。
     1) `SimpleRng` 関連の乱数生成ロジック
     2) `midi_to_kc_kf` 関連のMIDI変換ロジック
     3) `editor_rows_to_registers` および `push_reg_pair` 関連のレジスタエンコーディングロジック
     4) `ToneData` などの共通データ構造や定数

     確認事項: 各機能が他の機能に依存していないか、依存している場合はどのようにモジュール間で連携させるかを考慮してください。分割後のクレートの公開APIに影響がないよう留意してください。

     期待する出力: 各機能群の分割案と、それぞれに含める関数・定数のリストをMarkdown形式で出力してください。
     ```

2. WASM版ランダム音色機能の不具合原因調査 ([Issue #212](../issue-notes/212.md))
   - 最初の小さな一歩: `wasm/src/lib.rs` と `ym2151-tone-params` クレートの `generate_random_tone_registers` 関数が `bluesky-text-to-audio` リポジトリでどのように利用されているかを調査し、問題発生箇所の特定に着手する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `wasm/src/lib.rs`, `core/src/lib.rs`, および `bluesky-text-to-audio` リポジトリの関連する利用箇所（仮定として `bluesky-text-to-audio` の `package.json` と `src` ディレクトリ内で `ym2151-wasm` をimportしている箇所）。

     実行内容: `ym2151-wasm` クレートの `generate_random_tone_registers` が `bluesky-text-to-audio` プロジェクトで利用できない原因を特定するため、以下の観点から調査し、考えられる原因と調査方針を提案してください。
     1) `wasm/src/lib.rs` の変更点と、それに伴う `ym2151-tone-params` の変更が `bluesky-text-to-audio` でどのように影響しているか。
     2) `bluesky-text-to-audio` 側での `ym2151-wasm` の呼び出し方法に変更があったか。
     3) ビルド環境や依存関係のバージョンミスマッチの可能性。

     確認事項: `bluesky-text-to-audio` リポジトリのコードは直接参照できないため、一般的な問題パターンと、提供されたファイル情報（`wasm/src/lib.rs`, `core/src/lib.rs` の内容）に基づいて推論してください。

     期待する出力: 考えられる原因を複数リストアップし、それぞれの原因を検証するための具体的な調査ステップをMarkdown形式で出力してください。
     ```

3. Sixelによる音色波形描画の技術検証とPoC作成 ([Issue #177](../issue-notes/177.md))
   - 最初の小さな一歩: RustアプリケーションでSixel出力を生成し、対応するターミナルに表示するための基本的な技術調査と、概念実証 (PoC) のコードスニペットを作成する。
   - Agent実行プロンプト:
     ```
     対象ファイル: なし（新規コードの検討）

     実行内容: RustでSixelグラフィックを生成し、対応するターミナルで表示するための基本的なアプローチを調査し、以下の点について記述してください。
     1) Sixel形式でイメージデータをエンコードするためのRustクレートやライブラリの有無。
     2) Sixelデータを標準出力または特定のファイルに書き出す方法。
     3) 簡単な波形データ（例: 正弦波）をSixelとして描画するための概念的なコードスニペット。

     確認事項: Sixelのサポート状況はターミナルエミュレータに依存するため、一般的な利用可能性について言及してください。複雑なグラフィック描画ではなく、あくまで「基本的な波形表示」に焦点を当ててください。

     期待する出力: Sixel描画の技術調査結果と、波形表示の概念的なRustコードスニペットをMarkdown形式で出力してください。

---
Generated at: 2026-03-16 07:11:39 JST
