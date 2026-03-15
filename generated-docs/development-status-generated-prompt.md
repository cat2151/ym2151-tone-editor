Last updated: 2026-03-16

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
- issue-notes/176.md
- issue-notes/177.md
- issue-notes/210.md
- issue-notes/212.md
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
- src/ui/helpers.rs
- src/ui/mod.rs
- src/updater.rs
- src/variation_selector.rs
- tones/general_midi/000_AcousticGrand.json
- tones/general_midi/tone_names.json
- wasm/Cargo.lock
- wasm/Cargo.toml
- wasm/src/lib.rs
- ym2151-tone-editor.toml.example

## 現在のオープンIssues
## [Issue #213](../issue-notes/213.md): 大きなファイルの検出: 1個のファイルが500行を超えています
以下のファイルが500行を超えています。リファクタリングを検討してください。

## 検出されたファイル

| ファイル | 行数 | 超過行数 |
|---------|------|----------|
| `core/src/lib.rs` | 519 | +19 |

## テスト実施のお願い

- リファクタリング前後にテストを実行し、それぞれのテスト失敗件数を報告してください
- リファクタリング前後のどちらかでテストがredの場合、まず別issueでtest greenにしてからリファクタリングしてください

## 推奨事項

1. 単一責任の原則に従い、ファイルを分割する
2...
ラベル: refactoring, code-quality, automated
--- issue-notes/213.md の内容 ---

```markdown

```

## [Issue #212](../issue-notes/212.md): ランダム音色機能のWASM版が、bluesky-text-to-audioリポジトリで、利用できない状態になってしまっている
[issue-notes/212.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/212.md)

...
ラベル: 
--- issue-notes/212.md の内容 ---

```markdown
# issue ランダム音色機能のWASM版が、bluesky-text-to-audioリポジトリで、利用できない状態になってしまっている #212
[issues #212](https://github.com/cat2151/ym2151-tone-editor/issues/212)



```

## [Issue #177](../issue-notes/177.md): sixelを使って音色波形を描画できるか試し、UXを検証する。音色変更後5秒したらbackgroundでcat-play-mmlをwav生成モードで呼び出してwavを得て表示
[issue-notes/177.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/177.md)

...
ラベル: 
--- issue-notes/177.md の内容 ---

```markdown
# issue sixelを使って音色波形を描画できるか試し、UXを検証する。音色変更後5秒したらbackgroundでcat-play-mmlをwav生成モードで呼び出してwavを得て表示 #177
[issues #177](https://github.com/cat2151/ym2151-tone-editor/issues/177)



```

## [Issue #176](../issue-notes/176.md): sixelを使って各OPごとのエンベロープを描画できるか試し、UXを検証する
[issue-notes/176.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/176.md)

...
ラベル: 
--- issue-notes/176.md の内容 ---

```markdown
# issue sixelを使って各OPごとのエンベロープを描画できるか試し、UXを検証する #176
[issues #176](https://github.com/cat2151/ym2151-tone-editor/issues/176)



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
### .github/actions-tmp/issue-notes/12.md
```md
{% raw %}
# issue project-summary を他projectから使いやすくする #12
[issues #12](https://github.com/cat2151/github-actions/issues/12)

# 保留、別projectでの検証待ちのもの
- promptsをcall側ymlで指定可能にする
  - 保留の理由
    - YAGNI原則
      - 現状の共通workflow側のpromptsで問題ないうちは、保留とする
        - そのままで使える可能性が高い見込み
      - 検証が必要
      - 別promptsを実際に書く必要が出たときに、追加実装をする
# 課題、 docs/ をメンテする
- 対象は、 daily-summary-setup.md
- call-daily-project-summary.yml の導入手順を書く
- どうする？
  - 次の日次バッチでagent用promptを生成させる
- 結果
  - 生成させた
  - 導入手順をメンテさせた
  - 人力でさらにメンテした
  - これでOKと判断する。
  - あとは必要に応じてissue起票すればよい、今すぐのissue起票は不要（YAGNI原則）、と判断する

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/13.md
```md
{% raw %}
# issue issue-note を他projectから使いやすくする #13
[issues #13](https://github.com/cat2151/github-actions/issues/13)

- docs
    - call導入手順を書く

{% endraw %}
```

### .github/actions-tmp/issue-notes/2.md
```md
{% raw %}
# issue GitHub Actions「関数コールグラフhtmlビジュアライズ生成」を共通ワークフロー化する #2
[issues #2](https://github.com/cat2151/github-actions/issues/2)


# prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlファイルを、以下の2つのファイルに分割してください。
1. 共通ワークフロー       cat2151/github-actions/.github/workflows/callgraph_enhanced.yml
2. 呼び出し元ワークフロー cat2151/github-actions/.github/workflows/call-callgraph_enhanced.yml
まずplanしてください
```

# 結果
- indent
    - linter？がindentのエラーを出しているがyml内容は見た感じOK
    - テキストエディタとagentの相性問題と判断する
    - 別のテキストエディタでsaveしなおし、テキストエディタをreload
    - indentのエラーは解消した
- LLMレビュー
    - agent以外の複数のLLMにレビューさせる
    - prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
以下の2つのファイルをレビューしてください。最優先で、エラーが発生するかどうかだけレビューしてください。エラー以外の改善事項のチェックをするかわりに、エラー発生有無チェックに最大限注力してください。

--- 共通ワークフロー

# GitHub Actions Reusable Workflow for Call Graph Generation
name: Generate Call Graph

# TODO Windowsネイティブでのtestをしていた名残が残っているので、今後整理していく。今はWSL act でtestしており、Windowsネイティブ環境依存問題が解決した
#  ChatGPTにレビューさせるとそこそこ有用そうな提案が得られたので、今後それをやる予定
#  agentに自己チェックさせる手も、セカンドオピニオンとして選択肢に入れておく

on:
  workflow_call:

jobs:
  check-commits:
    runs-on: ubuntu-latest
    outputs:
      should-run: ${{ steps.check.outputs.should-run }}
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 50 # 過去のコミットを取得

      - name: Check for user commits in last 24 hours
        id: check
        run: |
          node .github/scripts/callgraph_enhanced/check-commits.cjs

  generate-callgraph:
    needs: check-commits
    if: needs.check-commits.outputs.should-run == 'true'
    runs-on: ubuntu-latest
    permissions:
      contents: write
      security-events: write
      actions: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Set Git identity
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

      - name: Remove old CodeQL packages cache
        run: rm -rf ~/.codeql/packages

      - name: Check Node.js version
        run: |
          node .github/scripts/callgraph_enhanced/check-node-version.cjs

      - name: Install CodeQL CLI
        run: |
          wget https://github.com/github/codeql-cli-binaries/releases/download/v2.22.1/codeql-linux64.zip
          unzip codeql-linux64.zip
          sudo mv codeql /opt/codeql
          echo "/opt/codeql" >> $GITHUB_PATH

      - name: Install CodeQL query packs
        run: |
          /opt/codeql/codeql pack install .github/codeql-queries

      - name: Check CodeQL exists
        run: |
          node .github/scripts/callgraph_enhanced/check-codeql-exists.cjs

      - name: Verify CodeQL Configuration
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs verify-config

      - name: Remove existing CodeQL DB (if any)
        run: |
          rm -rf codeql-db

      - name: Perform CodeQL Analysis
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs analyze

      - name: Check CodeQL Analysis Results
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs check-results

      - name: Debug CodeQL execution
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs debug

      - name: Wait for CodeQL results
        run: |
          node -e "setTimeout(()=>{}, 10000)"

      - name: Find and process CodeQL results
        run: |
          node .github/scripts/callgraph_enhanced/find-process-results.cjs

      - name: Generate HTML graph
        run: |
          node .github/scripts/callgraph_enhanced/generate-html-graph.cjs

      - name: Copy files to generated-docs and commit results
        run: |
          node .github/scripts/callgraph_enhanced/copy-commit-results.cjs

--- 呼び出し元
# 呼び出し元ワークフロー: call-callgraph_enhanced.yml
name: Call Call Graph Enhanced

on:
  schedule:
    # 毎日午前5時(JST) = UTC 20:00前日
    - cron: '0 20 * * *'
  workflow_dispatch:

jobs:
  call-callgraph-enhanced:
    # uses: cat2151/github-actions/.github/workflows/callgraph_enhanced.yml
    uses: ./.github/workflows/callgraph_enhanced.yml # ローカルでのテスト用
```

# レビュー結果OKと判断する
- レビュー結果を人力でレビューした形になった

# test
- #4 同様にローカル WSL + act でtestする
- エラー。userのtest設計ミス。
  - scriptの挙動 : src/ がある前提
  - 今回の共通ワークフローのリポジトリ : src/ がない
  - 今回testで実現したいこと
    - 仮のソースでよいので、関数コールグラフを生成させる
  - 対策
    - src/ にダミーを配置する
- test green
  - ただしcommit pushはしてないので、html内容が0件NG、といったケースの検知はできない
  - もしそうなったら別issueとしよう

# test green

# commit用に、yml 呼び出し元 uses をlocal用から本番用に書き換える

# closeとする
- もしhtml内容が0件NG、などになったら、別issueとするつもり

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

### core/src/lib.rs
```rs
{% raw %}
//! Platform-independent YM2151 FM tone parameter model and register encoding.
//!
//! This crate is the **Single Source of Truth** for the parameter-level YM2151
//! logic shared between the native TUI application (`ym2151-tone-editor`) and the
//! WASM bindings (`ym2151-wasm`).  It covers FM tone data structures, constants,
//! random tone generation, MIDI pitch conversion, and register hex encoding.
//!
//! It has **no external dependencies** and compiles for any target including
//! `wasm32-unknown-unknown`.

use core::fmt::Write as _;

// ---------------------------------------------------------------------------
// Grid dimensions
// ---------------------------------------------------------------------------

/// Number of parameter columns in the tone grid (one per operator parameter).
pub const GRID_WIDTH: usize = 12;

/// Number of rows in the tone grid (four operators + one channel row).
pub const GRID_HEIGHT: usize = 5;

// ---------------------------------------------------------------------------
// Operator parameter column indices
// (order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS)
// ---------------------------------------------------------------------------

pub const PARAM_SM: usize = 0;
pub const PARAM_TL: usize = 1;
pub const PARAM_MUL: usize = 2;
pub const PARAM_AR: usize = 3;
pub const PARAM_D1R: usize = 4;
pub const PARAM_D1L: usize = 5;
pub const PARAM_D2R: usize = 6;
pub const PARAM_RR: usize = 7;
pub const PARAM_DT: usize = 8;
pub const PARAM_DT2: usize = 9;
pub const PARAM_KS: usize = 10;
pub const PARAM_AMS: usize = 11;

// ---------------------------------------------------------------------------
// Channel-row parameter column indices
// ---------------------------------------------------------------------------

pub const CH_PARAM_ALG: usize = 0;
pub const CH_PARAM_FB: usize = 1;
pub const CH_PARAM_NOTE: usize = 2;

/// Row index for the channel settings row (below the four operator rows).
pub const ROW_CH: usize = 4;

// ---------------------------------------------------------------------------
// Parameter maximum values (respecting YM2151 bit-field widths)
// (order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS)
// ---------------------------------------------------------------------------

pub const PARAM_MAX: [u8; GRID_WIDTH] = [
    1,  // SM (SlotMask): 0 or 1
    99, // TL: 7 bits (0-127, displayed limit 99)
    15, // MUL: 4 bits (0-15)
    31, // AR: 5 bits (0-31)
    31, // D1R: 5 bits (0-31)
    15, // D1L: 4 bits (0-15)
    15, // D2R: 4 bits (0-15)
    15, // RR: 4 bits (0-15)
    7,  // DT: 3 bits (0-7)
    3,  // DT2: 2 bits (0-3)
    3,  // KS: 2 bits (0-3)
    3,  // AMS: 2 bits (0-3)
];

// ---------------------------------------------------------------------------
// Tone data type
// ---------------------------------------------------------------------------

/// A 5×12 grid of `u8` values representing all tone parameters.
///
/// Rows 0-3 hold operator parameters; row 4 (`ROW_CH`) holds channel settings.
pub type ToneData = [[u8; GRID_WIDTH]; GRID_HEIGHT];

// ---------------------------------------------------------------------------
// Hardware register layout constants
// ---------------------------------------------------------------------------

/// Maps the user-facing operator index (O1-O4) to the YM2151 hardware register
/// slot index.  The hardware order is O1, O3, O2, O4.
pub const REG_FROM_O1_O4: [usize; 4] = [0, 2, 1, 3];

// ---------------------------------------------------------------------------
// Random tone configuration (based on web-ym2151 `getDefaultConfig`)
// ---------------------------------------------------------------------------

/// Which operators are carriers for each ALG value (0-7).
pub const CARRIERS_PER_ALG: [[bool; 4]; 8] = [
    [false, false, false, true], // ALG=0: OP4 only
    [false, false, false, true], // ALG=1: OP4 only
    [false, false, false, true], // ALG=2: OP4 only
    [false, false, false, true], // ALG=3: OP4 only
    [false, false, true, true],  // ALG=4: OP3, OP4
    [false, true, true, true],   // ALG=5: OP2, OP3, OP4
    [false, true, true, true],   // ALG=6: OP2, OP3, OP4
    [true, true, true, true],    // ALG=7: all OPs
];

/// Modulator TL value per ALG value (`stage_count × 0x08`).
pub const MODULATOR_TL_PER_ALG: [u8; 8] = [
    0x20, // ALG=0: 4 stages
    0x20, // ALG=1: 4 stages
    0x20, // ALG=2: 4 stages
    0x20, // ALG=3: 4 stages
    0x18, // ALG=4: 3 stages
    0x10, // ALG=5: 2 stages
    0x10, // ALG=6: 2 stages
    0x00, // ALG=7: no external modulators
];

// ---------------------------------------------------------------------------
// Seed-based LCG pseudo-random number generator
// ---------------------------------------------------------------------------

/// A simple Linear Congruential Generator seeded by the caller.
///
/// This design avoids any dependency on `std::time::SystemTime` (unavailable
/// in bare WASM) while still being reproducible for testing.
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    /// Create a new RNG from an externally supplied seed.
    ///
    /// In a browser context, pass `Date.now()` to get a different tone on each
    /// call.  In the native app, pass a value derived from `SystemTime`.
    pub fn from_seed(seed: u64) -> Self {
        // Mix the seed to avoid trivial fixed-points for seed=0 or seed=1.
        let state = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        SimpleRng { state }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    pub fn range(&mut self, min: u8, max: u8) -> u8 {
        if min >= max {
            return min;
        }
        let span = (max - min) as u64 + 1;
        min + (self.next_u64() % span) as u8
    }
}

// ---------------------------------------------------------------------------
// Random tone generation
// ---------------------------------------------------------------------------

/// Generate a random YM2151 tone from a caller-supplied seed.
///
/// The parameter ranges are fixed to the defaults from `web-ym2151`
/// `getDefaultConfig()`:
///
/// | Param | Range |
/// |-------|-------|
/// | ALG   | 0–7   |
/// | FB    | 0–7   |
/// | AR    | 5–31  |
/// | D1R   | 0–9   |
/// | MUL   | 0–15  |
/// | DT    | 0–7   |
/// | KS    | 0–3   |
///
/// All other parameters are set to fixed values (D1L=15, D2R=0, RR=0, DT2=0,
/// AMS=0, SM=1).  Carrier TL is always 0; modulator TL is determined by ALG.
///
/// # Parameters
/// - `seed`: An externally supplied 64-bit seed.  Different seeds produce
///   different tones; the same seed always produces the same tone.
/// - `current_note`: MIDI note number (0–127) stored in the channel row.
pub fn generate_random_tone_with_seed(seed: u64, current_note: u8) -> ToneData {
    let mut rng = SimpleRng::from_seed(seed);
    let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];

    let alg = rng.range(0, 7);
    let modulator_tl = MODULATOR_TL_PER_ALG[alg as usize];

    for (op, row) in values.iter_mut().take(4).enumerate() {
        let is_carrier = CARRIERS_PER_ALG[alg as usize][op];

        row[PARAM_SM] = 1;
        row[PARAM_TL] = if is_carrier {
            0
        } else {
            modulator_tl.min(PARAM_MAX[PARAM_TL])
        };
        row[PARAM_MUL] = rng.range(0, 15);
        row[PARAM_AR] = rng.range(5, 31);
        row[PARAM_D1R] = rng.range(0, 9);
        row[PARAM_D1L] = 15;
        row[PARAM_D2R] = 0;
        row[PARAM_RR] = 0;
        row[PARAM_DT] = rng.range(0, 7);
        row[PARAM_DT2] = 0;
        row[PARAM_KS] = rng.range(0, 3);
        row[PARAM_AMS] = 0;
    }

    values[ROW_CH][CH_PARAM_ALG] = alg;
    values[ROW_CH][CH_PARAM_FB] = rng.range(0, 7);
    values[ROW_CH][CH_PARAM_NOTE] = current_note;

    values
}

// ---------------------------------------------------------------------------
// MIDI → YM2151 pitch conversion
// ---------------------------------------------------------------------------

/// Convert a MIDI note number (0–127) to a YM2151 `(KC, KF)` pair.
///
/// The conversion mirrors `smf_to_ym2151log::midi::midi_to_kc_kf`, including
/// the octave offset and the 14-value YM2151 note encoding.  `KF` is always 0
/// (no fine tuning), matching the native app behaviour.
pub fn midi_to_kc_kf(midi_note: u8) -> (u8, u8) {
    // YM2151 encodes 12 semitones as 14 values (0–14, skipping 3, 7, 11).
    const NOTE_MAP: [u8; 12] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14];

    // Subtract 1 to align MIDI octave numbering with YM2151 octave numbering.
    let adjusted = if midi_note > 0 { midi_note - 1 } else { 0 };
    let note_in_octave = (adjusted % 12) as usize;
    let ym_octave = ((adjusted / 12) as i8 - 2).clamp(0, 7) as u8;
    let kc_note = NOTE_MAP[note_in_octave];

    ((ym_octave << 4) | kc_note, 0)
}

// ---------------------------------------------------------------------------
// Register hex encoding
// ---------------------------------------------------------------------------

/// Encode a tone grid as a register hex string.
///
/// The output is a sequence of `AADD` pairs (2-digit uppercase hex address
/// followed by 2-digit uppercase hex data) that can be sent directly to the
/// YM2151 chip or consumed by `web-ym2151`.  This is the same format produced
/// by `editor_rows_to_registers` in the native app.
pub fn editor_rows_to_registers(values: &ToneData) -> String {
    // Pre-allocate: 6 operator regs × 4 ops + RL/FB/CON + KC + KF + KEY_ON = 28 pairs × 4 chars
    let mut result = String::with_capacity(28 * 4);

    let channel: usize = 0;

    for row_id in 0..4 {
        let op_offset = REG_FROM_O1_O4[row_id] * 8 + channel;

        // DT1 (bits 6-4) and MUL (bits 3-0) – Register $40-$5F
        let dt_mul = ((values[row_id][PARAM_DT] & 0x07) << 4) | (values[row_id][PARAM_MUL] & 0x0F);
        push_reg_pair(&mut result, (0x40 + op_offset) as u8, dt_mul);

        // TL – Register $60-$7F
        push_reg_pair(
            &mut result,
            (0x60 + op_offset) as u8,
            values[row_id][PARAM_TL] & 0x7F,
        );

        // KS (bits 7-6) and AR (bits 4-0) – Register $80-$9F
        let ks_ar = ((values[row_id][PARAM_KS] & 0x03) << 6) | (values[row_id][PARAM_AR] & 0x1F);
        push_reg_pair(&mut result, (0x80 + op_offset) as u8, ks_ar);

        // AMS (bits 7-6) and D1R (bits 4-0) – Register $A0-$BF
        let ams_d1r =
            ((values[row_id][PARAM_AMS] & 0x03) << 6) | (values[row_id][PARAM_D1R] & 0x1F);
        push_reg_pair(&mut result, (0xA0 + op_offset) as u8, ams_d1r);

        // DT2 (bits 7-6) and D2R (bits 3-0) – Register $C0-$DF
        let dt2_d2r =
            ((values[row_id][PARAM_DT2] & 0x03) << 6) | (values[row_id][PARAM_D2R] & 0x0F);
        push_reg_pair(&mut result, (0xC0 + op_offset) as u8, dt2_d2r);

        // D1L (bits 7-4) and RR (bits 3-0) – Register $E0-$FF
        let d1l_rr = ((values[row_id][PARAM_D1L] & 0x0F) << 4) | (values[row_id][PARAM_RR] & 0x0F);
        push_reg_pair(&mut result, (0xE0 + op_offset) as u8, d1l_rr);
    }

    // RL/FB/CON – Register $20-$27
    let alg = values[ROW_CH][CH_PARAM_ALG];
    let fb = values[ROW_CH][CH_PARAM_FB];
    let rl_fb_con = 0xC0u8 | ((fb & 0x07) << 3) | (alg & 0x07);
    push_reg_pair(&mut result, (0x20 + channel) as u8, rl_fb_con);

    // KC (Key Code) and KF (Key Fraction) – Registers $28-$2F and $30-$37
    let midi_note = values[ROW_CH][CH_PARAM_NOTE];
    let (kc, kf) = midi_to_kc_kf(midi_note);
    push_reg_pair(&mut result, (0x28 + channel) as u8, kc);
    push_reg_pair(&mut result, (0x30 + channel) as u8, kf);

    // Key On – Register $08
    let slot_mask = if values[0][PARAM_SM] != 0 { 0x08u8 } else { 0 }
        | if values[1][PARAM_SM] != 0 { 0x10 } else { 0 }
        | if values[2][PARAM_SM] != 0 { 0x20 } else { 0 }
        | if values[3][PARAM_SM] != 0 { 0x40 } else { 0 };
    push_reg_pair(&mut result, 0x08, slot_mask | channel as u8);

    result
}

#[inline]
fn push_reg_pair(out: &mut String, addr: u8, data: u8) {
    write!(out, "{:02X}{:02X}", addr, data).unwrap();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // SimpleRng
    // ------------------------------------------------------------------

    #[test]
    fn test_simple_rng_deterministic() {
        let mut a = SimpleRng::from_seed(42);
        let mut b = SimpleRng::from_seed(42);
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn test_simple_rng_range_in_bounds() {
        let mut rng = SimpleRng::from_seed(999);
        for _ in 0..100 {
            let v = rng.range(5, 31);
            assert!((5..=31).contains(&v));
        }
    }

    #[test]
    fn test_simple_rng_range_equal_min_max() {
        let mut rng = SimpleRng::from_seed(1);
        assert_eq!(rng.range(7, 7), 7);
    }

    // ------------------------------------------------------------------
    // midi_to_kc_kf
    // ------------------------------------------------------------------

    #[test]
    fn test_midi_to_kc_kf_kf_always_zero() {
        for note in 0u8..=127 {
            let (_, kf) = midi_to_kc_kf(note);
            assert_eq!(kf, 0, "KF must always be 0 for MIDI note {}", note);
        }
    }

    #[test]
    fn test_midi_to_kc_kf_middle_c() {
        let (kc, _) = midi_to_kc_kf(60);
        // Middle C (MIDI 60) → adjusted=59 → octave=4, note=11 → YM2151 octave=(4-2)=2, kc_note=NOTE_MAP[11]=14
        assert_eq!(
            kc,
            (2 << 4) | 14,
            "Unexpected KC for middle C: 0x{:02X}",
            kc
        );
    }

    #[test]
    fn test_midi_to_kc_kf_a4() {
        let (kc, _) = midi_to_kc_kf(69);
        // A4 (MIDI 69) → adjusted=68 → octave=5, note=8 → YM2151 octave=(5-2)=3, kc_note=NOTE_MAP[8]=10
        assert_eq!(kc, (3 << 4) | 10, "Unexpected KC for A4: 0x{:02X}", kc);
    }

    #[test]
    fn test_midi_to_kc_kf_kc_within_valid_range() {
        for note in 0u8..=127 {
            let (kc, _) = midi_to_kc_kf(note);
            // YM2151 KC max: octave=7 (bits 6-4), note=14 (bits 3-0) → 0x7E
            assert!(
                kc <= 0x7E,
                "KC out of valid YM2151 range for MIDI {}: 0x{:02X}",
                note,
                kc
            );
        }
    }

    // ------------------------------------------------------------------
    // generate_random_tone_with_seed
    // ------------------------------------------------------------------

    #[test]
    fn test_generate_random_tone_with_seed_deterministic() {
        let a = generate_random_tone_with_seed(42, 69);
        let b = generate_random_tone_with_seed(42, 69);
        assert_eq!(a, b);
    }

    #[test]
    fn test_generate_random_tone_with_seed_different_seeds() {
        let a = generate_random_tone_with_seed(1, 69);
        let b = generate_random_tone_with_seed(2, 69);
        assert_ne!(a, b);
    }

    #[test]
    fn test_generate_random_tone_with_seed_note_preserved() {
        let tone = generate_random_tone_with_seed(0, 60);
        assert_eq!(tone[ROW_CH][CH_PARAM_NOTE], 60);
    }

    #[test]
    fn test_generate_random_tone_with_seed_alg_in_range() {
        let tone = generate_random_tone_with_seed(12345, 69);
        assert!(tone[ROW_CH][CH_PARAM_ALG] <= 7);
        assert!(tone[ROW_CH][CH_PARAM_FB] <= 7);
    }

    #[test]
    fn test_generate_random_tone_with_seed_carrier_tl_zero() {
        let tone = generate_random_tone_with_seed(7777, 69);
        let alg = tone[ROW_CH][CH_PARAM_ALG] as usize;
        for (op, row) in tone.iter().take(4).enumerate() {
            if CARRIERS_PER_ALG[alg][op] {
                assert_eq!(
                    row[PARAM_TL],
                    0,
                    "Carrier OP{} TL must be 0 for ALG={}",
                    op + 1,
                    alg
                );
            } else {
                let expected = MODULATOR_TL_PER_ALG[alg];
                assert_eq!(
                    row[PARAM_TL],
                    expected,
                    "Modulator OP{} TL mismatch for ALG={}",
                    op + 1,
                    alg
                );
            }
        }
    }

    #[test]
    fn test_generate_random_tone_with_seed_ar_in_range() {
        let tone = generate_random_tone_with_seed(42, 60);
        for (op, row) in tone.iter().take(4).enumerate() {
            assert!(
                (5..=31).contains(&row[PARAM_AR]),
                "AR out of range for OP{}: {}",
                op + 1,
                row[PARAM_AR]
            );
        }
    }

    // ------------------------------------------------------------------
    // editor_rows_to_registers
    // ------------------------------------------------------------------

    #[test]
    fn test_editor_rows_to_registers_hex_format() {
        let tone = generate_random_tone_with_seed(12345, 69);
        let regs = editor_rows_to_registers(&tone);
        assert!(!regs.is_empty());
        assert_eq!(
            regs.len() % 4,
            0,
            "Register string must be multiple of 4 chars"
        );
        assert!(
            regs.chars().all(|c| c.is_ascii_hexdigit()),
            "All chars must be hex digits: {}",
            regs
        );
    }

    #[test]
    fn test_editor_rows_to_registers_includes_kc_and_kf() {
        let tone = generate_random_tone_with_seed(1234, 69);
        let regs = editor_rows_to_registers(&tone);
        let chars: Vec<char> = regs.chars().collect();
        let mut found_kc = false;
        let mut found_kf = false;
        for chunk in chars.chunks(4) {
            let addr = u8::from_str_radix(&chunk[0..2].iter().collect::<String>(), 16).unwrap();
            let data = u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap();
            if (0x28..=0x2F).contains(&addr) {
                found_kc = true;
                assert!(data <= 0x7E, "KC out of range: 0x{:02X}", data);
            }
            if (0x30..=0x37).contains(&addr) {
                found_kf = true;
                assert_eq!(data, 0, "KF must be 0, got 0x{:02X}", data);
            }
        }
        assert!(found_kc, "KC register (0x28) missing from output");
        assert!(found_kf, "KF register (0x30) missing from output");
    }

    #[test]
    fn test_editor_rows_to_registers_deterministic() {
        let tone = generate_random_tone_with_seed(999, 60);
        let r1 = editor_rows_to_registers(&tone);
        let r2 = editor_rows_to_registers(&tone);
        assert_eq!(r1, r2);
    }
}

{% endraw %}
```

### wasm/src/lib.rs
```rs
{% raw %}
//! WASM bindings for YM2151 tone generation.
//!
//! This crate is a thin `wasm-bindgen` wrapper around [`ym2151-tone-params`].  All
//! logic (random tone generation, register encoding, MIDI pitch conversion)
//! lives in `ym2151-tone-params` as the Single Source of Truth shared with the native
//! TUI application.
//!
//! # Usage from TypeScript
//! ```typescript
//! import init, { generate_random_tone_registers } from './ym2151_wasm.js';
//! await init();
//! const seed = Date.now();
//! const registers = generate_random_tone_registers(seed, 69); // 69 = A4, MIDI note 0–127
//! // registers: hex string e.g. "4000600080001F..."
//! ```

use wasm_bindgen::prelude::*;

/// Generate a random YM2151 tone and return it as a register hex string.
///
/// # Parameters
/// - `seed`: A numeric seed for the random number generator.  Pass `Date.now()`
///   from TypeScript to get a different tone on each call.
/// - `current_note`: MIDI note number (0–127) to embed in the tone data.
///   Pass 69 for A4 (concert pitch) if unsure.
///
/// # Returns
/// A hex string of register address/data pairs (4 chars each, e.g. `"4000..."`).
/// This is the same format used by `editor_rows_to_registers` in the native app.
#[wasm_bindgen]
pub fn generate_random_tone_registers(seed: f64, current_note: u8) -> String {
    let seed_u64 = seed.abs() as u64;
    let tone = ym2151_tone_params::generate_random_tone_with_seed(seed_u64, current_note);
    ym2151_tone_params::editor_rows_to_registers(&tone)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Decode a register hex string back into (alg, fb, ar[4], tl[4]).
    fn decode_registers(regs: &str) -> (u8, u8, [u8; 4], [u8; 4]) {
        let chars: Vec<char> = regs.chars().collect();
        assert!(chars.len().is_multiple_of(4));
        let mut alg = 0u8;
        let mut fb = 0u8;
        let mut ar = [0u8; 4];
        let mut tl = [0u8; 4];
        for chunk in chars.chunks(4) {
            let addr = u8::from_str_radix(&chunk[0..2].iter().collect::<String>(), 16).unwrap();
            let data = u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap();
            match addr {
                0x20..=0x27 => {
                    alg = data & 0x07;
                    fb = (data >> 3) & 0x07;
                }
                0x60..=0x7F => {
                    let reg = ((addr - 0x60) / 8) as usize;
                    if reg < 4 {
                        let op = [0usize, 2, 1, 3][reg];
                        tl[op] = data & 0x7F;
                    }
                }
                0x80..=0x9F => {
                    let reg = ((addr - 0x80) / 8) as usize;
                    if reg < 4 {
                        let op = [0usize, 2, 1, 3][reg];
                        ar[op] = data & 0x1F;
                    }
                }
                _ => {}
            }
        }
        (alg, fb, ar, tl)
    }

    #[test]
    fn test_generate_random_tone_registers_returns_hex_string() {
        let result = generate_random_tone_registers(12345.0, 69);
        assert!(!result.is_empty());
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(result.len() % 4, 0);
    }

    #[test]
    fn test_generate_random_tone_registers_alg_in_range() {
        let result = generate_random_tone_registers(99999.0, 69);
        let (alg, fb, _, _) = decode_registers(&result);
        assert!(alg <= 7);
        assert!(fb <= 7);
    }

    #[test]
    fn test_generate_random_tone_registers_ar_in_range() {
        let result = generate_random_tone_registers(42.0, 60);
        let (_, _, ar, _) = decode_registers(&result);
        for (op, &v) in ar.iter().enumerate() {
            assert!(
                (5..=31).contains(&v),
                "AR for op {} out of range: {}",
                op,
                v
            );
        }
    }

    #[test]
    fn test_generate_random_tone_registers_carrier_tl_zero() {
        use ym2151_tone_params::{CARRIERS_PER_ALG, MODULATOR_TL_PER_ALG};
        let result = generate_random_tone_registers(7777.0, 69);
        let (alg, _, _, tl) = decode_registers(&result);
        for (op, &v) in tl.iter().enumerate() {
            if CARRIERS_PER_ALG[alg as usize][op] {
                assert_eq!(v, 0);
            } else {
                assert_eq!(v, MODULATOR_TL_PER_ALG[alg as usize]);
            }
        }
    }

    #[test]
    fn test_different_seeds_produce_different_results() {
        let r1 = generate_random_tone_registers(1.0, 69);
        let r2 = generate_random_tone_registers(2.0, 69);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_same_seed_produces_same_result() {
        let r1 = generate_random_tone_registers(100.0, 69);
        let r2 = generate_random_tone_registers(100.0, 69);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_note_is_embedded_in_kc_and_kf_registers() {
        let result = generate_random_tone_registers(1234.0, 69);
        let chars: Vec<char> = result.chars().collect();
        let mut found_kc = false;
        let mut found_kf = false;
        for chunk in chars.chunks(4) {
            let addr = u8::from_str_radix(&chunk[0..2].iter().collect::<String>(), 16).unwrap();
            let data = u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap();
            if (0x28..=0x2F).contains(&addr) {
                found_kc = true;
                assert!(
                    data <= 0x7E,
                    "KC should be in valid YM2151 range (max 0x7E), got 0x{:02X}",
                    data
                );
            }
            if (0x30..=0x37).contains(&addr) {
                found_kf = true;
                assert_eq!(data, 0);
            }
        }
        assert!(found_kc, "KC register (0x28) not found");
        assert!(found_kf, "KF register (0x30) not found");
    }

    #[test]
    fn test_seed_zero_does_not_panic() {
        let result = generate_random_tone_registers(0.0, 60);
        assert!(!result.is_empty());
    }
}

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

### issue-notes/176.md
```md
{% raw %}
# issue sixelを使って各OPごとのエンベロープを描画できるか試し、UXを検証する #176
[issues #176](https://github.com/cat2151/ym2151-tone-editor/issues/176)



{% endraw %}
```

### issue-notes/177.md
```md
{% raw %}
# issue sixelを使って音色波形を描画できるか試し、UXを検証する。音色変更後5秒したらbackgroundでcat-play-mmlをwav生成モードで呼び出してwavを得て表示 #177
[issues #177](https://github.com/cat2151/ym2151-tone-editor/issues/177)



{% endraw %}
```

### issue-notes/212.md
```md
{% raw %}
# issue ランダム音色機能のWASM版が、bluesky-text-to-audioリポジトリで、利用できない状態になってしまっている #212
[issues #212](https://github.com/cat2151/ym2151-tone-editor/issues/212)



{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
4a3478e Merge pull request #211 from cat2151/copilot/help-dialog-grouping-display
9515ddd fix: dynamic dialog width, default-keybinds note, fix stale comment
3122b5a Add issue note for #212 [auto]
0766675 feat: replace help text with centered grouped dialog overlay
4cd8d21 Add issue note for #210 [auto]
bf4cb57 Initial plan
39649cc Auto-translate README.ja.md to README.md [auto]
0294e88 Add usage section for random tone color library
51a965b Merge pull request #209 from cat2151/copilot/fix-deploy-to-github-pages
a239a81 Fix: use chown instead of chmod to transfer _site ownership to runner user

### 変更されたファイル:
.github/workflows/deploy-demo-library.yml
README.ja.md
README.md
issue-notes/206.md
issue-notes/210.md
issue-notes/212.md
src/ui/mod.rs


---
Generated at: 2026-03-16 07:11:17 JST
