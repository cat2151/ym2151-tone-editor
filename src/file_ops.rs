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
    
    // Serialize to JSON
    let json_string = serde_json::to_string_pretty(&tone_file)
        .map_err(io::Error::other)?;
    
    // Ensure directory exists
    if let Some(parent) = std::path::Path::new(filename).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write to file
    fs::write(filename, json_string)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_to_json_creates_valid_file() {
        // Clean up any leftover test files first
        let _ = std::fs::remove_file("ym2151_tone.json");
        
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Initialize with test values
        values[0][PARAM_MUL] = 1;
        values[0][PARAM_TL] = 20;
        values[0][PARAM_SM] = 1;
        values[1][PARAM_SM] = 1;
        values[2][PARAM_SM] = 1;
        values[3][PARAM_SM] = 1;
        values[ROW_CH][CH_PARAM_ALG] = 4;
        values[ROW_CH][CH_PARAM_FB] = 0;
        
        // Save to JSON
        let result = save_to_json(&values);
        assert!(result.is_ok());
        
        // Check that the fixed filename was created
        let filename = "ym2151_tone.json";
        assert!(std::fs::metadata(filename).is_ok(), "JSON file was not created");
        
        // Read and parse the JSON
        let content = std::fs::read_to_string(filename).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        // Verify structure
        assert!(parsed.get("event_count").is_some());
        assert!(parsed.get("events").is_some());
        assert!(parsed["events"].is_array());
        assert_eq!(parsed["event_count"].as_u64().unwrap(), 28);
        
        // Clean up
        std::fs::remove_file(filename).ok();
    }

    #[test]
    fn test_load_from_json() {
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        values[0][PARAM_MUL] = 5;
        values[0][PARAM_TL] = 30;
        values[ROW_CH][CH_PARAM_ALG] = 3;
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_filename = format!("ym2151_tone_test_{}.json", timestamp);
        
        // Save current tone data
        let json_string = register::to_json_string(&values).unwrap();
        std::fs::write(&test_filename, json_string).unwrap();
        
        // Load it back
        let result = load_from_json(&test_filename);
        assert!(result.is_ok());
        
        let loaded_values = result.unwrap();
        
        // Verify loaded values match original (at least some key values)
        assert_eq!(loaded_values[0][PARAM_MUL], values[0][PARAM_MUL]);
        assert_eq!(loaded_values[0][PARAM_TL], values[0][PARAM_TL]);
        
        // Clean up
        std::fs::remove_file(&test_filename).ok();
    }

    #[test]
    fn test_find_newest_json_file() {
        // Clean up any test files first
        let _ = std::fs::remove_file("ym2151_tone.json");
        
        // Test 1: If fixed filename exists, it should be returned
        std::fs::write("ym2151_tone.json", "{}").unwrap();
        let result = find_newest_json_file();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ym2151_tone.json");
        std::fs::remove_file("ym2151_tone.json").ok();
        
        // Test 2: If fixed filename doesn't exist, fall back to timestamped files
        let base_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let file1 = format!("ym2151_tone_{}.json", base_time);
        let file2 = format!("ym2151_tone_{}.json", base_time + 1);
        let file3 = format!("ym2151_tone_{}.json", base_time + 2);
        
        std::fs::write(&file1, "{}").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        std::fs::write(&file2, "{}").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        std::fs::write(&file3, "{}").unwrap();
        
        // Find newest file (should be file3)
        let result = find_newest_json_file();
        assert!(result.is_ok());
        
        let newest = result.unwrap();
        assert_eq!(newest, file3);
        
        // Clean up
        std::fs::remove_file(&file1).ok();
        std::fs::remove_file(&file2).ok();
        std::fs::remove_file(&file3).ok();
    }

    #[test]
    fn test_save_and_load_gm_file() {
        use std::path::Path;
        
        let test_filename = "test_gm_tone.json";
        
        // Create test tone data
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        values[0][PARAM_MUL] = 5;
        values[0][PARAM_TL] = 30;
        values[ROW_CH][CH_PARAM_ALG] = 3;
        values[ROW_CH][CH_PARAM_FB] = 2;
        values[ROW_CH][CH_PARAM_NOTE] = 60;
        
        // Save to GM file format
        let result = save_to_gm_file(test_filename, &values, "Test Piano");
        assert!(result.is_ok(), "Failed to save GM file: {:?}", result.err());
        
        // Verify file exists
        assert!(Path::new(test_filename).exists(), "GM file was not created");
        
        // Load from GM file format
        let loaded_result = load_from_gm_file(test_filename);
        assert!(loaded_result.is_ok(), "Failed to load GM file: {:?}", loaded_result.err());
        
        let loaded_values = loaded_result.unwrap();
        
        // Verify key values match
        assert_eq!(loaded_values[0][PARAM_MUL], values[0][PARAM_MUL], "MUL should match");
        assert_eq!(loaded_values[0][PARAM_TL], values[0][PARAM_TL], "TL should match");
        assert_eq!(loaded_values[ROW_CH][CH_PARAM_ALG], values[ROW_CH][CH_PARAM_ALG], "ALG should match");
        assert_eq!(loaded_values[ROW_CH][CH_PARAM_FB], values[ROW_CH][CH_PARAM_FB], "FB should match");
        
        // Clean up
        std::fs::remove_file(test_filename).ok();
    }

    #[test]
    fn test_load_gm_file_format() {
        let test_filename = "test_gm_format.json";
        
        // Create test tone data to generate a valid registers string
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        values[0][PARAM_MUL] = 1;
        values[0][PARAM_SM] = 1;
        values[1][PARAM_SM] = 1;
        values[2][PARAM_SM] = 1;
        values[3][PARAM_SM] = 1;
        
        // Generate a valid registers string
        let registers = crate::register::tone_data_to_registers(&values);
        
        // Create a GM file manually with the valid registers string
        let json_content = format!(r#"{{
  "description": "GM:000 Acoustic Grand Piano family",
  "variations": [
    {{
      "description": "Test Tone",
      "note_number": 60,
      "registers": "{}"
    }}
  ]
}}"#, registers);
        
        std::fs::write(test_filename, json_content).unwrap();
        
        // Load the file
        let result = load_from_gm_file(test_filename);
        assert!(result.is_ok(), "Failed to load GM file: {:?}", result.err());
        
        // Clean up
        std::fs::remove_file(test_filename).ok();
    }

    #[test]
    fn test_load_gm_file_empty_variations() {
        let test_filename = "test_empty_variations.json";
        
        // Create a GM file with no variations
        let json_content = r#"{
  "description": "Empty file",
  "variations": []
}"#;
        
        std::fs::write(test_filename, json_content).unwrap();
        
        // Try to load - should fail
        let result = load_from_gm_file(test_filename);
        assert!(result.is_err(), "Should fail when no variations present");
        
        // Clean up
        std::fs::remove_file(test_filename).ok();
    }
}
