# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM tone editor. For Windows. TUI. Written in Rust.

## Status

Under development. Current progress is 80%. The remaining 20% involves adding keybinds and tone management.

- Future Outlook
    - * All are temporary specifications for verification and are subject to frequent breaking changes.
    - Build a saving mechanism suitable for tone saving and GitHub management. Details below.
    - Make significant keybind changes. Details below.

## Features

- Edit YM2151 tone parameters
- Operable with mouse alone
- Move with cursor keys, increment/decrement values with PageUp/PageDown/Home/End
- Preview tone with `P` or `SPACE` key
- Exit with `ESC` key
- Automatically saves the tone on exit and automatically loads it next time to continue editing
- Configurable keybinds

## Quick Start Guide

To be written in the future.
If Rust is installed on Windows, it can be easily installed from GitHub.

## Tips
- When you want to display full screen:
  - In Windows Terminal, you can maximize the window with `ALT+ENTER` or `F11`, then enlarge the font with `CTRL`+`+`.

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

## System Requirements

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

The editor automatically ensures the server is ready by using the `ensure_server_ready()` function from the `ym2151-log-play-server` library. This automatically handles server installation, startup, and readiness checks.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Interactive Mode (Default)

In interactive mode, the server continuously streams audio and sends only register write commands when parameters change. This provides more efficient and smoother audio feedback.

#### Legacy Mode

By default, the editor uses `send_json` to transmit complete tone data in JSON format via a named pipe. Each time a parameter is changed, the entire new JSON is sent.

### Comparison

| Feature | Legacy Mode | Interactive Mode |
|---------|---------------|------------------|
| Data Transmission | Full JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restart on parameter change | Continuous streaming |
| Use Case | For comparative verification | Normal editing workflow |

## How to Operate

* Subject to breaking changes in the future. For verification purposes.

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in the respective direction |
| **Value Modification** | |
| `PageUp` / `e` | Increase value at cursor position |
| `PageDown` / `q` | Decrease value at cursor position |
| `+` / `.` | Increase value by 1 |
| `-` / `,` | Decrease value by 1 |
| `Shift` + `.` (`>`) | Increase value by 10 |
| `Shift` + `,` (`<`) | Decrease value by 10 |
| `Home` | Set to maximum value of current parameter |
| `End` | Set to minimum value (0) |
| `r` / `R` | Set to random value (within valid range) |
| **Mouse** | |
| `Mouse wheel up` | Move cursor to mouse pointer position and increase value |
| `Mouse wheel down` | Move cursor to mouse pointer position and decrease value |
| **Other** | |
| `ESC` | Save and exit application |

## Command Line Options

| Option | Description |
|--------|-------------|
| `--value-by-mouse-move` | Enable legacy mouse behavior (change value at cursor position by moving mouse left/right) |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concepts
- Starts in 100ms, plays sound in 100ms *Rough numbers. The idea is significantly shorter than 1 second.*
- Pressing a key plays sound and changes the tone.
    - Prioritize addressing the issue of "doesn't play or edit when touched, unclear how to use."
- Colorful visualization
- Simple
- Easy-to-learn operation for basic editing (cursor, mouse)

## Out of Scope, Not Aiming For
- High-functionality editor
    - A perfect, versatile editor that satisfies everyone from beginners to super-advanced users
    - Unlimited intelligent UNDO
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features
- Interactive
    - Highly interactive performance with a virtual MIDI keyboard, with server also modified for low-latency, advanced real-time processing using shared memory
    - Highly responsive and interactive performance in general
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display refresh.
- Advanced librarian
    - Flexible and intuitive quick access, preview, selection, editing, and highly intelligent version management for all tones
    - Fully automatic or interactive advanced tone extraction from existing songs, with 100% success rate
    - Automatic identification and loading of all YM2151 tone formats, with 100% identification success rate
    - Automatic identification and conversion to load all FM tone formats, with 100% success rate
- Advanced extensibility
    - Advanced tone creation using automation
    - Advanced tone creation using all 8 channels, and even multiple YM2151s
    - Support for all FM sound sources beyond the YM2151 framework
    - Support for all DAWs and audio plugins, enabling playback for each, and import/export of FM sound source tones for each.

## Considering a Format for Tone Saving
- Past Issues
    - ym2151-log format
        - JSON data with many lines.
        - Cannot store multiple tone variations in one file.
        - Maintaining this as is on GitHub for General MIDI is not very practical.
        - Will continue to be used for server transmission. However, there's a feeling that a more suitable format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-describing
                - Directory hierarchy and filenames make the purpose and tone easily understandable.
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
    - The main body is `registers`. It's a mandatory item.
    - `mml`, `note_number`, `description` are optional items.
    - If `mml` and `note_number` are omitted, what plays is up to the application, e.g., middle C.
    - If both `mml` and `note_number` are specified, which one plays is also up to the application, e.g., `note_number`, then `mml`, alternating.
- Data Format Description
    - Address and Data
        - Repeating pairs of 2-character address and 2-character data.
    - Benefits
        - Structured
            - Being JSON, there's no ambiguity like natural language, allowing simple code for reading and writing.
        - Flexibility
            - If the format were restricted to specific registers and a fixed notation, problems like the following might arise, but these can be avoided:
                - Example: This format lacks necessary information.
                - Example: Cost of format consideration for what constitutes a sufficient format.
                - Example: Format changes later might require parser/output code changes or migration.
                    - Format changes include changes in notation or adding/removing target registers.
        - Self-describing
            - The `description` ensures readability and self-descriptiveness, as do the directory and file names.
                - The fact that it's JSON also contributes.
        - Variations
            - In practice, GM000 can have many variations,
                - which is handled by storing them in an array within the JSON.
        - Readability
            - Writing in a single line, with `description` at the beginning, offers high readability. Intended to be treated as a list of tone variation names.
        - Portability
            - A highly portable format; at this level, it's expected to be easy to write interconversion code.
        - Uniqueness
            - By using `registers` as a unique ID, some benefits of uniqueness are expected.
                - Benefit: Duplicate detection is possible, which can somewhat prevent excessive tone library bloat.
                - Benefit: Can be used as an ID when needing to uniquely identify a tone.
                    - Can be searched even if the `description` changes.
                    - Handling various aspects might become easier.
                - Benefit: Searching using `registers` allows identifying "This is YM2151 tone data from so-and-so's repository." The data is self-describing.
                    - For this reason, `registers` must maintain a format without delimiters.
                    - Prerequisites include being registered under GitHub management and the registration location being self-describing.
                - Note: This is only to a certain extent. Even almost identical tones will have different IDs if a single bit differs.
    - Supplement
        - Slot mask
            - By including 'note on' in `registers`, the slot mask can be represented. The application can extract the slot mask from there. ym2151-tone-editor has already implemented this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-op tone editing experience, among others.
        - Saving all 256 bytes of register information in JSON is not recommended. It's assumed there's a risk of the application behaving unexpectedly.
            - Detailed examination and consideration of that will be postponed. YAGNI. It's assumed the application can handle it later.
        - Note that advanced playing techniques such as modulator TL automation cannot be included in this tone data.
            - This means that "tone data including advanced playing techniques" that cannot be fully expressed by this format may exist, and compatibility with it will be limited.
- Issues and Countermeasures
    - Issue: 128 items is cumbersome.
    - Countermeasure: It's assumed that writing simple code for this will be sufficient.
        - For example, by preparing a list of 128 tone names and simple code, JSON filename generation and description generation are expected to be easy.

## Considering Keybinds
- * Each will be separated into individual issues. Safety first. Prevent confusion.
- * Expected to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.
- Concept
    - Basic operations can be completed with just cursor keys and Page Up/Down.
    - Supplement
        - Supplement quick editing and high functionality with shortcut keys.
        - Mouse left-click for cursor movement, wheel for value increment/decrement are also standard, so these will be implemented.
            - Right-click is confusing in TUI, so it's better to avoid it.
        - Also, for some functions like exiting, ESC alone is sufficient, as that is considered standard.
- Increment/decrement values with `+` and `-`. This is widely known and easy to understand, so it's considered an improvement in introductory UX.
- Move cursor with `CTRL hjkl`. `CTRL npfb` also moves the cursor.
    - Movement without arrow keys is already possible with other shortcut keys, but having these available could improve UX, especially during introduction.
- Play with `P` and `space`. Being able to repeatedly play the current sound as is improves UX.
- Increase FB with `F`, decrease with `SHIFT+F`. The cursor also moves to FB.
    - Other similar operations are also expected to perform cursor jump and value increment/decrement together, which should be quick. This will be verified.
- Increase current row's TL with `T`, decrease with `SHIFT+T`.
- Increase current row's MUL with `M`, decrease with `SHIFT+M`.
    - Memo: If `M` is prioritized for something else, use `X`. The letter `x` is conceptually close to "multiple".
- Increase current row's AR, D1R, D2R, RR with `A, D, S, R` respectively, decrease with `SHIFT+`.
    - Supplement: Discontinue WASD for cursor movement. It led to many errors for this purpose and didn't feel beneficial. It's assumed that the need to constantly shift the left hand one position left from the home row caused many errors.
- Increase D1L with `L`, decrease with `SHIFT+L`.
    - L for D1L. The explanation in the heading is easy to understand.
- Move directly to M1, C1, M2, C2 rows with `1, 2, 3, 4` respectively, and also increment the value in the cursor's current column.
    - Decrement if `SHIFT` key is pressed.
    - Purpose is for quickly incrementing/decrementing values across OPs.
        - Example: When working on OP1 and wanting to increment OP4, compared to 3 cursor key presses and PageUp,
            - pressing `4` is once, making it 4 times faster.
    - Note: Numbers are relatively difficult to touch-type, so `hjkl` will also be tested as aliases.
- Toggle SlotMask for OP1-4 with `5, 6, 7, 8`.
    - Toggle solo mode if `SHIFT` is pressed.
        - Even for modulators, in solo mode, force ALG7 to play,
            - for checking envelopes, etc. And the forced SlotMask will be on only for the respective row.
        - At this time, ALG and SM will be shown with a special color or background color for clarity.
        - The row with the cursor will always be in solo mode, meaning SM dynamically changes with cursor movement.
        - When toggled off, it reverts to the ALG held just before solo mode was toggled on.
            - Toggling off will be `SHIFT+5,6,7,8` (any of them), as a simple initial specification.
                - This means no soloing two OPs. Prioritize simplicity first.
- Toggle mouse multi-cursor lock with `K`. Easy to explain with "loc`K`".
    - When locked, pressing F key, etc., will not move the cursor.
        - Multiple targets can be locked. Each becomes a target for value increment/decrement by mouse.
        - Intended use is for previewing envelopes while collectively incrementing/decrementing their values.
    - When not locked, mouse behavior is:
        - Left-click moves cursor to position and increments value,
        - Right-click moves cursor to position and decrements value.
- Use `, .` for Note down and up. Use C Ionian scale centered on middle C.
    - However, these are also strong candidates for value increment/decrement, so keybind changes are anticipated in the future.
## Organizing How to Use MML for Preview
- Preview MML to SMF stage
    - Pass preview MML to the `mmlabc-to-smf-rust` library.
    - Memo: Passing tone data at this point is complex, so it will be postponed. It's assumed to be a must eventually.
- Log generation stage
    - Regarding the `smf_to_log` function in the `smf-to-ym2151log-rust` library:
        - Pass "`smf` and `tone data`" as arguments.