# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM sound chip tone editor. For Windows. TUI. Written in Rust.

## Status

Currently under development. Current progress is 80%. The remaining 20% involves adding keybinds and tone management.

- Future Outlook
    - *Note: All specifications are temporary for verification purposes and subject to frequent destructive changes.*
    - Establish a tone saving mechanism suitable for saving and GitHub management. Details below.
    - Implement significant keybind changes. Details below.

## Features

- Edit YM2151 tone parameters
- Operable with mouse only
- Navigate with cursor keys, increase/decrease values with PageUp/PageDown/Home/End
- `P` or `SPACE` key to preview tone
- `ESC` key to exit
- Automatically saves tone on exit and loads it automatically next time to resume editing
- Customizable keybinds

## Quick Start Guide

If Rust is installed on Windows, you can easily install it from GitHub.

```
cargo install --force --git https://github.com/cat2151/ym2151-tone-editor/
```

## Tips
- To display full screen:
  - In Windows Terminal, maximize the window with `ALT+ENTER` or `F11`, then enlarge the font with `CTRL`+`+`.

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
| D2R | Decay 1 Rate | 0-15 | Second decay/sustain rate (4 bits) |
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

## Real-time Audio Feedback (Windows Only)

The editor automatically ensures the server is ready by using the `ensure_server_ready()` function from the `ym2151-log-play-server` library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be automatically set up and started
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Interactive Mode (Default)

In interactive mode, the server continuously streams audio and only sends register write commands when parameters change. This provides more efficient and smoother audio feedback.

#### Legacy Mode

By default, the editor sends complete tone data in JSON format via named pipes using `send_json`. A new complete JSON is sent every time a parameter is changed.

### Comparison

| Feature | Legacy Mode | Interactive Mode |
|---------|---------------|-------------------|
| Data Transmission | Complete JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Usage | For comparative verification | Normal editing tasks |

## How to Operate

*Note: Subject to destructive changes in the future for verification purposes.*

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in the respective direction |
| **Value Change** | |
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

## Command-line Options

| Option | Description |
|--------|-------------|
| `--value-by-mouse-move` | Enable legacy mouse behavior (change value at cursor position by moving mouse left/right) |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concepts
- Startup in 100ms, sound in 100ms *The numbers are rough; the idea is significantly faster than 1 second.*
- Pressing a key plays a sound and changes the tone.
    - Prioritize addressing the issue of "it doesn't play or edit when touched, it's confusing."
- Colorful visualization
- Simple
- Easy-to-learn operation for basic editing (cursor, mouse)

## Out of Scope, Not Aimed For
- High-performance editor
    - A perfect, versatile editor that satisfies all users from beginners to super-experts.
    - Unlimited intelligent UNDO.
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing functions.
- Interactive
    - Highly interactive performance via virtual MIDI keyboard, with server also changing to low-latency, advanced real-time processing using shared memory.
    - Generally responsive, highly interactive performance.
- GUI
    - Graphical visualization of tones. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates.
- Advanced librarian
    - Easy and quick access, preview, selection, editing, and highly intelligent version management for all tones with flexible operations.
    - Fully automatic or interactive and advanced tone extraction from existing songs, with 100% success rate.
    - Automatic detection and loading of all YM2151 tone formats, with 100% success rate.
    - Automatic detection and conversion and loading of all FM tone formats, with 100% success rate.
- Advanced extensibility
    - Advanced tone creation using automation.
    - Advanced tone creation utilizing all 8 channels, and even multiple YM2151 chips.
    - Support for all FM sound chips beyond the YM2151 framework.
    - Compatibility with all DAWs and audio plugins, enabling playback and import/export of FM tones for each.

## Considering a Format for Tone Storage
- Past issues
    - ym2151-log format
        - JSON data with many lines.
        - Cannot store multiple tone variations in one file.
        - Maintaining this directly for General MIDI on GitHub is not very practical.
        - Will continue to be used for server transmission. However, there's a feeling that a more appropriate format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-descriptive
                - Directory hierarchy and filenames make purpose and tone clear.
    - Commit
        - Commit to the ym2151-tone-editor repository at a frequency of 0-1 times a day.
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
    - The main content is `registers`. This is a required item.
    - `mml`, `note_number`, `description` are optional items.
    - If `mml` and `note_number` are omitted, what plays is up to the application, e.g., middle C.
    - If both `mml` and `note_number` are provided, which one plays is also up to the application, e.g., `note_number`, then `mml`, alternating.
- Data Format Description
    - Address and Data
        - Repeated pairs of 2-character address and 2-character data.
    - Benefits
        - Structured
            - Being JSON, it has no ambiguity like natural language, allowing simple code for reading and writing.
        - Flexibility
            - If the format were fixed to a specific description method, limited to specific registers, it could lead to problems like the following, but these are avoided:
                - Example: The format might lack necessary information.
                - Example: High cost of format consideration to determine how much information is sufficient.
                - Example: Changes to the format later requiring changes to parser/output code or migration.
                    - Format changes include modifying description methods or increasing/decreasing target registers.
        - Self-descriptive
            - `description` ensures readability and self-descriptiveness, as do directory names and filenames.
                - Being JSON also contributes to this.
        - Variations
            - In practice, even GM000 can have many variations,
                - This is handled by storing them in an array within JSON.
        - Readability
            - Writing on a single line with `description` at the beginning provides high readability. Intended to be treated as a list of tone variation names.
        - Portability
            - A highly portable format; cross-conversion code should be easy to write at this level.
        - Uniqueness
            - Utilizing `registers` as a unique ID provides some benefits of uniqueness.
                - Benefit: Can detect duplicates, potentially preventing excessive tone library bloat.
                - Benefit: Can be used as an ID when uniquely identifying a tone.
                    - Searchable even if the `description` changes.
                    - Could simplify handling in various ways.
                - Benefit: Searching by `registers` can identify "YM2151 tone data from so-and-so's repository." The data is self-descriptive.
                    - For this, `registers` must maintain a format without delimiters.
                    - The premise is that it's registered in GitHub and the registration location is self-descriptive.
                - Caution: This is only to a certain extent. Even nearly identical tones will have different IDs if they differ by 1 bit.
    - Supplement
        - Slot mask
            - By including `note on` in `registers`, the slot mask can be expressed. The application can extract the slot mask from there. `ym2151-tone-editor` has implemented this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, etc.
        - Saving all 256 bytes of register information as JSON is not recommended. It risks unexpected application behavior.
            - Refinement and consideration of this will be postponed. YAGNI. It is assumed that it can be handled on the application side later.
        - Note that advanced playing techniques such as modulator TL automation cannot be included in this tone data.
            - This means that "tone data containing advanced playing techniques" that cannot be fully expressed in this format may exist, and compatibility with it will be limited.
- Challenges and Solutions
    - Challenge: 128 items is a lot of work.
    - Solution: It's assumed that this can be sufficiently addressed by writing simple code for it.
        - For example, preparing a list of 128 tone names and simple code should make JSON filename generation and description generation easy.

## Considering Keybinds
- *Note: Each point will be separated into individual issues. Safety first. Avoid confusion.*
- *Note: Intended to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.*
- Concept
    - Basic operations are completed with cursor keys and Page Up/Down.
    - Supplement
        - Shortcut keys provide quick editing and advanced functionality.
        - Mouse left-click for cursor movement, wheel for value increase/decrease, is standard and will be implemented.
            - Right-click in TUI is confusing, so it's best to avoid.
        - Note that some functions like exiting only need ESC, which is standard.
- `+` and `-` to increase/decrease values. This is widely known and improves UX for adoption.
- `CTRL` + `hjkl` for cursor movement. `CTRL` + `npfb` also for cursor movement.
    - Cursor-less movement is already possible with other shortcut keys, but these could improve UX, especially for new users.
- `P` and `space` for playing. The ability to repeatedly play the current sound improves UX.
- `F` to increase FB, `SHIFT` + `F` to decrease FB. Cursor also moves to FB.
    - Other similar operations should also combine cursor jump and value increase/decrease for speed. This will be verified.
- `T` to increase TL of the current row, `SHIFT` + `T` to decrease.
- `M` to increase MUL of the current row, `SHIFT` + `M` to decrease.
    - Memo: If `M` is prioritized for something else, use `X`. `x` is conceptually close to "multiple".
- `A`, `D`, `S`, `R` to increase AR, D1R, D2R, RR of the current row, `SHIFT` + to decrease.
    - Supplement: Discontinue WASD for cursor movement. It led to too many mistakes and didn't provide a benefit for this use case. Constantly shifting the left hand one position left from home row resulted in many errors.
- `L` to increase D1L, `SHIFT` + `L` to decrease.
    - `L` for D1L. Easy to understand with the heading description.
- `1`, `2`, `3`, `4` to directly move to the M1, C1, M2, C2 rows and increase the value of the column where the cursor is.
    - If `SHIFT` is pressed, decrease the value.
    - Purpose: For quickly increasing/decreasing values across operators.
        - Example: If you're working on OP1 and want to increase OP4, it's 1 press for `4` versus 3 cursor keys and `PageUp`, making it 4 times faster.
    - Caution: Numbers are relatively hard to touch-type, so `hjkl` as aliases will also be verified.
- `5`, `6`, `7`, `8` to toggle SlotMask for OP1-4.
    - If `SHIFT` is pressed, toggle solo mode.
        - Even for a modulator, solo mode forces ALG7 playback to check envelopes, etc. And the forced SlotMask is only on for that row.
            - In this case, ALG and SM should be made clear with a special color or background color.
        - The solo mode will always be for the row where the cursor is, meaning SM changes dynamically with cursor movement.
        - Toggling off will revert to the ALG held just before solo mode was toggled on.
            - `SHIFT` + `5`, `6`, `7`, `7`, or `8` will toggle off, keeping it simple first.
                - This means no soloing two operators. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for `lock` makes it easy to explain.
    - When locked, pressing `F` or other keys will not move the cursor.
        - Multiple items can be locked. Each becomes a target for mouse value increase/decrease.
        - Intended use: For previewing while increasing/decreasing envelopes as a group.
    - When not locked, mouse behavior will be:
        - Left-click moves cursor to location and increases value.
        - Right-click moves cursor to location and decreases value.
- `,.` to decrease and increase the Note. Use the C Ionian scale centered around middle C.
    - However, these are also strong candidates for value increase/decrease, so keybind changes are expected in the future.
## Organizing the Method for Using MML in Preview
- Preview MML to SMF stage
  - Pass preview MML to the `mmlabc-to-smf-rust` library to obtain SMF.
  - Memo: Passing tone data at this point is complex, so it will be postponed. It's assumed to be a must-have eventually.
- Log Generation stage
  - For the `smf_to_log` function of the `smf-to-ym2151log-rust` library,
    - Pass "`smf` and `tone data`" as arguments to obtain the log.
    - Method: https://github.com/cat2151/smf-to-ym2151log-rust/pull/46