Last updated: 2025-11-17


# プロジェクト概要生成プロンプト（来訪者向け）

## 生成するもの：
- projectを3行で要約する
- プロジェクトで使用されている技術スタックをカテゴリ別に整理して説明する
- プロジェクト全体のファイル階層ツリー（ディレクトリ構造を図解）
- プロジェクト全体のファイルそれぞれの説明
- プロジェクト全体の関数それぞれの説明
- プロジェクト全体の関数の呼び出し階層ツリー

## 生成しないもの：
- Issues情報（開発者向け情報のため）
- 次の一手候補（開発者向け情報のため）
- ハルシネーションしそうなもの（例、存在しない機能や計画を勝手に妄想する等）

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Project Overview

## プロジェクト概要
[以下の形式で3行でプロジェクトを要約]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 技術スタック
[使用している技術をカテゴリ別に整理して説明]
- フロントエンド: [フロントエンド技術とその説明]
- 音楽・オーディオ: [音楽・オーディオ関連技術とその説明]
- 開発ツール: [開発支援ツールとその説明]
- テスト: [テスト関連技術とその説明]
- ビルドツール: [ビルド・パース関連技術とその説明]
- 言語機能: [言語仕様・機能とその説明]
- 自動化・CI/CD: [自動化・継続的統合関連技術とその説明]
- 開発標準: [コード品質・統一ルール関連技術とその説明]

## ファイル階層ツリー
```
[プロジェクトのディレクトリ構造をツリー形式で表現]
```

## ファイル詳細説明
[各ファイルの役割と機能を詳細に説明]

## 関数詳細説明
[各関数の役割、引数、戻り値、機能を詳細に説明]

## 関数呼び出し階層ツリー
```
[関数間の呼び出し関係をツリー形式で表現]
```
```


以下のプロジェクト情報を参考にして要約を生成してください：

## プロジェクト情報
名前: 
説明: # ym2151-tone-editor

A Windows-compatible Rust TUI (Text User Interface) editor for YM2151 (OPM) FM synthesis tone parameters.

## 状況

開発中です。現在の進捗率は1%ざっくり

- 今後の展望
- ※すべて検証用の仮仕様であり、そのあと破壊的変更をします
- now : ESCで保存するとき、jsonにして保存。内部音色データ to YM2151-log-JSON
- 起動時、jsonがあれば、それを内部音色データに変換して読み込み
- 数値を増減したとき、都度、内部音色データをjson化して、ym2151-log-play-serverライブラリ経由で名前付きパイプで直接送信し演奏（高速処理により、キーリピート時も無音にならずスムーズに鳴らせます）
- 上記までの間に、音が鳴らない等の致命的な不具合が多数予想されるので、進め方をissueにできるだけノウハウとして残しつつ進めるつもり
- これで最低限、音色づくりの機能ができたので、ドッグフーディング

## Features

- Edit YM2151 tone parameters with parameter labels
- Display 11 parameters × 5 rows (4 operators + 1 channel row)
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS
- Cursor navigation with `hjkl` (Vim-style) or `wasd` keys
- Increase/decrease values with `e`/`q` keys (respects parameter max values)
- Exit with `ESC` key
- Initialized with a basic FM piano-like tone

## YM2151 Tone Data Format

This editor uses a provisional tone data format based on the YM2151 register map:

### Parameters (11 columns)

| Parameter | Name | Range | Description |
|-----------|------|-------|-------------|
| DT | Detune | 0-7 | Fine frequency detuning (3 bits) |
| MUL | Multiplier | 0-15 | Frequency multiplier (4 bits) |
| TL | Total Level | 0-99 | Operator output level (7 bits, limited to 99) |
| KS | Key Scale | 0-3 | Key scaling (2 bits) |
| AR | Attack Rate | 0-31 | Envelope attack rate (5 bits) |
| D1R | Decay 1 Rate | 0-31 | First decay rate (5 bits) |
| D1L | Decay 1 Level | 0-15 | Sustain level (4 bits) |
| D2R | Decay 2 Rate | 0-15 | Second decay/sustain rate (4 bits) |
| RR | Release Rate | 0-15 | Envelope release rate (4 bits) |
| DT2 | Detune 2 | 0-3 | Coarse frequency detuning (2 bits) |
| AMS | AM Sensitivity | 0-3 | Amplitude modulation sensitivity (2 bits) |

### Rows (5 operators/channels)

- **OP1**: Operator 1 (typically carrier in most algorithms)
- **OP2**: Operator 2 (modulator/carrier)
- **OP3**: Operator 3 (modulator/carrier)
- **OP4**: Operator 4 (modulator/carrier)
- **CH**: Channel settings (can be used for feedback, LFO, etc.)

This format allows creating basic YM2151 tones compatible with ym2151-log-play-server samples.

## Requirements

- Rust 1.70 or later

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

Or run the compiled binary directly:

```bash
./target/release/ym2151-tone-editor
```

## Live Audio Feedback (Windows only)

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the ym2151-log-play-server library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server is automatically set up and started
cargo run
```

The editor uses `send_json` to send tone updates via named pipe, which automatically chooses the optimal transmission method based on data size (direct or file-based). This provides instant audio feedback with improved response time, allowing sound to play even during key repeat operations.

**Note**: The library's `ensure_server_ready()` function handles all server management, including installation if needed.

## Controls

| Key | Action |
|-----|--------|
| `h` / `a` | Move cursor left |
| `j` / `s` | Move cursor down |
| `k` / `w` | Move cursor up |
| `l` / `d` | Move cursor right |
| `q` | Decrease value at cursor |
| `e` | Increase value at cursor |
| `Mouse Move` | Change value at cursor position based on horizontal mouse position (left = 0, middle third = proportional, right = max) |
| `ESC` | Exit application |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library


依存関係:
{}

## ファイル階層ツリー
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 NOTE_ON_VISUALIZATION.md
📖 README.md
📄 _config.yml
📁 generated-docs/
📁 issue-notes/
  📖 10.md
  📖 11.md
  📖 14.md
  📖 16.md
  📖 18.md
  📖 20.md
  📖 21.md
  📖 22.md
  📖 23.md
  📖 24.md
  📖 30.md
  📖 32.md
  📖 34.md
  📖 36.md
  📖 38.md
  📖 40.md
  📖 41.md
  📖 42.md
📁 src/
  📄 app.rs
  📄 file_ops.rs
  📄 main.rs
  📄 models.rs
  📄 register.rs
  📄 ui.rs

## ファイル詳細分析


## 関数呼び出し階層
関数呼び出し階層を分析できませんでした

## プロジェクト構造（ファイル一覧）
NOTE_ON_VISUALIZATION.md
README.md
issue-notes/10.md
issue-notes/11.md
issue-notes/14.md
issue-notes/16.md
issue-notes/18.md
issue-notes/20.md
issue-notes/21.md
issue-notes/22.md
issue-notes/23.md
issue-notes/24.md
issue-notes/30.md
issue-notes/32.md
issue-notes/34.md
issue-notes/36.md
issue-notes/38.md
issue-notes/40.md
issue-notes/41.md
issue-notes/42.md

上記の情報を基に、プロンプトで指定された形式でプロジェクト概要を生成してください。
特に以下の点を重視してください：
- 技術スタックは各カテゴリごとに整理して説明
- ファイル階層ツリーは提供された構造をそのまま使用
- ファイルの説明は各ファイルの実際の内容と機能に基づく
- 関数の説明は実際に検出された関数の役割に基づく
- 関数呼び出し階層は実際の呼び出し関係に基づく


---
Generated at: 2025-11-17 07:07:45 JST
