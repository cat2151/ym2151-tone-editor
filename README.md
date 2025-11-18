# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM tone editor. For Windows. A Rust TUI (Text User Interface) editor.

## Status

Under development. Current progress is roughly 50%.

- Future Plans
    - ※All of these are temporary specifications for verification purposes and are subject to frequent breaking changes.
    - Format suitable for tone saving and GitHub management. Tone data body described in about 100 characters per line. Details below.
    - Significant keybind changes. Details below.

## Features

- Edit YM2151 tone parameters with parameter labels
- Display with 11 parameters × 5 rows (4 operators + 1 channel row)
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS
- Cursor navigation with arrow keys, `hjkl` (Vim style), or `wasd` keys
- Increase/decrease values with PageUp/PageDown or `e`/`q` keys (respecting parameter maximums)
- Fast value setting with Home (max), End (min), R (random)
- Exit with `ESC` key
- Save tone to JSON on exit and load the latest JSON on next startup

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

Alternatively, run the compiled binary directly:

```bash
./target/release/ym2151-tone-editor
```

## Real-time Audio Feedback (Windows only)

The editor automatically ensures the server is ready by using the `ensure_server_ready()` function from the ym2151-log-play-server library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Legacy Mode (Default)

By default, the editor uses `send_json` to send complete tone data in JSON format via a named pipe. Each time a parameter changes, a new complete JSON is sent.

```bash
cargo run
```

#### Interactive Mode (New Feature)

In interactive mode, the server continuously streams audio and only register write commands are sent when parameters change. This provides more efficient and smoother audio feedback.

```bash
cargo run -- --use-client-interactive-mode-access
```

To enable interactive mode:
- The editor calls `start_interactive()` at startup to begin continuous audio streaming on the server
- Uses `write_register()` to update only the affected YM2151 registers when parameters change
- Calls `stop_interactive()` on exit to stop audio streaming

**Note**: The library's `ensure_server_ready()` function handles all server management, including installation if necessary.

### Mode Comparison

| Feature | Legacy Mode | Interactive Mode |
|---------|-------------|------------------|
| Data Transmission | Complete JSON | Register writes only |
| Efficiency | Low (sends all data each time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Use Case | Comparison and verification | Normal editing work |

## How to Use

※Subject to breaking changes in the future, for verification purposes.

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
| Mouse movement | Change value at cursor position based on horizontal mouse position (Left=0, Center=Proportional, Right=Max) |
| `ESC` | Save and exit application |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concept
- Starts in 100ms, plays sound in 100ms ※Numbers are rough. Image is significantly shorter than 1 second.
- Pressing a key plays sound and changes the tone
    - Prioritize addressing the "doesn't make sound when touched, can't edit, unclear" issue.
- Colorful visualization
- Simple
- Easy-to-learn operation for minimal editing (cursor, mouse)

## Out of Scope, Not Aimed For
- High-end editor
    - A perfect, versatile editor that satisfies everyone from beginners to super-advanced users
    - Unlimited intelligent UNDO
    - Various intelligent, fully automatic, easy-to-understand, error-free, flexible, and advanced editing features
- Interactive
    - Highly interactive performance with a virtual MIDI keyboard, changing the server to advanced low-latency real-time processing using shared memory
    - Highly interactive performance with good responsiveness in general
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates
- Advanced librarian
    - Easy and quick access, preview, selection, editing, and highly intelligent version management for all tones with flexible operations
    - Fully automatic or interactive advanced tone extraction from existing songs, with 100% success rate
    - Automatic detection and loading of all YM2151 tone formats, with 100% automatic detection success rate
    - Automatic detection and conversion of all FM tone formats for loading, with 100% success rate
- Advanced extensibility
    - Advanced tone creation using automation
    - Advanced tone creation utilizing all 8 channels, and even multiple YM2151s
    - Support for all FM sound sources beyond the YM2151 framework
    - Support for all DAWs and audio plugins, enabling playback for each, and import/export of FM tone data for each

## Considering a Tone Saving Format
- Previous Issues
    - ym2151-log format
        - JSON data with many lines.
        - Cannot include multiple tone variations in one file.
        - Maintaining this as-is on GitHub for General MIDI is not very realistic.
        - Will continue to use it for server transmission. However, the feeling is that a more suitable format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-describing
                - Purpose and tone are clear from directory hierarchy and file name
    - Commit
        - Commit to the `ym2151-tone-editor` repository at a frequency of 0 to 1 times per day
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
- JSON File Format Explanation
    - The main body is `registers`. This is a required field.
    - `mml`, `note_number`, `description` are optional fields.
    - If `mml` and `note_number` are omitted, what plays is up to the app, for example, middle C.
    - If both `mml` and `note_number` are written, which one plays is also up to the app, for example, `note_number` then `mml` playing alternately.
- Data Format Explanation
    - Address and Data
        - Pairs of 2-character address, 2-character data, repeated.
    - Benefits
        - Structured
            - Being JSON, there is no ambiguity like natural language, and it can be read and written with simple code.
        - Flexibility
            - If the format were limited to specific registers and fixed to a specific description method, the following problems could arise, but these can be avoided:
                - E.g., this format lacks necessary information.
                - E.g., how much to record to make it a sufficient format, incurring format design costs.
                - E.g., later format changes would require modifications to parser/output code or migration.
                    - Format changes include changes in description methods, or increasing/decreasing target registers.
        - Self-describing
            - Readability and self-describing nature are ensured by `description`, and similarly by directory and file names.
                - The same applies to being JSON.
        - Variations
            - Practically, even GM000 can have many variations, so,
                - this is handled by holding them in an array within JSON.
        - Readability
            - Writing in one line, with `description` at the beginning, offers high readability. Intended to be treated as a list of tone variation names.
        - Portability
            - A highly portable format; at this level, cross-conversion code can be easily written.
        - Uniqueness
            - By using `registers` as a unique ID, some benefits of uniqueness are expected.
                - Benefit: Duplicate detection can help prevent excessive tone library bloat to some extent.
                - Benefit: Can be used as an ID when needing to uniquely identify a tone.
                    - Searchable even if `description` is changed.
                    - Handling various aspects may become easier.
                - Benefit: Searching with `registers` reveals "This is YM2151 tone data from so-and-so's repository." The data is self-describing.
                    - For this reason, `registers` must maintain a format without delimiters.
                    - The prerequisites are that it's registered under GitHub management and the registration location is self-describing.
                - Caution: This is only to a certain extent. Even nearly identical tones will have different IDs if a single bit differs.
    - Supplement
        - slot mask
            - By including "note on" in `registers`, the slot mask can be represented. The application can extract the slot mask from it. `ym2151-tone-editor` has already implemented this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, among other things.
        - Saving all 256 bytes of register information to JSON is not recommended. It's assumed there's a risk of the application behaving unexpectedly.
            - Further investigation and consideration of this will be postponed. YAGNI (You Ain't Gonna Need It). It's assumed the application can handle it later.
        - Please note that advanced performance techniques such as modulator TL automation cannot be included in this tone data.
            - In other words, "tone data containing advanced performance techniques" that cannot be fully expressed by this format may exist, and compatibility with such data will be limited.
- Issues and Solutions
    - Issue: 128 items is cumbersome.
    - Solution: It's assumed that writing simple code for this purpose would be sufficient.
        - For example, if a list of 128 tone names and simple code are prepared, JSON filename generation and description generation are expected to be easy.

## Considering Keybinds
- ※Each will be separated into individual issues. Safety first. Prevent confusion.
- ※Intended to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.
- Concept
    - Basic operations can be completed with just cursor keys and Page Up/Down.
    - Supplement
        - Shortcut keys supplement quick editing and advanced features.
        - Left-click mouse for cursor movement, wheel for value increase/decrease, also standard, so will implement.
            - Right-click is confusing in TUI, so it's better to avoid it.
        - Also, for some functions like exiting, `ESC` alone is sufficient, which is a standard approach.
- Increase/decrease values with `+` and `-`. This is a widely known and easily understood approach, expected to improve onboarding UX.
- Move cursor with CTRL `hjkl`. CTRL `npfb` also for cursor movement.
    - Although cursor-key-less movement can be achieved with other shortcut keys, using these might improve UX, especially during initial use.
- Play with `P` and `space`. Being able to repeatedly play the current sound improves UX.
- Increase FB with `F`, decrease FB with SHIFT+`F`. The cursor also moves to FB.
    - Other similar operations are also expected to perform cursor jumps and value changes together, which should be fast. Will verify.
- Increase TL of current row with `T`, decrease with SHIFT+`T`.
- Increase MUL of current row with `M`, decrease with SHIFT+`M`.
    - Memo: If `M` is prioritized elsewhere, use `X`. `X` is conceptually close to "multiple".
- Increase AR, D1R, D2R, RR of current row with `A`, `D`, `S`, `R`, decrease with SHIFT+.
    - Supplement: Discontinue WASD for cursor movement. It led to many errors for this purpose, and no benefits were felt. It was assumed that errors frequently occurred because the left hand constantly had to shift one position left from the home row.
- Increase D1L with `L`, decrease with SHIFT+`L`.
    - `L` for D1L. Easy to understand from the heading.
- With 1, 2, 3, 4, move directly to the M1, C1, M2, C2 rows respectively, while incrementing the value in the current cursor column.
    - Decrease if SHIFT key is pressed.
    - Purpose: For quickly increasing/decreasing values across operators.
        - E.g., if working on OP1 and want to increment OP4, compared to 3 cursor key presses and page up,
            - pressing `4` is one press, making it 4 times faster.
    - Note: Numbers are relatively difficult for touch typing, so `hjkl` aliases will also be evaluated.
- Toggle SlotMask for OP1-4 with 5, 6, 7, 8.
    - Toggle solo mode if SHIFT is pressed.
        - Even if it's a modulator, playing with forced ALG7 during solo mode is for
            - checking envelopes etc. And forced SlotMask will be ON only for that row.
                - ALG and SM in this state should be a special color or background color for clarity.
        - The row with the cursor will always be in solo mode, meaning SM changes dynamically with cursor movement.
        - When solo mode is untoggled, it returns to the ALG held just before toggling it on.
            - Untoggling will be done with any of SHIFT+5,6,7,8 for a simple initial specification.
                - Meaning, no solo for two operators. Simplicity first.
- Toggle mouse multi-cursor lock with `K`. Easy to explain with `locK` (visual hint).
    - When locked, pressing F key etc. will not move the cursor.
        - Multiple items can be locked. Each becomes subject to value increase/decrease by mouse.
        - Intended use case: for previewing while adjusting multiple envelope parameters simultaneously.
    - When not locked, mouse behavior is as follows:
        - Left-click moves cursor to location and increments value,
        - Right-click moves cursor to location and decrements value.
- Use `,` and `.` for Note down and up respectively. Use a C Ionian scale centered on middle C.
    - However, these are also strong candidates for value increment/decrement, so future keybind changes are anticipated.