# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM sound source tone editor. For Windows. Rust TUI (Text User Interface) editor.

## Status

Under development. Current progress is roughly 50%.

- Future Outlook
    - *All are provisional specifications for testing and are subject to frequent breaking changes.*
    - A format suitable for tone saving and GitHub management. Tone data itself described in approximately 100 characters per line. See below.
    - Significant keybind changes. See below.

## Features

- Edit YM2151 tone parameters with labels
- Display 11 parameters × 5 rows (4 operators + 1 channel row)
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS
- Cursor navigation with arrow keys, `hjkl` (Vim style), or `wasd` keys
- Increase/decrease values with PageUp/PageDown or `e`/`q` keys (respects parameter maximums)
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

Or, run the compiled binary directly:

```bash
./target/release/ym2151-tone-editor
```

## Real-time Audio Feedback (Windows Only)

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the ym2151-log-play-server library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Legacy Mode (Default)

By default, the editor sends complete tone data in JSON format via named pipes using `send_json`. Each time a parameter is changed, the entire new JSON is sent.

```bash
cargo run
```

#### Interactive Mode (New Feature)

In interactive mode, the server continuously streams audio and only sends register write commands when parameters are changed. This provides more efficient and smoother audio feedback.

```bash
cargo run -- --use-client-interactive-mode-access
```

To enable interactive mode:
- The editor calls `start_interactive()` on startup to begin continuous audio streaming on the server.
- It uses `write_register()` to update only the affected YM2151 registers when parameters change.
- It calls `stop_interactive()` on exit to stop audio streaming.

**Note**: The library's `ensure_server_ready()` function handles all server management, including installation if necessary.

### Mode Comparison

| Feature | Legacy Mode | Interactive Mode |
|------|---------------|---------------------|
| Data Transmission | Complete JSON | Register Writes Only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Use Case | For comparison/verification | For regular editing |

## How to Use

*Subject to breaking changes in the future. For testing purposes.*

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in respective direction |
| `h` / `a` | Move cursor left |
| `j` / `s` | Move cursor down |
| `k` / `w` | Move cursor up |
| `l` / `d` | Move cursor right |
| **Value Modification** | |
| `PageUp` / `e` | Increase value at cursor position |
| `PageDown` / `q` | Decrease value at cursor position |
| `Home` | Set to current parameter's maximum value |
| `End` | Set to minimum value (0) |
| `r` / `R` | Set to random value (within valid range) |
| **Mouse** | |
| `Mouse wheel up` | Move cursor to mouse pointer position and increase value |
| `Mouse wheel down` | Move cursor to mouse pointer position and decrease value |
| **Others** | |
| `ESC` | Save and exit application |

## Command Line Options

| Option | Description |
|--------|-------------|
| `--use-client-interactive-mode-access` | Use interactive mode for more efficient audio feedback (continuously streams audio and sends only register changes) |
| `--value-by-mouse-move` | Enable legacy mouse behavior (change value at cursor position by moving mouse left/right) |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concept
- Starts in 100ms, plays sound in 100ms *Numbers are approximate. Imagine significantly less than 1 second.*
- Pressing a key plays sound and changes the tone.
    - Prioritize addressing "Can't play, can't edit, unclear."
- Colorful visualization
- Simple
- Easy-to-learn operation for basic editing (cursor, mouse)

## Out of Scope, Not Aimed For
- High-functionality Editor
    - A perfect, versatile editor that satisfies everyone from beginners to super-advanced users.
    - Unlimited, intelligent UNDO.
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features.
- Interactive
    - Highly interactive performance via virtual MIDI keyboard, with server also switched to low-latency, advanced real-time processing using shared memory.
    - Generally highly interactive performance with good responsiveness.
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display refresh rate.
- Advanced Librarian
    - Easy and quick access, preview, selection, editing, and highly intelligent version management for all tones with flexible operations.
    - Fully automatic or interactive advanced tone extraction from existing songs, with 100% success rate.
    - Automatic detection and loading of all YM2151 tone formats, with 100% success rate.
    - Automatic detection and conversion of all FM tone formats for loading, with 100% success rate.
- Advanced Extensibility
    - Advanced tone creation using automation.
    - Advanced tone creation using all 8 channels, and even multiple YM2151s.
    - Support for all FM sound sources beyond the YM2151 framework.
    - Compatibility with all DAWs and audio plugins, allowing playback and import/export of FM tone data for each.

## Considering a Tone Saving Format
- Past Issues
    - ym2151-log format
        - JSON data with many lines.
        - Cannot include multiple tone variations in one file.
        - Maintaining this as-is for General MIDI on GitHub is not very practical.
        - It will continue to be used for server transmission. However, there's a feeling that a more suitable format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-descriptiveness
                - Directory structure and file names make usage and tones clear.
    - Commit
        - Commit to the ym2151-tone-editor repository 0-1 times a day.
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
    - `mml`, `note_number`, `description` are optional fields.
    - If `mml` and `note_number` are omitted, what plays is up to the app, e.g., middle C.
    - If both `mml` and `note_number` are provided, which one plays is also up to the app, e.g., alternating between `note_number` and `mml`.
- Data Format Description
    - Address and Data
        - Repeats pairs of 2-character address, 2-character data.
    - Benefits
        - Structured
            - Being JSON, it avoids natural language ambiguity and allows simple code for reading and writing.
        - Flexibility
            - If the format were restricted to specific registers and fixed to a particular description method, problems like the following might arise, but these are avoided:
                - E.g., This format lacks necessary information.
                - E.g., How much to record to make a sufficient format, incurring format consideration costs.
                - E.g., Later format changes requiring parser/output code modification or migration.
                    - Format changes include changes in description method or increases/decreases in target registers.
        - Self-descriptiveness
            - `description` ensures readability and self-descriptiveness, as do directory and file names.
                - Being JSON also contributes.
        - Variations
            - Practically, GM000 can have many variations, so:
                - This is addressed by storing them in an array within the JSON.
        - Readability
            - Written on a single line, with `description` at the beginning, readability is high. Intended to be treated as a list of tone variation names.
        - Portability
            - Highly portable format; at this level, it's easy to write mutual conversion code.
        - Uniqueness
            - By using `registers` as a unique ID, some benefits of uniqueness are expected.
                - Benefit: Duplicate detection can help prevent excessive tone library bloat to some extent.
                - Benefit: Can be used as an ID when you want to uniquely identify a tone.
                    - Can be searched even if the description changes.
                    - May simplify various handling tasks.
                - Benefit: Searching by `registers` reveals "This is YM2151 tone data from so-and-so's repository." The data is self-descriptive.
                    - Therefore, `registers` must maintain a format without delimiters.
                    - The premise is that it's registered under GitHub management and the registration location is self-descriptive.
                - Note: This is only to a certain extent. Even nearly identical tones will have different IDs if they differ by 1 bit.
    - Supplement
        - Slot Mask
            - By including `note on` in `registers`, the slot mask can be expressed. The app can extract the slot mask from it. ym2151-tone-editor has already implemented this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, for example.
        - Saving all 256 bytes of register information to JSON is not recommended. This carries the risk of the app behaving unexpectedly.
            - Thorough review and consideration of that will be postponed. YAGNI. It's assumed to be addressable by the app later.
        - Note that advanced performance techniques such as modulator TL automation cannot be included in this tone data.
            - This means that "tone data containing advanced performance techniques" that cannot be fully expressed in this format may exist, and compatibility with such data will be limited.
- Issues and Countermeasures
    - Issue: 128 items is tedious.
    - Countermeasure: It's assumed that writing simple code for this will be sufficient.
        - For example, preparing a list of 128 tone names and simple code would make JSON filename and description generation easy.

## Considering Keybinds
- *Each will be separated into individual issues. Prioritize safety. Prevent confusion.*
- *Assumed to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.*
- Concept
    - Basic operations can be completed with just cursor keys and Page Up/Down.
    - Supplement
        - Supplement with shortcut keys for quick editing and advanced features.
        - Left-click to move cursor, mouse wheel to increase/decrease values are also standard, so they will be implemented.
            - Right-click is confusing in TUI, so it's best to avoid it.
        - For some functions like exit, ESC alone is sufficient; this is considered standard.
- Increase/decrease values with `+` and `-`. This is widely known and easy to understand, improving introductory UX.
- CTRL hjkl for cursor movement. CTRL npfb also for cursor movement.
    - While movement without arrow keys can be achieved with other shortcut keys, using these might improve UX, especially for new users.
- `P` and `Space` for playback. Being able to repeatedly play the current sound improves UX.
- `F` to increase FB, `SHIFT+F` to decrease FB. Cursor also moves to FB.
    - Similar operations are also expected to perform cursor jump and value increment/decrement as a set, which would be fast. This will be verified.
- `T` to increase TL for the current row, `SHIFT+T` to decrease.
- `M` to increase MUL for the current row, `SHIFT+M` to decrease.
    - Memo: If `M` is prioritized for something else, use `X`. `X` is close in meaning to `multiple`.
- `A, D, S, R` to increase AR, D1R, D2R, RR for the current row, `SHIFT+` to decrease.
    - Note: Discontinue WASD for cursor movement. It resulted in many errors for this purpose, and no benefits were perceived. It was assumed to cause many errors due to needing to shift the left hand one position left from home row constantly.
- `L` to increase D1L, `SHIFT+L` to decrease.
    - `L` for D1L. The heading makes it easy to understand.
- `1, 2, 3, 4` to directly move to M1, C1, M2, C2 rows while increasing the value in the cursor's column.
    - `SHIFT` key pressed makes it decrease.
    - Purpose: For quickly incrementing/decrementing values across operators.
    - E.g., If working on OP1 and want to increase OP4, compared to 3 cursor key presses and Page Up,
        - using `4` is 1 press, making it 4 times faster.
    - Note: Numbers are relatively hard to touch-type, so `hjkl` aliases will also be evaluated.
- `5, 6, 7, 8` to toggle SlotMask for OP1-4.
    - `SHIFT` pressed toggles solo mode.
        - Even for a modulator, solo mode forces playback with ALG7,
            - for checking envelopes, etc. And the forced SlotMask is enabled only for that row.
                - ALG and SM in this state should be made clear with a special color or background.
        - The row with the cursor is always in solo mode, meaning SM changes dynamically with cursor movement.
        - Toggling off returns to the ALG held just before solo mode was toggled on.
            - Toggling off will be `SHIFT+5,6,7,8` for any of them, a simple specification first.
                - That is, no solo for two operators. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for `locK` makes it easy to explain.
    - When locked, pressing `F` or other keys will not move the cursor.
        - Multiple targets can be locked. Each becomes a target for value increment/decrement by mouse.
        - Intended use: For previewing while simultaneously increasing/decreasing multiple envelope parameters.
    - When not locked, mouse behavior is:
        - Left-click moves cursor to position and increments value,
        - Right-click moves cursor to position and decrements value.
- `, .` for Note down and up. Use a C Ionian scale centered around middle C.
    - However, since these are also strong candidates for value increment/decrement, keybind changes are anticipated in the future.