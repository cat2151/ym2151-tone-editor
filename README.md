# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM synthesizer tone editor. For Windows. A Rust TUI (Text User Interface) editor.

## Status

Currently under development. Current progress is roughly 50%.

- Future Outlook
    - \*All specifications are temporary for verification purposes and subject to frequent breaking changes.
    - Tone saving format suitable for GitHub management. Tone data itself described in approximately 100 characters per line. See below.
    - Significant keybind changes. See below.

## Features

- Edit YM2151 tone parameters with labeled parameters.
- Display 11 parameters across 5 rows (4 operators + 1 channel row).
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS.
- Arrow keys for cursor navigation.
- PageUp/PageDown or `e`/`q` keys to increase/decrease values (respecting parameter maximums).
- Fast value setting with Home (max), End (min), R (random).
- `P` or `SPACE` key to play the currently edited tone (preview sound without changing parameter values).
- `ESC` key to exit.
- Saves tone as JSON on exit and loads the latest JSON on next launch.

## YM2151 Tone Data Format

### Parameters

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

The editor automatically ensures the server is ready by using the `ensure_server_ready()` function from the `ym2151-log-play-server` library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Interactive Mode (Default)

In interactive mode, the server continuously streams audio, and only register write commands are sent when parameters change. This provides more efficient and smoother audio feedback.

#### Legacy Mode

By default, the editor uses `send_json` to transmit complete tone data in JSON format via a named pipe. Each time a parameter is changed, the entire new JSON is sent.

### Comparison

| Feature | Legacy Mode | Interactive Mode |
|---------|-------------|------------------|
| Data Transmission | Full JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter changes | Continuous streaming |
| Usage | For comparison/verification | For regular editing |

## How to Operate

\*Subject to breaking changes in the future, for verification purposes.

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in corresponding direction |
| **Value Adjustment** | |
| `PageUp` / `e` | Increase value at cursor position |
| `PageDown` / `q` | Decrease value at cursor position |
| `+` / `.` | Increase value by 1 |
| `-` / `,` | Decrease value by 1 |
| `Shift` + `.` (`>`) | Increase value by 10 |
| `Shift` + `,` (`<`) | Decrease value by 10 |
| `Home` | Set to maximum value for current parameter |
| `End` | Set to minimum value (0) |
| `r` / `R` | Set to random value (within valid range) |
| **Mouse** | |
| `Mouse Wheel Up` | Move cursor to mouse pointer position and increase value |
| `Mouse Wheel Down` | Move cursor to mouse pointer position and decrease value |
| **Other** | |
| `ESC` | Save and exit application |

## Command Line Options

| Option | Description |
|--------|-------------|
| `--value-by-mouse-move` | Enable legacy mouse behavior (change value at cursor position with horizontal mouse movement) |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concepts
- Launches in 100ms, plays sound in 100ms. \*Numbers are rough; image is significantly faster than 1 second.
- Pressing a key plays sound and changes the tone.
    - Prioritize addressing the issue of "it doesn't play sound or allow editing, and it's confusing."
- Colorful visualization.
- Simple.
- Easy-to-learn operation (cursor, mouse) for basic editing.

## Out of Scope, Not Aimed For
- **Feature-rich editor:**
    - A perfect, versatile editor that satisfies all users from beginners to super-advanced.
    - Unlimited intelligent UNDO.
    - Fully automated, easy-to-use, error-free, flexible, and advanced editing features.
- **Interactive:**
    - Highly interactive performance using a virtual MIDI keyboard; changing the server to use shared memory for low-latency, advanced real-time processing.
    - Responsive, highly interactive performance in general.
- **GUI:**
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates.
- **Advanced Librarian:**
    - Quick and easy access, preview, selection, editing, and highly intelligent version control for all tones with flexible operations.
    - Fully automatic or interactive advanced tone extraction from existing music, with 100% success rate.
    - Automatically detect and load all YM2151 tone formats, with 100% detection success rate.
    - Automatically detect and convert all FM tone formats for loading, with 100% success rate.
- **Advanced Extensibility:**
    - Advanced tone creation using automation.
    - Advanced tone creation using all 8 channels, and even multiple YM2151s.
    - Support all FM sound sources beyond the YM2151 framework.
    - Compatible with all DAWs and audio plugins, enabling playback and import/export of FM tone data for each.

## Considering Tone Storage Format
- Past Issues
    - ym2151-log format:
        - Verbose JSON data.
        - Cannot store multiple tone variations in one file.
        - Maintaining this on GitHub for General MIDI is not very realistic.
        - Will continue to be used for server transmission. However, there's a feeling that a more appropriate format is needed for tone management.
### Proposed Solution
- Operation
    - Placement:
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits:
            - Self-descriptive:
                - Directory hierarchy and filenames make the purpose and tone clear.
    - Commit:
        - Commit to the `ym2151-tone-editor` repository at a frequency of 0-1 times per day.
- File Format
```json
{
  "description": "GM:000 Acoustic Grand Piano family",
  "variations": [
    { "description": "GM:000 Bright Piano", "mml": "t120 o5 l4 cdefgab", "registers": "204F204C364037808003812D" },
    { "description": "GM:000 Soft Piano", "note_number": 60, "registers": "204F204C364037808001812D" }
  ]
}
```
- JSON File Format Explanation
    - The core is `registers`. This is a required field.
    - `mml`, `note_number`, and `description` are optional fields.
    - If `mml` and `note_number` are omitted, what plays is left to the application, e.g., middle C.
    - If both `mml` and `note_number` are provided, which one plays is also left to the application, e.g., `note_number`, then `mml`, alternating.
- Data Format Explanation
    - Address and Data:
        - Repeating pairs of 2-character address and 2-character data.
    - Benefits:
        - Structured:
            - Being JSON, it has no ambiguity like natural language and can be read/written with simple code.
        - Flexibility:
            - If a format were fixed to specific registers and a specific description method, problems like those below might arise, which this approach avoids:
                - E.g., This format lacks necessary information.
                - E.g., High cost for format consideration (how much information is sufficient for the format).
                - E.g., Later format changes require modifications to parser/output code or migration.
                    - Format changes include changes in description methods or additions/removals of target registers.
        - Self-descriptive:
            - `description` ensures readability and self-descriptiveness, as do directory and filenames.
                - Being JSON also contributes to this.
        - Variations:
            - In practice, even GM000 might have many variations, so this is handled by storing them in an array within the JSON.
        - Readability:
            - High readability if written on one line with `description` at the beginning. Intended to be treated as a list of tone variation names.
        - Portability:
            - Highly portable format, easy to write mutual conversion code at this level.
        - Uniqueness:
            - Intended to gain some benefit of uniqueness by using `registers` as a unique ID.
                - Benefit: Can detect duplicates, potentially preventing excessive tone library bloat.
                - Benefit: Can be used as an ID to uniquely identify a tone.
                    - Searchable even if the description changes.
                    - Could simplify various handling.
                - Benefit: Searching by `registers` reveals "this is YM2151 tone data from someone's repository." The data is self-descriptive.
                    - For this, `registers` must maintain a format without delimiters.
                    - Precondition: Data is registered under GitHub management and the registration location is self-descriptive.
                - Note: This is only to a certain extent. Even nearly identical tones will have different IDs if a single bit differs.
    - Supplement:
        - Slot Mask:
            - By including "note on" in `registers`, the slot mask can be represented. The application can extract the slot mask from there. `ym2151-tone-editor` already implements this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, etc.
        - Saving all 256 bytes of register information in JSON is not recommended. This carries the risk of unexpected application behavior.
            - Detailed examination and consideration of this will be deferred (YAGNI). It's assumed to be manageable on the application side later.
        - Note that advanced playing techniques like modulator TL automation cannot be included in this tone data.
            - This means "advanced performance technique-including tone data" that cannot be expressed by this format may exist, and compatibility with such data will be limited.
- Issues and Solutions
    - Issue: 128 items is cumbersome.
    - Solution: It is assumed that a simple code snippet would be sufficient to handle this.
        - For instance, preparing a list of 128 tone names and a simple script would make JSON filename and description generation easier.

## Considering Keybinds
- \*Each will be separated into individual issues. Safety first. Prevent confusion.
- \*Intended to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.
- Concept:
    - Basic operations can be completed using just arrow keys and Page Up/Down.
    - Supplement:
        - Shortcut keys supplement quick editing and advanced features.
        - Left-click to move the cursor, mouse wheel to increase/decrease values, is also standard, so it will be implemented.
            - Right-click is confusing in a TUI, so it's better to avoid it.
        - The idea is that for some functions like exiting, `ESC` alone is sufficient, as it is standard.
- `+` and `-` to increase/decrease values. This is widely known and easily understood, thus improving UX for adoption.
- `CTRL + hjkl` for cursor movement. `CTRL + npfb` also for cursor movement.
    - While movement without arrow keys is already possible with other shortcut keys, using these could improve UX, especially for beginners.
- `P` and `SPACE` for playback. Being able to repeatedly play the current sound improves UX.
- `F` to increase FB, `SHIFT + F` to decrease FB. The cursor also moves to FB.
    - It's assumed that other similar operations will involve a cursor jump and value change together for speed. This will be verified.
- `T` to increase TL for the current row, `SHIFT + T` to decrease.
- `M` to increase MUL for the current row, `SHIFT + M` to decrease.
    - Memo: If `M` conflicts with a higher priority, use `X`. The letter `x` is close in meaning to "multiple."
- `A, D, S, R` to increase AR, D1R, D2R, RR for the current row, `SHIFT +` to decrease.
    - Supplement: Discontinue `WASD` for cursor movement. It felt error-prone for this purpose, with no perceived benefit. It's assumed errors were frequent due to needing to constantly shift the left hand one position left from the home row.
- `L` to increase D1L, `SHIFT + L` to decrease.
    - `L` for D1L. The explanation in the heading is easy to understand.
- `1, 2, 3, 4` to directly move to M1, C1, M2, C2 rows and increase the value in the current cursor column.
    - `SHIFT` key pressed for decrease.
    - Intended use: for quickly adjusting values across multiple operators.
        - Example: If you're working on OP1 and want to increase OP4, using `4` is one press compared to three arrow key presses and a Page Up, making it 4 times faster.
    - Note: Numbers are relatively difficult for touch typing, so `hjkl` will also be tested as aliases.
- `5, 6, 7, 8` to toggle SlotMask for OP1-4.
    - `SHIFT` pressed for solo mode toggle.
        - Even for modulators, in solo mode, playback is forced with ALG7 to check envelopes, etc. In this case, the forced SlotMask is ON only for that row.
            - The ALG and SM at this time should be clearly indicated with special colors or background colors.
        - The row with the cursor is always in solo mode, meaning SM dynamically changes with cursor movement.
        - Toggling off returns to the ALG held just before toggling on.
            - For simplicity, any of `SHIFT + 5, 6, 7, 8` will toggle off.
                - This means no soloing two operators. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for "locK" makes it easy to explain visually.
    - When locked, pressing the `F` key etc. does not move the cursor.
        - Multiple lock targets are possible. Each becomes a target for value increase/decrease by mouse.
        - Intended use: previewing envelopes while increasing/decreasing them together.
    - When not locked, mouse behavior: left-click moves the cursor and increases the value, right-click moves the cursor and decreases the value.
- `.` and `,` for Note down and up. Using the C Ionian scale centered on middle C.
    - However, these are also strong candidates for value increase/decrease, so keybind changes are anticipated in the future.