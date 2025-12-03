Last updated: 2025-12-04

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
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-recent-human-commit.yml
- .github/actions-tmp/.github/workflows/daily-project-summary.yml
- .github/actions-tmp/.github/workflows/issue-note.yml
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
- .github/workflows/call-translate-readme.yml
- .gitignore
- Cargo.lock
- Cargo.toml
- LICENSE
- README.ja.md
- README.md
- _config.yml
- docs/KEYBINDS.ja.md
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
- src/tests/verbose_logging_tests.rs
- src/ui.rs
- tones/general_midi/000_AcousticGrand.json
- tones/general_midi/tone_names.json
- ym2151-tone-editor.toml.example

## 現在のオープンIssues
## [Issue #131](../issue-notes/131.md): Refactor: Replace magic number 0.005 with configurable envelope_delay_seconds
The envelope delay time (5ms) was hardcoded as `0.005` in 20 locations across audio and register modules, violating DRY.

## Changes

**Configuration system:**
- Added `DEFAULT_ENVELOPE_DELAY_SECONDS = 0.005` constant in `models.rs`
- Extended `Config` with `AudioConfig` structure containing `envelo...
ラベル: 
--- issue-notes/131.md の内容 ---

```markdown

```

## [Issue #130](../issue-notes/130.md): PR 129 において、時刻0.005が多数の場所でマジックナンバーとして書かれておりDRY違反
[issue-notes/130.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/130.md)

...
ラベル: 
--- issue-notes/130.md の内容 ---

```markdown
# issue PR 129 において、時刻0.005が多数の場所でマジックナンバーとして書かれておりDRY違反 #130
[issues #130](https://github.com/cat2151/ym2151-tone-editor/issues/130)



```

## [Issue #100](../issue-notes/100.md): CTRL+Oで、GM000 json variations をfzfに与えて、カーソルのある行の variationsを演奏し、ENTERでそのvariationsをtone editorに読み込む。仮仕様。検証用。
[issue-notes/100.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/100.md)

...
ラベル: 
--- issue-notes/100.md の内容 ---

```markdown
# issue CTRL+Oで、GM000 json variations をfzfに与えて、カーソルのある行の variationsを演奏し、ENTERでそのvariationsをtone editorに読み込む。仮仕様。検証用。 #100
[issues #100](https://github.com/cat2151/ym2151-tone-editor/issues/100)



```

## [Issue #99](../issue-notes/99.md): CTRL+Sで、音色データを GM000 jsonのvariationsの末尾に追記保存する。仮仕様。検証用。
[issue-notes/99.md](https://github.com/cat2151/ym2151-tone-editor/blob/main/issue-notes/99.md)

...
ラベル: 
--- issue-notes/99.md の内容 ---

```markdown
# issue CTRL+Sで、音色データを GM000 jsonのvariationsの末尾に追記保存する。仮仕様。検証用。 #99
[issues #99](https://github.com/cat2151/ym2151-tone-editor/issues/99)



```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/30.md
```md
{% raw %}
# issue 進捗状況生成時、issueに紐付くissue-notesがないときエラー終了してしまう #30
[issues #30](https://github.com/cat2151/github-actions/issues/30)

# 何が困るの？
- 生成されない

# 分析
- issue紐付くissue-notesが存在しないことは普通にある
- 今回も、そうなっていることを確認済み
    - issue 1～8はissue-notesがあった
    - 当該のissue 9は、issue本体のコメントに書いて進行していた
        - issue-notesの仕組みを使う前に書いたissueなので、そうなっていた
- こうするのがよい
    - エラーにならず、空文字として扱う

# close条件
- 当該部分で落ちなくなること
    - 当該部分とは：
    - https://github.com/cat2151/fighting-game-button-challenge
        - issue 9



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

### issue-notes/100.md
```md
{% raw %}
# issue CTRL+Oで、GM000 json variations をfzfに与えて、カーソルのある行の variationsを演奏し、ENTERでそのvariationsをtone editorに読み込む。仮仕様。検証用。 #100
[issues #100](https://github.com/cat2151/ym2151-tone-editor/issues/100)



{% endraw %}
```

### issue-notes/130.md
```md
{% raw %}
# issue PR 129 において、時刻0.005が多数の場所でマジックナンバーとして書かれておりDRY違反 #130
[issues #130](https://github.com/cat2151/ym2151-tone-editor/issues/130)



{% endraw %}
```

### issue-notes/99.md
```md
{% raw %}
# issue CTRL+Sで、音色データを GM000 jsonのvariationsの末尾に追記保存する。仮仕様。検証用。 #99
[issues #99](https://github.com/cat2151/ym2151-tone-editor/issues/99)



{% endraw %}
```

### src/models.rs
```rs
{% raw %}
use serde::{Deserialize, Serialize};

// Grid dimensions for the UI layout
pub const GRID_WIDTH: usize = 12;
pub const GRID_HEIGHT: usize = 5;

// Parameter names for each column
// New order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
pub const PARAM_NAMES: [&str; GRID_WIDTH] = [
    "SM", "TL", "MUL", "AR", "D1R", "D1L", "D2R", "RR", "DT", "DT2", "KS", "AMS",
];

// CH row has 3 parameters: ALG, FB, and MIDI note number
pub const CH_PARAM_COUNT: usize = 3;
pub const CH_PARAM_NAMES: [&str; CH_PARAM_COUNT] = ["ALG", "FB", "Note"];

// Maximum values for each parameter (respecting YM2151 bit ranges)
// New order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
pub const PARAM_MAX: [u8; GRID_WIDTH] = [
    1,  // SM (SlotMask): 0 or 1
    99, // TL: 7 bits (0-127, limited to 99 for display)
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

// Maximum values for CH row parameters
pub const CH_PARAM_MAX: [u8; CH_PARAM_COUNT] = [
    7,   // ALG: 3 bits (0-7) - Algorithm
    7,   // FB: 3 bits (0-7) - Feedback
    127, // MIDI Note Number: 0-127 (60 = middle C)
];

// Row names for operators
pub const ROW_NAMES: [&str; GRID_HEIGHT] = ["O1", "O2", "O3", "O4", "CH"];

// Parameter column indices for operator rows (matching PARAM_NAMES order)
// order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
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

// Parameter column indices for CH row (matching CH_PARAM_NAMES order)
pub const CH_PARAM_ALG: usize = 0;
pub const CH_PARAM_FB: usize = 1;
pub const CH_PARAM_NOTE: usize = 2;

// Row index for channel settings
pub const ROW_CH: usize = 4;

/// Type alias for tone data grid
pub type ToneData = [[u8; GRID_WIDTH]; GRID_HEIGHT];

/// JSON event structure for ym2151-log-play-server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ym2151Event {
    pub time: f64,
    pub addr: String,
    pub data: String,
}

/// JSON log structure for ym2151-log-play-server
#[derive(Serialize, Deserialize, Debug)]
pub struct Ym2151Log {
    pub events: Vec<Ym2151Event>,
}

/// Tone variation structure for General MIDI tone files
/// Represents a single tone variation with optional MML or note number for playback
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToneVariation {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_number: Option<u8>,
    pub registers: String,
}

/// Tone file structure for General MIDI tone files
/// Contains a description and array of tone variations
#[derive(Serialize, Deserialize, Debug)]
pub struct ToneFile {
    pub description: String,
    pub variations: Vec<ToneVariation>,
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
5a6ee19 Add issue note for #130 [auto]
c3dcb25 Merge pull request #129 from cat2151/copilot/fix-attack-envelope-preview
79a1222 Fix timing and eliminate code duplication per review feedback
75d0140 Use HashSet for better performance in test
8c69509 Improve test robustness based on code review feedback
a6eca20 Add test for envelope reset functionality
6494b87 Fix envelope continuation across notes in audio preview (issue #115)
231762a Initial plan
eb65ad6 Merge pull request #128 from cat2151/copilot/add-key-guide-display
0ed44a4 Add keybinding guide letters to parameter display

### 変更されたファイル:
README.ja.md
README.md
issue-notes/130.md
src/audio.rs
src/register.rs
src/tests/register_tests.rs
src/tests/ui_tests.rs
src/ui.rs


---
Generated at: 2025-12-04 07:08:57 JST
