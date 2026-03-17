Last updated: 2026-03-18

# 開発状況生成プロンプト（開発者向け）

## 生成するもの：
- 現在openされているissuesを3行で要約する
- 次の一手の候補を3つlistする
- 次の一手の候補3つそれぞれについて、極力小さく分解して、その最初の小さな一歩を書く

## 生成しないもの：
- 「今日のissue目標」などuserに提案するもの
  - ハルシネーションの温床なので生成しない
- ハルシネーションしそうなものは生成しない（例、無価値なtaskや新issueを勝手に妄想してそれをuserに提案する等）
- プロジェクト構造情報（来訪者向け情報のため、別ファイルで管理）

## 「Agent実行プロンプト」生成ガイドライン：
「Agent実行プロンプト」作成時は以下の要素を必ず含めてください：

### 必須要素
1. **対象ファイル**: 分析/編集する具体的なファイルパス
2. **実行内容**: 具体的な分析や変更内容（「分析してください」ではなく「XXXファイルのYYY機能を分析し、ZZZの観点でmarkdown形式で出力してください」）
3. **確認事項**: 変更前に確認すべき依存関係や制約
4. **期待する出力**: markdown形式での結果や、具体的なファイル変更

### Agent実行プロンプト例

**良い例（上記「必須要素」4項目を含む具体的なプロンプト形式）**:
```
対象ファイル: `.github/workflows/translate-readme.yml`と`.github/workflows/call-translate-readme.yml`

実行内容: 対象ファイルについて、外部プロジェクトから利用する際に必要な設定項目を洗い出し、以下の観点から分析してください：
1) 必須入力パラメータ（target-branch等）
2) 必須シークレット（GEMINI_API_KEY）
3) ファイル配置の前提条件（README.ja.mdの存在）
4) 外部プロジェクトでの利用時に必要な追加設定

確認事項: 作業前に既存のworkflowファイルとの依存関係、および他のREADME関連ファイルとの整合性を確認してください。

期待する出力: 外部プロジェクトがこの`call-translate-readme.yml`を導入する際の手順書をmarkdown形式で生成してください。具体的には：必須パラメータの設定方法、シークレットの登録手順、前提条件の確認項目を含めてください。
```

**避けるべき例**:
- callgraphについて調べてください
- ワークフローを分析してください
- issue-noteの処理フローを確認してください

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Development Status

## 現在のIssues
[以下の形式で3行でオープン中のissuesを要約。issue番号を必ず書く]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 次の一手候補
1. [候補1のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

2. [候補2のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

3. [候補3のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```
```


# 開発状況情報
- 以下の開発状況情報を参考にしてください。
- Issue番号を記載する際は、必ず [Issue #番号](../issue-notes/番号.md) の形式でMarkdownリンクとして記載してください。

## プロジェクトのファイル一覧
- .github/actions-tmp/.github/workflows/call-callgraph.yml
- .github/actions-tmp/.github/workflows/call-check-large-files.yml
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-check.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-large-files.yml
- .github/actions-tmp/.github/workflows/check-recent-human-commit.yml
- .github/actions-tmp/.github/workflows/daily-project-summary.yml
- .github/actions-tmp/.github/workflows/issue-note.yml
- .github/actions-tmp/.github/workflows/rust-windows-check.yml
- .github/actions-tmp/.github/workflows/translate-readme.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/callgraph.ql
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/codeql-pack.lock.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/qlpack.yml
- .github/actions-tmp/.github_automation/callgraph/config/example.json
- .github/actions-tmp/.github_automation/callgraph/docs/callgraph.md
- .github/actions-tmp/.github_automation/callgraph/presets/callgraph.js
- .github/actions-tmp/.github_automation/callgraph/presets/style.css
- .github/actions-tmp/.github_automation/callgraph/scripts/analyze-codeql.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/callgraph-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-codeql-exists.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-node-version.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/common-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/copy-commit-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/extract-sarif-info.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/find-process-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generate-html-graph.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generateHTML.cjs
- .github/actions-tmp/.github_automation/check-large-files/README.md
- .github/actions-tmp/.github_automation/check-large-files/check-large-files.toml.default
- .github/actions-tmp/.github_automation/check-large-files/scripts/check_large_files.py
- .github/actions-tmp/.github_automation/check_recent_human_commit/scripts/check-recent-human-commit.cjs
- .github/actions-tmp/.github_automation/project_summary/docs/daily-summary-setup.md
- .github/actions-tmp/.github_automation/project_summary/prompts/development-status-prompt.md
- .github/actions-tmp/.github_automation/project_summary/prompts/project-overview-prompt.md
- .github/actions-tmp/.github_automation/project_summary/scripts/ProjectSummaryCoordinator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/GitUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/IssueTracker.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/generate-project-summary.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/CodeAnalyzer.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectAnalysisOrchestrator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataCollector.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataFormatter.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectOverviewGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/BaseGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/FileSystemUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/ProjectFileUtils.cjs
- .github/actions-tmp/.github_automation/translate/docs/TRANSLATION_SETUP.md
- .github/actions-tmp/.github_automation/translate/scripts/translate-readme.cjs
- .github/actions-tmp/.gitignore
- .github/actions-tmp/.vscode/settings.json
- .github/actions-tmp/LICENSE
- .github/actions-tmp/README.ja.md
- .github/actions-tmp/README.md
- .github/actions-tmp/_config.yml
- .github/actions-tmp/generated-docs/callgraph.html
- .github/actions-tmp/generated-docs/callgraph.js
- .github/actions-tmp/generated-docs/development-status-generated-prompt.md
- .github/actions-tmp/generated-docs/development-status.md
- .github/actions-tmp/generated-docs/project-overview-generated-prompt.md
- .github/actions-tmp/generated-docs/project-overview.md
- .github/actions-tmp/generated-docs/style.css
- .github/actions-tmp/googled947dc864c270e07.html
- .github/actions-tmp/issue-notes/10.md
- .github/actions-tmp/issue-notes/11.md
- .github/actions-tmp/issue-notes/12.md
- .github/actions-tmp/issue-notes/13.md
- .github/actions-tmp/issue-notes/14.md
- .github/actions-tmp/issue-notes/15.md
- .github/actions-tmp/issue-notes/16.md
- .github/actions-tmp/issue-notes/17.md
- .github/actions-tmp/issue-notes/18.md
- .github/actions-tmp/issue-notes/19.md
- .github/actions-tmp/issue-notes/2.md
- .github/actions-tmp/issue-notes/20.md
- .github/actions-tmp/issue-notes/21.md
- .github/actions-tmp/issue-notes/22.md
- .github/actions-tmp/issue-notes/23.md
- .github/actions-tmp/issue-notes/24.md
- .github/actions-tmp/issue-notes/25.md
- .github/actions-tmp/issue-notes/26.md
- .github/actions-tmp/issue-notes/27.md
- .github/actions-tmp/issue-notes/28.md
- .github/actions-tmp/issue-notes/29.md
- .github/actions-tmp/issue-notes/3.md
- .github/actions-tmp/issue-notes/30.md
- .github/actions-tmp/issue-notes/35.md
- .github/actions-tmp/issue-notes/38.md
- .github/actions-tmp/issue-notes/4.md
- .github/actions-tmp/issue-notes/40.md
- .github/actions-tmp/issue-notes/44.md
- .github/actions-tmp/issue-notes/52.md
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/copilot-instructions.md
- .github/workflows/build-wasm.yml
- .github/workflows/call-check-large-files.yml
- .github/workflows/call-daily-project-summary.yml
- .github/workflows/call-issue-note.yml
- .github/workflows/call-rust-windows-check.yml
- .github/workflows/call-translate-readme.yml
- .github/workflows/deploy-demo-library.yml
- .github/workflows/rust-test.yml
- .gitignore
- Cargo.lock
- Cargo.toml
- LICENSE
- README.ja.md
- README.md
- README_generate_gm_templates.md
- _config.yml
- build.rs
- core/Cargo.toml
- core/src/lib.rs
- core/src/tests.rs
- demo-library/index.html
- docs/KEYBINDS.ja.md
- generate_gm_templates.rs
- generated-docs/project-overview-generated-prompt.md
- googled947dc864c270e07.html
- issue-notes/113.md
- issue-notes/115.md
- issue-notes/139.md
- issue-notes/141.md
- issue-notes/148.md
- issue-notes/155.md
- issue-notes/156.md
- issue-notes/164.md
- issue-notes/167.md
- issue-notes/174.md
- issue-notes/177.md
- issue-notes/218.md
- issue-notes/219.md
- issue-notes/220.md
- issue-notes/223.md
- issue-notes/224.md
- issue-notes/95.md
- issue-notes/96.md
- src/app/mod.rs
- src/app/shortcuts.rs
- src/app_init.rs
- src/audio.rs
- src/config.rs
- src/event_loop.rs
- src/favorites.rs
- src/file_ops.rs
- src/history.rs
- src/history_selector.rs
- src/logging.rs
- src/main.rs
- src/midi_conversion.rs
- src/models.rs
- src/random_tone.rs
- src/register.rs
- src/register_list.rs
- src/tests/app_adsr_mul_sm_tests.rs
- src/tests/app_ch_param_tests.rs
- src/tests/app_ks_ams_tests.rs
- src/tests/app_tests.rs
- src/tests/app_tl_d1l_dt_dt2_tests.rs
- src/tests/app_value_by_tests.rs
- src/tests/favorites_tests.rs
- src/tests/file_ops_tests.rs
- src/tests/history_tests.rs
- src/tests/midi_conversion_tests.rs
- src/tests/mod.rs
- src/tests/random_tone_tests.rs
- src/tests/register_roundtrip_tests.rs
- src/tests/register_tests.rs
- src/tests/ui_tests.rs
- src/tests/variation_selector_tests.rs
- src/tests/verbose_logging_tests.rs
- src/ui/help.rs
- src/ui/helpers.rs
- src/ui/mod.rs
- src/updater.rs
- src/variation_selector.rs
- src/waveform.rs
- tones/general_midi/000_AcousticGrand.json
- tones/general_midi/tone_names.json
- wasm/Cargo.lock
- wasm/Cargo.toml
- wasm/src/lib.rs
- ym2151-tone-editor.toml.example

## 現在のオープンIssues
## [Issue #226](../issue-notes/226.md): 大きなファイルの検出: 1個のファイルが500行を超えています
以下のファイルが500行を超えています。リファクタリングを検討してください。

## 検出されたファイル

| ファイル | 行数 | 超過行数 |
|---------|------|----------|
| `src/event_loop.rs` | 511 | +11 |

## テスト実施のお願い

- リファクタリング前後にテストを実行し、それぞれのテスト失敗件数を報告してください
- リファクタリング前後のどちらかでテストがredの場合、まず別issueでtest greenにしてからリファクタリングしてください

## 推奨事項

1. 単一責任の原則に従い、ファイルを分割する...
ラベル: refactoring, code-quality, automated
--- issue-notes/226.md の内容 ---

```markdown

```

## [Issue #224](../issue-notes/224.md): windows appdata roaming側にファイルができてしまっている
[issue-notes/224.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/224.md)

...
ラベル: 
--- issue-notes/224.md の内容 ---

```markdown
# issue windows appdata roaming側にファイルができてしまっている #224
[issues #224](https://github.com/cat2151/ym2151-tone-editor/issues/224)

- 現状 : `C:\Users\<your name>\AppData\Roaming\ym2151-tone-editor`
- userがほしいのは
    - `C:\Users\<your name>\AppData\Local\ym2151-tone-editor`
    - にファイルができること

```

## [Issue #223](../issue-notes/223.md): 音色ファイルの読み書きの仕様変更。すべてwindows appdata local config dirs で読み書きとする。カレントディレクトリをやめる
[issue-notes/223.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/223.md)

...
ラベル: 
--- issue-notes/223.md の内容 ---

```markdown
# issue 音色ファイルの読み書きの仕様変更。すべてwindows appdata local config dirs で読み書きとする。カレントディレクトリをやめる #223
[issues #223](https://github.com/cat2151/ym2151-tone-editor/issues/223)



```

## [Issue #219](../issue-notes/219.md): helpから、hjklとwasdを外して、矢印キーを書く
[issue-notes/219.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/219.md)

...
ラベル: 
--- issue-notes/219.md の内容 ---

```markdown
# issue helpから、hjklとwasdを外して、矢印キーを書く #219
[issues #219](https://github.com/cat2151/ym2151-tone-editor/issues/219)

- なぜならhjklとwasdは現状廃止されているから
- より具体的には：
  - help画面の表示は、keybinds設定から生成すること
  - 常時、画面左下に、?:help を表示すること

```

## [Issue #218](../issue-notes/218.md): envelope折れ線グラフを改善する
[issue-notes/218.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/218.md)

...
ラベル: 
--- issue-notes/218.md の内容 ---

```markdown
# issue envelope折れ線グラフを改善する #218
[issues #218](https://github.com/cat2151/ym2151-tone-editor/issues/218)

- 課題、sixelで、と指示したのにsixelになってない。
- 課題、OP1,OP2,OP3,OP4 それぞれ個別で表示してUX検証したい。
- 課題、どれがキャリアか？どれがモジュレータか？がわからない。
- 課題、どれも似たような見た目になってしまっている。slow attackや、long decayの区別がつかない。
- 課題、envelopeの、どこまでがアタックで、どこからディケイで、どこからサステインで、どこからリリースか？区別がつかない。

```

## [Issue #174](../issue-notes/174.md): issue 149 の結果を利用し、userがlocalに音色template jsonファイルを生成する
[issue-notes/174.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/174.md)

...
ラベル: 
--- issue-notes/174.md の内容 ---

```markdown
# issue issue 149 の結果を利用し、userがlocalに音色template jsonファイルを生成する #174
[issues #174](https://github.com/cat2151/ym2151-tone-editor/issues/174)



```

## [Issue #167](../issue-notes/167.md): （待ち）プレビューを鳴らすとき、前の音のkeyoff時にプチノイズが乗ってしまう
[issue-notes/167.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/167.md)

...
ラベル: 
--- issue-notes/167.md の内容 ---

```markdown
# issue プレビューを鳴らすとき、前の音のkeyoff時にプチノイズが乗ってしまう #167
[issues #167](https://github.com/cat2151/ym2151-tone-editor/issues/167)

# わかっていること
- これまでの不具合は一通り解決した
    - slow attack音色を問題なくプレビューできるようになった
    - プチノイズ以外はプレビュー音が適切に演奏できるようになった
- slow attack音色でプチノイズが乗っている、
    - よって、keyonにはプチノイズは乗っていない
- なお、cat-play-mmlで普通に演奏してもプチノイズが乗る
    - 注意、それは切り分けて別途考えるほうがよい
# 仮説
- 以下いずれかが必要
    - キャリアのTLを127にしてからRR15
        - 仮説、かえってプチノイズになる可能性
    - キャリアだけはRR14
    - ADSRのうちRだけを15にし、のち、Rを編集中の音色の値に復帰する
        - 仮説、ADSRすべてを最速にした瞬間、かえってプチノイズになる可能性
# 考察
- 毎回試行錯誤にissueを立て、レビューして、動作確認、というサイクルをまわすのは、本件のようなissueだと非効率に感じる
  - より正確には、このissueは堂々巡りに入りそうな予感がある、それを問題視している
- もっとlocalで素早くサイクルをまわす方法の案を洗いだす
- 例
    - jsonを作って再生してプチノイズが乗るかを確認する
        - データ内容
            - long decayを0.5秒鳴らす
            - keyoff ～ 次のkeyon
                - プレビューと同じ
    - 課題
        - json編集のコストが大きい、編集ミスのリスクが高い
        - 案
            - STed2のようなjson editorのrepositoryを検討する
                - eventが可視化されている、読みやすい、機械語に対するニモニックのような表示で
                - time部分を、累積時間と、時刻とを、toggle切り替えできる
                    - 保存時は時刻
                    - これならwait増減が楽である
                        - 「選択範囲の時刻をまとめて増減」より楽
                        - シンプルなUIで時間編集ができる
# どうする？
- そのprojectを作る方向でいく
- これまでもそのprojectの必要性は予想していた
- 「次にそのprojectがないと困る段階」になったら、実施しよう、と計画していた
- なぜならそれが一番のそのprojectのtestになるので

# 状況
- 別リポジトリで、webpageで軽量GUIを作成し、json編集でプチノイズ調査、を進めている
- それを待つ

```

## [Issue #155](../issue-notes/155.md): ドッグフーディングする
[issue-notes/155.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/155.md)

...
ラベル: 
--- issue-notes/155.md の内容 ---

```markdown
# issue ドッグフーディングする #155
[issues #155](https://github.com/cat2151/ym2151-tone-editor/issues/155)



```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/18.md
```md
{% raw %}
# issue DevelopmentStatusGenerator.cjs 内に、Geminiに与えるpromptがハードコーディングされてしまっている #18
[issues #18](https://github.com/cat2151/github-actions/issues/18)

# 何が困るの？
- project把握しづらい。どこにpromptが書いてあるのか、把握しづらい。
- prompts/ にほかのpromptがあるため、方針がブレていると、読みづらい。
- 備忘、いくらテンプレートリテラルとプレースホルダーで密結合しているからとはいえ、ハードコーディングはNG。
    - それらはreplaceを使う等で楽に切り出しできるので。

# 問題のcjsの場所は？
- ファイルパス : .github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
- 関数 : generateDevelopmentStatus

# 結果
- Geminiに生成させたpromptを、agentに投げて、リファクタリングさせてみた
- ハルシネーションした。使い物にならなかった
- 人力でやる

# 結果
- test green

# closeとする


{% endraw %}
```

### .github/actions-tmp/issue-notes/19.md
```md
{% raw %}
# issue project-summary の development-status 生成時、issue-notes/ 配下のmdファイルの内容を参照させる #19
[issues #19](https://github.com/cat2151/github-actions/issues/19)

# 何が困るの？
- issue解決に向けての次の一手の内容が実態に即していないことが多い。

# 対策案
- issue-notes/ 配下のmdファイルの内容を参照させる

# 備考
- さらにmd内に書かれているfileも、project内をcjsに検索させて添付させると、よりGeminiの生成品質が向上する可能性がある。
    - [issues #20](https://github.com/cat2151/github-actions/issues/20)
- さらにproject overviewでGeminiがまとめたmdも、Geminiに与えると、よりGeminiの生成品質が向上する可能性がある。
    - [issues #21](https://github.com/cat2151/github-actions/issues/21)
- さらに、Geminiに与えたpromptをfileにしてcommit pushしておくと、デバッグに役立つ可能性がある。
    - [issues #22](https://github.com/cat2151/github-actions/issues/22)

# close条件
- issues #22 がcloseされること。
- commitされたpromptを確認し、issue-notes/ 配下のmdファイルがpromptに添付されていること、が確認できること。

# 状況
- 課題、実装したがtestができていない
- 対策、issues #22 が実装されれば、testができる
- 対策、issues #22 のcloseを待つ

# 状況
- issues #22 がcloseされた
- testできるようになった
- commitされたpromptを確認した。issue-notes/ 配下のmdファイルがpromptに添付されていること、が確認できた

# closeする

{% endraw %}
```

### .github/actions-tmp/issue-notes/23.md
```md
{% raw %}
# issue issue 17が再発してしまっている #23
[issues #23](https://github.com/cat2151/github-actions/issues/23)

# 症状は？
- issue 17と同じ

# どうする？
- development-status-generated-prompt.md を確認する
- 結果
    - >Issue番号を記載する際は、必ず [Issue #番号](issue-notes/番号.md) の形式でMarkdownリンクとして記載してください。
    - 仮説、これが残っており、ほかの ../ 指定と競合し、どちらかがランダムで選ばれていた
    - 対策、ここを ../ 指定にする

# 結果
- test green

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/24.md
```md
{% raw %}
# issue Geminiが503で落ちたのでretryを実装する #24
[issues #24](https://github.com/cat2151/github-actions/issues/24)

# 何が困るの？
- 朝起きて、development statusがgenerateされてないのは困る
    - それをタスク実施のヒントにしているので
    - 毎朝generatedな状態を維持したい

# 方法
- retryを実装する
    - 現在は `this.model.generateContent(developmentPrompt);`
    - 実装後は `this.generateContent(developmentPrompt);`
    - BaseGenerator 側に、
        - generateContent関数を実装する
            - そこで、
                - `this.model.generateContent(developmentPrompt);` する
                - 503のとき、
                    - retryあり
                    - Exponential Backoff

# 結果
- 直近の実行結果をlog確認した
    - 本番で503が発生しなかったことをlog確認した
- 本番の503 testは、今回発生しなかったので、できず
- ここ1週間で2回発生しているので、次の1週間で1回発生する想定
- ソース机上確認した

# どうする？
- このissueはcloseしたほうがわかりやすい、と判断する
- 1週間503を毎日チェック、は省略とする
- もし今後503が発生したら別issueとする
- 2日チェックして503なし

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/26.md
```md
{% raw %}
# issue userによるcommitがなくなって24時間超経過しているのに、毎日ムダにproject summaryとcallgraphの自動生成が行われてしまっている #26
[issues #26](https://github.com/cat2151/github-actions/issues/26)

# どうする？
- logを確認する。24時間チェックがバグっている想定。
- もしlogから判別できない場合は、logを改善する。

# log確認結果
- botによるcommitなのに、user commitとして誤判別されている
```
Checking for user commits in the last 24 hours...
User commits found: true
Recent user commits:
7654bf7 Update callgraph.html [auto]
abd2f2d Update project summaries (overview & development status)
```

# ざっくり調査結果
- #27 が判明した

# どうする？
- [x] #27 を修正する。これで自動的に #26 も修正される想定。
    - 当該処理を修正する。
    - もしデータ不足なら、より詳細なlog生成を実装する。
- 別件として、このチェックはむしろworkflow ymlの先頭で行うのが適切と考える。なぜなら、以降のムダな処理をカットできるのでエコ。
    - [x] #28 を起票したので、そちらで実施する。

# close条件は？
- 前提
    - [x] 先行タスクである #27 と #28 が完了済みであること
- 誤爆がなくなること。
    - つまり、userによるcommitがなくなって24時間超経過後の日次バッチにて、
        - ムダなdevelopment status生成、等がないこと
        - jobのlogに「commitがないので処理しません」的なmessageが出ること
- どうする？
    - 日次バッチを本番を流して本番testする

# 結果
- github-actions logより：
    - 直近24hのcommitはbotによる1件のみであった
    - よって後続jobはskipとなった
    - ことを確認した
- close条件を満たした、と判断する
```
Run node .github_automation/check_recent_human_commit/scripts/check-recent-human-commit.cjs
BOT: Commit 5897f0c6df6bc2489f9ce3579b4f351754ee0551 | Author: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com> | Message: Update project summaries (overview & development status) [auto]
has_recent_human_commit=false
```

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/3.md
```md
{% raw %}
# issue GitHub Actions「issue note生成」を共通ワークフロー化する #3
[issues #3](https://github.com/cat2151/github-actions/issues/3)

- 前提
  - userは、translateのworkflowを共通ワークフロー化し、動くようにしたので、共通ワークフロー化の知識がある
  - よって、今回は、promptでplanさせ、そのplanをuserがレビューし、planの改善を指示できるので、ハルシネーションのリスクを下げることができる

- prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlはまだ共通ワークフロー化されず、
呼び出し元からコピーされたままの状態です。
このymlファイル名のまま、
中身を共通ワークフロー化してください。
呼び出し元ymlは別途書きますので気にしないでください。
まずplanしてください
```

- plan 生成結果
```
共通ワークフロー化のためのplanは以下の通りです。

1. 目的
現在の issue-note.yml を「共通ワークフロー」として再設計し、他リポジトリや他ワークフローから workflow_call で呼び出せるようにする。
2. 主な変更点
on: トリガーを workflow_call に変更（直接イベントで起動しない）。
必要な入力（inputs）やシークレット（secrets）を workflow_call セクションで定義。
github.event.issue などのイベント依存部分を、inputs経由で受け取る形に修正。
既存の処理（issueノート生成、コミット、issue body更新）はそのまま活かすが、パラメータ化する。
3. 具体的な修正内容
on: issues: → on: workflow_call: に変更。
inputs で必要な値（例: issue_title, issue_number, issue_body, issue_url など）を受け取る。
スクリプト内の ${{ github.event.issue.xxx }} を ${{ inputs.xxx }} に置換。
github-script ステップも同様にinputsを参照。
必要に応じて secrets も workflow_call で受け取れるようにする。
4. ファイル名・配置
ファイル名はそのまま issue-note.yml。
配置場所は workflows でOK（共通ワークフローとしてもこの場所で良い）。
5. 呼び出し元への影響
呼び出し元ワークフローで uses: <repo>/.github/workflows/issue-note.yml@main のように参照し、inputsを渡す形になる。
このplanで進めます。次に、実際の共通ワークフロー化のためのyml修正案を作成しますか？
```

- userがplanレビューし、OKを出した
- agentに生成結果ymlをレビューさせた、OKとなった
  - userはこのレビューは認知負荷が高いのでskipとする（単純な細かい間違い探し作業になるので、それはagentに任せた）
-  エラー。呼び出し元ymlのエラーがあり修正。共通ワークフローのエラー指摘があり、共通ワークフロー側の修正が必要となった
```
原因の分析：

呼び出し元（tonejs-mml-to-jsonのissue-note.yml）は、reusable workflow（cat2151/github-actions/.github/workflows/issue-note.yml@main）をusesで呼び出し、inputsをwithで渡しています。
共通ワークフロー（reusable workflow）は inputs を workflow_call で受け取り、各ステップで ${{ inputs.xxx }} で参照しています。
しかし、エラーが発生したのは actions/github-script@v7 の script 内です。
エラーの本質的な原因：

actions/github-script の script: で ${{ inputs.issue_number }} のような式を直接 JavaScript テンプレートリテラル内で使っていますが、これはサポートされていません。
script: の中は JavaScript であり、YAMLの ${{ ... }} は展開されません（または意図通りに展開されません）。
そのため、inputs 変数は script 内の JavaScript で未定義となり、ReferenceError になります。
正しい書き方：

actions/github-script の script: でワークフローの inputs を使いたい場合は、with: で input を environment 変数や script の引数として渡し、JavaScript で process.env から取得するか、github-script の input 機能を使う必要があります。
例（修正版）：

このように、inputs の値を env で渡し、script 内では process.env で参照してください。

まとめ：

actions/github-script の script 内で ${{ inputs.xxx }} を直接使うと ReferenceError になる
env: で値を渡し、process.env で参照するのが正しい
修正が必要です。修正版のワークフローが必要な場合はお知らせください。
```

- test green
- closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/4.md
```md
{% raw %}
# issue GitHub Actions「project概要生成」を共通ワークフロー化する #4
[issues #4](https://github.com/cat2151/github-actions/issues/4)

# prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlファイルを、以下の2つのファイルに分割してください。
1. 共通ワークフロー       cat2151/github-actions/.github/workflows/daily-project-summary.yml
2. 呼び出し元ワークフロー cat2151/github-actions/.github/workflows/call-daily-project-summary.yml
まずplanしてください
```

# 結果、あちこちハルシネーションのあるymlが生成された
- agentの挙動があからさまにハルシネーション
    - インデントが修正できない、「失敗した」という
    - 構文誤りを認識できない
- 人力で修正した

# このagentによるセルフレビューが信頼できないため、別のLLMによるセカンドオピニオンを試す
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
以下の2つのファイルをレビューしてください。最優先で、エラーが発生するかどうかだけレビューてください。エラー以外の改善事項のチェックをするかわりに、エラー発生有無チェックに最大限注力してください。

--- 呼び出し元

name: Call Daily Project Summary

on:
  schedule:
    # 日本時間 07:00 (UTC 22:00 前日)
    - cron: '0 22 * * *'
  workflow_dispatch:

jobs:
  call-daily-project-summary:
    uses: cat2151/github-actions/.github/workflows/daily-project-summary.yml
    secrets:
      GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}

--- 共通ワークフロー
name: Daily Project Summary
on:
  workflow_call:

jobs:
  generate-summary:
    runs-on: ubuntu-latest

    permissions:
      contents: write
      issues: read
      pull-requests: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          fetch-depth: 0  # 履歴を取得するため

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: |
          # 一時的なディレクトリで依存関係をインストール
          mkdir -p /tmp/summary-deps
          cd /tmp/summary-deps
          npm init -y
          npm install @google/generative-ai @octokit/rest
          # generated-docsディレクトリを作成
          mkdir -p $GITHUB_WORKSPACE/generated-docs

      - name: Generate project summary
        env:
          GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          GITHUB_REPOSITORY: ${{ github.repository }}
          NODE_PATH: /tmp/summary-deps/node_modules
        run: |
          node .github/scripts/generate-project-summary.cjs

      - name: Check for generated summaries
        id: check_summaries
        run: |
          if [ -f "generated-docs/project-overview.md" ] && [ -f "generated-docs/development-status.md" ]; then
            echo "summaries_generated=true" >> $GITHUB_OUTPUT
          else
            echo "summaries_generated=false" >> $GITHUB_OUTPUT
          fi

      - name: Commit and push summaries
        if: steps.check_summaries.outputs.summaries_generated == 'true'
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"
          # package.jsonの変更のみリセット（generated-docsは保持）
          git restore package.json 2>/dev/null || true
          # サマリーファイルのみを追加
          git add generated-docs/project-overview.md
          git add generated-docs/development-status.md
          git commit -m "Update project summaries (overview & development status)"
          git push

      - name: Summary generation result
        run: |
          if [ "${{ steps.check_summaries.outputs.summaries_generated }}" == "true" ]; then
            echo "✅ Project summaries updated successfully"
            echo "📊 Generated: project-overview.md & development-status.md"
          else
            echo "ℹ️ No summaries generated (likely no user commits in the last 24 hours)"
          fi
```

# 上記promptで、2つのLLMにレビューさせ、合格した

# 細部を、先行する2つのymlを参照に手直しした

# ローカルtestをしてからcommitできるとよい。方法を検討する
- ローカルtestのメリット
    - 素早く修正のサイクルをまわせる
    - ムダにgit historyを汚さない
        - これまでの事例：「実装したつもり」「エラー。修正したつもり」「エラー。修正したつもり」...（以降エラー多数）
- 方法
    - ※検討、WSL + act を環境構築済みである。test可能であると判断する
    - 呼び出し元のURLをコメントアウトし、相対パス記述にする
    - ※備考、テスト成功すると結果がcommit pushされる。それでよしとする
- 結果
    - OK
    - secretsを簡略化できるか試した、できなかった、現状のsecrets記述が今わかっている範囲でベストと判断する
    - OK

# test green

# commit用に、yml 呼び出し元 uses をlocal用から本番用に書き換える

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/7.md
```md
{% raw %}
# issue issue note生成できるかのtest用 #7
[issues #7](https://github.com/cat2151/github-actions/issues/7)

- 生成できた
- closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/8.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズ生成の対象ソースファイルを、呼び出し元ymlで指定できるようにする #8
[issues #8](https://github.com/cat2151/github-actions/issues/8)

# これまでの課題
- 以下が決め打ちになっていた
```
  const allowedFiles = [
    'src/main.js',
    'src/mml2json.js',
    'src/play.js'
  ];
```

# 対策
- 呼び出し元ymlで指定できるようにする

# agent
- agentにやらせることができれば楽なので、初手agentを試した
- 失敗
    - ハルシネーションしてscriptを大量破壊した
- 分析
    - 修正対象scriptはagentが生成したもの
    - 低品質な生成結果でありソースが巨大
    - ハルシネーションで破壊されやすいソース
    - AIの生成したソースは、必ずしもAIフレンドリーではない

# 人力リファクタリング
- 低品質コードを、最低限agentが扱えて、ハルシネーションによる大量破壊を防止できる内容、にする
- 手短にやる
    - そもそもビジュアライズは、agentに雑に指示してやらせたもので、
    - 今後別のビジュアライザを選ぶ可能性も高い
    - 今ここで手間をかけすぎてコンコルド効果（サンクコストバイアス）を増やすのは、project群をトータルで俯瞰して見たとき、損
- 対象
    - allowedFiles のあるソース
        - callgraph-utils.cjs
            - たかだか300行未満のソースである
            - この程度でハルシネーションされるのは予想外
            - やむなし、リファクタリングでソース分割を進める

# agentに修正させる
## prompt
```
allowedFilesを引数で受け取るようにしたいです。
ないならエラー。
最終的に呼び出し元すべてに波及して修正したいです。

呼び出し元をたどってエントリポイントも見つけて、
エントリポイントにおいては、
引数で受け取ったjsonファイル名 allowedFiles.js から
jsonファイル allowedFiles.jsonの内容をreadして
変数 allowedFilesに格納、
後続処理に引き渡す、としたいです。

まずplanしてください。
planにおいては、修正対象のソースファイル名と関数名を、呼び出し元を遡ってすべて特定し、listしてください。
```

# 修正が順調にできた
- コマンドライン引数から受け取る作りになっていなかったので、そこだけ指示して修正させた
- yml側は人力で修正した

# 他のリポジトリから呼び出した場合にバグらないよう修正する
- 気付いた
    - 共通ワークフローとして他のリポジトリから使った場合はバグるはず。
        - ymlから、共通ワークフロー側リポジトリのcheckoutが漏れているので。
- 他のyml同様に修正する
- あわせて全体にymlをリファクタリングし、修正しやすくし、今後のyml読み書きの学びにしやすくする

# local WSL + act : test green

# closeとする
- もし生成されたhtmlがNGの場合は、別issueとするつもり

{% endraw %}
```

### .github/actions-tmp/issue-notes/9.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズが0件なので、原因を可視化する #9
[issues #9](https://github.com/cat2151/github-actions/issues/9)

# agentに修正させたり、人力で修正したりした
- agentがハルシネーションし、いろいろ根の深いバグにつながる、エラー隠蔽などを仕込んでいたため、検知が遅れた
- 詳しくはcommit logを参照のこと
- WSL + actの環境を少し変更、act起動時のコマンドライン引数を変更し、generated-docsをmountする（ほかはデフォルト挙動であるcpだけにする）ことで、デバッグ情報をコンテナ外に出力できるようにし、デバッグを効率化した

# test green

# closeとする

{% endraw %}
```

### issue-notes/155.md
```md
{% raw %}
# issue ドッグフーディングする #155
[issues #155](https://github.com/cat2151/ym2151-tone-editor/issues/155)



{% endraw %}
```

### issue-notes/167.md
```md
{% raw %}
# issue プレビューを鳴らすとき、前の音のkeyoff時にプチノイズが乗ってしまう #167
[issues #167](https://github.com/cat2151/ym2151-tone-editor/issues/167)

# わかっていること
- これまでの不具合は一通り解決した
    - slow attack音色を問題なくプレビューできるようになった
    - プチノイズ以外はプレビュー音が適切に演奏できるようになった
- slow attack音色でプチノイズが乗っている、
    - よって、keyonにはプチノイズは乗っていない
- なお、cat-play-mmlで普通に演奏してもプチノイズが乗る
    - 注意、それは切り分けて別途考えるほうがよい
# 仮説
- 以下いずれかが必要
    - キャリアのTLを127にしてからRR15
        - 仮説、かえってプチノイズになる可能性
    - キャリアだけはRR14
    - ADSRのうちRだけを15にし、のち、Rを編集中の音色の値に復帰する
        - 仮説、ADSRすべてを最速にした瞬間、かえってプチノイズになる可能性
# 考察
- 毎回試行錯誤にissueを立て、レビューして、動作確認、というサイクルをまわすのは、本件のようなissueだと非効率に感じる
  - より正確には、このissueは堂々巡りに入りそうな予感がある、それを問題視している
- もっとlocalで素早くサイクルをまわす方法の案を洗いだす
- 例
    - jsonを作って再生してプチノイズが乗るかを確認する
        - データ内容
            - long decayを0.5秒鳴らす
            - keyoff ～ 次のkeyon
                - プレビューと同じ
    - 課題
        - json編集のコストが大きい、編集ミスのリスクが高い
        - 案
            - STed2のようなjson editorのrepositoryを検討する
                - eventが可視化されている、読みやすい、機械語に対するニモニックのような表示で
                - time部分を、累積時間と、時刻とを、toggle切り替えできる
                    - 保存時は時刻
                    - これならwait増減が楽である
                        - 「選択範囲の時刻をまとめて増減」より楽
                        - シンプルなUIで時間編集ができる
# どうする？
- そのprojectを作る方向でいく
- これまでもそのprojectの必要性は予想していた
- 「次にそのprojectがないと困る段階」になったら、実施しよう、と計画していた
- なぜならそれが一番のそのprojectのtestになるので

# 状況
- 別リポジトリで、webpageで軽量GUIを作成し、json編集でプチノイズ調査、を進めている
- それを待つ

{% endraw %}
```

### issue-notes/174.md
```md
{% raw %}
# issue issue 149 の結果を利用し、userがlocalに音色template jsonファイルを生成する #174
[issues #174](https://github.com/cat2151/ym2151-tone-editor/issues/174)



{% endraw %}
```

### issue-notes/218.md
```md
{% raw %}
# issue envelope折れ線グラフを改善する #218
[issues #218](https://github.com/cat2151/ym2151-tone-editor/issues/218)

- 課題、sixelで、と指示したのにsixelになってない。
- 課題、OP1,OP2,OP3,OP4 それぞれ個別で表示してUX検証したい。
- 課題、どれがキャリアか？どれがモジュレータか？がわからない。
- 課題、どれも似たような見た目になってしまっている。slow attackや、long decayの区別がつかない。
- 課題、envelopeの、どこまでがアタックで、どこからディケイで、どこからサステインで、どこからリリースか？区別がつかない。

{% endraw %}
```

### issue-notes/219.md
```md
{% raw %}
# issue helpから、hjklとwasdを外して、矢印キーを書く #219
[issues #219](https://github.com/cat2151/ym2151-tone-editor/issues/219)

- なぜならhjklとwasdは現状廃止されているから
- より具体的には：
  - help画面の表示は、keybinds設定から生成すること
  - 常時、画面左下に、?:help を表示すること

{% endraw %}
```

### issue-notes/223.md
```md
{% raw %}
# issue 音色ファイルの読み書きの仕様変更。すべてwindows appdata local config dirs で読み書きとする。カレントディレクトリをやめる #223
[issues #223](https://github.com/cat2151/ym2151-tone-editor/issues/223)



{% endraw %}
```

### issue-notes/224.md
```md
{% raw %}
# issue windows appdata roaming側にファイルができてしまっている #224
[issues #224](https://github.com/cat2151/ym2151-tone-editor/issues/224)

- 現状 : `C:\Users\<your name>\AppData\Roaming\ym2151-tone-editor`
- userがほしいのは
    - `C:\Users\<your name>\AppData\Local\ym2151-tone-editor`
    - にファイルができること

{% endraw %}
```

### src/event_loop.rs
```rs
{% raw %}
use crate::app::App;
#[cfg(windows)]
use crate::audio;
use crate::config::{Action, Config};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;

use crate::models::{CH_PARAM_ALG, ROW_CH};

/// Convert KeyCode and KeyModifiers to a key string for config lookup
fn key_to_string(code: KeyCode, modifiers: KeyModifiers) -> Option<String> {
    match code {
        KeyCode::Char(c) => {
            // Handle CTRL+SHIFT modifier (for CTRL+SHIFT+1,2,3,4)
            if modifiers.contains(KeyModifiers::CONTROL) && modifiers.contains(KeyModifiers::SHIFT)
            {
                Some(format!("Ctrl+Shift+{}", c))
            }
            // Handle CTRL modifier (for CTRL+1,2,3,4)
            else if modifiers.contains(KeyModifiers::CONTROL) {
                Some(format!("Ctrl+{}", c))
            }
            // Handle space key
            else if c == ' ' {
                Some("Space".to_string())
            }
            // Handle SHIFT modifier for special characters
            else if modifiers.contains(KeyModifiers::SHIFT) {
                // For shifted characters, return the character as-is
                Some(c.to_string())
            } else {
                Some(c.to_string())
            }
        }
        KeyCode::Left => Some("Left".to_string()),
        KeyCode::Right => Some("Right".to_string()),
        KeyCode::Up => Some("Up".to_string()),
        KeyCode::Down => Some("Down".to_string()),
        KeyCode::Home => Some("Home".to_string()),
        KeyCode::End => Some("End".to_string()),
        KeyCode::PageUp => Some("PageUp".to_string()),
        KeyCode::PageDown => Some("PageDown".to_string()),
        KeyCode::Esc => Some("Esc".to_string()),
        KeyCode::F(n) => Some(format!("F{}", n)),
        _ => None,
    }
}

/// Handle variation selector action by suspending TUI, running selector, and restoring state
/// Returns Ok(()) if successful, Err if terminal operations fail
fn handle_open_variation_selector<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    // Suspend terminal UI to allow variation selector to take over
    let mut stdout = io::stdout();
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    // Run variation selector
    let selection_result = crate::variation_selector::open_variation_selector();

    // Restore terminal UI first
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    terminal.clear()?;

    // Process selection result after UI is restored
    match selection_result {
        Ok(Some(tone_data)) => {
            app.values = tone_data;
            #[cfg(windows)]
            {
                if app.use_interactive_mode {
                    // Play the loaded tone with current cursor position
                    audio::play_tone(
                        &app.values,
                        app.use_interactive_mode,
                        app.cursor_x,
                        app.cursor_y,
                        app.envelope_delay_seconds,
                    );
                }
            }
        }
        Ok(None) => {
            // User cancelled selection, do nothing
        }
        Err(e) => {
            eprintln!("Error loading variation: {}", e);
        }
    }

    Ok(())
}

/// Handle history selector action by suspending TUI, running selector, and restoring state
/// Returns Ok(()) if successful, Err if terminal operations fail
fn handle_open_history_selector<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    // Suspend terminal UI to allow history selector to take over
    let mut stdout = io::stdout();
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    // Run history selector
    #[cfg(windows)]
    let use_interactive_mode = app.use_interactive_mode;
    #[cfg(not(windows))]
    let use_interactive_mode = false;
    let selection_result = crate::history_selector::open_history_selector(use_interactive_mode);

    // Restore terminal UI first
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    terminal.clear()?;

    // Process selection result after UI is restored
    match selection_result {
        Ok(Some(tone_data)) => {
            app.values = tone_data;
            #[cfg(windows)]
            {
                if app.use_interactive_mode {
                    // Play the loaded tone with current cursor position
                    audio::play_tone(
                        &app.values,
                        app.use_interactive_mode,
                        app.cursor_x,
                        app.cursor_y,
                        app.envelope_delay_seconds,
                    );
                }
            }
        }
        Ok(None) => {
            // User pressed ESC without selecting, do nothing
        }
        Err(e) => {
            eprintln!("Error loading history entry: {}", e);
        }
    }

    Ok(())
}

pub(crate) fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    config: &Config,
) -> io::Result<()> {
    // 初回描画
    terminal.draw(|f| {
        crate::ui::ui(f, app);
    })?;
    #[cfg(windows)]
    print_sixel_waveform(app)?;

    loop {
        // アップデートが利用可能になったら保存・後始末してループを抜ける
        if app.is_update_available() {
            app.save_to_json()?;
            #[cfg(windows)]
            app.cleanup();
            return Ok(());
        }

        // アイドル検出: 5秒間音色変更がなければsixel波形を生成する
        #[cfg(windows)]
        {
            if app.use_interactive_mode
                && !app.waveform_generating
                && app
                    .sixel_waveform
                    .lock()
                    .ok()
                    .map(|g| g.is_none())
                    .unwrap_or(false)
                && app.last_tone_change.elapsed() >= std::time::Duration::from_secs(5)
            {
                app.waveform_generating = true;
                let sixel_arc = std::sync::Arc::clone(&app.sixel_waveform);
                let expected_gen = app
                    .waveform_generation
                    .load(std::sync::atomic::Ordering::SeqCst);
                let generation_arc = std::sync::Arc::clone(&app.waveform_generation);
                crate::waveform::spawn_waveform_generation(
                    app.values,
                    sixel_arc,
                    expected_gen,
                    generation_arc,
                );
            }
        }

        // イベントをポーリング（タイムアウト付き）。イベントがなければ再描画せずに次ループへ
        if !event::poll(std::time::Duration::from_millis(50))? {
            // sixel生成が完了していたら再描画して表示を更新する。
            // waveform_generating フラグは使わない: 生成カウンタが世代ミスマッチを防ぐため
            // sixel_ready が true なら常に有効な波形が格納されている。
            #[cfg(windows)]
            {
                let sixel_ready = app
                    .sixel_waveform
                    .lock()
                    .ok()
                    .map(|g| g.is_some())
                    .unwrap_or(false);
                if sixel_ready {
                    terminal.draw(|f| {
                        crate::ui::ui(f, app);
                    })?;
                    print_sixel_waveform(app)?;
                }
            }
            continue;
        }

        // イベント処理前の音色データを記録（変更検出用）
        #[cfg(windows)]
        let values_before = app.values;

        match event::read()? {
            Event::Key(key) => {
                // Only process key press and repeat events, ignore release events
                // This follows crossterm/ratatui best practices for avoiding duplicate
                // actions while still supporting key repeat functionality
                if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
                    // Convert key to string for config lookup
                    if let Some(key_string) = key_to_string(key.code, key.modifiers) {
                        // Look up action in config
                        if let Some(action) = config.get_action(&key_string) {
                            match action {
                                Action::DecreaseValue => app.decrease_value(),
                                Action::IncreaseValue => app.increase_value(),
                                Action::SetValueToMax => app.set_value_to_max(),
                                Action::SetValueToMin => app.set_value_to_min(),
                                Action::SetValueToRandom => app.set_value_to_random(),
                                Action::IncreaseValueBy1 => app.increase_value_by(1),
                                Action::IncreaseValueBy2 => app.increase_value_by(2),
                                Action::IncreaseValueBy3 => app.increase_value_by(3),
                                Action::IncreaseValueBy4 => app.increase_value_by(4),
                                Action::IncreaseValueBy5 => app.increase_value_by(5),
                                Action::IncreaseValueBy6 => app.increase_value_by(6),
                                Action::IncreaseValueBy7 => app.increase_value_by(7),
                                Action::IncreaseValueBy8 => app.increase_value_by(8),
                                Action::IncreaseValueBy9 => app.increase_value_by(9),
                                Action::IncreaseValueBy10 => app.increase_value_by(10),
                                Action::DecreaseValueBy1 => app.decrease_value_by(1),
                                Action::DecreaseValueBy2 => app.decrease_value_by(2),
                                Action::DecreaseValueBy3 => app.decrease_value_by(3),
                                Action::DecreaseValueBy4 => app.decrease_value_by(4),
                                Action::DecreaseValueBy5 => app.decrease_value_by(5),
                                Action::DecreaseValueBy6 => app.decrease_value_by(6),
                                Action::DecreaseValueBy7 => app.decrease_value_by(7),
                                Action::DecreaseValueBy8 => app.decrease_value_by(8),
                                Action::DecreaseValueBy9 => app.decrease_value_by(9),
                                Action::DecreaseValueBy10 => app.decrease_value_by(10),
                                Action::PlayCurrentTone => app.play_current_tone(),
                                Action::IncreaseFb => app.increase_fb(),
                                Action::DecreaseFb => app.decrease_fb(),
                                Action::IncreaseAlg => app.increase_alg(),
                                Action::DecreaseAlg => app.decrease_alg(),
                                Action::MoveCursorLeft => app.move_cursor_left(),
                                Action::MoveCursorRight => app.move_cursor_right(),
                                Action::MoveCursorUp => app.move_cursor_up(),
                                Action::MoveCursorDown => app.move_cursor_down(),
                                Action::JumpToOp1AndIncrease => {
                                    app.jump_to_operator_and_increase(0)
                                }
                                Action::JumpToOp2AndIncrease => {
                                    app.jump_to_operator_and_increase(1)
                                }
                                Action::JumpToOp3AndIncrease => {
                                    app.jump_to_operator_and_increase(2)
                                }
                                Action::JumpToOp4AndIncrease => {
                                    app.jump_to_operator_and_increase(3)
                                }
                                Action::JumpToOp1AndDecrease => {
                                    app.jump_to_operator_and_decrease(0)
                                }
                                Action::JumpToOp2AndDecrease => {
                                    app.jump_to_operator_and_decrease(1)
                                }
                                Action::JumpToOp3AndDecrease => {
                                    app.jump_to_operator_and_decrease(2)
                                }
                                Action::JumpToOp4AndDecrease => {
                                    app.jump_to_operator_and_decrease(3)
                                }
                                Action::JumpToArAndIncrease => app.jump_to_ar_and_increase(),
                                Action::JumpToD1rAndIncrease => app.jump_to_d1r_and_increase(),
                                Action::JumpToD2rAndIncrease => app.jump_to_d2r_and_increase(),
                                Action::JumpToRrAndIncrease => app.jump_to_rr_and_increase(),
                                Action::JumpToArAndDecrease => app.jump_to_ar_and_decrease(),
                                Action::JumpToD1rAndDecrease => app.jump_to_d1r_and_decrease(),
                                Action::JumpToD2rAndDecrease => app.jump_to_d2r_and_decrease(),
                                Action::JumpToRrAndDecrease => app.jump_to_rr_and_decrease(),
                                Action::JumpToMulAndIncrease => app.jump_to_mul_and_increase(),
                                Action::JumpToMulAndDecrease => app.jump_to_mul_and_decrease(),
                                Action::JumpToSmAndIncrease => app.jump_to_sm_and_increase(),
                                Action::JumpToSmAndDecrease => app.jump_to_sm_and_decrease(),
                                Action::JumpToTlAndIncrease => app.jump_to_tl_and_increase(),
                                Action::JumpToTlAndDecrease => app.jump_to_tl_and_decrease(),
                                Action::JumpToD1lAndIncrease => app.jump_to_d1l_and_increase(),
                                Action::JumpToD1lAndDecrease => app.jump_to_d1l_and_decrease(),
                                Action::JumpToDtAndIncrease => app.jump_to_dt_and_increase(),
                                Action::JumpToDtAndDecrease => app.jump_to_dt_and_decrease(),
                                Action::JumpToDt2AndIncrease => app.jump_to_dt2_and_increase(),
                                Action::JumpToDt2AndDecrease => app.jump_to_dt2_and_decrease(),
                                Action::JumpToKsAndIncrease => app.jump_to_ks_and_increase(),
                                Action::JumpToKsAndDecrease => app.jump_to_ks_and_decrease(),
                                Action::JumpToAmsAndIncrease => app.jump_to_ams_and_increase(),
                                Action::JumpToAmsAndDecrease => app.jump_to_ams_and_decrease(),
                                Action::JumpToNoteAndIncrease => app.jump_to_note_and_increase(),
                                Action::JumpToNoteAndDecrease => app.jump_to_note_and_decrease(),
                                Action::SaveToGmVariations => {
                                    let _ = app.save_to_gm_variations();
                                }
                                Action::OpenVariationSelector => {
                                    handle_open_variation_selector(terminal, app)?;
                                }
                                Action::OpenHistorySelector => {
                                    handle_open_history_selector(terminal, app)?;
                                }
                                Action::RandomizeTone => app.randomize_tone(),
                                Action::ToggleHelp => app.toggle_help(),
                                Action::Exit => {
                                    // Save tone data to JSON before exiting
                                    app.save_to_json()?;
                                    // Stop interactive mode if active (Windows only)
                                    #[cfg(windows)]
                                    app.cleanup();
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
            Event::Mouse(mouse) => {
                if mouse.kind == MouseEventKind::Moved {
                    // ペンタトニック鍵盤ホバー座標を更新
                    let term_size = terminal.size().unwrap_or(ratatui::prelude::Size {
                        width: 80,
                        height: 24,
                    });
                    // ui.rsのレイアウト計算を再現
                    let inner_x = 1u16; // Block border
                    let inner_y = 1u16;
                    let inner = ratatui::layout::Rect {
                        x: inner_x,
                        y: inner_y,
                        width: term_size.width - 2,
                        height: term_size.height - 2,
                    };
                    let label_offset = 1u16;
                    let ch_row_y = inner.y + label_offset + 5;
                    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
                    let diagram = crate::ui::get_algorithm_diagram(alg_value);
                    let diagram_start_y = ch_row_y + 2;
                    let penta_keyboard_y = diagram_start_y + diagram.len() as u16 + 1;
                    // Only update hover if keyboard is within terminal bounds
                    if penta_keyboard_y < term_size.height - 1 {
                        app.update_hovered_penta_x(
                            mouse.column,
                            mouse.row,
                            inner,
                            penta_keyboard_y,
                        );
                    } else {
                        app.hovered_penta_x = None;
                    }
                    // 旧モード: パラメータ値も更新
                    if app.value_by_mouse_move {
                        app.update_value_from_mouse_x(mouse.column, term_size.width);
                    }
                } else {
                    // Default mode: Handle mouse wheel events at mouse pointer position
                    match mouse.kind {
                        MouseEventKind::ScrollUp => {
                            app.move_cursor_to_mouse_position(mouse.column, mouse.row);
                            app.increase_value();
                        }
                        MouseEventKind::ScrollDown => {
                            app.move_cursor_to_mouse_position(mouse.column, mouse.row);
                            app.decrease_value();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        // 音色データが変更されたらアイドルタイマーをリセットする
        #[cfg(windows)]
        if app.values != values_before {
            app.on_tone_changed();
        }

        // イベント処理後に再描画
        terminal.draw(|f| {
            crate::ui::ui(f, app);
        })?;
        // sixel波形が生成済みなら再描画後に端末へ書き出す
        #[cfg(windows)]
        print_sixel_waveform(app)?;
    }
}

/// Print the sixel waveform to stdout at the envelope display area position.
///
/// Called after each ratatui draw so that the sixel waveform replaces the
/// braille envelope canvas when a waveform has been generated.
///
/// If `app.sixel_waveform` is `None` (generation not yet complete) or the
/// mutex is poisoned the function returns early without printing anything.
///
/// # Terminal compatibility
/// Terminals that do not support sixel will display the raw DCS escape
/// sequence, which is an accepted limitation for this experimental feature.
#[cfg(windows)]
fn print_sixel_waveform(app: &App) -> io::Result<()> {
    use std::io::Write;

    let sixel = {
        match app.sixel_waveform.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => return Ok(()),
        }
    };

    let Some(sixel_str) = sixel else {
        return Ok(());
    };

    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let envelope_y = crate::ui::compute_envelope_area_y(alg_value);

    let mut stdout = io::stdout();
    // カーソルをエンベロープ表示エリアの先頭に移動してsixelを書き出す
    execute!(stdout, crossterm::cursor::MoveTo(0, envelope_y))?;
    stdout.write_all(sixel_str.as_bytes())?;
    stdout.flush()?;

    Ok(())
}

#[cfg(test)]
mod key_to_string_tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_space_maps_to_space_string() {
        let result = key_to_string(KeyCode::Char(' '), KeyModifiers::NONE);
        assert_eq!(result, Some("Space".to_string()));
    }

    #[test]
    fn test_shift_space_maps_to_space_string() {
        let result = key_to_string(KeyCode::Char(' '), KeyModifiers::SHIFT);
        assert_eq!(result, Some("Space".to_string()));
    }

    #[test]
    fn test_regular_char_maps_to_itself() {
        let result = key_to_string(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(result, Some("a".to_string()));
    }

    #[test]
    fn test_function_key_f5_maps_to_f5_string() {
        let result = key_to_string(KeyCode::F(5), KeyModifiers::NONE);
        assert_eq!(result, Some("F5".to_string()));
    }

    #[test]
    fn test_function_key_maps_generically() {
        let result = key_to_string(KeyCode::F(1), KeyModifiers::NONE);
        assert_eq!(result, Some("F1".to_string()));
        let result = key_to_string(KeyCode::F(12), KeyModifiers::NONE);
        assert_eq!(result, Some("F12".to_string()));
    }

    #[test]
    fn test_question_mark_shift_slash_maps_to_question_mark() {
        // On most keyboard layouts, '?' is Shift+/ and crossterm delivers it as Char('?') with SHIFT
        let result = key_to_string(KeyCode::Char('?'), KeyModifiers::SHIFT);
        assert_eq!(result, Some("?".to_string()));
    }

    #[test]
    fn test_question_mark_no_modifier_maps_to_question_mark() {
        let result = key_to_string(KeyCode::Char('?'), KeyModifiers::NONE);
        assert_eq!(result, Some("?".to_string()));
    }
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
cc89f48 Merge pull request #225 from cat2151/copilot/refactor-large-file-mod-rs
95a9472 refactor: extract draw_keybind_hints and draw_help_dialog into src/ui/help.rs
d9cc5ad Initial plan
2be951e Merge pull request #222 from cat2151/copilot/update-q-to-quit
24652d0 Clarify desired file location for issue #224
2c4a2be Add issue note for #224 [auto]
ca8544f Remove q/e decrement/increment keybinds and make q quit
f9b2e46 Add issue note for #223 [auto]
ddb3aa2 Initial plan
9869bba Merge pull request #217 from cat2151/copilot/test-drawing-waveform-with-sixel

### 変更されたファイル:
generated-docs/development-status-generated-prompt.md
generated-docs/development-status.md
generated-docs/project-overview-generated-prompt.md
generated-docs/project-overview.md
issue-notes/223.md
issue-notes/224.md
src/app/mod.rs
src/app_init.rs
src/config.rs
src/event_loop.rs
src/main.rs
src/ui/help.rs
src/ui/helpers.rs
src/ui/mod.rs
src/waveform.rs
ym2151-tone-editor.toml.example


---
Generated at: 2026-03-18 07:15:42 JST
