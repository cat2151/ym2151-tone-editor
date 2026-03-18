use crate::models::*;
use crate::register;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Returns the application data directory: `{config_local_dir}/ym2151-tone-editor`
pub fn app_data_dir() -> Option<PathBuf> {
    dirs::config_local_dir().map(|dir| dir.join("ym2151-tone-editor"))
}

/// Returns the path for the main tone JSON file in the app data directory.
pub fn tone_file_path() -> Option<PathBuf> {
    app_data_dir().map(|dir| dir.join("ym2151_tone.json"))
}

/// Returns the path for the General MIDI tone file in the app data directory.
pub fn gm_file_path() -> Option<PathBuf> {
    app_data_dir().map(|dir| {
        dir.join("tones")
            .join("general_midi")
            .join("000_AcousticGrand.json")
    })
}

/// Find the newest JSON file in the app data directory matching the pattern ym2151_tone*.json.
/// Prioritizes the fixed filename "ym2151_tone.json" if it exists, otherwise falls back to
/// timestamped files (ym2151_tone_*.json) for backwards compatibility.
pub fn find_newest_json_file() -> io::Result<PathBuf> {
    let dir = app_data_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find app data directory")
    })?;
    find_newest_json_file_in_dir(&dir)
}

/// Find the newest JSON file in the given directory matching the pattern ym2151_tone*.json.
/// Used internally and in tests to allow specifying an explicit directory.
pub fn find_newest_json_file_in_dir(dir: &Path) -> io::Result<PathBuf> {
    // First, check if the fixed filename exists as a file
    let fixed_path = dir.join("ym2151_tone.json");
    if fs::metadata(&fixed_path)
        .map(|m| m.is_file())
        .unwrap_or(false)
    {
        return Ok(fixed_path);
    }

    // Fall back to finding timestamped files
    let entries = fs::read_dir(dir)?;

    let mut json_files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let is_file = e.metadata().map(|m| m.is_file()).unwrap_or(false);
            let name_matches = e
                .path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.starts_with("ym2151_tone_") && s.ends_with(".json"))
                .unwrap_or(false);
            is_file && name_matches
        })
        .collect();

    if json_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No JSON files found",
        ));
    }

    // Sort by modification time (newest first)
    json_files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
    json_files.reverse();

    // SAFETY: json_files is non-empty at this point
    Ok(json_files[0].path())
}

/// Load tone data from a JSON file
pub fn load_from_json(path: impl AsRef<Path>) -> io::Result<ToneData> {
    let json_string = fs::read_to_string(path)?;
    let log: Ym2151Log = serde_json::from_str(&json_string).map_err(io::Error::other)?;

    register::json_events_to_editor_rows(&log.events)
}

/// Load the newest JSON file and convert to tone data
pub fn load_newest_json() -> io::Result<ToneData> {
    let path = find_newest_json_file()?;
    load_from_json(path)
}

/// Save tone data to JSON file in the app data directory.
pub fn save_to_json(values: &ToneData) -> io::Result<()> {
    let path = tone_file_path().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find app data directory")
    })?;
    save_to_json_at_path(&path, values)
}

/// Save tone data to JSON file at the given path (creates parent directories if needed).
/// Used internally and in tests to allow specifying an explicit path.
pub fn save_to_json_at_path(path: &Path, values: &ToneData) -> io::Result<()> {
    let json_string = register::to_json_string(values).map_err(io::Error::other)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, json_string)?;
    Ok(())
}

/// Load tone data from General MIDI tone file format.
/// Returns the first variation's tone data.
pub fn load_from_gm_file(path: impl AsRef<Path>) -> io::Result<ToneData> {
    let json_string = fs::read_to_string(path)?;
    let tone_file: crate::models::ToneFile =
        serde_json::from_str(&json_string).map_err(io::Error::other)?;

    // Load the first variation
    if tone_file.variations.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No variations found in tone file",
        ));
    }

    let variation = &tone_file.variations[0];
    register::registers_to_editor_rows(&variation.registers)
}

/// Save tone data to General MIDI tone file format.
/// Creates a single variation with the current tone data.
pub fn save_to_gm_file(
    path: impl AsRef<Path>,
    values: &ToneData,
    description: &str,
) -> io::Result<()> {
    let path = path.as_ref();

    // Convert tone data to registers hex string
    let registers = register::editor_rows_to_registers(values);

    // Get the current MIDI note from the tone data
    let note_number = values[crate::models::ROW_CH][crate::models::CH_PARAM_NOTE];

    // Create a single variation
    let variation = crate::models::ToneVariation {
        description: description.to_string(),
        mml: None,
        note_number: Some(note_number),
        registers,
    };

    // Create the tone file structure
    let tone_file = crate::models::ToneFile {
        description: "Acoustic Grand Piano".to_string(),
        variations: vec![variation],
    };

    // Serialize to JSON with minified variations
    let json_string =
        serialize_tone_file_with_minified_variations(&tone_file).map_err(io::Error::other)?;

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write to file
    fs::write(path, json_string)?;
    Ok(())
}

/// Append tone data as a new variation to General MIDI tone file.
/// Reads existing file, adds new variation to the end, and writes back.
pub fn append_to_gm_file(
    path: impl AsRef<Path>,
    values: &ToneData,
    description: &str,
) -> io::Result<()> {
    let path = path.as_ref();

    // Convert tone data to registers hex string
    let registers = register::editor_rows_to_registers(values);

    // Get the current MIDI note from the tone data
    let note_number = values[crate::models::ROW_CH][crate::models::CH_PARAM_NOTE];

    // Create the new variation
    let new_variation = crate::models::ToneVariation {
        description: description.to_string(),
        mml: None,
        note_number: Some(note_number),
        registers,
    };

    // Try to load existing file, or create new if it doesn't exist
    let mut tone_file = match fs::read_to_string(path) {
        Ok(json_string) => serde_json::from_str::<crate::models::ToneFile>(&json_string)
            .map_err(io::Error::other)?,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // File doesn't exist, create new structure
            crate::models::ToneFile {
                description: "Acoustic Grand Piano".to_string(),
                variations: vec![],
            }
        }
        Err(e) => return Err(e),
    };

    // Append the new variation
    tone_file.variations.push(new_variation);

    // Serialize to JSON with minified variations
    let json_string =
        serialize_tone_file_with_minified_variations(&tone_file).map_err(io::Error::other)?;

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write to file
    fs::write(path, json_string)?;
    Ok(())
}

/// Serialize ToneFile with minified variations (one line per variation)
/// The outer structure is pretty-printed, but each variation is on a single line
fn serialize_tone_file_with_minified_variations(
    tone_file: &crate::models::ToneFile,
) -> Result<String, serde_json::Error> {
    // Serialize each variation as a minified string
    let variation_strings: Result<Vec<String>, _> = tone_file
        .variations
        .iter()
        .map(serde_json::to_string)
        .collect();

    let variation_strings = variation_strings?;

    // Build the JSON manually with proper formatting
    let mut result = String::from("{\n");

    // Add description field
    result.push_str("  \"description\": ");
    result.push_str(&serde_json::to_string(&tone_file.description)?);
    result.push_str(",\n");

    // Add variations array with each element on a single line
    result.push_str("  \"variations\": [\n");
    for (i, var_str) in variation_strings.iter().enumerate() {
        result.push_str("    ");
        result.push_str(var_str);
        if i < variation_strings.len() - 1 {
            result.push(',');
        }
        result.push('\n');
    }
    result.push_str("  ]\n");
    result.push('}');

    Ok(result)
}
