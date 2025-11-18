# ym2151-tone-editor

YM2151 (OPM) FM synthesizer tone editor. For Windows. A Rust TUI (Text User Interface) editor.

## Status

Currently under development. Current progress is roughly 50%.

- Future Outlook
    - Note: All specifications are provisional for testing and subject to frequent breaking changes.
    - Tone save format suitable for GitHub management. Tone data itself will be described in approximately 100 characters per line. Details below.
    - Significant keybind changes. Details below.

## Features

- Edit YM2151 tone parameters with parameter labels
- Display 11 parameters × 5 rows (4 operators + 1 channel row)
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS
- Cursor navigation with arrow keys, `hjkl` (Vim-style), or `wasd` keys
- Increase/decrease values with PageUp/PageDown or `e`/`q` keys (respecting parameter maximums)
- Quick value setting with Home (max), End (min), R (random)
- Exit with `ESC` key
- Saves the tone as JSON on exit and loads the latest JSON on next launch

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

## Real-time Audio Feedback (Windows only)

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the ym2151-log-play-server library. This means server installation, startup, and readiness checks are automatically handled.

```bash
# Just run the tone editor - the server will be automatically set up and started
cargo run
```

The editor sends performance data via named pipes using `send_json`, providing instant playback upon editing.

**Note**: The library's `ensure_server_ready()` function handles all server management, including installation if necessary.

## Usage

Note: Subject to breaking changes in the future for validation purposes.

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
| `Home` | Set to maximum value for current parameter |
| `End` | Set to minimum value (0) |
| `r` / `R` | Set to random value (within valid range) |
| **Miscellaneous** | |
| `Mouse Movement` | Change cursor position value based on horizontal mouse position (Left=0, Center=proportional, Right=max) |
| `ESC` | Save and exit application |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concept
- Starts in 100ms, plays sound in 100ms (Note: These numbers are rough estimates, implying significantly less than 1 second).
- Pressing a key plays a sound and changes the tone.
    - Prioritize addressing the frustration of "can't hear it, can't edit it, it's confusing".
- Colorful visualization.
- Simple.
- Easy-to-grasp operation (cursor, mouse) for basic editing.

## Out of Scope, Not Aimed For
- Advanced Editor
    - A perfect, versatile editor that satisfies all users, from beginners to super-advanced.
    - Unlimited, intelligent UNDO.
    - Various intelligent, fully automatic, user-friendly, error-free, flexible, and advanced editing features.
- Interactive
    - Highly interactive performance via a virtual MIDI keyboard, with the server also changed to low-latency, highly real-time processing using shared memory.
    - Responsive, highly interactive performance in general.
- GUI
    - Graphical tone visualization. Envelope and waveform visualization using a dedicated terminal emulator, and a high-performance oscilloscope with 16ms display updates.
- Advanced Librarian
    - Flexible, easy, and quick access, preview, selection, editing, and highly intelligent version management for all tones.
    - Fully automatic or interactive and advanced tone extraction from existing music, with 100% success rate.
    - Automatic identification and loading of all YM2151 tone formats, with 100% success rate.
    - Automatic identification and conversion of all FM tone formats for loading, with 100% success rate.
- Advanced Extensibility
    - Advanced tone creation using automation.
    - Advanced tone creation utilizing all 8 channels, and even multiple YM2151 chips.
    - Support for all FM synthesizers, beyond the YM2151 framework.
    - Compatibility with all DAWs and audio plugins, enabling playback within each and import/export of FM synth tones.

## Considering a Tone Save Format
- Previous Challenges
    - ym2151-log format
        - Multi-line JSON data.
        - Cannot store multiple tone variations in a single file.
        - Maintaining this as-is on GitHub for General MIDI is not very practical.
        - Will continue to be used for server communication. However, I feel a more appropriate format is needed for tone management.
### Proposed Solution
- Workflow
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-descriptive
                - Directory hierarchy and file names make the purpose and tone easily understandable.
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
    - If `mml` and `note_number` are omitted, the application determines what plays, e.g., middle C.
    - If both `mml` and `note_number` are provided, the application also determines which plays, e.g., alternating between `note_number` and `mml`.
- Data Format Description
    - Address and Data
        - Repeating pairs of 2-character address and 2-character data.
    - Benefits
        - Structured
            - Being JSON, it avoids the ambiguity of natural language and allows for simple code for reading and writing.
        - Flexibility
            - If the format were to restrict to specific registers and fix their description method, problems such as the following might arise, which can be avoided:
                - Example: This format lacks necessary information.
                - Example: High cost of format design to determine what information is sufficient to record.
                - Example: Subsequent format changes require modifications to parser/output code and migration.
                    - Format changes include modifications to description methods or increasing/decreasing target registers.
        - Self-descriptive
            - `description` ensures readability and self-descriptiveness, as do directory and file names.
                - This also applies to being JSON.
        - Variations
            - In practice, even GM000 can have many variations,
                - which is handled by storing them as an array within the JSON.
        - Readability
            - Writing in a single line with `description` at the beginning enhances readability. It's intended to be treated as a list of tone variation names.
        - Portability
            - A highly portable format; it's assumed that mutual conversion code can be written easily at this level.
        - Uniqueness
            - Utilizing `registers` as a unique ID is expected to offer some benefits of uniqueness.
                - Benefit: Duplicate detection can potentially mitigate excessive tone library bloat.
                - Benefit: Can be used as an ID when uniquely identifying a tone.
                    - Searchable even if the description changes.
                    - Can simplify various handling.
                - Benefit: Searching by `registers` reveals "this is YM2151 tone data from so-and-so's repository." The data is self-descriptive.
                    - For this, `registers` must maintain a format without delimiters.
                    - The premise is that it's managed on GitHub and its registration location is self-descriptive.
                - Caution: This is only to a certain extent; even a 1-bit difference in an almost identical tone results in a different ID.
    - Notes
        - Slot Mask
            - Including 'note on' in `registers` can express the slot mask. The application can extract the slot mask from it. `ym2151-tone-editor` has already implemented this.
            - The slot mask is used for purposes such as providing an easy-to-edit 2-op tone editing experience.
        - Saving all 256 bytes of register information as JSON is not recommended, as it carries the risk of the application behaving unexpectedly.
            - Detailed examination and consideration of that will be postponed (YAGNI). It's assumed the application can handle it later.
- Challenges and Solutions
    - Challenge: 128 items is tedious.
    - Solution: It's assumed that a simple piece of code written for this purpose would be sufficient.
        - For example, if a list of 128 tone names and simple code are prepared, generating JSON filenames and descriptions is expected to be easy.

## Considering Keybinds
- Note: Each point will be separated into individual issues. Prioritize safety. Prevent confusion.
- Note: Expected to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.
- Concept
    - Basic operations are fully accomplished with cursor keys and Page Up/Down.
    - Additional notes:
        - Shortcut keys supplement quick editing and advanced features.
        - Left-click for cursor movement, and mouse wheel for value adjustment are standard and will be implemented.
            - Right-click in a TUI is often confusing, so it's best avoided.
        - Furthermore, for some functions like exiting, `ESC` alone is sufficient, as it's a standard practice.
- `+` and `-` to increase/decrease values. This is a UX improvement for onboarding, as it's widely known and easy to understand.
- `CTRL + hjkl` for cursor movement. `CTRL + npfb` also for cursor movement.
    - Although movement without arrow keys is possible with other shortcuts, using these could improve UX, especially for beginners.
- `P` and `Space` for playback. Being able to repeatedly play the current sound improves UX.
- `F` to increase FB, `SHIFT+F` to decrease FB. The cursor will also move to FB.
    - Other similar operations are expected to perform cursor jumps and value changes simultaneously for speed. This will be tested.
- `T` to increase TL for the current row, `SHIFT+T` to decrease.
- `M` to increase MUL for the current row, `SHIFT+M` to decrease.
    - Memo: If `M` is prioritized elsewhere, use `X`. `X` is similar in meaning to 'multiple'.
- `A, D, S, R` to increase AR, D1R, D2R, RR for the current row, `SHIFT+` to decrease.
    - Note: Discontinue WASD for cursor movement. For this use case, it led to many errors and offered no perceived benefit. The need to constantly shift the left hand one position left from the home row was presumed to cause many mistakes.
- `L` to increase D1L, `SHIFT+L` to decrease.
    - `L` for D1L. The heading makes it easy to understand.
- `1, 2, 3, 4` to directly move to the M1, C1, M2, C2 rows and increment the value in the cursor's current column.
    - `SHIFT` key pressed will decrement.
    - Purpose: To quickly adjust values across operators (OPs).
        - Example: If you're on OP1 and want to increment OP4, using '4' is 1 action compared to 3 cursor key presses + Page Up, making it 4 times faster.
    - Note: Numbers are relatively difficult for touch-typing, so `hjkl` aliases will also be evaluated.
- `5, 6, 7, 8` to toggle SlotMask for OP1-4.
    - `SHIFT + (5, 6, 7, 8)` to toggle solo mode.
        - Even for modulators, solo mode will force playback with ALG7 to check envelopes, etc.
        - And the forced SlotMask will be on only for that row.
        - ALG and SM will be displayed with a special color or background color for clarity.
        - The row with the cursor will always be in solo mode, meaning the SM changes dynamically with cursor movement.
        - Toggling off returns to the ALG held just before solo mode was toggled on.
        - `SHIFT+5, 6, 7,` or `8` will toggle off solo mode; this is a simple initial specification.
        - This means no soloing of two OPs simultaneously. Simplicity first.
- `K` to toggle mouse multi-cursor lock. `K` for 'locK' makes the description clear.
    - When locked, pressing `F` key etc. will not move the cursor.
        - Multiple items can be locked. Each becomes a target for value adjustment via mouse.
        - Intended use: For previewing while collectively increasing/decreasing envelope values.
    - When not locked, mouse behavior will be: left-click moves cursor to location and increments value, right-click moves cursor to location and decrements value.
- `,` and `.` for Note down and up, respectively. Based on a C Ionian scale centered around middle C.
    - However, since they are also strong candidates for value increase/decrease, future keybind changes are anticipated.