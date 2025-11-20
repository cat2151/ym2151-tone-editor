# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM sound tone editor for Windows. A Rust TUI (Text User Interface) editor.

## Status

Currently under development. Current progress is approximately 50%.

- Future Outlook
    - *Note: All specifications are provisional for verification and are subject to frequent breaking changes.*
    - Tone saving format suitable for GitHub management. Tone data itself described in approximately 100 characters per line. Details below.
    - Significant keybind changes. Details below.

## Features

- Edit YM2151 tone parameters with parameter labels.
- Display 11 parameters across 5 rows (4 operators + 1 channel row).
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS.
- Cursor navigation using arrow keys, `hjkl` (Vim-style), or `wasd` keys.
- Increase/decrease values using PageUp/PageDown or `e`/`q` keys (respects parameter maximums).
- Fast value setting with Home (maximum), End (minimum), R (random).
- Play the currently edited tone with `P` or `SPACE` key (check sound without changing parameter values).
- Exit with `ESC` key.
- Saves the tone as JSON on exit and loads the latest JSON on next startup.

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

Or run the compiled binary directly:

```bash
./target/release/ym2151-tone-editor
```

## Real-time Audio Feedback (Windows Only)

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the `ym2151-log-play-server` library. This automatically handles server installation, startup, and readiness checks.

```bash
# Just run the tone editor - the server will be set up and launched automatically
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Legacy Mode (Default)

By default, the editor uses `send_json` to transmit complete tone data in JSON format via a named pipe. The entire new JSON is sent every time a parameter is changed.

```bash
cargo run
```

#### Interactive Mode (New Feature)

In interactive mode, the server continuously streams audio, and only register write commands are sent when parameters change. This provides more efficient and smoother audio feedback.

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
| Data Transmission | Full JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Use Case | For comparison/verification | For regular editing tasks |

## How to Use

*Note: This is subject to breaking changes in the future for verification purposes.*

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
| `+` / `.` | Increase value by 1 |
| `-` / `,` | Decrease value by 1 |
| `Shift` + `.` (`>`) | Increase value by 10 |
| `Shift` + `-` (`_`) | Decrease value by 10 |
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
| `--use-client-interactive-mode-access` | Use interactive mode for more efficient audio feedback (continuously streams audio and sends only register changes) |
| `--value-by-mouse-move` | Enable legacy mouse behavior (mouse horizontal movement changes value at cursor position) |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concept
- Launch in 100ms, sound in 100ms * (These numbers are rough; the idea is significantly faster than 1 second)
- Pressing a key plays sound and changes the tone.
    - Prioritize addressing the frustration of "can't play sound, can't edit, unclear how to use."
- Colorful visualization.
- Simple.
- Easy-to-learn operation (cursor, mouse) for basic editing.

## Out of Scope, Not Aimed For
- Feature-rich editor
    - A perfect, all-purpose editor that satisfies everyone from beginners to super-advanced users.
    - Unlimited intelligent UNDO.
    - Various intelligent, fully automatic, easy-to-use, flexible, and advanced editing features without mistakes.
- Interactive
    - Highly interactive performance via a virtual MIDI keyboard; changing the server to use shared memory for low-latency, advanced real-time processing.
    - Overall highly interactive performance with good responsiveness.
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, high-performance oscilloscope with 16ms display updates.
- Advanced librarian
    - Flexible, easy, and quick access, preview, selection, editing, and highly intelligent version management for all tones.
    - Fully automatic or interactive and advanced tone extraction from existing songs, with a 100% success rate.
    - Automatic identification and loading of all YM2151 tone formats, with a 100% identification success rate.
    - Automatic identification and conversion of all FM tone formats for loading, with a 100% success rate.
- Advanced extensibility
    - Advanced tone creation using automation.
    - Advanced tone creation utilizing all 8 channels, and even multiple YM2151 chips.
    - Support for all FM sound chips beyond the YM2151 framework.
    - Compatibility with all DAWs and audio plugins, enabling playback and tone import/export for each FM sound chip.

## Considering a Tone Data Storage Format
- Previous challenges
    - ym2151-log format
        - JSON data with many lines.
        - Cannot include multiple tone variations in one file.
        - Maintaining this as-is on GitHub for General MIDI is not very practical.
        - Will continue to be used for server transmission. However, for tone management, a more suitable format is needed.
### Proposed solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Advantages
            - Self-documenting
                - Directory hierarchy and filename clearly indicate purpose and tone.
    - Commit
        - Commit to the ym2151-tone-editor repository at a frequency of 0-1 times per day.
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
    - The core is `registers`. This is a mandatory item.
    - `mml`, `note_number`, `description` are optional items.
    - If `mml` and `note_number` are omitted, what sounds is up to the application (e.g., middle C).
    - If both `mml` and `note_number` are written, which one sounds is also up to the application (e.g., `note_number`, then `mml`, alternating).
- Data Format Explanation
    - Address and Data
        - Repeated pairs of 2-character address and 2-character data.
    - Advantages
        - Structure
            - Being JSON, it has no ambiguity like natural language, allowing simple code for reading and writing.
        - Flexibility
            - If the format were restricted to specific registers and fixed to a particular description method, problems like the following could arise, but this approach avoids them:
                - Example: In this format, necessary information might be insufficient.
                - Example: High format consideration cost to determine what information is sufficient to record.
                - Example: If the format is changed later, requiring changes to parser/output code and migration.
                    - Format changes include modifications to description methods or increasing/decreasing target registers.
        - Self-documenting
            - `description` ensures readability and self-documentation, as do directory and file names.
                - Being JSON also contributes to this.
        - Variations
            - In practice, even GM000 can have many variations; this is handled by keeping them in an array within the JSON.
        - Readability
            - Writing on a single line, with `description` at the beginning, provides high readability. Intended to be treated as a list of tone variation names.
        - Portability
            - Highly portable format; at this level, it's easy to write code for mutual conversion.
        - Uniqueness
            - Utilizing `registers` as a unique ID provides some benefits of uniqueness.
                - Advantage: Can detect duplicates, potentially preventing excessive tone library bloat.
                - Advantage: Can be used as an ID to uniquely identify a tone when needed.
                    - Can be searched even if the `description` has changed.
                    - Can simplify various handling tasks.
                - Advantage: Searching by `registers` can identify "this is YM2151 tone data from so-and-so's repository." The data is self-describing.
                    - For this, it is necessary to maintain a format for `registers` without delimiters.
                    - This assumes GitHub management and self-describing registration locations.
                - Note: This is only to a certain extent. Almost identical tones will have different IDs if even 1 bit differs.
    - Supplement
        - Slot Mask
            - Including `note on` in `registers` can represent the slot mask. The application can extract the slot mask from it. `ym2151-tone-editor` already implements this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-operator tone editing experience, etc.
        - Saving all 256 bytes of register information as JSON is not recommended. This carries the risk of the application behaving unexpectedly.
            - Detailed examination and consideration will be postponed. YAGNI. It is assumed that the application can handle it later.
        - Note that advanced performance techniques such as modulator TL automation cannot be included in this tone data.
            - This means that "advanced performance tone data" that cannot be expressed in this format may exist, and compatibility with such data will be limited.
- Challenges and Solutions
    - Challenge: 128 entries is tedious.
    - Solution: It is assumed that this can be sufficiently handled by writing simple code for it.
        - For example, preparing a list of 128 tone names and simple code would make JSON filename generation and description generation easy.

## Considering Keybinds
- *Note: Each will be separated into individual issues. Prioritize safety, prevent confusion.*
- *Note: It is assumed that keybinds can be configured in the `keybinds` section of `ym2151-tone-editor.toml`.*
- Concept
    - Basic operations are fully achievable with cursor keys and Page Up/Down.
    - Supplement
        - Shortcut keys provide quick editing and advanced functionality.
        - Mouse left-click for cursor movement, wheel for value increment/decrement will also be implemented as they are standard.
            - Right-click in TUI is confusing, so it's best avoided.
        - For some functions, like exiting, `ESC` alone is sufficient, as it is standard.
- Use `+` and `-` to increment/decrement values. This is widely known and improves UX for new users.
- `CTRL hjkl` for cursor movement. `CTRL npfb` also for cursor movement.
    - While cursor movement is achievable with other shortcut keys, using these could also improve UX, especially for new users.
- `P` and `SPACE` for playback. Being able to repeatedly play the current sound improves the UX.
- `F` to increase FB, `SHIFT+F` to decrease FB. The cursor will also move to FB.
    - Similar operations will also combine cursor jump and value increment/decrement for faster workflow. This will be tested.
- `T` to increase TL of the current row, `SHIFT+T` to decrease.
- `M` to increase MUL of the current row, `SHIFT+M` to decrease.
    - Memo: If `M` is prioritized elsewhere, use `X`. `X` is conceptually close to "multiply".
- `A, D, S, R` to increase AR, D1R, D2R, RR of the current row, `SHIFT+` for decrease.
    - Supplement: Discontinue `WASD` for cursor movement. It led to too many mistakes for this purpose and offered no noticeable advantages. It required constantly shifting the left hand one position to the left from the home row, which often caused errors.
- `L` to increase D1L, `SHIFT+L` to decrease.
    - The `L` in D1L. The heading explanation makes it clear.
- `1, 2, 3, 4` to directly move to M1, C1, M2, C2 rows and increment the value in the current column.
    - Holding `SHIFT` will decrement.
    - Intended use: For quickly increasing/decreasing values across multiple operators.
        - Example: If you're working on OP1 and want to increment OP4, `4` takes 1 press compared to 3 cursor key presses and Page Up, making it 4 times faster.
    - Note: Since numbers are relatively hard to touch-type, `hjkl` aliases will also be tested.
- `5, 6, 7, 8` to toggle SlotMask for OP1-4.
    - Holding `SHIFT` will toggle solo mode.
        - Even for modulators, solo mode will force playback with ALG7 to check envelopes, etc.
            - In this case, ALG and SM will be highlighted with a special color or background color for clarity.
        - The row with the cursor will always be in solo mode, meaning SM dynamically changes with cursor movement.
        - Toggling solo mode off reverts to the ALG held just before solo mode was toggled on.
            - Toggling off will occur with any of `SHIFT+5, 6, 7, 8` for simplicity first.
                - This means no soloing of two ops simultaneously. Prioritizing simplicity.
- `K` to toggle mouse multi-cursor lock. The 'K' in 'lock' makes it easy to explain.
    - When locked, pressing keys like `F` will not move the cursor.
        - Multiple items can be locked. Each becomes a target for value increment/decrement via mouse.
        - Intended use: To preview envelopes while adjusting them in groups.
    - When not locked, mouse behavior will be:
        - Left-click moves cursor to position and increments value.
        - Right-click moves cursor to position and decrements value.
- `,` and `.` for Note down and up, respectively. Uses a C Ionian scale centered on middle C.
    - However, since these are also strong candidates for value increment/decrement, keybind changes are anticipated in the future.