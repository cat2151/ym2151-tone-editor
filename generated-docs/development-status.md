Last updated: 2026-03-17

# Development Status

## 現在のIssues
- 現在、[Issue #221](../issue-notes/221.md)で`src/ui/mod.rs`の500行超過リファクタリングが推奨されており、UIコード品質の向上が求められています。
- [Issue #220](../issue-notes/220.md)と[Issue #219](../issue-notes/219.md)ではTUIのキーバインドとヘルプ表示のUX改善が計画されており、操作性の一貫性が目指されています。
- [Issue #218](../issue-notes/218.md)と[Issue #177](../issue-notes/177.md)はsixelを用いたエンベロープ/音色波形グラフの表示改善とUX検証を進めています。

## 次の一手候補
1.  `src/ui/mod.rs` のリファクタリング ([Issue #221](../issue-notes/221.md))
    -   最初の小さな一歩: `src/ui/mod.rs` 内の`draw_virtual_pentatonic_keyboard_at_y`、`draw_keybind_hints`、`draw_help_dialog`関数を`src/ui/helpers.rs`に移動する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: src/ui/mod.rs, src/ui/helpers.rs

        実行内容: `src/ui/mod.rs` 内の`draw_virtual_pentatonic_keyboard_at_y`、`draw_keybind_hints`、`draw_help_dialog`関数を`src/ui/helpers.rs`に移動し、`src/ui/mod.rs`から呼び出すように修正してください。移動後、`src/ui/mod.rs`から`src/ui/helpers.rs`で定義された関数を`use`ステートメントで適切にインポートしてください。

        確認事項:
        - すべての関数移動後、`src/ui/mod.rs`がコンパイルエラーなく、以前と同じようにUIを描画できることを確認してください。
        - `draw_keybind_hints`と`draw_help_dialog`は`app.show_help`に依存するので、ヘルプ表示のトグル機能が引き続き正しく動作することを確認してください。
        - `draw_virtual_pentatonic_keyboard_at_y`はWindowsでのみ`audio::play_tone`を呼び出すため、`#[cfg(windows)]`属性が適切に引き継がれているか確認してください。
        - `src/app/mod.rs`内で定義されている`ROW_CH`等の定数が適切に参照されていることを確認してください。

        期待する出力: `src/ui/mod.rs`と`src/ui/helpers.rs`の修正されたコード。および、修正箇所の説明をmarkdown形式で出力。
        ```

2.  キーバインドとヘルプ表示のUX改善 ([Issue #220](../issue-notes/220.md), [Issue #219](../issue-notes/219.md))
    -   最初の小さな一歩: `src/app/mod.rs`から`q`と`e`キーの`decrease_value`/`increase_value`へのマッピングを削除し、`q`キーでアプリケーションが終了するように`src/app/shortcuts.rs`を更新する。
    -   Agent実行プロンプト:
        ```
        対象ファイル: src/app/mod.rs, src/app/shortcuts.rs, src/ui/mod.rs

        実行内容:
        1. `src/app/mod.rs`から`q`と`e`キーの`decrease_value`/`increase_value`へのマッピングを削除してください。
        2. `q`キーがアプリケーションを終了するように、`src/app/shortcuts.rs`または関連する入力処理ロジックを更新してください（現在の`ESC`キーの終了ロジックを参考にしてください）。
        3. `src/ui/mod.rs`の`draw_keybind_hints`および`draw_help_dialog`から、`q/e:dec/inc`および`hjkl/wasd:move`の表記を削除してください。
        4. `src/ui/mod.rs`でヘルプ画面の表示が`keybinds`設定から生成されるように、`src/app/shortcuts.rs`の内容を読み込み、動的にヘルプテキストを生成する仕組みを導入してください。`src/app/shortcuts.rs`に新しい関数`get_keybind_help_text()`を実装し、それが`src/ui/mod.rs`から呼び出されるようにしてください。

        確認事項:
        - `q`キーを押下するとアプリケーションが終了すること。
        - `e`キーがどの機能にもマッピングされていないこと（または適切に再マッピングされていること）。
        - ヘルプ画面から`q/e:dec/inc`と`hjkl/wasd:move`の表記が消えていること。
        - 新しいヘルプ表示が、`src/app/shortcuts.rs`の内容を反映していること。
        - アプリケーションが引き続き正常にコンパイル・実行できること。

        期待する出力: 修正された`src/app/mod.rs`, `src/app/shortcuts.rs`, `src/ui/mod.rs`のコード。および、変更内容のmarkdown形式での説明。
        ```

3.  Envelope折れ線グラフの改善 ([Issue #218](../issue-notes/218.md))
    -   最初の小さな一歩: `src/ui/mod.rs`内の`draw_envelope_canvas`関数を分析し、OP1, OP2, OP3, OP4それぞれのエンベロープを個別のグラフとして描画するために必要な変更点を特定する。
    -   Agent実行プロンプト:
        ```
        対象ファイル: src/ui/mod.rs, src/ui/helpers.rs

        実行内容: `src/ui/mod.rs`の`draw_envelope_canvas`関数を、オペレータごとに個別のキャンバスまたはサブグラフとして描画するように修正してください。具体的には、
        1. 各オペレータ(OP1-OP4)のエンベロープが、隣接してまたは重ねて描画されつつも、それぞれが明確に区別できるようにしてください。
        2. キャリア/モジュレータの区別を視覚的に表示するための要素（例: キャリアを太線にする、異なるパターンで描画する、ラベルを追加するなど）を導入してください。`get_operator_roles_for_alg(alg_value)`の結果を参考にしてください。
        3. Slow attackやlong decayなど、エンベロープの形状がより明確に視覚化されるように、グラフの描画ロジックを調整してください。

        確認事項:
        - 4つのオペレータのエンベロープが個別に、かつ明確に表示されていること。
        - アルゴリズムに応じたキャリア/モジュレータの区別が視覚的に表現されていること。
        - 異なるADSR設定がグラフ形状に反映され、視覚的に区別できること。
        - UIのレイアウトが崩れていないこと。
        - コンパイルエラーがないこと。

        期待する出力: 修正された`src/ui/mod.rs`と`src/ui/helpers.rs`のコード。および、エンベロープ描画改善内容のmarkdown形式での説明。

---
Generated at: 2026-03-17 07:16:40 JST
