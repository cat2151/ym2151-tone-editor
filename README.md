# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM sound editor. For Windows. A Rust TUI (Text User Interface) editor.

## Status

Under development. Current progress is roughly 50%.

- Future Prospects
    - *All are temporary specifications for testing and are subject to frequent breaking changes.*
    - A format suitable for tone saving and GitHub management. Tone data itself will be described in approximately 100 characters per line. Details below.
    - Major keybind changes. Details below.

## Features

- Edit YM2151 tone parameters with labels
- Display 11 parameters × 5 rows (4 operators + 1 channel row)
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS
- Cursor navigation with arrow keys, `hjkl` (Vim-style), or `wasd` keys
- Increase/decrease values with PageUp/PageDown or `e`/`q` keys (respects parameter maximums)
- Fast value setting with Home (maximum), End (minimum), R (random)
- Exit with `ESC` key
- Saves tone to JSON on exit and loads the latest JSON on next startup

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

Or run the compiled binary directly:

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

By default, the editor sends complete tone data in JSON format via a named pipe using `send_json`. Each time a parameter is changed, the entire new JSON is sent.

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
- When parameters are changed, it updates only the affected YM2151 registers using `write_register()`.
- Calls `stop_interactive()` on exit to stop audio streaming.

**Note**: The library's `ensure_server_ready()` function handles all server management, including installation as needed.

### Mode Comparison

| Feature | Legacy Mode | Interactive Mode |
|------|---------------|---------------------|
| Data Transmission | Full JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changed parts) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Usage | For comparison/verification | For normal editing tasks |

## How to Use

*Note: This will be subject to breaking changes in the future for verification purposes.*

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in corresponding direction |
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
| **Mouse** | |
| `Mouse Wheel Up` | Move cursor to mouse position and increase value |
| `Mouse Wheel Down` | Move cursor to mouse position and decrease value |
| **Other** | |
| `ESC` | Save and exit application |

## Command-Line Options

| Option | Description |
|--------|-------------|
| `--use-client-interactive-mode-access` | Use interactive mode for more efficient audio feedback (continuously streams audio and sends only register changes) |
| `--value-by-mouse-move` | Enable legacy mouse behavior (horizontal mouse movement changes value at cursor position) |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concept
- Startup in 100ms, sound plays in 100ms *Note: these numbers are rough estimates, implying significantly less than 1 second.*
- Press a key, sound plays and tone changes
    - Prioritize addressing "Can't hear anything, can't edit, it's confusing."
- Colorful visualization
- Simple
- Easy-to-learn controls for basic editing (cursor, mouse)

## Out of Scope, Not Aimed For
- High-functionality editor
    - A perfect, all-purpose editor that satisfies everyone from beginners to super-advanced users.
    - Unlimited intelligent UNDO
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features.
- Interactive
    - Highly interactive performance using a virtual MIDI keyboard, with the server also changing to low-latency, advanced real-time processing using shared memory.
    - Highly responsive and interactive performance in general.
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates.
- Advanced Librarian
    - Flexible and intuitive quick access, preview, selection, editing, and highly intelligent version control for all tones.
    - Fully automatic or interactive and advanced tone extraction from existing songs, with a 100% success rate.
    - Automatic detection and loading of all YM2151 tone formats, with a 100% success rate.
    - Automatic detection and conversion of all FM tone formats for loading, with a 100% success rate.
- Advanced Extensibility
    - Advanced tone creation using automation.
    - Advanced tone creation utilizing all 8 channels, and even multiple YM2151 chips.
    - Support for all FM sound chips beyond the YM2151 framework.
    - Support for all DAWs and audio plugins, enabling playback and import/export of FM sound chip tones for each.

## Considering a Tone Saving Format
- Past Challenges
    - ym2151-log format
        - JSON data with many lines.
        - Cannot store multiple tone variations in one file.
        - Maintaining this as-is on GitHub for General MIDI is not very practical.
        - It will continue to be used for server transmission. However, there's a feeling that a more suitable format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-describing
                - Directory structure and file names clearly indicate purpose and tone.
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
- JSON File Format Description
    - The core is `registers`. This is a required field.
    - `mml`, `note_number`, `description` are optional fields.
    - If `mml` and `note_number` are omitted, what plays is up to the application, e.g., middle C.
    - If both `mml` and `note_number` are provided, which one plays is also up to the application, e.g., `note_number`, then `mml`, alternating.
- Data Format Description
    - Address and Data
        - Repeating pairs of 2-character address, 2-character data.
    - Benefits
        - Structured
            - Being JSON, there is no ambiguity like natural language, allowing for simple code to read and write.
        - Flexibility
            - If the format were fixed to specific registers and a specific notation, the following problems might arise, but this approach avoids them:
                - Example: This format lacks necessary information.
                - Example: It would require format consideration costs to determine how much to record for a sufficient format.
                - Example: Format changes later would require parser and output code changes, or migration.
                    - Format changes include changes in notation or additions/deletions of target registers.
        - Self-Describing
            - `description` ensures readability and self-descriptiveness, as do the directory and file names.
                - Being JSON also contributes.
        - Variations
            - Practically, GM000 can have many variations, so
                - this is handled by storing them as an array within the JSON.
        - Readability
            - Writing in a single line with `description` at the beginning provides high readability. It can be treated as a list of tone variation names.
        - Portability
            - A highly portable format; at this level, it's expected to be easy to write code for mutual conversion.
        - Uniqueness
            - Utilizing `registers` as a unique ID is expected to provide some benefits of uniqueness.
                - Benefit: Duplicate detection can help prevent excessive tone library bloat to some extent.
                - Benefit: Can be used as an ID to uniquely identify a specific tone.
                    - Searchable even if the description changes.
                    - May simplify various handling procedures.
                - Benefit: Searching by `registers` reveals "This is YM2151 tone data from so-and-so's repository." The data is self-descriptive.
                    - Therefore, `registers` must maintain a format without delimiters.
                    - The premise is that it's registered under GitHub management and the registration location is self-describing.
                - Note: This is only to a certain extent. Even nearly identical tones will have different IDs if a single bit differs.
    - Supplement
        - Slot Mask
            - By including "note on" in `registers`, the slot mask can be represented. The application can extract the slot mask from it. `ym2151-tone-editor` has already implemented this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, among others.
        - Saving all 256 bytes of register information to JSON is not recommended. This carries the risk of the application behaving unexpectedly.
            - Detailed examination and consideration of this will be postponed. YAGNI. It's assumed the application can handle it later.
        - It should be noted that advanced performance techniques like modulator TL automation cannot be included in this tone data.
            - This means "tone data containing advanced performance techniques" that cannot be fully expressed by this format may exist, and compatibility with it will be limited.
- Challenges and Solutions
    - Challenge: 128 items is tedious.
    - Solution: It's assumed that writing simple code for this will be sufficient to handle it.
        - For example, preparing a 128-line list of tone names and simple code should make JSON filename generation and description generation easy.

## Considering Keybinds
- *Note: Each will be separated into individual issues. Prioritize safety. Prevent confusion.*
- *Note: Expected to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.*
- Concept
    - Basic operations are complete with just cursor keys and Page Up/Down.
    - Supplement
        - Shortcut keys supplement quick editing and advanced functions.
        - Left-click for cursor movement, and mouse wheel for value increase/decrease are also standard, so these will be implemented.
            - Right-click is confusing in TUI, so it's better to avoid it.
        - Furthermore, for some functions like exiting, `ESC` alone is sufficient, as this is considered standard.
- Use `+` and `-` to increase/decrease values. This is widely known and easy to understand, thus improving the introductory UX.
- CTRL + `hjkl` for cursor movement. CTRL + `npfb` for cursor movement as well.
    - While cursor-key-less movement is achievable with other shortcuts, having these options might improve UX, especially for new users.
- `P` and `Space` for playback. Being able to repeatedly play the current sound improves UX.
- `F` to increase Feedback (FB), `SHIFT+F` to decrease FB. The cursor also moves to FB.
    - Other similar operations are also assumed to combine cursor jumps and value changes for speed. This will be tested.
- `T` to increase Total Level (TL) for the current row, `SHIFT+T` to decrease.
- `M` to increase Multiplier (MUL) for the current row, `SHIFT+M` to decrease.
    - Memo: If `M` is prioritized for something else, use `X`. The letter `x` could be thought of as close in meaning to "multiple."
- `A`, `D`, `S`, `R` to increase AR, D1R, D2R, RR for the current row, `SHIFT+` to decrease.
    - Supplement: Discontinue WASD for cursor movement. It led to many errors for this use case, and no benefits were felt. It was assumed that constant shifting of the left hand one position to the left from the home row resulted in many mistakes.
- `L` to increase D1L, `SHIFT+L` to decrease.
    - The 'L' in D1L. Easy to understand from the heading.
- `1`, `2`, `3`, `4` to directly move to the M1, C1, M2, C2 rows respectively, and increase the value in the current cursor column.
    - `SHIFT` key pressed simultaneously for decrease.
    - Purpose: To quickly increase/decrease values across operators (OPs).
        - Example: When working on OP1 and wanting to increase OP4, compared to 3 cursor key presses and Page Up,
            - using `4` is one press, making it 4 times faster.
    - Note: Numbers are relatively hard to touch-type, so `hjkl` aliases will also be considered.
- `5`, `6`, `7`, `8` to toggle SlotMask for OP1~4.
    - `SHIFT` pressed simultaneously for solo mode toggle.
        - Even for modulators, in solo mode, it will force ALG7 playback,
            - for checking envelopes, etc. And the forced SlotMask will be ON only for the corresponding row.
                - The ALG and SM will be displayed in a special color or background color for clarity.
        - The row with the cursor will always be in solo mode, meaning the SlotMask (SM) dynamically changes with cursor movement.
        - When solo mode is untoggled, it returns to the ALG held just before solo mode was toggled ON.
            - Untoggling will be done by any of `SHIFT+5,6,7,8`. This is a simple specification for now.
                - This means no solo for two operators. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for `locK` makes it easy to explain in the display.
    - When locked, pressing `F` key, etc., will not move the cursor.
        - Multiple targets can be locked. Each becomes a target for value increase/decrease via mouse.
        - Intended use: For previewing while increasing/decreasing envelopes collectively.
    - When not locked, mouse behavior is:
        - Left-click moves cursor to location and increases value.
        - Right-click moves cursor to location and decreases value.
- `,` and `.` for Note down and up, respectively. Based on a C Ionian scale centered around middle C.
    - However, since they are also strong candidates for value increase/decrease, future keybind changes are anticipated.