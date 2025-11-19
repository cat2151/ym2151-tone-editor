# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.com/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A YM2151 (OPM) FM tone editor for Windows. Built as a Rust TUI (Text User Interface) editor.

## Status

Currently under development. Progress is roughly 50%.

- Future Outlook
    - \* All are provisional specifications for verification and are subject to frequent breaking changes.
    - Tone saving in a GitHub-friendly format. Tone data itself to be described in lines of approximately 100 characters. See below.
    - Significant keybind changes. See below.

## Features

- Edit YM2151 tone parameters with labeled controls
- Display 11 parameters across 5 rows (4 operators + 1 channel row)
- Visual parameter names: DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS
- Cursor navigation using arrow keys, `hjkl` (Vim style), or `wasd` keys
- Increment/decrement values using PageUp/PageDown or `e`/`q` keys (respecting parameter maximums)
- Fast value setting with Home (max), End (min), R (random)
- Play the currently edited tone with `P` or `SPACE` key (check sound without changing parameter values)
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

Alternatively, run the compiled binary directly:

```bash
./target/release/ym2151-tone-editor
```

## Real-time Audio Feedback (Windows Only)

The editor automatically ensures the server is ready by calling the `ensure_server_ready()` function from the `ym2151-log-play-server` library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

### Operation Modes

The editor operates in two modes:

#### Legacy Mode (Default)

By default, the editor uses `send_json` to transmit complete tone data in JSON format via a named pipe. Each time a parameter is changed, the entire new JSON is sent.

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
- When parameters change, it uses `write_register()` to update only the affected YM2151 registers.
- It calls `stop_interactive()` on exit to stop audio streaming.

**Note**: The library's `ensure_server_ready()` function handles all server management, including installation if needed.

### Mode Comparison

| Feature | Legacy Mode | Interactive Mode |
|---------|-------------|------------------|
| Data Transmission | Complete JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Usage | For comparison/verification | Normal editing workflow |

## How to Use

\* Subject to breaking changes in the future. For verification purposes.

| Key | Action |
|-----|--------|
| **Cursor Movement** | |
| Arrow keys (←↓↑→) | Move cursor in the respective direction |
| `h` / `a` | Move cursor left |
| `j` / `s` | Move cursor down |
| `k` / `w` | Move cursor up |
| `l` / `d` | Move cursor right |
| **Value Change** | |
| `PageUp` / `e` | Increase value at cursor position |
| `PageDown` / `q` | Decrease value at cursor position |
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
| `--use-client-interactive-mode-access` | Use interactive mode for more efficient audio feedback (continuously streams audio, sends only register changes) |
| `--value-by-mouse-move` | Enable legacy mouse behavior (change value at cursor position by moving mouse left/right) |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concepts
- Startup in 100ms, sound plays in 100ms. \* Numbers are rough; idea is significantly faster than 1 second.
- Press a key, sound plays and tone changes.
    - Prioritize addressing the feeling of "It doesn't play or edit, I don't understand it."
- Colorful visualization.
- Simple.
- Easy to grasp controls for minimal editing (cursor, mouse).

## Out of Scope, Not Aiming For
- High-performance editor
    - A perfect, versatile editor that satisfies all users from beginners to super-experts.
    - Unlimited intelligent UNDO.
    - Various intelligent, fully automatic, user-friendly, error-free, flexible, and advanced editing features.
- Interactive
    - Highly interactive performance with a virtual MIDI keyboard; server also changed to low-latency, advanced real-time processing using shared memory.
    - Highly interactive performance in general with good responsiveness.
- GUI
    - Graphical visualization of tones. Dedicated terminal emulator for envelope and waveform visualization, high-performance oscilloscope updating every 16ms.
- High-functionality librarian
    - Flexible, clear, and quick access, preview, selection, editing, and highly intelligent version control for all tones.
    - Fully automatic or interactive and advanced tone extraction from existing songs, with a 100% success rate.
    - Automatic detection and loading of all YM2151 tone formats, with a 100% detection success rate.
    - Automatic detection and conversion for loading all FM tone formats, with a 100% success rate.
- Advanced Extensibility
    - Advanced tone creation using automation.
    - Advanced tone creation utilizing all 8 channels, and even multiple YM2151s.
    - Compatibility with all FM sound sources, beyond the scope of YM2151.
    - Compatibility with all DAWs and audio plugins, allowing playback and import/export of FM sound source tones for each.

## Considering Tone Storage Format
- Past Challenges
    - `ym2151-log` format
        - JSON data with many lines.
        - Cannot include multiple tone variations in one file.
        - Maintaining this directly on GitHub for General MIDI is not very practical.
        - Will continue to be used for server transmission. However, there's a feeling that a more appropriate format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-describing
                - Directory hierarchy and filenames make purpose and tone easy to understand.
    - Commit
        - Commit to the `ym2151-tone-editor` repository 0-1 times a day.
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
    - If both `mml` and `note_number` are present, which one plays is also up to the application, e.g., `note_number`, then `mml`, playing alternately.
- Data Format Description
    - Address and Data
        - Repeated pairs of 2-character address, 2-character data.
    - Benefits
        - Structured
            - Being JSON, it lacks the ambiguity of natural language and can be read/written with simple code.
        - Flexibility
            - If a format were to limit to specific registers and fix their description method, problems like the following could arise, but this approach avoids them:
                - E.g., This format lacks necessary information.
                - E.g., How much information is sufficient for the format, incurring format design costs.
                - E.g., Later format changes require parser/output code changes or migration.
                    - Format changes include changes in description methods or increases/decreases in target registers.
        - Self-describing
            - `description` ensures readability and self-descriptiveness, as do directory and filenames.
                - The JSON nature also contributes to this.
        - Variations
            - In practice, GM000 might have many variations, so this is handled by storing them in an array within the JSON.
        - Readability
            - Writing on one line with `description` first leads to high readability. Intended to be treated as a list of tone variation names.
        - Portability
            - A highly portable format; code for mutual conversion at this level should be easy to write.
        - Uniqueness
            - Using `registers` as a unique ID provides a certain level of uniqueness.
                - Benefit: Duplicate detection can help prevent excessive tone library bloat.
                - Benefit: Can be used as an ID when uniquely identifying a tone.
                    - Searchable even if the `description` changes.
                    - Could make handling various aspects easier.
                - Benefit: Searching by `registers` can reveal "This is YM2151 tone data from so-and-so's repository." The data has self-descriptiveness.
                    - For this reason, `registers` must maintain a format without delimiters.
                    - Prerequisite: Being registered on GitHub and having a self-describing registration location.
                - Note: This is only to a certain extent. Tones that are almost identical but differ by 1 bit will have different IDs.
    - Supplement
        - Slot mask
            - Including "note on" in `registers` can represent the slot mask. The application can extract the slot mask from it. `ym2151-tone-editor` already implements this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-op tone editing experience, etc.
        - Saving all 256 bytes of register information to JSON is not recommended. This carries the risk of the application behaving unexpectedly.
            - Fine-tuning and consideration of this will be deferred. YAGNI. It's assumed to be manageable by the application later.
        - It should be noted that advanced performance techniques such as modulator TL automation cannot be included in this tone data.
            - This means "advanced performance data including tones" that cannot be expressed in this format may exist, and compatibility with it will be limited.
- Challenges and Solutions
    - Challenge: 128 items is a lot of effort.
    - Solution: It is assumed that this can be sufficiently handled by writing simple code for it.
        - For example, preparing a list of 128 tone names and simple code should make JSON filename generation and description generation easy.

## Considering Keybinds
- \* Each point will be separated into individual issues. Prioritize safety. Prevent confusion.
- \* Assumed to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.
- Concept
    - Basic operations are completed using only cursor keys and Page Up/Down.
    - Supplement
        - Shortcut keys provide fast editing and advanced functionality.
        - Mouse left-click for cursor movement, wheel for value increase/decrease, is also standard and will be implemented.
            - Right-click in TUI is confusing and best avoided.
        - Note that for some functions like exit, `ESC` alone is sufficient, as it is standard.
- `+` and `-` to increase/decrease values. This is widely known and improves UX for new users.
- `CTRL hjkl` for cursor movement. `CTRL npfb` also for cursor movement.
    - While cursor-key-less movement is achievable with other shortcuts, these could improve UX, especially for new users.
- `P` and `SPACE` for playback. Being able to repeatedly play the current sound improves UX.
- `F` to increase FB (Feedback), `SHIFT+F` to decrease FB. Cursor also moves to FB.
    - Other similar operations should combine cursor jump and value increase/decrease for speed. To be verified.
- `T` to increase TL (Total Level) of the current row, `SHIFT+T` to decrease.
- `M` to increase MUL (Multiplier) of the current row, `SHIFT+M` to decrease.
    - Memo: If `M` is prioritized for something else, use `X`. `x` is similar in meaning to "multiple".
- `A`, `D`, `S`, `R` to increase AR, D1R, D2R, RR of the current row, `SHIFT+` to decrease.
    - Supplement: Stop using `WASD` for cursor movement. It resulted in too many errors and didn't feel beneficial. The need to constantly shift the left hand one position left from home row led to many mistakes.
- `L` to increase D1L (Decay 1 Level), `SHIFT+L` to decrease.
    - The `L` in D1L. The heading explanation is easy to understand.
- `1`, `2`, `3`, `4` to directly move to M1, C1, M2, C2 rows, respectively, and increase the value of the column the cursor is on.
    - `SHIFT` key press will decrease the value.
    - Purpose: For quickly increasing/decreasing values across operators.
        - E.g., when working on OP1 and wanting to increase OP4, it's 1 press for `4` vs. 3 cursor key presses and page up, so 4 times faster.
    - Note: Numbers are relatively difficult for touch typing, so `hjkl` aliases will also be verified.
- `5`, `6`, `7`, `8` to toggle Slot Mask for OP1-4.
    - `SHIFT` key press will toggle solo mode.
        - Even for modulators, solo mode will force ALG7 playback to check envelopes, etc.
            - In this case, ALG and SM will be made clear with a special color or background.
        - The row with the cursor will always be in solo mode, meaning SM dynamically changes with cursor movement.
        - Toggling off solo mode will revert to the ALG held immediately before solo mode was toggled on.
            - `SHIFT+5,6,7,8` will all act as toggle off, a simple initial specification.
                - This means no soloing of two ops simultaneously. Prioritize simplicity first.
- `K` to toggle mouse multi-cursor lock. Easy to explain with `K` for `lock`.
    - When locked, pressing `F` key, etc., will not move the cursor.
        - Multiple lock targets are possible. Each will be subject to numerical increase/decrease by mouse.
        - Intended use case: Group-adjusting envelopes while previewing.
    - When not locked, mouse behavior will be:
        - Left-click moves cursor to position and increases value.
        - Right-click moves cursor to position and decreases value.
- `,` and `.` for Note down and up, respectively. Will use a C Ionian scale centered on middle C.
    - However, these keys are also strong candidates for value increase/decrease, so keybind changes are anticipated in the future.