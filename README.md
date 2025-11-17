# ym2151-tone-editor

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
- Cursor navigation with arrow keys, `hjkl` (Vim-style), or `wasd` keys
- Increase/decrease values with PageUp/PageDown or `e`/`q` keys (respects parameter max values)
- Quick value setting with Home (max), End (min), and R (random)
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
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in respective direction |
| `h` / `a` | Move cursor left |
| `j` / `s` | Move cursor down |
| `k` / `w` | Move cursor up |
| `l` / `d` | Move cursor right |
| **Value Modification** | |
| `PageUp` / `e` | Increase value at cursor |
| `PageDown` / `q` | Decrease value at cursor |
| `Home` | Set value to maximum for current parameter |
| `End` | Set value to minimum (0) |
| `r` / `R` | Set value to random (within valid range) |
| **Other** | |
| `Mouse Move` | Change value at cursor position based on horizontal mouse position (left = 0, middle third = proportional, right = max) |
| `ESC` | Save and exit application |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library
