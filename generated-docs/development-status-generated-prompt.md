Last updated: 2025-12-11

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
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-check.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
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
- .github/actions-tmp/issue-notes/4.md
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/copilot-instructions.md
- .github/workflows/call-daily-project-summary.yml
- .github/workflows/call-issue-note.yml
- .github/workflows/call-rust-windows-check.yml
- .github/workflows/call-translate-readme.yml
- .github/workflows/rust-test.yml
- .gitignore
- Cargo.lock
- Cargo.toml
- LICENSE
- README.ja.md
- README.md
- README_generate_gm_templates.md
- _config.yml
- docs/KEYBINDS.ja.md
- generate_gm_templates.rs
- generated-docs/project-overview-generated-prompt.md
- googled947dc864c270e07.html
- issue-notes/100.md
- issue-notes/101.md
- issue-notes/102.md
- issue-notes/103.md
- issue-notes/104.md
- issue-notes/105.md
- issue-notes/106.md
- issue-notes/107.md
- issue-notes/108.md
- issue-notes/109.md
- issue-notes/110.md
- issue-notes/111.md
- issue-notes/112.md
- issue-notes/113.md
- issue-notes/114.md
- issue-notes/115.md
- issue-notes/116.md
- issue-notes/130.md
- issue-notes/134.md
- issue-notes/136.md
- issue-notes/138.md
- issue-notes/139.md
- issue-notes/141.md
- issue-notes/144.md
- issue-notes/146.md
- issue-notes/147.md
- issue-notes/148.md
- issue-notes/149.md
- issue-notes/150.md
- issue-notes/151.md
- issue-notes/155.md
- issue-notes/156.md
- issue-notes/158.md
- issue-notes/164.md
- issue-notes/165.md
- issue-notes/166.md
- issue-notes/167.md
- issue-notes/95.md
- issue-notes/96.md
- issue-notes/97.md
- issue-notes/99.md
- src/app.rs
- src/app_init.rs
- src/audio.rs
- src/config.rs
- src/file_ops.rs
- src/main.rs
- src/midi_conversion.rs
- src/models.rs
- src/register.rs
- src/tests/app_tests.rs
- src/tests/file_ops_tests.rs
- src/tests/midi_conversion_tests.rs
- src/tests/mod.rs
- src/tests/register_tests.rs
- src/tests/ui_tests.rs
- src/tests/variation_selector_tests.rs
- src/tests/verbose_logging_tests.rs
- src/ui.rs
- src/variation_selector.rs
- tones/general_midi/000_AcousticGrand.json
- tones/general_midi/tone_names.json
- ym2151-tone-editor.toml.example

## 現在のオープンIssues
## [Issue #167](../issue-notes/167.md): プレビューを鳴らすとき、前の音のkeyoff時にプチノイズが乗ってしまう
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

```

## [Issue #166](../issue-notes/166.md): カーソルが、ALG や FB の行にあるときも、OP1～4のいずれか1行と1列に、操作ガイドを表示し続ける
[issue-notes/166.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/166.md)

...
ラベル: 
--- issue-notes/166.md の内容 ---

```markdown
# issue カーソルが、ALG や FB の行にあるときも、OP1～4のいずれか1行と1列に、操作ガイドを表示し続ける #166
[issues #166](https://github.com/cat2151/ym2151-tone-editor/issues/166)



```

## [Issue #165](../issue-notes/165.md): 現在カーソルのある列に、「1」～「4」という操作ガイドを表示する。ADSRガイド表示を参考にする
[issue-notes/165.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/165.md)

...
ラベル: 
--- issue-notes/165.md の内容 ---

```markdown
# issue 現在カーソルのある列に、「1」～「4」という操作ガイドを表示する。ADSRガイド表示を参考にする #165
[issues #165](https://github.com/cat2151/ym2151-tone-editor/issues/165)



```

## [Issue #164](../issue-notes/164.md): compile時にwarningが出ている。それもcompileチェック時にエラーとみなすか、試して検証する
[issue-notes/164.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/164.md)

...
ラベル: 
--- issue-notes/164.md の内容 ---

```markdown
# issue compile時にwarningが出ている。それもcompileチェック時にエラーとみなすか、試して検証する #164
[issues #164](https://github.com/cat2151/ym2151-tone-editor/issues/164)



```

## [Issue #161](../issue-notes/161.md): Cargo test failed (387f691)
Cargo test failed in push event.

Branch/Ref: refs/heads/main
Commit: 387f691991240b139bdca07a871b1f581c8e4ec1

Please investigate the test failures and fix them.

Workflow run: https://github.com/cat2151/ym2151-tone-editor/actions/runs/20045332056...
ラベル: bug, test-failure
--- issue-notes/161.md の内容 ---

```markdown

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

### issue-notes/155.md
```md
{% raw %}
# issue ドッグフーディングする #155
[issues #155](https://github.com/cat2151/ym2151-tone-editor/issues/155)



{% endraw %}
```

### issue-notes/164.md
```md
{% raw %}
# issue compile時にwarningが出ている。それもcompileチェック時にエラーとみなすか、試して検証する #164
[issues #164](https://github.com/cat2151/ym2151-tone-editor/issues/164)



{% endraw %}
```

### issue-notes/165.md
```md
{% raw %}
# issue 現在カーソルのある列に、「1」～「4」という操作ガイドを表示する。ADSRガイド表示を参考にする #165
[issues #165](https://github.com/cat2151/ym2151-tone-editor/issues/165)



{% endraw %}
```

### issue-notes/166.md
```md
{% raw %}
# issue カーソルが、ALG や FB の行にあるときも、OP1～4のいずれか1行と1列に、操作ガイドを表示し続ける #166
[issues #166](https://github.com/cat2151/ym2151-tone-editor/issues/166)



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

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
04eb99b Document findings and hypotheses for issue #167
b6b8e17 Add issue note for #167 [auto]
d8fd87f Add issue note for #166 [auto]
4236ca2 Add issue note for #165 [auto]
1d8b8ac Add issue note for #164 [auto]
ee45e40 Merge pull request #162 from cat2151/copilot/fix-cargo-test-failures
4430bc8 Update project summaries (overview & development status) [auto]
a4722c3 Revert slot mask changes - keep original implementation, fix tests instead
5423bd5 Remove unused import to fix clippy warning
6ad640f Fix jump+increase/decrease tests and remaining test failures

### 変更されたファイル:
generated-docs/development-status-generated-prompt.md
generated-docs/development-status.md
generated-docs/project-overview-generated-prompt.md
generated-docs/project-overview.md
issue-notes/164.md
issue-notes/165.md
issue-notes/166.md
issue-notes/167.md
src/config.rs
src/tests/app_tests.rs
src/tests/midi_conversion_tests.rs
src/tests/register_tests.rs
src/tests/ui_tests.rs


---
Generated at: 2025-12-11 07:08:52 JST
