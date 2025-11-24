# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM synthesizer tone editor for Windows. TUI. Written in Rust.

## Status

In development. Current progress is roughly 50%.

- Future Outlook
    - *All specifications are provisional for testing and subject to frequent breaking changes.*
    - Establish a saving mechanism suitable for tone preservation and GitHub management. Details below.
    - Implement significant keybind changes. Details below.

## Features

- Edit YM2151 tone parameters
- Operable with mouse only
- Move with arrow keys, increase/decrease values with PageUp/PageDown/Home/End
- Preview tone with `P` or `SPACE` key
- Exit with `ESC` key
- Automatically saves the tone upon exit and loads it automatically next time for continued editing

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

## Real-time Audio Feedback (Windows only)

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the ym2151-log-play-server library. This handles the server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Interactive Mode (Default)

In interactive mode, the server continuously streams audio and only sends register write commands when parameters change. This provides more efficient and smoother audio feedback.

#### Legacy Mode

By default, the editor uses `send_json` to transmit complete tone data in JSON format via a named pipe. Each time a parameter is changed, the entire new JSON is sent.

### Comparison

| Feature | Legacy Mode | Interactive Mode |
|---------|-------------|------------------|
| Data Transmission | Complete JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Use Case | For comparison/verification | Standard editing work |

## How to Operate

*Subject to breaking changes in the future. For verification purposes.*

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in corresponding direction |
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
| `Mouse wheel up` | Move cursor to mouse pointer position and increase value |
| `Mouse wheel down` | Move cursor to mouse pointer position and decrease value |
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
- Startup in 100ms, sound in 100ms. *Numbers are rough estimates, just the idea of significantly faster than 1 second.*
- Press a key to hear sound and change tones.
    - Prioritize addressing the issue of "it doesn't play or edit, I don't understand it."
- Colorful visualization
- Simple
- Approachable operation (cursor, mouse) for basic editing

## Out of Scope, Not Aiming For
- High-performance editor
    - A perfect, versatile editor that satisfies everyone from beginners to super-advanced users.
    - Unlimited intelligent UNDO.
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features.
- Interactive
    - Highly interactive performance with a virtual MIDI keyboard; server changed to low-latency, advanced real-time processing using shared memory.
    - Highly interactive performance in general, with good responsiveness.
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates.
- Advanced Librarian
    - Quick and easy access, preview, selection, editing, and highly intelligent version management for all tones with flexible operations.
    - Fully automatic or interactive and advanced tone extraction from existing music, with 100% success rate.
    - Automatic detection and loading of all YM2151 tone formats, with 100% success rate.
    - Automatic detection and conversion, then loading of all FM tone formats, with 100% success rate.
- Advanced Extensibility
    - Advanced tone creation using automation.
    - Advanced tone creation utilizing all 8 channels, and even multiple YM2151s.
    - Support for all FM synthesizers, beyond the YM2151 framework.
    - Compatibility with all DAWs and audio plugins, enabling playback and import/export of FM synthesizer tones for each.

## Considering Tone Saving Format
- Previous Issues
    - ym2151-log format
        - JSON data with many lines.
        - Cannot store multiple tone variations in one file.
        - Maintaining this directly for General MIDI on GitHub is not very realistic.
        - Will continue to be used for server transmission. However, there is a feeling that a more appropriate format is needed for tone management.
### Proposed Solution
- Workflow
    - Deployment
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-describing
                - Directory hierarchy and filename make the purpose and tone clear.
    - Commit
        - Commit to the `ym2151-tone-editor` repository at a frequency of 0-1 times per day.
- File format
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
    - The core is `registers`. This is a required item.
    - `mml`, `note_number`, `description` are optional items.
    - If `mml` and `note_number` are omitted, what plays is left to the application, e.g., middle C.
    - If both `mml` and `note_number` are provided, which one plays is also left to the application, e.g., `note_number`, then `mml`, playing alternately.
- Data Format Explanation
    - Address and Data
        - Repeated pairs of 2-character address and 2-character data.
    - Benefits
        - Structured
            - Being JSON, it has no ambiguity like natural language, allowing for simple code to read and write.
        - Flexibility
            - If a format were to be limited to specific registers and fixed to a specific description method, problems like those below might arise, but this format avoids them:
                - Example: In this format, necessary information is missing.
                - Example: How much to record to make the format sufficient, format design costs are high.
                - Example: Changes to the parser or output code, or migration, are required due to subsequent format changes.
                    - Format changes include changes in description method or increase/decrease of target registers.
        - Self-describing
            - `description` ensures readability and self-descriptiveness, as do directory names and filenames.
                - The fact that it's JSON also contributes to this.
        - Variations
            - In practice, even GM000 can have many variations, so this is handled by keeping them in an array within the JSON.
        - Readability
            - Writing on one line, with `description` at the beginning, makes it highly readable. Intended to be treated as a list of tone variation names.
        - Portability
            - Highly portable format; it is assumed that mutual conversion code can be written easily at this level.
        - Uniqueness
            - Using `registers` as a unique ID is expected to provide some benefit of uniqueness.
                - Benefit: Duplicate detection is possible, which can somewhat prevent excessive tone library bloat.
                - Benefit: When wanting to uniquely identify a tone, it can be used as an ID.
                    - It can be searched even if the `description` has changed.
                    - This can simplify various handling.
                - Benefit: Searching by `registers` reveals "this is YM2151 tone data from so-and-so's repository." The data is self-describing.
                    - For this reason, `registers` must maintain a format without delimiters.
                    - The premise is that it is registered under GitHub management and the registration location is self-describing.
                - Note: This is only to a certain extent. Even nearly identical tones will have different IDs if one bit differs.
    - Supplement
        - Slot mask
            - Including `note on` in `registers` can represent the slot mask. The application can extract the slot mask from there. `ym2151-tone-editor` already implements this.
            - The slot mask is used, for example, to provide an easy-to-edit 2-operator tone editing experience.
        - Saving all 256 bytes of register information in JSON is not recommended. This carries the risk of the application behaving unexpectedly.
            - The detailed examination and consideration of this will be postponed. YAGNI. It is assumed that the application can handle it later.
        - It should be noted that advanced performance techniques like modulator TL automation cannot be included in this tone data.
            - This means that "tone data containing advanced performance techniques" that cannot be expressed in this format may exist, and compatibility with them will be limited.
- Issues and Countermeasures
    - Issue: 128 items is cumbersome.
    - Countermeasure: It is assumed that this can be sufficiently addressed by writing simple code.
        - For example, if a list of 128 tone names is prepared and simple code is provided, generating JSON filenames and descriptions is assumed to be easy.

## Considering Keybinds
- *Each point will be separated into individual issues. Safety first. Confusion prevention.*
- *It is assumed that these will be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.*
- Concept
    - Basic operations are completed solely with arrow keys and Page Up/Down.
    - Supplement
        - Shortcut keys provide quick editing and advanced functionality.
        - Mouse left-click for cursor movement, wheel for value increase/decrease will also be implemented as it's standard.
            - Right-click in TUI is confusing, so it's better to avoid it.
        - For some functions like exiting, `ESC` alone is sufficient, as it's standard.
- Increase/decrease values with `+` and `-`. This is widely known and easily understood, improving the UX for introduction.
- Move cursor with `CTRL hjkl`. Also `CTRL npfb` for cursor movement.
    - Cursor movement without arrow keys is already possible with other shortcuts, but using these could improve UX, especially for beginners.
- Play with `P` and `space`. Being able to repeatedly play the current sound improves UX.
- `F` to increase FB, `Shift`+`F` to decrease FB. Cursor also moves to FB.
    - For other similar operations, cursor jump and value increase/decrease are done as a set, which is assumed to be fast. Will be tested.
- `T` to increase TL of current row, `Shift`+`T` to decrease.
- `M` to increase MUL of current row, `Shift`+`M` to decrease.
    - Memo: If `M` is prioritized elsewhere, then `X`. `X` is similar in meaning to `multiple`.
- `A`, `D`, `S`, `R` to increase AR, D1R, D2R, RR of current row, `Shift`+ for decrease.
    - Note: Stop using WASD for cursor movement. It led to too many mistakes in this context, and no benefit was felt. It was assumed that having to shift the left hand one position left from home row constantly led to many mistakes.
- `L` to increase D1L, `Shift`+`L` to decrease.
    - `L` for D1L. The heading explanation is easy to understand.
- `1`, `2`, `3`, `4` to directly move to M1, C1, M2, C2 rows and increment the value in the current column.
    - If `Shift` key is pressed, decrement.
    - Purpose: For quickly incrementing/decrementing values across operators.
        - Example: If you're on OP1 and want to increment OP4, compared to 3 arrow key presses and PageUp,
            - `4` is 1 press, so it's 4 times faster.
    - Note: Numbers are relatively hard to touch-type, so `hjkl` will also be tested as aliases.
- `5`, `6`, `7`, `8` to toggle SlotMask for OP1-4.
    - If `Shift` is pressed, toggle solo mode.
        - Even for modulators, in solo mode, force ALG7 to play to
            - check envelopes, etc. And force SlotMask to be on only for that row.
                - In this case, ALG and SM will be made clear with a special color or background color.
        - The row with the cursor will always be in solo mode, meaning SM dynamically changes with cursor movement.
        - Toggling solo off restores the ALG that was held just before toggling on.
            - For now, toggling off will be done with `Shift`+`5`, `6`, `7`, or `8`—a simple specification first.
                - This means no soloing two ops. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for `locK` makes it easy to explain.
    - When locked, pressing `F` key, etc., will not move the cursor.
        - Multiple items can be locked. Each becomes a target for mouse-based value increase/decrease.
        - Intended use case: Previewing envelopes while simultaneously increasing/decreasing them.
    - When not locked, mouse behavior is:
        - Left-click moves the cursor to that location and increments the value.
        - Right-click moves the cursor to that location and decrements the value.
- `, .` for Note down and up. Use C Ionian scale centered around middle C.
    - However, since these keys are also strong candidates for value increase/decrease, keybind changes are anticipated in the future.