Last updated: 2026-03-19

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
- src/tests/event_loop_tests.rs
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
## [Issue #231](../issue-notes/231.md): help: generate keybind display from config; remove hjkl/wasd; always show ?:help
- [x] Update `src/ui/help.rs`: generate help from keybinds config, remove hjkl/wasd, show arrow keys, always show `?:help` hint
- [x] Update `src/ui/mod.rs`: pass `config: &Config` to `ui()` and `draw_keybind_hints()`
- [x] Update `src/event_loop.rs`: pass `config` to `crate::ui::ui()` calls
- [x] F...
ラベル: 
--- issue-notes/231.md の内容 ---

```markdown

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

### src/app/mod.rs
```rs
{% raw %}
mod shortcuts;

#[cfg(windows)]
use crate::audio;
use crate::file_ops;
use crate::models::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct App {
    pub values: ToneData,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub value_by_mouse_move: bool,
    #[cfg(windows)]
    pub use_interactive_mode: bool,
    /// ペンタトニック鍵盤のマウスホバー座標(Noneなら未ホバー)
    pub hovered_penta_x: Option<usize>,
    /// Envelope delay in seconds before tone parameters are set (default: 0.005)
    #[allow(dead_code)] // Used on Windows builds for audio playback
    pub envelope_delay_seconds: f64,
    /// Last operator row (0-3) the cursor was on before moving to CH row
    /// Used for displaying operation guides when cursor is on CH row
    pub last_operator_row: usize,
    /// Whether the keybind help overlay is shown
    pub show_help: bool,
    /// バックグラウンドのアップデートチェックがtrueにセットしたらアップデートを実行
    pub update_available: Arc<AtomicBool>,
    /// 音色が最後に変更された時刻（アイドル検出用）
    #[cfg(windows)]
    pub last_tone_change: std::time::Instant,
    /// バックグラウンドスレッドが生成したsixelグラフィクス文字列（Noneなら未生成）
    #[cfg(windows)]
    pub sixel_waveform: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    /// sixel波形生成スレッドが起動中かどうか
    #[cfg(windows)]
    pub waveform_generating: bool,
    /// 波形生成の世代カウンタ。音色変更時にインクリメントし、古いスレッドの結果を無効化する
    #[cfg(windows)]
    pub waveform_generation: std::sync::Arc<std::sync::atomic::AtomicU32>,
}

impl App {
    /// 仮想ペンタトニック鍵盤上のマウス座標からホバーx座標を更新
    /// ALG図直下の描画位置に合わせて判定
    pub fn update_hovered_penta_x(
        &mut self,
        mouse_x: u16,
        mouse_y: u16,
        inner: ratatui::layout::Rect,
        penta_keyboard_y: u16,
    ) {
        if mouse_y != penta_keyboard_y {
            self.hovered_penta_x = None;
            return;
        }
        if mouse_x >= inner.x && mouse_x < inner.x + inner.width {
            let rel_x = mouse_x - inner.x;
            self.hovered_penta_x = Some(rel_x as usize);
        } else {
            self.hovered_penta_x = None;
        }
    }
    pub fn new(
        #[allow(unused_variables)] use_interactive_mode: bool,
        value_by_mouse_move: bool,
        envelope_delay_seconds: f64,
    ) -> App {
        let mut app = crate::app_init::init_app(
            use_interactive_mode,
            value_by_mouse_move,
            envelope_delay_seconds,
        );
        app.hovered_penta_x = None;
        app
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let max_x = if self.cursor_y == ROW_CH {
            CH_PARAM_COUNT - 1
        } else {
            GRID_WIDTH - 1
        };

        if self.cursor_x < max_x {
            self.cursor_x += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;

            // Track the new position if it's an operator row
            if self.cursor_y < ROW_CH {
                self.last_operator_row = self.cursor_y;
            }

            // Clamp cursor_x if moving from CH row to operator row or vice versa
            let max_x = if self.cursor_y == ROW_CH {
                CH_PARAM_COUNT - 1
            } else {
                GRID_WIDTH - 1
            };

            if self.cursor_x > max_x {
                self.cursor_x = max_x;
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y < GRID_HEIGHT - 1 {
            // Track current position if it's an operator row (before moving)
            if self.cursor_y < ROW_CH {
                self.last_operator_row = self.cursor_y;
            }

            self.cursor_y += 1;

            // Clamp cursor_x if moving from operator row to CH row or vice versa
            let max_x = if self.cursor_y == ROW_CH {
                CH_PARAM_COUNT - 1
            } else {
                GRID_WIDTH - 1
            };

            if self.cursor_x > max_x {
                self.cursor_x = max_x;
            }
        }
    }

    /// Play audio feedback for the current tone.
    /// History saving runs on all platforms; audio playback is Windows-only.
    fn play_audio(&self) {
        let _ = crate::history::save_to_history(&self.values);
        #[cfg(windows)]
        audio::play_tone(
            &self.values,
            self.use_interactive_mode,
            self.cursor_x,
            self.cursor_y,
            self.envelope_delay_seconds,
        );
    }

    /// Get the maximum allowed value for the current cursor position
    fn get_current_max(&self) -> u8 {
        if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        }
    }

    /// Set cursor_x to the given parameter column and increase or decrease its value.
    /// Only applies when the cursor is on an operator row (not CH row).
    fn jump_to_op_param(&mut self, param_x: usize, increase: bool) {
        self.cursor_x = param_x;
        if self.cursor_y < ROW_CH {
            if increase {
                self.increase_value();
            } else {
                self.decrease_value();
            }
        }
    }

    /// Jump to a CH row parameter and increase or decrease its value
    fn jump_to_ch_param(&mut self, ch_param: usize, increase: bool) {
        self.cursor_y = ROW_CH;
        self.cursor_x = ch_param;
        if increase {
            self.increase_value();
        } else {
            self.decrease_value();
        }
    }

    pub fn increase_value(&mut self) {
        let data_row = self.cursor_y;
        let current = self.values[data_row][self.cursor_x];
        let max = self.get_current_max();
        if current < max {
            self.values[data_row][self.cursor_x] = current + 1;
            self.play_audio();
        }
    }

    pub fn decrease_value(&mut self) {
        let data_row = self.cursor_y;
        let current = self.values[data_row][self.cursor_x];
        if current > 0 {
            self.values[data_row][self.cursor_x] = current - 1;
            self.play_audio();
        }
    }

    /// Increase the current parameter value by a specified amount
    /// Used for number key shortcuts (1-9 for +1 to +9, 0 for +10)
    pub fn increase_value_by(&mut self, amount: u8) {
        let data_row = self.cursor_y;
        let current = self.values[data_row][self.cursor_x];
        let max = self.get_current_max();
        let new_value = current.saturating_add(amount).min(max);
        if new_value != current {
            self.values[data_row][self.cursor_x] = new_value;
            self.play_audio();
        }
    }

    /// Decrease the current parameter value by a specified amount
    /// Used for SHIFT + number key shortcuts (SHIFT+1-9 for -1 to -9, SHIFT+0 for -10)
    pub fn decrease_value_by(&mut self, amount: u8) {
        let data_row = self.cursor_y;
        let current = self.values[data_row][self.cursor_x];
        let new_value = current.saturating_sub(amount);
        if new_value != current {
            self.values[data_row][self.cursor_x] = new_value;
            self.play_audio();
        }
    }

    pub fn set_value_to_max(&mut self) {
        let max = self.get_current_max();
        self.values[self.cursor_y][self.cursor_x] = max;
        self.play_audio();
    }

    pub fn set_value_to_min(&mut self) {
        self.values[self.cursor_y][self.cursor_x] = 0;
        self.play_audio();
    }

    pub fn set_value_to_random(&mut self) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let max = self.get_current_max();
        let random_state = RandomState::new();
        let mut hasher = random_state.build_hasher();
        std::time::SystemTime::now().hash(&mut hasher);
        self.cursor_x.hash(&mut hasher);
        self.cursor_y.hash(&mut hasher);
        let hash = hasher.finish();
        let random_value = (hash % (max as u64 + 1)) as u8;
        self.values[self.cursor_y][self.cursor_x] = random_value;
        self.play_audio();
    }

    /// Randomize all tone parameters using web-ym2151 random-tone logic.
    /// Triggered by F5 key.
    pub fn randomize_tone(&mut self) {
        use crate::random_tone::generate_random_tone;
        let current_note = self.values[ROW_CH][CH_PARAM_NOTE];
        self.values = generate_random_tone(current_note);
        self.play_audio();
    }

    /// Move cursor to a specific mouse position
    /// Maps mouse x,y coordinates to cursor position in the grid
    /// Based on the UI layout from ui.rs
    pub fn move_cursor_to_mouse_position(&mut self, mouse_x: u16, mouse_y: u16) {
        // UI layout constants (from ui.rs)
        const ROW_LABEL_WIDTH: u16 = 4;
        const CELL_WIDTH: u16 = 4;
        const LABEL_OFFSET: u16 = 1;
        const INNER_X: u16 = 1; // Border takes 1 character
        const INNER_Y: u16 = 1; // Border takes 1 character

        // Check if mouse is within the grid area (after row labels)
        if mouse_x < INNER_X + ROW_LABEL_WIDTH {
            return; // Mouse is in row label area
        }

        // Calculate column from mouse X position
        let relative_x = mouse_x - INNER_X - ROW_LABEL_WIDTH;
        let col = (relative_x / CELL_WIDTH) as usize;

        // Calculate row from mouse Y position
        // Operator rows: y = INNER_Y + LABEL_OFFSET + row (1-4)
        // CH row header: y = INNER_Y + LABEL_OFFSET + 4 (5)
        // CH row values: y = INNER_Y + LABEL_OFFSET + 5 (6)
        if mouse_y < INNER_Y + LABEL_OFFSET {
            return; // Mouse is in header area
        }

        let relative_y = mouse_y - INNER_Y - LABEL_OFFSET;

        // Determine which row the mouse is on
        let new_cursor_y = match relative_y {
            0..=3 => relative_y as usize, // Operator rows
            5 => ROW_CH,                  // CH row (skip row 4 which is CH header)
            _ => return,                  // Outside valid rows
        };

        // Validate column bounds
        let max_x = if new_cursor_y == ROW_CH {
            CH_PARAM_COUNT - 1
        } else {
            GRID_WIDTH - 1
        };

        if col > max_x {
            return; // Column out of bounds
        }

        // Update cursor position
        self.cursor_x = col;
        self.cursor_y = new_cursor_y;
    }

    /// Update the parameter value based on mouse X position
    /// Maps mouse X position to parameter value range (0 to PARAM_MAX)
    /// Uses the middle third of the terminal width for full range
    /// Left of middle third sets to min (0), right of middle third sets to max
    pub fn update_value_from_mouse_x(&mut self, mouse_x: u16, terminal_width: u16) {
        if terminal_width == 0 {
            return; // Avoid division by zero
        }

        // Calculate middle third boundaries
        let third_width = terminal_width / 3;
        let left_boundary = third_width;
        let right_boundary = third_width * 2;

        let max_value = self.get_current_max();

        let new_value = if mouse_x < left_boundary {
            // Mouse is left of middle third -> set to minimum (0)
            0
        } else if mouse_x > right_boundary {
            // Mouse is right of middle third -> set to maximum
            max_value
        } else {
            // Mouse is within middle third -> map proportionally
            // left_boundary -> 0, right_boundary -> max value
            let middle_width = right_boundary - left_boundary;
            let relative_x = mouse_x - left_boundary;
            let normalized = if middle_width == 0 {
                0.0
            } else {
                relative_x as f32 / middle_width as f32
            };
            (normalized * max_value as f32).round() as u8
        };

        // Only update and play sound if the value actually changed
        let data_row = self.cursor_y;
        if self.values[data_row][self.cursor_x] != new_value {
            self.values[data_row][self.cursor_x] = new_value;
            self.play_audio();
        }
    }

    /// Save tone data to JSON file
    pub fn save_to_json(&self) -> std::io::Result<()> {
        // Save to GM format in app data directory
        let gm_path = file_ops::gm_file_path().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find app data directory",
            )
        })?;
        file_ops::save_to_gm_file(gm_path, &self.values, "Edited Tone")?;

        // Also save to legacy format for backward compatibility
        file_ops::save_to_json(&self.values)?;

        Ok(())
    }

    /// Append current tone data as a new variation to GM file
    /// This is triggered by CTRL+S
    pub fn save_to_gm_variations(&self) -> std::io::Result<()> {
        let gm_path = file_ops::gm_file_path().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find app data directory",
            )
        })?;

        // Append to GM format variations array
        file_ops::append_to_gm_file(gm_path, &self.values, "Edited Tone")?;

        Ok(())
    }

    /// Play the current tone without modifying any parameters
    /// This is triggered by 'P' or 'SPACE' key
    pub fn play_current_tone(&self) {
        self.play_audio();
    }

    /// Toggle the keybind help overlay
    /// This is triggered by '?' (SHIFT+/) key
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Move cursor to FB parameter and increase its value
    /// This is triggered by 'F' key
    pub fn increase_fb(&mut self) {
        self.jump_to_ch_param(CH_PARAM_FB, true);
    }

    /// Move cursor to FB parameter and decrease its value
    /// This is triggered by 'Shift+F' key
    pub fn decrease_fb(&mut self) {
        self.jump_to_ch_param(CH_PARAM_FB, false);
    }

    /// Move cursor to ALG parameter and increase its value
    /// This is triggered by 'g' key
    pub fn increase_alg(&mut self) {
        self.jump_to_ch_param(CH_PARAM_ALG, true);
    }

    /// Move cursor to ALG parameter and decrease its value
    /// This is triggered by 'G' key (Shift+g)
    pub fn decrease_alg(&mut self) {
        self.jump_to_ch_param(CH_PARAM_ALG, false);
    }

    /// Jump to operator row and increase value at current column
    pub fn jump_to_operator_and_increase(&mut self, operator_row: usize) {
        if operator_row >= 4 {
            return; // Invalid operator row
        }
        self.cursor_y = operator_row;
        self.last_operator_row = operator_row;
        if self.cursor_x > GRID_WIDTH - 1 {
            self.cursor_x = GRID_WIDTH - 1;
        }
        self.increase_value();
    }

    /// Jump to operator row and decrease value at current column
    pub fn jump_to_operator_and_decrease(&mut self, operator_row: usize) {
        if operator_row >= 4 {
            return; // Invalid operator row
        }
        self.cursor_y = operator_row;
        self.last_operator_row = operator_row;
        if self.cursor_x > GRID_WIDTH - 1 {
            self.cursor_x = GRID_WIDTH - 1;
        }
        self.decrease_value();
    }

    /// Jump to Note Number parameter and increase its value
    /// This is triggered by 'j' key
    pub fn jump_to_note_and_increase(&mut self) {
        self.jump_to_ch_param(CH_PARAM_NOTE, true);
    }

    /// Jump to Note Number parameter and decrease its value
    /// This is triggered by 'J' key (Shift+j)
    pub fn jump_to_note_and_decrease(&mut self) {
        self.jump_to_ch_param(CH_PARAM_NOTE, false);
    }

    /// 音色パラメータが変更されたときに呼び出す。
    /// アイドルタイマーをリセットし、古いsixel波形をクリアする。
    #[cfg(windows)]
    pub fn on_tone_changed(&mut self) {
        self.last_tone_change = std::time::Instant::now();
        self.waveform_generating = false;
        // 世代をインクリメントして実行中スレッドの結果を無効化する
        self.waveform_generation
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if let Ok(mut guard) = self.sixel_waveform.lock() {
            *guard = None;
        }
    }

    /// Cleanup - stop interactive mode if active
    #[cfg(windows)]
    pub fn cleanup(&self) {
        if self.use_interactive_mode {
            audio::cleanup_interactive_mode();
        }
    }

    /// アップデートが利用可能かどうかを返す
    pub fn is_update_available(&self) -> bool {
        self.update_available.load(Ordering::Relaxed)
    }
}

{% endraw %}
```

### src/tests/mod.rs
```rs
{% raw %}
//! Unit tests separated from main source files
//!
//! This module structure allows tests to access private functions
//! while keeping them separate to prevent hallucination issues.

#[cfg(test)]
mod app_tests;

#[cfg(test)]
mod app_ch_param_tests;

#[cfg(test)]
mod app_value_by_tests;

#[cfg(test)]
mod app_adsr_mul_sm_tests;

#[cfg(test)]
mod app_tl_d1l_dt_dt2_tests;

#[cfg(test)]
mod app_ks_ams_tests;

#[cfg(test)]
mod file_ops_tests;

#[cfg(test)]
mod midi_conversion_tests;

#[cfg(test)]
mod register_tests;

#[cfg(test)]
mod register_roundtrip_tests;

#[cfg(test)]
mod ui_tests;

#[cfg(test)]
mod variation_selector_tests;

#[cfg(test)]
mod verbose_logging_tests;

#[cfg(test)]
mod random_tone_tests;

#[cfg(test)]
mod history_tests;

#[cfg(test)]
mod favorites_tests;

#[cfg(test)]
mod event_loop_tests;

{% endraw %}
```

### src/ui/mod.rs
```rs
{% raw %}
mod helpers;
pub use helpers::*;
mod help;

use crate::{app::App, models::*};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};

/// Background color for shortcut key guides
const KEY_GUIDE_BG_COLOR: Color = Color::Rgb(40, 40, 40);

/// Height (in character rows) of the operator envelope canvas.
/// Each row in Braille mode provides 4 pixels of vertical resolution.
const ENVELOPE_CANVAS_HEIGHT: u16 = 6;

/// Terminal row at which the CH parameter values are drawn, assuming the outer
/// block starts at y=0 (full-screen mode).
/// Derivation: border_top(1) + label_offset(1) + op_rows(4) + ch_header(1) = 7.
pub const LAYOUT_CH_ROW_Y: u16 = 7;

/// Colors used to draw the four operator envelopes (O1–O4).
const OP_ENVELOPE_COLORS: [Color; 4] = [Color::Cyan, Color::Green, Color::Yellow, Color::Magenta];

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    let block = Block::default()
        .title("YM2151 Tone Editor")
        .borders(Borders::ALL);
    let inner = block.inner(size);
    f.render_widget(block, size);

    // Calculate cell dimensions
    let cell_width = 4; // 2 digits + spacing
    let cell_height = 1;
    let label_offset = 1; // Space for parameter name labels
    let row_label_width = 4; // Width for row labels (e.g., "OP1 ")

    // Draw parameter names (column headers) for operator rows
    for (col, param_name) in PARAM_NAMES.iter().enumerate().take(GRID_WIDTH) {
        let x = inner.x + row_label_width + (col as u16 * cell_width);
        let y = inner.y;

        let area = Rect {
            x,
            y,
            width: cell_width,
            height: 1,
        };

        let color = get_param_color(col, false);
        let paragraph = Paragraph::new(Span::styled(
            *param_name,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let operator_roles = get_operator_roles_for_alg(alg_value);
    // Draw grid values with row labels for operators (rows 0-3)
    for display_row in 0..4 {
        let slot_mask_enabled = app.values[display_row][PARAM_SM] != 0;
        // Draw row label (operator name)
        let row_label_area = Rect {
            x: inner.x,
            y: inner.y + label_offset + display_row as u16,
            width: row_label_width,
            height: cell_height,
        };
        let row_name = ROW_NAMES[display_row];
        let row_label_color = if slot_mask_enabled {
            if operator_roles[display_row] {
                Color::White
            } else {
                Color::Green
            }
        } else {
            Color::DarkGray
        };
        let row_label =
            Paragraph::new(Span::styled(row_name, Style::default().fg(row_label_color)));
        f.render_widget(row_label, row_label_area);
        // Draw values
        for col in 0..GRID_WIDTH {
            let value = app.values[display_row][col];
            let x = inner.x + row_label_width + (col as u16 * cell_width);
            let y = inner.y + label_offset + display_row as u16;
            let area = Rect {
                x,
                y,
                width: cell_width,
                height: cell_height,
            };
            let value_style = if app.cursor_x == col && app.cursor_y == display_row {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                let color = if slot_mask_enabled {
                    if operator_roles[display_row] {
                        Color::White
                    } else {
                        Color::Green
                    }
                } else {
                    Color::DarkGray
                };
                Style::default().fg(color)
            };

            // Display guide to the left of the value
            // Show operator number guide in current column, or parameter key guide on current row
            // When cursor is on CH row, show guides on the last operator row the cursor was on
            let is_current_row = app.cursor_y == display_row;
            let is_current_col = app.cursor_x == col;
            let show_guide_for_ch_row =
                app.cursor_y == ROW_CH && display_row == app.last_operator_row;

            let line = if is_current_col {
                // In current column, show operator number guide
                if let Some(op_guide) = get_operator_guide(display_row) {
                    let op_guide_style =
                        Style::default().fg(Color::DarkGray).bg(KEY_GUIDE_BG_COLOR);
                    Line::from(vec![
                        Span::styled(op_guide.to_string(), op_guide_style),
                        Span::styled(format!("{:2}", value), value_style),
                    ])
                } else {
                    // No guide for non-operator rows in current column
                    Line::from(Span::styled(format!(" {:2}", value), value_style))
                }
            } else if let Some(key_guide) = get_key_guide(col) {
                if is_current_row || show_guide_for_ch_row {
                    // Show parameter key guide on current row (for non-current columns)
                    // or on last operator row when cursor is on CH row
                    let key_guide_style =
                        Style::default().fg(Color::DarkGray).bg(KEY_GUIDE_BG_COLOR);
                    Line::from(vec![
                        Span::styled(key_guide.to_string(), key_guide_style),
                        Span::styled(format!("{:2}", value), value_style),
                    ])
                } else {
                    // No guide on non-current rows in non-current columns
                    Line::from(Span::styled(format!(" {:2}", value), value_style))
                }
            } else {
                Line::from(Span::styled(format!(" {:2}", value), value_style))
            };
            let paragraph = Paragraph::new(line);
            f.render_widget(paragraph, area);
        }
    }

    // Draw CH row header (parameter names for CH row)
    let ch_header_y = inner.y + label_offset + 4;
    for (col, ch_param_name) in CH_PARAM_NAMES.iter().enumerate().take(CH_PARAM_COUNT) {
        let x = inner.x + row_label_width + (col as u16 * cell_width);

        let area = Rect {
            x,
            y: ch_header_y,
            width: cell_width,
            height: 1,
        };

        let color = get_param_color(col, true);
        let paragraph = Paragraph::new(Span::styled(
            *ch_param_name,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    // Draw CH row (row 4) with ALG, FB, and MIDI note number
    let ch_row_y = inner.y + label_offset + 5;

    // Draw row label (CH)
    let row_label_area = Rect {
        x: inner.x,
        y: ch_row_y,
        width: row_label_width,
        height: cell_height,
    };
    let row_label = Paragraph::new(Span::styled(
        ROW_NAMES[ROW_CH],
        Style::default().fg(Color::Yellow),
    ));
    f.render_widget(row_label, row_label_area);

    // Draw all CH row values (ALG, FB, and MIDI note number)
    for col in 0..CH_PARAM_COUNT {
        let value = app.values[ROW_CH][col];
        let x = inner.x + row_label_width + (col as u16 * cell_width);

        let area = Rect {
            x,
            y: ch_row_y,
            width: cell_width,
            height: cell_height,
        };

        let value_style = if app.cursor_x == col && app.cursor_y == ROW_CH {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            let color = get_param_color(col, true);
            Style::default().fg(color)
        };

        // Display guide to the left of the value on the CH row
        // ALG and FB guides are always shown because 'g'/'G' and 'f'/'F' can jump to them from anywhere
        let line = if let Some(key_guide) = get_ch_key_guide(col) {
            let key_guide_style = Style::default().fg(Color::DarkGray).bg(KEY_GUIDE_BG_COLOR);
            Line::from(vec![
                Span::styled(key_guide.to_string(), key_guide_style),
                Span::styled(format!("{:2}", value), value_style),
            ])
        } else {
            // No guide for parameters without keybindings
            Line::from(Span::styled(format!(" {:2}", value), value_style))
        };

        let paragraph = Paragraph::new(line);
        f.render_widget(paragraph, area);
    }

    // Draw algorithm diagram below the CH row
    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let diagram = get_algorithm_diagram(alg_value);
    let diagram_start_y = ch_row_y + 2; // Leave one line of space

    for (i, line) in diagram.iter().enumerate() {
        let y = diagram_start_y + i as u16;
        if y < size.height - 1 {
            // Make sure we don't draw outside the terminal
            let area = Rect {
                x: inner.x,
                y,
                width: inner.width,
                height: 1,
            };
            let paragraph = Paragraph::new(Span::styled(*line, Style::default().fg(Color::Green)));
            f.render_widget(paragraph, area);
        }
    }

    let penta_keyboard_y = diagram_start_y + diagram.len() as u16 + 1;
    // Only draw keyboard if it fits within terminal bounds
    if penta_keyboard_y < size.height - 1 {
        draw_virtual_pentatonic_keyboard_at_y(f, app, inner, penta_keyboard_y);
    }

    // Draw envelope canvas below keyboard if there is enough vertical space.
    // The canvas needs ENVELOPE_CANVAS_HEIGHT character rows + 1 gap row.
    let envelope_y = penta_keyboard_y + 1;
    // Reserve 1 row at the bottom for keybind hints and 1 row for border.
    let available_for_envelope = size.height.saturating_sub(2).saturating_sub(envelope_y);

    // On Windows, prefer the sixel waveform when it has been generated.
    // The sixel is printed by event_loop.rs after each terminal draw,
    // so we only need to skip the braille canvas here to avoid overlap.
    #[cfg(windows)]
    let has_sixel_waveform = app
        .sixel_waveform
        .lock()
        .ok()
        .map(|g| g.is_some())
        .unwrap_or(false);
    #[cfg(not(windows))]
    let has_sixel_waveform = false;

    if available_for_envelope >= ENVELOPE_CANVAS_HEIGHT && !has_sixel_waveform {
        let envelope_area = Rect {
            x: inner.x,
            y: envelope_y,
            width: inner.width,
            height: ENVELOPE_CANVAS_HEIGHT,
        };
        draw_envelope_canvas(f, app, envelope_area);
    }

    // Draw keybind hints at the bottom of the screen (left-aligned)
    help::draw_keybind_hints(f, app, inner);
}

/// Draw operator envelope shapes for all 4 OPs into `area` using ratatui's Braille Canvas.
///
/// Each operator's ADSR-like envelope is rendered as a line-chart using a distinct colour:
/// - O1: Cyan, O2: Green, O3: Yellow, O4: Magenta.
///
/// The canvas title shows each operator's role (C = Carrier, M = Modulator) derived
/// from the current algorithm value, so the user can immediately see which operators
/// contribute to the output and which are modulators.
///
/// A vertical dark-gray marker is drawn at x = [`ENVELOPE_NOTE_OFF_T`] to indicate the note-off point,
/// visually separating the sustain phase from the release phase.
///
/// Operators whose slot-mask (SM) is 0 are drawn in dark-gray to indicate they are muted.
///
/// The x-axis represents normalised time (note-on → note-off → release).
/// The y-axis represents normalised amplitude (0 = silent, 1 = max).
fn draw_envelope_canvas(f: &mut Frame, app: &App, area: Rect) {
    // Build all envelope point-sets before the closure (avoids capturing `app` by ref inside FnMut).
    let envelope_points: Vec<Vec<(f64, f64)>> = (0..4)
        .map(|op| compute_op_envelope_points(&app.values[op]))
        .collect();
    let ops_enabled: [bool; 4] = std::array::from_fn(|op| app.values[op][PARAM_SM] != 0);

    // Build title with per-operator carrier (C) / modulator (M) role labels.
    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let operator_roles = get_operator_roles_for_alg(alg_value);
    let role_label = |op: usize| if operator_roles[op] { "C" } else { "M" };
    let title = format!(
        "Env O1({})=Cy O2({})=Gn O3({})=Ye O4({})=Mg",
        role_label(0),
        role_label(1),
        role_label(2),
        role_label(3),
    );

    let canvas = Canvas::default()
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .marker(Marker::Braille)
        .x_bounds([0.0, 1.0])
        .y_bounds([0.0, 1.0])
        .paint(move |ctx| {
            // Draw note-off marker at ENVELOPE_NOTE_OFF_T to show where sustain ends and release begins.
            ctx.draw(&CanvasLine {
                x1: ENVELOPE_NOTE_OFF_T,
                y1: 0.0,
                x2: ENVELOPE_NOTE_OFF_T,
                y2: 1.0,
                color: Color::DarkGray,
            });
            // Draw operator envelope polylines on top of the marker.
            for (op, points) in envelope_points.iter().enumerate() {
                let color = if ops_enabled[op] {
                    OP_ENVELOPE_COLORS[op]
                } else {
                    Color::DarkGray
                };
                for segment in points.windows(2) {
                    let (x1, y1) = segment[0];
                    let (x2, y2) = segment[1];
                    ctx.draw(&CanvasLine {
                        x1,
                        y1,
                        x2,
                        y2,
                        color,
                    });
                }
            }
        });

    f.render_widget(canvas, area);
}

fn draw_virtual_pentatonic_keyboard_at_y(f: &mut Frame, app: &App, inner: Rect, keyboard_y: u16) {
    let center_note = 60;
    let width = inner.width as i16;
    const PENTA_INTERVALS: [i16; 5] = [0, 2, 4, 7, 9];
    const PENTA_LABELS: [&str; 5] = ["C", "D", "E", "G", "A"];

    let center_x = width / 2;
    #[cfg(windows)]
    let mut hovered_note: Option<u8> = None;
    for x in 0..width {
        let rel = x - center_x;
        let octave = rel.div_euclid(5);
        let penta_idx = rel.rem_euclid(5);
        let note = center_note as i16 + octave * 12 + PENTA_INTERVALS[penta_idx as usize];
        if !(0..=127).contains(&note) {
            continue;
        }
        let label = PENTA_LABELS[penta_idx as usize];
        let area = Rect {
            x: inner.x + x as u16,
            y: keyboard_y,
            width: 1,
            height: 1,
        };
        let is_hovered = match app.hovered_penta_x {
            Some(hx) => hx == x as usize,
            None => false,
        };
        #[cfg(windows)]
        if is_hovered {
            hovered_note = Some(note as u8);
        }
        let style = if is_hovered {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        let paragraph = Paragraph::new(Span::styled(label, style));
        f.render_widget(paragraph, area);
    }

    #[cfg(windows)]
    if let Some(note_num) = hovered_note {
        use crate::audio;
        let mut preview_values = app.values;
        preview_values[ROW_CH][CH_PARAM_NOTE] = note_num;
        audio::play_tone(
            &preview_values,
            app.use_interactive_mode,
            CH_PARAM_NOTE,
            ROW_CH,
            app.envelope_delay_seconds,
        );
    }
}

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
pub(crate) fn key_to_string(code: KeyCode, modifiers: KeyModifiers) -> Option<String> {
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

{% endraw %}
```

### src/ui/help.rs
```rs
{% raw %}
use crate::app::App;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub(super) fn draw_keybind_hints(f: &mut Frame, app: &App, inner: Rect) {
    // Bottom line inside the inner area (inside the block border)
    let inner_bottom = inner.y + inner.height.saturating_sub(1);
    if inner.height == 0 {
        return;
    }

    if app.show_help {
        draw_help_dialog(f, inner);
    } else {
        // Brief hint on the last line of the inner area
        let area = Rect {
            x: inner.x,
            y: inner_bottom,
            width: inner.width,
            height: 1,
        };
        let paragraph = Paragraph::new(Span::styled(
            "?:help | hjkl/wasd:move  q/e:dec/inc  H:history  ESC:quit",
            Style::default().fg(Color::DarkGray),
        ));
        f.render_widget(paragraph, area);
    }
}

/// Render a centered help dialog with key bindings grouped by category.
fn draw_help_dialog(f: &mut Frame, inner: Rect) {
    // Group definitions: (header, lines...)
    let groups: &[(&str, &[&str])] = &[
        (
            " Navigation ",
            &[
                "hjkl / wasd  : Move cursor",
                "1 - 4        : Jump to OP row",
            ],
        ),
        (
            " Value Edit ",
            &[
                "q / e        : Decrease / Increase",
                ". / ,        : +1 / -1",
                "> / <        : +10 / -10",
                "Home / End   : Max / Min",
            ],
        ),
        (
            " Operator Parameters ",
            &[
                "a/A : AR    d/D : D1R   s/S : D2R   r/R : RR",
                "t/T : TL    m/M : MUL   l/L : D1L",
                "u/U : DT    n/N : DT2   k/K : KS",
                "i/I : AMS   o/O : SM",
            ],
        ),
        (
            " Channel Parameters ",
            &["f/F : FB    g/G : ALG   j/J : Note"],
        ),
        (
            " App ",
            &[
                "Space / p    : Play",
                "F5           : Random tone",
                "Ctrl+s       : Save",
                "Ctrl+o       : Open / Select file",
                "H            : History",
                "?            : Close this help",
                "ESC          : Quit",
            ],
        ),
    ];

    // Build content lines: group header + key lines, separated by blank lines between groups.
    // A footer note clarifies that these are the default keybinds (may differ if TOML overrides exist).
    let mut content_lines: Vec<Line> = Vec::new();
    let header_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default().fg(Color::Cyan);
    let note_style = Style::default().fg(Color::DarkGray);

    for (i, (group_header, lines)) in groups.iter().enumerate() {
        if i > 0 {
            content_lines.push(Line::from(""));
        }
        content_lines.push(Line::from(Span::styled(*group_header, header_style)));
        for line in *lines {
            content_lines.push(Line::from(Span::styled(*line, key_style)));
        }
    }
    content_lines.push(Line::from(""));
    content_lines.push(Line::from(Span::styled(
        "(default keybinds — may differ if ym2151-tone-editor.toml overrides exist)",
        note_style,
    )));

    // Compute dialog width from the longest content line + 2 for left/right borders
    let max_content_width = content_lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16;
    let dialog_width: u16 = max_content_width + 2;
    // +2 for top and bottom border lines
    let dialog_height: u16 = content_lines.len() as u16 + 2;

    // Center the dialog within the inner area
    let x = inner
        .x
        .saturating_add(inner.width.saturating_sub(dialog_width) / 2);
    let y = inner
        .y
        .saturating_add(inner.height.saturating_sub(dialog_height) / 2);
    let width = dialog_width.min(inner.width);
    let height = dialog_height.min(inner.height);

    let dialog_area = Rect {
        x,
        y,
        width,
        height,
    };

    // Clear the background behind the dialog
    f.render_widget(Clear, dialog_area);

    let block = Block::default()
        .title(Span::styled(
            " Help ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().bg(Color::Rgb(20, 20, 40)));

    let paragraph = Paragraph::new(Text::from(content_lines))
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, dialog_area);
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
d41badc Merge pull request #230 from cat2151/copilot/improve-envelope-line-chart
e19b0f1 refactor: extract ENVELOPE_NOTE_OFF_T constant shared by helpers and mod
f322896 feat: improve envelope polyline graph - dynamic time model, C/M labels, note-off marker
dbb8174 Initial plan
7516028 Merge pull request #229 from cat2151/copilot/update-tone-file-read-write-path
89b4a83 Address PR review comments: fix filter, error message, log message, temp dir uniqueness
9ff03e8 Move tone file I/O from current directory to AppData Local config dir
459ce39 Initial plan
bc07bde Merge pull request #228 from cat2151/copilot/fix-windows-appdata-roaming-issue
8b74442 fix: migrate history/favorites from AppData\Roaming to AppData\Local on first run

### 変更されたファイル:
generated-docs/development-status-generated-prompt.md
generated-docs/development-status.md
generated-docs/project-overview-generated-prompt.md
generated-docs/project-overview.md
src/app/mod.rs
src/app_init.rs
src/config.rs
src/event_loop.rs
src/favorites.rs
src/file_ops.rs
src/history.rs
src/tests/event_loop_tests.rs
src/tests/favorites_tests.rs
src/tests/file_ops_tests.rs
src/tests/history_tests.rs
src/tests/mod.rs
src/tests/ui_tests.rs
src/ui/help.rs
src/ui/helpers.rs
src/ui/mod.rs
src/variation_selector.rs
ym2151-tone-editor.toml.example


---
Generated at: 2026-03-19 07:14:51 JST
