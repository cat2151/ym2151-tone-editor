# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM synthesizer tone editor. For Windows. TUI. Written in Rust.

## Status

Currently under development. Current progress is 80%. The remaining 20% involves adding keybinds and tone management.

-   Future Outlook
    -   *Note: All of these are provisional specifications for testing purposes and are subject to frequent breaking changes.*
    -   Build a save mechanism suitable for tone saving and GitHub management. (Details below).
    -   Implement significant keybind changes. (Details below).

## Features

-   Edit YM2151 tone parameters
-   Operable with just a mouse
-   Move with arrow keys, increase/decrease values with PageUp/PageDown/Home/End
-   Preview tone with `P` or `SPACE` key
-   Exit with `ESC` key
-   Automatically saves tone on exit and automatically loads it next time to resume editing
-   Configurable keybinds

## Quick Start Guide

Will be written in the future.
If Rust is installed on Windows, you can easily install it from GitHub.

## Tips
-   When you want to display full screen:
    -   In Windows Terminal, you can maximize the window with `ALT+ENTER` or `F11`, then enlarge the font with `CTRL`+`+`.

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

-   Rust 1.70 or later

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

## Real-time Audio Feedback (Windows only)

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the `ym2151-log-play-server` library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

### Operating Modes

The editor operates in two modes:

#### Interactive Mode (Default)

In interactive mode, the server continuously streams audio and only sends register write commands when parameters are changed. This provides more efficient and smoother audio feedback.

#### Legacy Mode

By default, the editor sends complete tone data in JSON format via a named pipe using `send_json`. Each time a parameter is changed, the entire new JSON is sent.

### Comparison

| Feature | Legacy Mode | Interactive Mode |
|------|---------------|---------------------|
| Data Transmission | Full JSON | Register Writes Only |
| Efficiency | Low (sends all data every time) | High (sends only changed parts) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Usage | For comparison/verification | For normal editing work |

## How to Use

*Note: Subject to breaking changes in the future. This is for verification purposes.*

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
| `Home` | Set to maximum value for current parameter |
| `End` | Set to minimum value (0) |
| `r` / `R` | Set to random value (within valid range) |
| **Mouse** | |
| Mouse wheel up | Move cursor to mouse pointer position and increase value |
| Mouse wheel down | Move cursor to mouse pointer position and decrease value |
| **Other** | |
| `ESC` | Save and exit application |

## Command Line Options

| Option | Description |
|--------|-------------|
| `--value-by-mouse-move` | Enable legacy mouse behavior (change value at cursor position by moving mouse left/right) |

## Dependencies

-   `ratatui` 0.28 - Terminal UI framework
-   `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concept
-   Starts in 100ms, plays sound in 100ms *Note: these numbers are rough estimates, just significantly shorter than 1 second.*
-   Pressing a key plays a sound and changes the tone
    -   Prioritize addressing the issue of "it doesn't play when I touch it, I can't edit, I don't understand."
-   Colorful visualization
-   Simple
-   Easy-to-learn operation for basic editing (cursor, mouse)

## Out of Scope, What Not to Aim For
-   High-functionality editor
    -   A perfect, universal editor that satisfies everyone from beginners to super-experts
    -   Unlimited intelligent UNDO
    -   Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features
-   Interactive
    -   Highly interactive performance with virtual MIDI keyboard, changing the server to advanced low-latency real-time processing using shared memory
    -   Highly interactive performance with good responsiveness in general
-   GUI
    -   Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates
-   High-functionality librarian
    -   Easy and quick access, preview, selection, editing, and highly intelligent version management for all tones with flexible operations
    -   Fully automatic or interactive advanced tone extraction from existing songs, with 100% success rate
    -   Automatic detection and loading of all YM2151 tone formats, with 100% automatic detection success rate
    -   Automatic detection and conversion of all FM tone formats for loading, with 100% success rate
-   Advanced expandability
    -   Advanced tone creation using automation
    -   Advanced tone creation using all 8 channels, and even multiple YM2151s
    -   Support for all FM sound generators beyond the scope of YM2151
    -   Support for all DAWs and audio plugins, allowing playback of each, and import/export of FM tone data for each

## Considering Tone Save Format
-   Past Challenges
    -   ym2151-log format
        -   JSON data with many lines.
        -   Cannot include multiple tone variations in a single file.
        -   Maintaining this as-is on GitHub for General MIDI is not very practical.
        -   It will continue to be used for server transmission. However, for tone management, a more suitable format is needed.
### Proposed Solution
-   Operation
    -   Placement
        -   `tones/general_midi/000_AcousticGrand.json`
        -   Pros
            -   Self-describing
                -   Purpose and tone are clear from directory hierarchy and filename
    -   Commit
        -   Commit to the `ym2151-tone-editor` repository 0-1 times a day
-   File Format
```
{
  "description": "GM:000 Acoustic Grand Piano family",
  "variations": [
    { "description": "GM:000 Bright Piano", "mml": "t120 o5 l4 cdefgab", "registers": "204F204C364037808003812D" },
    { "description": "GM:000 Soft Piano", "note_number": 60, "registers": "204F204C364037808001812D" }
  ]
}
```
-   JSON File Format Description
    -   The core is `registers`. This is a required field.
    -   `mml`, `note_number`, `description` are optional fields.
    -   If `mml` and `note_number` are omitted, what plays is up to the application, e.g., middle C.
    -   If both `mml` and `note_number` are provided, which one plays is also up to the application, e.g., `note_number`, then `mml`, alternating.
-   Data Format Description
    -   Address and Data
        -   Pairs of 2-character address and 2-character data, repeated.
    -   Pros
        -   Structured
            -   It's JSON, so there's no ambiguity like natural language, and it can be read/written with simple code.
        -   Flexibility
            -   If the format were limited to specific registers and fixed to a particular description method, issues like the following could arise, but these are avoided:
                -   Example: This format lacks necessary information.
                -   Example: How much data is sufficient for a format, incurring format design costs.
                -   Example: Later format changes necessitate parser/output code modifications and migration.
                    -   Format changes include changes in description methods or additions/removals of target registers.
        -   Self-describing
            -   The `description` ensures readability and self-descriptiveness, as do directory and file names.
                -   Being JSON also contributes to this.
        -   Variations
            -   In practice, GM000 can have many variations, so...
                -   ...this is handled by holding them in an array within the JSON.
        -   Readability
            -   Writing in a single line, with the `description` at the beginning, yields high readability. Expected to be treated as a list of tone variation names.
        -   Portability
            -   Highly portable format; mutual conversion code can be easily written at this level.
        -   Uniqueness
            -   Using `registers` as a unique ID is expected to provide some benefit of uniqueness.
                -   Pro: Duplicate detection can help somewhat prevent excessive tone library bloat.
                -   Pro: Can be used as an ID when unique identification of a tone is desired.
                    -   Can be searched even if the `description` is changed.
                    -   Can make various handling easier.
                -   Pro: Searching with `registers` reveals "this is YM2151 tone data from so-and-so's repository". The data is self-describing.
                    -   For this reason, `registers` must be kept in a format without delimiters.
                    -   The premise is that it's registered under GitHub management and the registration location is self-describing.
                -   Caution: This is only to a certain extent. Even almost identical tones will have different IDs if a single bit differs.
    -   Supplement
        -   Slot Mask
            -   By including note-on in `registers`, the slot mask can be expressed. The application can extract the slot mask from it. `ym2151-tone-editor` has already implemented this.
            -   The purpose of the slot mask is to provide an easy-to-edit 2-op tone editing experience, among others.
        -   Saving all 256 bytes of register information in JSON is not recommended. There's a risk the application might behave unexpectedly.
            -   Refinement and consideration of this will be deferred. YAGNI (You Aren't Gonna Need It). It's assumed the application can handle it later.
        -   Note that advanced playing techniques like modulator TL automation cannot be included in this tone data.
            -   This means that "tone data containing advanced playing techniques" that cannot be fully expressed by this format may exist, and compatibility with such data will be limited.
    -   Challenges and Solutions
        -   Challenge: 128 items is a lot of work
        -   Solution: It's assumed that this can be adequately addressed by writing simple code for it.
            -   For example, by preparing a list of 128 tone names and simple code, JSON filename generation and description generation are expected to be easy.

## Considering Keybinds
-   *Note: Each will be separated into individual issues. Prioritize safety. Prevent confusion.*
-   *Note: Expected to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.*
-   Concept
    -   Basic operations are completed with just arrow keys and Page Up/Down
    -   Supplement
        -   Supplement with shortcut keys for quick editing and advanced functions.
        -   Left-click mouse for cursor movement, wheel for value increase/decrease, also standard, so implement.
            -   Right-click is unclear in TUI, so it's better to avoid it.
        -   Furthermore, some functions like exiting only require ESC, which is considered standard.
-   Increase/decrease values with `+` and `-`. This is widely known and easy to understand, thus improving the introductory UX.
-   Move cursor with `CTRL hjkl`. `CTRL npfb` also moves cursor.
    -   While movement without arrow keys can be achieved with other shortcuts, using these might improve UX, especially during initial use.
-   Play with `P` and `SPACE`. Being able to repeatedly play the current sound improves UX.
-   `F` increases FB, `SHIFT+F` decreases FB. The cursor also moves to FB.
    -   It's assumed that other similar operations (cursor jump and value increment/decrement combined) would be quick. Will verify.
-   `T` increases TL of the current row, `SHIFT+T` decreases.
-   `M` increases MUL of the current row, `SHIFT+M` decreases.
    -   Memo: If `M` is prioritized elsewhere, use `X`. The idea is that 'x' is close in meaning to 'multiple'.
-   `A`, `D`, `S`, `R` increase AR, D1R, D2R, RR of the current row, `SHIFT+` decreases.
    -   Supplement: Discontinue WASD for cursor movement. It resulted in many errors for this use case and didn't feel beneficial. It's assumed that the need to constantly shift the left hand one position left from the home row caused many errors.
-   `L` increases D1L, `SHIFT+L` decreases.
    -   The `L` in D1L. The explanation in the heading is easy to understand.
-   `1`, `2`, `3`, `4` directly move to M1, C1, M2, C2 rows while increasing the value in the current cursor column.
    -   `SHIFT` key pressed decreases the value.
    -   Purpose: For quickly increasing/decreasing values across OPs.
        -   Example: When working on OP1 and wanting to increment OP4, compared to 3 arrow key presses and page up...
            -   ...using 4 is 1 press, making it 4 times faster.
    -   Note: Numbers are relatively difficult for touch typing, so `hjkl` as aliases will also be considered.
-   `5`, `6`, `7`, `8` toggle SlotMask for OP1-4.
    -   `SHIFT` key pressed toggles solo mode.
        -   Even for modulators, in solo mode, force playback with ALG7 to...
            -   ...check envelopes etc. And the forced SlotMask will be on only for that row.
                -   At this time, ALG and SM should be made clear with a special color or background color.
        -   The row with the cursor is always in solo mode, meaning SM changes dynamically with cursor movement.
        -   When solo mode is untoggled, it reverts to the ALG held just before toggling on.
            -   Any of `SHIFT+5,6,7,8` will untoggle solo mode – start with a simple specification.
                -   This means no solo for two OPs simultaneously. Simplicity first.
-   `K` toggles mouse multi-cursor lock. `K` for `locK` makes it easier to explain in display.
    -   When locked, pressing F key etc. does not move the cursor.
        -   Multiple lock targets are possible. Each becomes a target for value increment/decrement via mouse.
        -   Intended use: Previewing while collectively increasing/decreasing envelope values.
    -   When not locked, mouse behavior is as follows:
        -   Left-click moves cursor to location and increments value,
        -   Right-click moves cursor to location and decrements value.
-   `,` and `.` for Note down and up. Use a C Ionian scale centered on middle C.
    -   However, as they are also strong candidates for value increment/decrement, keybind changes are anticipated in the future.
## Organizing How to Use MML for Preview
-   Preview MML to SMF Stage
    -   Pass preview MML to the `mmlabc-to-smf-rust` library to get SMF.
    -   Memo: Passing tone data at this stage is complex, so defer. It's assumed to be a must eventually.
-   Log Generation Stage
    -   Regarding the `smf-to-ym2151log-rust` library's `smf to log` function:
        -   Pass "`smf` and `tone data`" as arguments to get the log.
        -   Method: https://github.com/cat2151/smf-to-ym2151log-rust/pull/46