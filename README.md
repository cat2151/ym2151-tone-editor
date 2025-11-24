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
    - Establish a tone saving mechanism suitable for GitHub management. Details below.
    - Implement significant keybind changes. Details below.

## Features

- Edit YM2151 tone parameters
- Operable with mouse alone
- Move with cursor keys, increase/decrease values with PageUp/PageDown/Home/End
- Preview tone with `P` or `SPACE` key
- Exit with `ESC` key
- Automatically saves tone on exit and automatically loads it next time to resume editing
- Customizable keybinds

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

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the ym2151-log-play-server library. This handles the server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and launched automatically
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Interactive Mode (Default)

In interactive mode, the server continuously streams audio, sending only register write commands when parameters change. This provides more efficient and smoother audio feedback.

#### Legacy Mode

By default, the editor uses `send_json` to transmit complete tone data in JSON format via a named pipe. Each time a parameter is changed, the entire new JSON is sent.

### Comparison

| Feature | Legacy Mode | Interactive Mode |
|------|---------------|---------------------|
| Data Transmission | Complete JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changed parts) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Usage | For comparison/verification | Normal editing workflow |

## How to Operate

*Note: Subject to breaking changes in the future for verification purposes.*

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

## Concept
- Launch in 100ms, play sound in 100ms *These values are rough estimates; the idea is significantly faster than 1 second.*
- Sound plays and tone changes when a key is pressed
    - Prioritize addressing the feeling of "It doesn't play or let me edit, I don't understand."
- Colorful visualization
- Simple
- Easy-to-use basic editing (cursor, mouse)

## Out of Scope, Not Aimed For
- High-performance editor
    - A perfect, all-around editor that satisfies all users from beginners to super-experts
    - Unlimited intelligent UNDO
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features
- Interactive
    - Highly interactive performance using a virtual MIDI keyboard, with the server also changed to low-latency, advanced real-time processing using shared memory
    - Responsive, highly interactive performance in general
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates.
- Advanced Librarian
    - Quick and easy access, preview, selection, editing, and highly intelligent version management for all tones with flexible operations.
    - Fully automatic or interactive and advanced tone extraction from existing music, with 100% success rate.
    - Automatically identify and load all YM2151 tone formats, with 100% identification success rate.
    - Automatically identify and convert all FM tone formats for loading, with 100% success rate.
- Advanced Extensibility
    - Advanced tone creation using automation
    - Advanced tone creation using all 8 channels, and even multiple YM2151 chips
    - Support for all FM sound sources beyond the YM2151 framework
    - Support for all DAWs and audio plugins, enabling playback with each and import/export of FM sound source tones

## Considering a Format for Tone Saving
- Past Challenges
    - ym2151-log format
        - JSON data with many lines.
        - Cannot store multiple tone variations in one file.
        - Maintaining this directly on GitHub for General MIDI is not very realistic.
        - Will continue to be used for server transmission. However, there's a sense that a more appropriate format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-descriptive
                - Directory hierarchy and file names make the purpose and tone clear.
    - Commit
        - Commit to the ym2151-tone-editor repository 0-1 times per day.
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
    - The core is `registers`. This is a required field.
    - `mml`, `note_number`, `description` are optional fields.
    - If `mml` and `note_number` are omitted, the application decides what plays (e.g., middle C).
    - If both `mml` and `note_number` are provided, the application decides which plays (e.g., `note_number`, then `mml`, alternating).
- Data Format Explanation
    - Address and Data
        - A sequence of address (2 chars) and data (2 chars) pairs.
    - Benefits
        - Structured
            - Being JSON, it avoids the ambiguity of natural language and allows simple code for reading and writing.
        - Flexibility
            - If a format were to constrain to specific registers and fixed descriptive methods, it could encounter issues like:
                - Example: Insufficient information in this format.
                - Example: High cost to determine how much information is sufficient for the format.
                - Example: Need for parser/output code changes or migration if the format changes later.
                    - Format changes include modifications to descriptive methods or changes in target registers.
            - This proposed format avoids these problems.
        - Self-descriptive
            - `description` ensures readability and self-descriptiveness, similar to directory and file names.
                - JSON itself also contributes to this.
        - Variations
            - In practice, even GM000 can have many variations, so this is handled by storing them in a JSON array.
        - Readability
            - Writing on a single line with `description` at the beginning makes it highly readable. Intended to be treated as a list of tone variation names.
        - Portability
            - A highly portable format, making mutual conversion code easy to write.
        - Uniqueness
            - Using `registers` as a unique ID provides some benefits of uniqueness.
                - Benefit: Can detect duplicates, potentially preventing excessive tone library bloat.
                - Benefit: Can be used as an ID when uniquely identifying a tone.
                    - Can be searched even if the `description` changes.
                    - Can simplify handling in various ways.
                - Benefit: Searching by `registers` can reveal "This is YM2151 tone data from so-and-so's repository." The data has self-descriptiveness.
                    - For this, `registers` must be kept in a format without delimiters.
                    - The prerequisite is that it's registered under GitHub management and the registration location is self-descriptive.
                - Caution: This is only to a certain extent. If even 1 bit differs, it's a different ID, even for nearly identical tones.
    - Supplementary Notes
        - Slot mask
            - By including "note on" in `registers`, the slot mask can be represented. The application can extract the slot mask from it. `ym2151-tone-editor` already implements this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, etc.
        - Storing all 256 bytes of register information in JSON is not recommended. It risks unexpected application behavior.
            - Thorough review and consideration of this will be deferred. YAGNI. It's assumed to be handled by the application later.
        - Note that advanced playing techniques like modulator TL automation cannot be included in this tone data.
            - This means "tone data containing advanced playing techniques" that cannot be fully expressed by this format may exist, and compatibility with them will be limited.
- Challenges and Solutions
    - Challenge: 128 items is cumbersome.
    - Solution: It is assumed that this can be sufficiently handled by writing simple code for it.
        - For example, by preparing a list of 128 tone names and simple code, JSON filename generation and description generation would be easy.

## Considering Keybinds
- *Note: Each point will be separated into individual issues. Prioritize safety. Prevent confusion.*
- *Note: Will be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.*
- Concept
    - Basic operations are completed solely with cursor keys and Page Up/Down.
    - Supplementary notes
        - Shortcut keys will complement quick editing and advanced features.
        - Mouse left-click for cursor movement, wheel for value increase/decrease will also be implemented as it's standard.
            - Right-click in TUI is confusing, so it's best avoided.
        - For termination and some other functions, ESC alone is sufficient, as it's a standard practice.
- `+` and `-` to increase/decrease values. This is widely known and easily understood, improving UX for new users.
- `CTRL hjkl` for cursor movement. `CTRL npfb` also for cursor movement.
    - While cursor-key-less movement is already possible with other shortcuts, being able to use these might improve UX, especially for beginners.
- `P` and `space` for playback. Being able to repeatedly play the current sound improves UX.
- `F` to increase FB, `SHIFT+F` to decrease FB. Cursor also moves to FB.
    - Other similar operations should combine cursor jump and value change, for speed. Will verify.
- `T` to increase TL of current row, `SHIFT+T` to decrease.
- `M` to increase MUL of current row, `SHIFT+M` to decrease.
    - Reminder: If `M` is prioritized for something else, use `X`. `X` for multiplier has a similar meaning.
- `A,D,S,R` to increase AR, D1R, D2R, RR of current row, `SHIFT+` to decrease.
    - Supplementary: Discontinue WASD for cursor movement. It led to many errors for this purpose, and no benefits were felt. It was assumed that constantly shifting the left hand one position left from home row caused many errors.
- `L` to increase D1L, `SHIFT+L` to decrease.
    - `L` for D1L. Explanation in the heading is clear.
- `1,2,3,4` to move directly to M1, C1, M2, C2 rows and increase the value in the current column.
    - `SHIFT` + key will decrease the value.
    - Purpose: For quickly increasing/decreasing values across operators.
        - Example: When working on OP1 and want to increase OP4, `4` is 4x faster than 3 cursor key presses and Page Up.
    - Caution: Numbers are relatively hard to touch-type, so `hjkl` will also be tested as aliases.
- `5,6,7,8` to toggle SlotMask for OP1-4.
    - `SHIFT` + key to toggle solo mode.
        - Even for modulators, in solo mode, force ALG7 playback to check envelopes, etc. In this case, the forced SlotMask is only on for the current row.
            - Display ALG and SM with a special color or background color to make it clear.
        - The row with the cursor is always in solo mode, meaning SM dynamically changes with cursor movement.
        - Toggling solo off reverts to the ALG held just before solo was toggled on.
            - `SHIFT+5,6,7,8` will all toggle solo off, for a simple initial specification.
                - This means no soloing two ops. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for "locK" is easy to explain.
    - When locked, pressing `F` key etc. will not move the cursor.
        - Multiple locks are possible. Each locked position becomes a target for mouse value increase/decrease.
        - Intended use: For collectively increasing/decreasing envelopes while previewing.
    - When unlocked, mouse behavior:
        - Left-click moves cursor to position and increases value.
        - Right-click moves cursor to position and decreases value.
- `,` and `.` for Note down and up, respectively. Use a C Ionian scale centered around middle C.
    - However, since these keys are also strong candidates for value increase/decrease, keybind changes are anticipated in the future.