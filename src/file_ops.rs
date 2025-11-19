use std::{fs, io};
use crate::models::*;
use crate::register;

/// Find the newest JSON file in the current directory matching the pattern ym2151_tone*.json
/// Prioritizes the fixed filename "ym2151_tone.json" if it exists, otherwise falls back to
/// timestamped files (ym2151_tone_*.json) for backwards compatibility
pub fn find_newest_json_file() -> io::Result<String> {
    // First, check if the fixed filename exists
    let fixed_filename = "ym2151_tone.json";
    if fs::metadata(fixed_filename).is_ok() {
        return Ok(fixed_filename.to_string());
    }
    
    // Fall back to finding timestamped files
    let entries = fs::read_dir(".")?;
    
    let mut json_files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.starts_with("ym2151_tone_") && s.ends_with(".json"))
                .unwrap_or(false)
        })
        .collect();

    if json_files.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No JSON files found"));
    }

    // Sort by modification time (newest first)
    json_files.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.modified())
            .ok()
    });
    json_files.reverse();

    json_files
        .first()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not get filename"))
}

/// Load tone data from a JSON file
pub fn load_from_json(filename: &str) -> io::Result<ToneData> {
    let json_string = fs::read_to_string(filename)?;
    let log: Ym2151Log = serde_json::from_str(&json_string)
        .map_err(io::Error::other)?;

    register::events_to_tone_data(&log.events)
}

/// Load the newest JSON file and convert to tone data
pub fn load_newest_json() -> io::Result<ToneData> {
    let filename = find_newest_json_file()?;
    load_from_json(&filename)
}

/// Save tone data to JSON file in ym2151-log-play-server format
pub fn save_to_json(values: &ToneData) -> io::Result<()> {
    let json_string = register::to_json_string(values)
        .map_err(io::Error::other)?;

    // Use fixed filename without timestamp
    let filename = "ym2151_tone.json";

    fs::write(&filename, json_string)?;
    Ok(())
}

/// Load tone data from General MIDI tone file format
/// Reads from tones/general_midi/000_AcousticGrand.json
/// Returns the first variation's tone data
pub fn load_from_gm_file(filename: &str) -> io::Result<ToneData> {
    let json_string = fs::read_to_string(filename)?;
    let tone_file: crate::models::ToneFile = serde_json::from_str(&json_string)
        .map_err(io::Error::other)?;
    
    // Load the first variation
    if tone_file.variations.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No variations found in tone file"
        ));
    }
    
    let variation = &tone_file.variations[0];
    register::registers_to_tone_data(&variation.registers)
}

/// Save tone data to General MIDI tone file format
/// Writes to tones/general_midi/000_AcousticGrand.json
/// Creates a single variation with the current tone data
pub fn save_to_gm_file(filename: &str, values: &ToneData, description: &str) -> io::Result<()> {
    // Convert tone data to registers hex string
    let registers = register::tone_data_to_registers(values);
    
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
    let json_string = serialize_tone_file_with_minified_variations(&tone_file)
        .map_err(io::Error::other)?;
    
    // Ensure directory exists
    if let Some(parent) = std::path::Path::new(filename).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write to file
    fs::write(filename, json_string)?;
    Ok(())
}

/// Serialize ToneFile with minified variations (one line per variation)
/// The outer structure is pretty-printed, but each variation is on a single line
fn serialize_tone_file_with_minified_variations(tone_file: &crate::models::ToneFile) -> Result<String, serde_json::Error> {
    // Serialize each variation as a minified string
    let variation_strings: Result<Vec<String>, _> = tone_file.variations
        .iter()
        .map(|v| serde_json::to_string(v))
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
