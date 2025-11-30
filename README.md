# ym2151-tone-editor

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151 (OPM) FM tone editor. For Windows. TUI. Written in Rust.

## Status

In development. Current progress is 80%. The remaining 20% includes keybinds and tone management.

- Future Outlook
    - ※All are tentative specifications for testing, and breaking changes will occur frequently.
    - Establish a tone saving mechanism suitable for saving and GitHub management. Details below.
    - Significant keybind changes will be made. Details below.

## Features

- Edit YM2151 tone parameters
- Operable with mouse only
- Move with cursor keys, increase/decrease values with PageUp/PageDown/Home/End
- `P` or `SPACE` key for tone preview
- `ESC` key to exit
- Automatically saves the tone on exit and loads it automatically next time to resume editing
- Configurable keybinds

## Quick Start Guide

Will be written in the future.
If Rust is installed on Windows, you can easily install it from GitHub.

## Tips
- To display full screen:
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

The editor automatically ensures the server is ready using the `ensure_server_ready()` function from the ym2151-log-play-server library. This handles server installation, startup, and readiness checks automatically.

```bash
# Just run the tone editor - the server will be set up and started automatically
cargo run
```

### Operating Modes

The editor operates in two modes:

#### Interactive Mode (Default)

In interactive mode, the server continuously streams audio and only sends register write commands when parameters change. This provides more efficient and smoother audio feedback.

#### Legacy Mode

By default, the editor uses `send_json` to transmit complete tone data in JSON format via a named pipe. The entire new JSON is sent every time a parameter changes.

### Comparison

| Feature | Legacy Mode | Interactive Mode |
|---------|-------------|------------------|
| Data Transmission | Full JSON | Register writes only |
| Efficiency | Low (sends all data every time) | High (sends only changes) |
| Audio Continuity | Restarts on parameter change | Continuous streaming |
| Purpose | For comparative validation | For regular editing work |

## Operation Guide

※Subject to breaking changes in the future. For validation purposes.

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
| `Home` | Sets value to maximum for current parameter |
| `End` | Sets value to minimum (0) |
| `r` / `R` | Sets to a random value (within valid range) |
| **Mouse** | |
| `Mouse wheel up` | Moves cursor to mouse pointer position and increases value |
| `Mouse wheel down` | Moves cursor to mouse pointer position and decreases value |
| **Others** | |
| `ESC` | Save and exit application |

## Command Line Options

| Option | Description |
|--------|-------------|
| `--value-by-mouse-move` | Enables legacy mouse behavior (change value at cursor position by moving mouse left/right) |

## Dependencies

- `ratatui` 0.28 - Terminal UI framework
- `crossterm` 0.28 - Cross-platform terminal manipulation library

## Concept
- Starts in 100ms, plays sound in 100ms ※These numbers are approximate; imagine significantly faster than 1 second.
- Pressing a key plays a sound and changes the tone.
    - Prioritize addressing the issue of "it doesn't play and can't be edited when I try, it's confusing."
- Colorful visualization
- Simple
- Easy-to-learn operation for basic editing (cursor, mouse)

## Out of Scope, Not Aimed For
- High-performance editor
    - A perfect, versatile editor that satisfies everyone from beginners to super advanced users
    - Unlimited intelligent UNDO
    - Various intelligent, fully automatic, easy-to-use, error-free, flexible, and advanced editing features
- Interactive
    - Highly interactive performance with a virtual MIDI keyboard, and changing the server to use shared memory for low-latency, advanced real-time processing
    - Highly interactive performance in general
- GUI
    - Graphical visualization of tones. Envelope and waveform visualization using a dedicated terminal emulator, a high-performance oscilloscope with 16ms display refresh.
- Advanced Librarian
    - Flexible, easy, and quick access, preview, selection, editing, and highly intelligent version control for all tones
    - Fully automatic or interactive advanced tone extraction from existing music, with 100% success rate
    - Automatic detection and loading of all YM2151 tone formats, with 100% success rate for automatic detection
    - Automatic detection and conversion for loading all FM tone formats, with 100% success rate
- High Extensibility
    - Advanced tone creation using automation
    - Advanced tone creation using all 8 channels, and even multiple YM2151 chips
    - Support for all FM sound sources beyond the YM2151 framework
    - Compatibility with all DAWs and audio plugins, enabling playback and import/export of FM tone data for each

## Considering Tone Saving Format
- Past Challenges
    - ym2151-log format
        - JSON data with many lines.
        - Cannot store multiple tone variations in one file.
        - Maintaining this as is on GitHub for General MIDI is not very practical.
        - Will continue to be used for server transmission. However, there's a feeling that a more appropriate format is needed for tone management.
### Proposed Solution
- Operation
    - Placement
        - `tones/general_midi/000_AcousticGrand.json`
        - Benefits
            - Self-describing
                - Directory hierarchy and file name make the purpose and tone easy to understand.
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
    - If `mml` and `note_number` are omitted, what plays is up to the application, for example, middle C.
    - If both `mml` and `note_number` are provided, which one plays is also up to the application, for example, alternating between `note_number` and `mml`.
- Data Format Description
    - Address and Data
        - A repeating pair of 2-character address and 2-character data.
    - Benefits
        - Structured
            - Being JSON, there is no ambiguity like natural language, allowing for simple code to read and write.
        - Flexibility
            - If restricted to specific registers and a fixed description method, problems like those below might arise, but this format avoids them:
                - Example: This format lacks necessary information.
                - Example: Costly format review to determine what information is sufficient to record.
                - Example: Later format changes require modifications to parser/output code or migration.
                    - Format changes include changes in description methods, or increasing/decreasing target registers.
        - Self-describing
            - `description` ensures readability and self-descriptiveness, as do directory and file names.
                - The JSON format itself also contributes to this.
        - Variations
            - In practice, even GM000 can have many variations, so this is handled by keeping them in an array within the JSON.
        - Readability
            - Writing in a single line with `description` at the beginning enhances readability. Intended to be treated as a list of tone variation names.
        - Portability
            - A highly portable format, at this level, it is easy to write mutual conversion code.
        - Uniqueness
            - Utilizing `registers` as a unique ID is expected to provide a certain level of uniqueness.
                - Benefit: Can detect duplicates, potentially preventing excessive tone library bloat.
                - Benefit: Can be used as an ID when identifying a specific tone.
                    - Searchable even if the description changes.
                    - Could make handling various tasks easier.
                - Benefit: Searching by `registers` can identify "YM2151 tone, data from so-and-so's repository." The data itself is self-describing.
                    - For this, it is necessary to maintain a format for `registers` that does not use delimiters.
                    - This assumes that it is managed on GitHub and the registration location is self-describing.
                - Note: This is only to a certain extent. Even nearly identical tones will have different IDs if they differ by 1 bit.
    - Notes
        - Slot Mask
            - Including 'note on' in `registers` can express the slot mask. The application can extract the slot mask from it. `ym2151-tone-editor` has already implemented this.
            - The purpose of the slot mask is to provide an easy-to-edit 2-op tone editing experience, etc.
        - Saving all 256 bytes of register information in JSON is not recommended, as it carries the risk of the application behaving unexpectedly.
            - Detailed examination and consideration of this will be deferred. YAGNI. It is assumed that it can be handled by the application later.
        - Please note that advanced playing techniques such as modulator TL automation cannot be included in this tone data.
            - This means that "advanced playing technique tone data" that cannot be expressed by this format may exist, and compatibility with it will be limited.
- Challenges and Solutions
    - Challenge: 128 items is cumbersome.
    - Solution: It is assumed that this can be adequately addressed by writing simple code for it.
        - For example, if a list of 128 tone names and simple code are prepared, JSON filename generation and description generation would be easy.

## Considering Keybinds
- ※Each will be separated into individual issues. Prioritize safety. Prevent confusion.
- ※Intended to be configurable in the `keybinds` section of `ym2151-tone-editor.toml`.
- Concept
    - Basic operations are completed with just cursor keys and Page Up/Down.
    - Notes
        - Shortcut keys will supplement for quick editing and advanced features.
        - Mouse left-click for cursor movement, wheel for value increment/decrement, will also be implemented as it is standard.
            - Right-click in TUI is confusing, so it should be avoided.
        - Some functions, like exiting, only need `ESC`, which is considered standard.
- Increase/decrease values with `+` and `-`. This is widely known and easy to understand, improving the introductory UX.
- Move cursor with `CTRL hjkl`. `CTRL npfb` also moves the cursor.
    - Cursor movement without arrow keys is already possible with other shortcut keys, but having these available could improve UX, especially for new users.
- Play with `P` and `SPACE`. The ability to repeatedly play the current sound as is offers a good UX.
- Use `F` to increase FB, `SHIFT+F` to decrease FB. The cursor also moves to FB.
    - Other similar operations will involve cursor jump and value adjustment as a set, which is expected to be faster. This will be verified.
- Use `T` to increase TL for the current row, `SHIFT+T` to decrease.
- Use `M` to increase MUL for the current row, `SHIFT+M` to decrease.
    - Memo: If `M` is prioritized elsewhere, use `X`. `X` is close in meaning to 'multiple'.
- Use `A,D,S,R` to increase AR, D1R, D2R, RR for the current row, `SHIFT+` to decrease.
    - Note: Discontinuing WASD for cursor movement. For this use case, it led to many errors and offered no perceived benefits. It was assumed that errors frequently occurred due to the need to constantly shift the left hand one position left from the home row.
- Use `L` to increase D1L, `SHIFT+L` to decrease.
    - The `L` in D1L. The explanation in the heading is easy to understand.
- Use `1,2,3,4` to directly move to the M1, C1, M2, C2 rows while increasing the value in the cursor's current column.
    - Holding `SHIFT` will decrease the value.
    - Purpose: To quickly increase/decrease values across operators.
        - Example: If you are editing OP1 and want to increase OP4, using `4` is 4 times faster (1 press) compared to 3 cursor key presses and a Page Up.
    - Caution: Numbers are relatively difficult to touch-type, so `hjkl` will also be tested as aliases.
- Use `5,6,7,8` to toggle the SlotMask for OP1-4.
    - Holding `SHIFT` toggles solo mode.
        - Even for modulators, solo mode will force playback with ALG7 to check envelopes, etc. In this case, the SlotMask for that specific row will be forced ON.
        - The ALG and SM in this state should be clearly indicated with a special color or background color.
        - The row with the cursor will always be in solo mode, meaning SM dynamically changes with cursor movement.
        - When solo mode is toggled off, it reverts to the ALG that was active just before solo mode was toggled on.
            - Any `SHIFT+5,6,7,8` will toggle solo mode off, prioritizing simplicity.
        - This means no soloing of two operators simultaneously. Simplicity first.
- Use `K` to toggle mouse multi-cursor lock. `locK` makes the display explanation easy.
    - When locked, pressing keys like `F` will not move the cursor.
        - Multiple items can be locked. Each becomes a target for value adjustment via mouse.
        - Intended use: For previewing while increasing/decreasing envelopes collectively.
    - When not locked, mouse behavior will be: left-click moves the cursor to the clicked location and increases the value, right-click moves the cursor to the clicked location and decreases the value.
- Use `,` and `.` for Note down and up, respectively. This will use a C Ionian scale centered around middle C.
    - However, these are also strong candidates for general value increment/decrement, so keybind changes are anticipated in the future.