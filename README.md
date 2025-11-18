# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM synthesizer tone editor. For Windows. A Rust TUI (Text User Interface) editor.

## Status

Currently under development. Progress is roughly 50%.

- Future Plans
    - *All specifications are temporary for validation purposes and are subject to frequent breaking changes.
    - A format suitable for tone saving and GitHub management. Tone data body described in approximately 100 characters per line. See below.
    - Significant keybind changes. See below.

## Features

- Edit YM2151 tone parameters with labels
- Display 11 parameters × 5 rows (4 operators + 1 channel row)
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS
- Cursor navigation with arrow keys, `hjkl` (Vim-style), or `wasd` keys
- Increase/decrease values with PageUp/PageDown or `e`/`q` keys (respects parameter maximums)
- Fast value setting with Home (max), End (min), R (random)
- Exit with `ESC` key
- Save tone as JSON on exit and load the latest JSON on next startup

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

## Requirements

- Rust 1.70 or later

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run
```

Or, run the compiled binary directly:

```bash
./target/release/ym2151-tone-editor
```

## Real-time Audio Feedback (Windows Only)

The editor automatically ensures the server is ready by using the `ensure_server_ready()` function from the ym2151-log-play-server library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

The editor sends performance data via named pipes using `send_json`. This provides immediate playback as you edit.

**Note**: The library's `ensure_server_ready()` function handles all server management, including installation as needed.

## Controls

*Subject to breaking changes in the future. This is for validation purposes.

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in the respective direction |
| `h` / `a` | Move cursor left |
| `j` / `s` | Move cursor down |
| `k` / `w` | Move cursor up |
| `l` / `d` | Move cursor right |
| **Value Modification** | |
| `PageUp` / `e` | Increase value at cursor position |
| `PageDown` / `q` | Decrease value at cursor position |
| `Home` | Set to maximum value for current parameter |
| `End` | Set to minimum value (0) |
| `r` / `R` | Set to random value (within valid range) |
| **Other** | |
| `Mouse movement` | Change value at cursor position based on horizontal mouse position (Left=0, Center=proportional, Right=max) |
| `ESC` | Save and exit application |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concept
- Starts in 100ms, plays sound in 100ms *These values are rough. The idea is significantly faster than 1 second.
- Pressing a key plays a sound and changes the tone
    - Prioritize addressing issues like 'it doesn't play sound when interacted with, it can't be edited, and it's confusing'.
- Colorful visualization
- Simple
- Easy-to-learn controls (cursor, mouse) for basic editing

## Out of Scope, Not Aimed For
- High-functionality editor
    - A perfect, universal editor that satisfies everyone from beginners to super-advanced users
    - Unlimited intelligent UNDO
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features
- Interactive
    - Highly interactive performance via virtual MIDI keyboard, with the server also changed to advanced low-latency real-time processing using shared memory
    - Generally highly interactive and responsive performance
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display refresh.
- Advanced librarian
    - Flexible, clear, and quick access, preview, selection, editing, and highly intelligent version control for all tones
    - Fully automatic or interactive and advanced tone extraction from existing songs, with 100% success rate
    - Automatic detection and loading of all YM2151 tone formats, with 100% detection success rate
    - Automatic detection and conversion of all FM tone formats for loading, with 100% success rate
- Advanced Extensibility
    - Advanced tone creation using automation
    - Advanced tone creation using all 8 channels, and even multiple YM2151s
    - Support for all FM synthesizers beyond the YM2151 framework
    - Support for all DAWs and audio plugins, enabling playback and import/export of FM synthesizer tones for each.

## Considering a Format for Tone Saving
- Previous Issues
    - ym2151-log format
        - JSON data with many lines.
        - Cannot store multiple tone variations in one file.
        - Maintaining this as-is on GitHub for General MIDI is not very practical.
        - Will continue to use it for server transmission. However, there's a feeling that a more suitable format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Pros
            - Self-descriptiveness
                - Directory structure and file names clearly indicate purpose and tone
    - Commit
        - Commit to the ym2151-tone-editor repository 0-1 times a day
- File Format
```
{
  "description": "GM:000 Acoustic Grand Piano family",
  "variations": [
    { "description": "GM:000 Bright Piano", "mml": "t120 o5 l4 cdefgab", "registers": "204F204C364037808003812D" },
    { "description": "GM:000 Soft Piano", "note_number": 60, "registers": "204F204C364037808001812D" }
  ]
}
```
- JSON File Format Description
    - The core is `registers`. This is a required field.
    - `mml`, `note_number`, and `description` are optional fields.
    - If `mml` and `note_number` are omitted, what plays is up to the application, e.g., middle C.
    - If both `mml` and `note_number` are provided, which one plays is also up to the application, e.g., `note_number`, then `mml`, alternating.
- Data Format Description
    - Address and Data
        - Repetition of pairs: 2 characters for address, 2 characters for data.
    - Pros
        - Structured
            - Being JSON, it avoids the ambiguity of natural language and allows for simple code to read and write.
        - Flexibility
            - If the format were to focus only on specific registers and fix their description method, it might encounter the following issues, which can be avoided here:
                - E.g., This format lacks necessary information.
                - E.g., How much information is enough for a sufficient format, increasing format design costs.
                - E.g., Later format changes would require changes to parser/output code and migrations.
                    - Format changes include changes in description methods or additions/removals of target registers.
        - Self-descriptiveness
            - `description` ensures readability and self-descriptiveness, as do directory and file names.
                - Being JSON also contributes to this.
        - Variations
            - In practice, GM000 can have many variations, so:
                - This is handled by storing them as an array within the JSON.
        - Readability
            - Written on a single line, with `description` at the beginning, it offers high readability. Intended to be treated as a list of tone variation names.
        - Portability
            - A highly portable format; cross-conversion code should be easy to write at this level.
        - Uniqueness
            - Using `registers` as a unique ID provides some benefits of uniqueness.
                - Pro: Duplicate detection can help prevent excessive tone library bloat to some extent.
                - Pro: Can be used as an ID when needing to uniquely identify a tone.
                    - Can be searched even if the description changes.
                    - Makes various handling easier.
                - Pro: Searching using `registers` can identify "this is a YM2151 tone data from so-and-so's repository." The data has self-descriptiveness.
                    - Therefore, `registers` must maintain a format without delimiters.
                    - The prerequisites are that it's registered under GitHub management and the registration location is self-descriptive.
                - Note: This is only to a certain extent. Even nearly identical tones will have different IDs if a single bit differs.
    - Supplement
        - Slot Mask
            - By including "note on" in `registers`, the slot mask can be represented. The application can extract the slot mask from there. ym2151-tone-editor has already implemented this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, among others.
        - Saving all 256 bytes of register information to JSON is not recommended. This is due to the risk of unexpected application behavior.
            - Further scrutiny and consideration of this will be postponed. YAGNI. It's assumed to be addressable by the application later.
- Issues and Solutions
    - Issue: 128 items is cumbersome
    - Solution: It's assumed that this can be adequately addressed by writing simple code for it.
        - For example, if a list of 128 tone names is prepared and simple code is provided, JSON filename generation and description generation are expected to be easy.

## Considering Keybinds
    - *Each should be separated into individual issues. Prioritize safety. Prevent confusion.
    - *Intended to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.
- Concept
    - Basic operations can be completed with just cursor keys and Page Up/Down.
    - Supplement
        - Supplement with shortcut keys for quick editing and advanced features.
        - Left-click for cursor movement, and mouse wheel for value increment/decrement are also standard, so they will be implemented.
            - Right-click is unclear in TUI, so it's better to avoid it.
        - Furthermore, the idea is that some functions, like exiting, can be handled by ESC alone, as that is standard.
- Use `+` and `-` for value increment/decrement. This is widely known and easy to understand, thus improving the initial UX.
- CTRL + `hjkl` for cursor movement. CTRL + `npfb` also for cursor movement.
    - Although movement without cursor keys can be achieved with other shortcut keys, having these options can improve UX, especially during initial use.
- `P` and `Space` for playback. Being able to repeatedly play the current sound improves UX.
- `F` for FB increase, SHIFT+`F` for FB decrease. Cursor also moves to FB.
    - Other similar operations are also expected to perform cursor jump and value increment/decrement together for speed. This will be validated.
- `T` for TL increase in the current row, SHIFT+`T` for decrease.
- `M` for MUL increase in the current row, SHIFT+`M` for decrease.
    - Memo: If M is prioritized elsewhere, use X. 'x' is close in meaning to 'multiple'.
- `A`, `D`, `S`, `R` for AR, D1R, D2R, RR increase in the current row, SHIFT+ for decrease.
    - Note: WASD for cursor movement will be discontinued. It led to many errors for this purpose, and no benefits were felt. It was assumed that errors were frequent due to the need to constantly shift the left hand one position left from the home row.
- `L` for D1L increase, SHIFT+`L` for decrease.
    - The 'L' in D1L. This makes the explanation in the heading clear.
- `1`, `2`, `3`, `4` to directly move to M1, C1, M2, C2 rows, while incrementing the value in the current cursor column.
    - SHIFT key held down will decrement.
    - Purpose is to quickly increment/decrement values across operators (OP).
        - E.g., When working on OP1 and wanting to increment OP4: 3 cursor key presses + Page Up, compared to:
            - Just '4' for a single press, making it 4 times faster.
    - Note: Numbers are relatively difficult for touch typing, so `hjkl` aliases will also be considered.
- `5`, `6`, `7`, `8` to toggle SlotMask for OP1-4.
    - SHIFT + key to toggle solo mode.
        - Even for a modulator, solo mode forces playback with ALG7 to:
            - Check envelopes, etc. And the forced SlotMask will be ON only for the current row.
                - In this case, ALG and SM should be made clear with a special color or background color.
        - The row with the cursor will always be in solo mode, meaning the SM changes dynamically with cursor movement.
        - Toggling off returns to the ALG held just before solo mode was toggled on.
            - SHIFT+5,6,7,8 will all toggle off solo mode, initially for simplicity.
                - This means no soloing of two operators simultaneously. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for `locK` makes it easy to explain.
    - When locked, pressing F keys, etc., will not move the cursor.
        - Multiple targets can be locked. Each becomes subject to value increment/decrement by mouse.
        - Intended for previewing while collectively increasing/decreasing envelopes.
    - When not locked, mouse behavior is:
        - Left-click moves cursor to position and increments value.
        - Right-click moves cursor to position and decrements value.
- `,` and `.` for Note down and up. Uses C Ionian scale centered around middle C.
    - However, as these are also strong candidates for value increment/decrement, keybind changes are anticipated in the future.