Last updated: 2025-12-07

# Development Status

## 現在のIssues
- [Issue #151](../issue-notes/151.md)では、音源パラメータのデバッグのため、ADSRのrateとlevel、wait値を最大にして問題の切り分けを進めています。
- [Issue #149](../issue-notes/149.md)は、General MIDI (GM) 音色データ保存用のテンプレートファイルを`tones/`ディレクトリに生成する作業を進めています。
- 現在の開発は、音源の正確な制御と、GM音色データ管理の基盤構築に焦点を当てています。

## 次の一手候補
1. [Issue #151](../issue-notes/151.md) の音源パラメータ調整とデバッグの継続
   - 最初の小さな一歩: `src/midi_conversion.rs`や`src/app.rs`といった関連ファイルで、ADSRパラメータ（Rate/Level）とWaitパラメータの設定箇所を特定し、指定された最大値に一時的に変更して、実際の音源再生での挙動変化を確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/midi_conversion.rs, src/app.rs, src/audio.rs

     実行内容: [Issue #151](../issue-notes/151.md)の指示に基づき、ADSRのRateおよびLevel、Waitパラメータを一時的に最大値に設定する変更案を作成してください。これらのパラメータが、音源のエンベロープ（アタック、ディケイ、サスティン、リリース）とノートオフ後の待機時間にどのように影響するかを分析し、変更前後の挙動の違いを検証するためのテスト計画をmarkdown形式で出力してください。

     確認事項: ADSRパラメータとWaitパラメータがMIDIイベント処理または音源レジスタ設定のどの部分に影響するか、既存の音源制御ロジックとの依存関係、および変更がアプリケーション全体の安定性に与える可能性のある影響を確認してください。

     期待する出力: 変更が必要な具体的なコード箇所（ファイル名、関数名、行番号など）、変更後のテスト手順（例：特定のMIDIシーケンスでの再生、デバッグログの確認、サウンド出力の評価方法など）、および変更を元に戻すための手順をまとめた詳細なmarkdownレポート。
     ```

2. [Issue #149](../issue-notes/149.md) General MIDI音色データテンプレートの生成
   - 最初の小さな一歩: `tones/general_midi/000_AcousticGrand.json`の構造を詳細に分析し、`tones/general_midi/tone_names.json`に含まれるGM音色名リストを基に、GM001 (Bright Acoustic Piano) のテンプレートJSONファイルを`tones/general_midi/001_BrightAcousticPiano.json`として手動で作成する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/general_midi/000_AcousticGrand.json, tones/general_midi/tone_names.json, src/file_ops.rs

     実行内容: [Issue #149](../issue-notes/149.md)の要求を満たすため、General MIDI (GM) 001から0127までの音色データ保存用JSONテンプレートファイルを`tones/general_midi/`ディレクトリに生成する自動化スクリプトまたは詳細な手順書を検討してください。この際、`000_AcousticGrand.json`をベースとし、`tone_names.json`からGM音色名と番号を抽出し、適切なファイル名と初期内容を持つダミーJSONファイルを生成するロジックを設計してください。

     確認事項: `tone_names.json`のフォーマットがGM音色リストの抽出に適しているか、生成されるJSONファイルのデータ構造がアプリケーションの音色ロード機能（`src/file_ops.rs`など）と互換性があるか、およびファイル名が命名規則に沿っているかを確認してください。

     期待する出力: GM音色テンプレートファイルを効率的に生成するためのPythonまたはNode.jsスクリプトのコード案（または詳細な擬似コード）、または手動でファイルを生成する際の一連のコマンドと考慮事項を記述したmarkdown形式の手順書。生成されるファイル構造の例を少なくとも3つ含めてください。
     ```

3. [Issue #149](../issue-notes/149.md) 関連: `tones/general_midi/tone_names.json` の内容の検証と整合性の確保
   - 最初の小さな一歩: `tones/general_midi/tone_names.json` の内容を、一般的なGeneral MIDI (GM) 音色リストの標準情報源（例: WikipediaやMIDI協会のドキュメント）と照合し、記載されている音色名と番号が正確で完全であるかを確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/general_midi/tone_names.json

     実行内容: [Issue #149](../issue-notes/149.md)で利用される`tones/general_midi/tone_names.json`の内容について、General MIDI標準の音色名と番号リストとの整合性を分析してください。具体的には、このJSONファイルがすべてのGM音色（000-127）を網羅しているか、各音色名が正確であるか、および将来的な拡張（例: 音色カテゴリー情報の追加）を考慮した構造になっているかを評価してください。

     確認事項: 参照するGM標準音色リストが最新かつ信頼できる情報源であること。`tone_names.json`の構造が現在のアプリケーション設計（特にUI表示や音色データ管理）にどのように影響しているか。

     期待する出力: `tone_names.json`の現状分析結果、GM標準との差異のリスト、不足している音色や誤っている音色名の修正提案、およびファイル構造の改善案（もしあれば）を含むmarkdown形式のレポート。
     ```

---
Generated at: 2025-12-07 07:08:05 JST
