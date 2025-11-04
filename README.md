# ym2151-tone-editor

A Windows-compatible Rust TUI (Text User Interface) editor for editing 2-digit numeric values.

## Features

- Display 10 columns × 5 rows of 2-digit numbers (0-99)
- Vim-style cursor navigation with `hjkl` keys
- Increase/decrease values with `e`/`q` keys
- Exit with `ESC` key

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