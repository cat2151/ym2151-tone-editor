//! Unit tests for file_ops module

use crate::file_ops::*;
use crate::models::*;
use crate::register;

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
    assert!(
        std::fs::metadata(filename).is_ok(),
        "JSON file was not created"
    );

    // Read and parse the JSON
    let content = std::fs::read_to_string(filename).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Verify structure
    assert!(parsed.get("events").is_some());
    assert!(parsed["events"].is_array());

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
    assert!(
        loaded_result.is_ok(),
        "Failed to load GM file: {:?}",
        loaded_result.err()
    );

    let loaded_values = loaded_result.unwrap();

    // Verify key values match
    assert_eq!(
        loaded_values[0][PARAM_MUL], values[0][PARAM_MUL],
        "MUL should match"
    );
    assert_eq!(
        loaded_values[0][PARAM_TL], values[0][PARAM_TL],
        "TL should match"
    );
    assert_eq!(
        loaded_values[ROW_CH][CH_PARAM_ALG], values[ROW_CH][CH_PARAM_ALG],
        "ALG should match"
    );
    assert_eq!(
        loaded_values[ROW_CH][CH_PARAM_FB], values[ROW_CH][CH_PARAM_FB],
        "FB should match"
    );

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
    let registers = crate::register::editor_rows_to_registers(&values);

    // Create a GM file manually with the valid registers string
    let json_content = format!(
        r#"{{
  "description": "GM:000 Acoustic Grand Piano family",
  "variations": [
    {{
      "description": "Test Tone",
      "note_number": 60,
      "registers": "{}"
    }}
  ]
}}"#,
        registers
    );

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

#[test]
fn test_gm_file_minified_variations_format() {
    let test_filename = "test_minified_format.json";

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

    // Read the file content
    let content = std::fs::read_to_string(test_filename).unwrap();

    // Verify the format
    // 1. Should contain "description" on a separate line
    assert!(
        content.contains("\"description\":"),
        "Should contain description field"
    );

    // 2. Should contain "variations" array
    assert!(
        content.contains("\"variations\":"),
        "Should contain variations array"
    );

    // 3. Each variation should be on a single line (not spread across multiple lines)
    // Count the lines in the variations array
    let lines: Vec<&str> = content.lines().collect();

    // Find the variations array
    let mut in_variations = false;
    let mut variation_lines = 0;
    for line in &lines {
        if line.contains("\"variations\":") {
            in_variations = true;
            continue;
        }
        if in_variations {
            if line.trim().starts_with("{") {
                // This line should contain a complete variation object
                variation_lines += 1;
                // Verify it's a complete object on one line
                assert!(
                    line.contains("\"description\""),
                    "Variation line should contain description"
                );
                assert!(
                    line.contains("\"registers\""),
                    "Variation line should contain registers"
                );
                assert!(
                    line.contains("}"),
                    "Variation line should end with closing brace"
                );
            }
            if line.trim().starts_with("]") {
                break;
            }
        }
    }

    assert!(
        variation_lines > 0,
        "Should have at least one variation on a single line"
    );

    // Clean up
    std::fs::remove_file(test_filename).ok();
}
