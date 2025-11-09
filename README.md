# ym2151-tone-editor

A Windows-compatible Rust TUI (Text User Interface) editor for YM2151 (OPM) FM synthesis tone parameters.

## 状況

開発中です。現在の進捗率は1%ざっくり

## Features

- Edit YM2151 tone parameters with parameter labels
- Display 10 parameters × 5 rows (4 operators + 1 channel row)
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, ALG
- Vim-style cursor navigation with `hjkl` keys
- Increase/decrease values with `e`/`q` keys (respects parameter max values)
- Exit with `ESC` key
- Initialized with a basic FM piano-like tone

## YM2151 Tone Data Format

This editor uses a provisional tone data format based on the YM2151 register map:

### Parameters (10 columns)

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
| ALG | Algorithm | 0-7 | FM algorithm selection (3 bits) |

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

## Controls

| Key | Action |
|-----|--------|
| `h` | Move cursor left |
| `j` | Move cursor down |
| `k` | Move cursor up |
| `l` | Move cursor right |
| `q` | Decrease value at cursor |
| `e` | Increase value at cursor |
| `ESC` | Exit application |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library
