# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM sound source tone editor. For Windows. TUI. Written in Rust.

## Status

Currently under development. Current progress is 80%. The remaining 20% involves adding keybinds and tone management.

- Future Outlook
    - *All specifications are temporary for verification and subject to frequent breaking changes.*
    - Establish a tone saving mechanism suitable for saving and GitHub management. Details below.
    - Implement significant keybind changes. Details below.

## Features

- Edit YM2151 tone parameters
- Operable with mouse alone
- Move with cursor keys, increment/decrement values with PageUp/PageDown/Home/End
- Preview tone with `P` or `SPACE` key
- Exit with `ESC` key
- Automatically saves tone on exit and loads it automatically next time to resume editing
- Customizable keybinds

## Quick Start Guide

If Rust is installed on Windows, you can easily install it from GitHub:

```
cargo install --force --git https://github.com/cat2151/ym2151-tone-editor/
```

## How to use the random tone function library

https://cat2151.github.io/ym2151-tone-editor/demo-library/

## Tips
- To display in full screen:
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

## Real-time Audio Feedback (Windows Only)

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the ym2151-log-play-server library. This handles the server installation, startup, and readiness check automatically.

```bash
# Just run the tone editor - the server will be automatically set up and started
cargo run
```

### Operating Modes

The editor operates in two modes:

#### Interactive Mode (Default)

In interactive mode, the server continuously streams audio, and only register write commands are sent when parameters change. This provides more efficient and smoother audio feedback.

#### Legacy Mode

By default, the editor sends complete tone data in JSON format via a named pipe using `send_json`. Each time a parameter is changed, the entire new JSON is sent.

### Comparison

| Feature | Legacy Mode | Interactive Mode |
|---------|---------------|---------------------|
| Data Transmission | Complete JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Use Case | For comparison/verification | Normal editing workflow |

## How to Operate

*Subject to breaking changes in the future. For verification purposes.*

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in the respective direction |
| **Value Change** | |
| `PageUp` / `e` | Increase the value at the cursor position |
| `PageDown` / `q` | Decrease the value at the cursor position |
| `+` / `.` | Increase value by 1 |
| `-` / `,` | Decrease value by 1 |
| `Shift` + `.` (`>`) | Increase value by 10 |
| `Shift` + `,` (`<`) | Decrease value by 10 |
| `Home` | Set to maximum value for the current parameter |
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

## Concept
- Launch in 100ms, sound in 100ms * (These numbers are approximate, implying significantly less than 1 second)
- Pressing a key plays sound and changes the tone
    - Prioritize addressing "It doesn't play sound or allow editing, I don't understand it"
- Colorful visualization
- Simple
- Easy-to-learn operations (cursor, mouse) for basic editing

## Out of Scope, Not Aiming For
- High-functionality editor
    - A perfect, versatile editor that satisfies all users from beginners to advanced experts
    - Unlimited intelligent UNDO
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features
- Interactive
    - Highly interactive performance using a virtual MIDI keyboard; server also changed to low-latency, advanced real-time processing using shared memory
    - Generally highly interactive performance with good responsiveness
- GUI
    - Graphical visualization of tones. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates
- High-functionality librarian
    - Quick and easy access, preview, selection, editing, and advanced version control of all tones with flexible operations
    - Fully automatic or interactive and advanced tone extraction from existing music, with 100% success rate
    - Automatic identification and loading of all YM2151 tone formats, with 100% automatic identification success rate
    - Automatic identification and conversion of all FM tone formats for loading, with 100% success rate
- Advanced Extensibility
    - Advanced tone creation using automation
    - Advanced tone creation using all 8 channels, and even multiple YM2151s
    - Support for all FM sound sources, beyond the YM2151 framework
    - Support for all DAWs and audio plugins, allowing playback with each, and import/export of FM tone patches

## Considering Tone Storage Format
- Past Issues
    - ym2151-log format
        - JSON data with many lines.
        - Cannot store multiple tone variations in one file.
        - Maintaining this directly for General MIDI on GitHub is not very realistic.
        - Will continue to be used for server transmission. However, there's a feeling that a more appropriate format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-describing
                - Directory hierarchy and file name make the purpose and tone clear
    - Commit
        - Commit to the ym2151-tone-editor repository 0-1 times per day
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
    - The core is `registers`. This is a mandatory field.
    - `mml`, `note_number`, `description` are optional fields.
    - If `mml` and `note_number` are omitted, what sound is played is left to the app (e.g., middle C).
    - If both `mml` and `note_number` are provided, which one plays is also left to the app (e.g., `note_number`, then `mml`, playing alternately).
- Data Format Description
    - Address and Data
        - Repeated pairs of 2-character address and 2-character data.
    - Benefits
        - Structured
            - Being JSON, it avoids the ambiguity of natural language and allows reading/writing with simple code.
        - Flexible
            - If the format were fixed to narrow down to specific registers and a specific notation method, it could lead to the following problems, which are avoided:
                - Example: The format lacks necessary information.
                - Example: High cost of format design to determine how much information is sufficient.
                - Example: Future format changes would require modifying parser/output code or migration.
                    - Format changes include changes in notation or increasing/decreasing target registers.
        - Self-describing
            - `description` ensures readability and self-descriptiveness, as do directory and file names.
                - Being JSON also contributes to this.
        - Variations
            - In practice, even GM000 can have many variations, so this is handled by keeping them in an array within the JSON.
        - Readability
            - Writing it on a single line, with the `description` at the beginning, provides high readability. Intended to be treated as a list of tone variation names.
        - Portability
            - Highly portable format; it's easy to write mutual conversion code at this level.
        - Uniqueness
            - Using `registers` as a unique ID provides some benefits of uniqueness.
                - Benefit: Can detect duplicates, potentially preventing excessive growth of the tone library.
                - Benefit: Can be used as an ID when needing to uniquely identify a tone.
                    - Can be searched even if the description changes.
                    - Potentially simplifies various handling.
                - Benefit: Searching using `registers` can identify "this is a YM2151 tone data from so-and-so's repository." The data is self-describing.
                    - For this, `registers` must maintain a format without delimiters.
                    - The prerequisite is that it's registered with GitHub management and the registration location is self-describing.
                - Note: This is only to a certain extent. Almost identical tones will have different IDs if even 1 bit differs.
    - Supplementary Notes
        - Slot Mask
            - Including `note on` in `registers` allows expressing the slot mask. The app can extract the slot mask from it. `ym2151-tone-editor` has already implemented this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, etc.
        - Saving all 256 bytes of register information in JSON is not recommended. It's assumed to risk unexpected app behavior.
            - Refinement and consideration of this will be postponed. YAGNI. It's assumed to be manageable by the app later.
        - It should be noted that advanced performance techniques such as modulator TL automation cannot be included in this tone data.
            - This means that "tone data containing advanced performance techniques" that cannot be expressed by this format may exist, and compatibility with them will be limited.
- Challenges and Solutions
    - Challenge: 128 items is a lot of work.
    - Solution: It is assumed that writing a simple code for this purpose would be sufficient to handle it.
        - For example, preparing a list of 128 tone names and a simple code would make JSON filename generation and description generation easy.

## Considering Keybinds
- *Each will be split into individual issues. Prioritize safety. Prevent confusion.*
- *Assumed to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.*
- Concept
    - Basic operations are completed with cursor keys and Page Up/Down.
    - Supplementary Notes
        - Shortcut keys complement fast editing and advanced features.
        - Mouse left-click for cursor movement, wheel for value increment/decrement is also standard, so it will be implemented.
            - Right-click in TUI is confusing, so it's better to avoid it.
        - Furthermore, for some functions like exiting, `ESC` alone is sufficient, as it is standard, according to this idea.
- `+` and `-` for increasing/decreasing values. This is widely known and easy to understand, improving the UX for introduction.
- `CTRL hjkl` for cursor movement. `CTRL npfb` also for cursor movement.
    - Cursor movement without arrow keys is already possible with other shortcut keys, but these might improve UX, especially during introduction.
- `P` and `space` for playing. Being able to repeatedly play the current sound improves UX.
- `F` to increase FB, `Shift+F` to decrease FB. Cursor also moves to FB.
    - Other similar operations should also perform cursor jump and value increment/decrement as a set; this is assumed to be faster. Will verify.
- `T` to increase TL of the current row, `Shift+T` to decrease.
- `M` to increase MUL of the current row, `Shift+M` to decrease.
    - Note: If `M` has a higher priority for something else, use `X`. `x` for `multiplier` feels close in meaning.
- `A`, `D`, `S`, `R` to increase AR, D1R, D2R, RR of the current row, `Shift+` to decrease.
    - Note: Stop using WASD for cursor movement. It led to many errors for this purpose, and no benefit was felt. It often required shifting the left hand one position to the left from the home row, leading to many errors.
- `L` to increase D1L, `Shift+L` to decrease.
    - `L` for D1L. The heading makes it easy to understand.
- `1`, `2`, `3`, `4` to directly move to the M1, C1, M2, C2 row and increment the value in the cursor's column.
    - If `Shift` key is pressed, it decrements.
    - Purpose: For quickly incrementing/decrementing values across operators.
        - Example: If you're on OP1 and want to increment OP4, it's 1 press for `4` compared to 3 cursor key presses and PageUp, which is 4 times faster.
    - Note: Numbers are relatively difficult to touch-type, so `hjkl` will also be tested as aliases.
- `5`, `6`, `7`, `8` to toggle the SlotMask for OP1-4.
    - If `Shift` is pressed, it toggles solo mode.
        - Even for modulators, in solo mode, it forces ALG7 playback to check envelopes, etc. And the forced SlotMask is on only for that row.
            - The ALG and SM will be visually distinguished with a special color or background color.
        - The row with the cursor will always be in solo mode, meaning SM dynamically changes with cursor movement.
        - Toggling off solo mode reverts to the ALG that was held just before toggling it on.
            - Toggling off will be `Shift+5`, `6`, `7`, or `8`; keeping it simple initially.
                - This means no soloing two operators. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for `Lock` makes it easy to explain.
    - When locked, pressing `F` or other keys will not move the cursor.
        - Multiple lock targets are possible. Each becomes a target for mouse-based value increment/decrement.
        - Intended use: For collectively increasing/decreasing envelopes while previewing.
    - When not locked, mouse behavior will be:
        - Left-click: Move cursor to click position and increment value.
        - Right-click: Move cursor to click position and decrement value.
- `,.` for Note down and up, respectively. Based on the C Ionian scale centered around middle C.
    - However, since these keys are also strong candidates for value increment/decrement, a future keybind change is anticipated.
## Organizing How to Use MML for Preview
- Preview MML to SMF Stage
  - Pass the preview MML to the `mmlabc-to-smf-rust` library to obtain an SMF.
  - Note: Passing tone data at this point is complex, so it will be postponed. Eventually assumed to be a must.
- Log Generation Stage
  - For the `smf-to-log` function in the `smf-to-ym2151log-rust` library,
    - Pass "`smf` and `tone data`" as arguments to obtain the log.
    - Method: https://github.com/cat2151/smf-to-ym2151log-rust/pull/46