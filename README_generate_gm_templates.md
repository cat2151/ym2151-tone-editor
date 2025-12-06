# GM Tone Template Generator

This script generates 128 GM (General MIDI) tone template JSON files from `tones/general_midi/tone_names.json`.

## Quick Start

### Method 1: Direct Compilation (Simplest)

```bash
rustc --edition 2021 generate_gm_templates.rs && ./generate_gm_templates
```

This works without any external dependencies and uses a built-in JSON parser.

### Method 2: Using rust-script (Recommended for development)

```bash
# Install rust-script (once)
cargo install rust-script

# Run the script
rust-script generate_gm_templates.rs
```

This method automatically handles dependencies and is ideal for script-style execution.

## What It Does

The script:
1. Reads `tones/general_midi/tone_names.json` (input)
2. Generates 128 JSON files: `000_AcousticGrand.json` through `127_Gunshot.json`
3. Each file contains a template tone with default register values
4. Files are created in `tones/general_midi/` directory

## Output Format

Each generated file follows this structure:

```json
{
  "description": "Acoustic Grand Piano",
  "variations": [
    {"description":"Edited Tone","note_number":60,"registers":"40016014801FA00AC005E0574801681E8819A808C804E866500270009014B006D003F075580178009816B807D804F86620C4283E30000878"}
  ]
}
```

The format matches the existing `000_AcousticGrand.json` template file exactly.

## Testing

The script includes unit tests that can be run with:

```bash
rustc --edition 2021 --test generate_gm_templates.rs -o test_runner && ./test_runner
```

## Notes

- The script uses the default register values from the original `000_AcousticGrand.json`
- Filenames are generated automatically from instrument descriptions
- The script will overwrite existing files, so back up any custom tones before running
