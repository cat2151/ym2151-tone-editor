//! Unit tests for file_ops module

use crate::file_ops::*;
use crate::models::*;
use crate::register;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir() -> PathBuf {
    let id = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("ym2151_file_ops_test_{}_{id}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_save_to_json_at_path_creates_valid_file() {
    let dir = temp_dir();
    let path = dir.join("ym2151_tone.json");

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
    let result = save_to_json_at_path(&path, &values);
    assert!(result.is_ok());

    // Check that the file was created
    assert!(path.exists(), "JSON file was not created");

    // Read and parse the JSON
    let content = std::fs::read_to_string(&path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Verify structure
    assert!(parsed.get("events").is_some());
    assert!(parsed["events"].is_array());

    // Clean up
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_load_from_json() {
    let dir = temp_dir();

    let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
    values[0][PARAM_MUL] = 5;
    values[0][PARAM_TL] = 30;
    values[ROW_CH][CH_PARAM_ALG] = 3;

    let test_path = dir.join("ym2151_tone_test.json");

    // Save current tone data
    let json_string = register::to_json_string(&values).unwrap();
    std::fs::write(&test_path, json_string).unwrap();

    // Load it back
    let result = load_from_json(&test_path);
    assert!(result.is_ok());

    let loaded_values = result.unwrap();

    // Verify loaded values match original (at least some key values)
    assert_eq!(loaded_values[0][PARAM_MUL], values[0][PARAM_MUL]);
    assert_eq!(loaded_values[0][PARAM_TL], values[0][PARAM_TL]);

    // Clean up
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_find_newest_json_file_in_dir() {
    let dir = temp_dir();

    // Test 1: If fixed filename exists, it should be returned
    let fixed_path = dir.join("ym2151_tone.json");
    std::fs::write(&fixed_path, "{}").unwrap();
    let result = find_newest_json_file_in_dir(&dir);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), fixed_path);
    std::fs::remove_file(&fixed_path).ok();

    // Test 2: If fixed filename doesn't exist, fall back to timestamped files
    let base_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let file1 = dir.join(format!("ym2151_tone_{}.json", base_time));
    let file2 = dir.join(format!("ym2151_tone_{}.json", base_time + 1));
    let file3 = dir.join(format!("ym2151_tone_{}.json", base_time + 2));

    std::fs::write(&file1, "{}").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    std::fs::write(&file2, "{}").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    std::fs::write(&file3, "{}").unwrap();

    // Find newest file (should be file3)
    let result = find_newest_json_file_in_dir(&dir);
    assert!(result.is_ok());

    let newest = result.unwrap();
    assert_eq!(newest, file3);

    // Clean up
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_app_data_dir_ends_with_ym2151_tone_editor() {
    if let Some(dir) = app_data_dir() {
        assert!(
            dir.ends_with("ym2151-tone-editor"),
            "app_data_dir should end with ym2151-tone-editor, got: {:?}",
            dir
        );
    }
    // If config_local_dir() returns None (some CI environments), skip the assertion
}

#[test]
fn test_tone_file_path_ends_with_ym2151_tone_json() {
    if let Some(path) = tone_file_path() {
        assert!(
            path.ends_with("ym2151_tone.json"),
            "tone_file_path should end with ym2151_tone.json, got: {:?}",
            path
        );
    }
}

#[test]
fn test_gm_file_path_ends_with_expected_path() {
    if let Some(path) = gm_file_path() {
        let path_str = path.to_string_lossy();
        assert!(
            path_str.contains("000_AcousticGrand.json"),
            "gm_file_path should contain 000_AcousticGrand.json, got: {:?}",
            path
        );
        assert!(
            path_str.contains("general_midi"),
            "gm_file_path should contain general_midi, got: {:?}",
            path
        );
    }
}

#[test]
fn test_save_and_load_gm_file() {
    let dir = temp_dir();
    let test_path = dir.join("test_gm_tone.json");

    // Create test tone data
    let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
    values[0][PARAM_MUL] = 5;
    values[0][PARAM_TL] = 30;
    values[ROW_CH][CH_PARAM_ALG] = 3;
    values[ROW_CH][CH_PARAM_FB] = 2;
    values[ROW_CH][CH_PARAM_NOTE] = 60;

    // Save to GM file format
    let result = save_to_gm_file(&test_path, &values, "Test Piano");
    assert!(result.is_ok(), "Failed to save GM file: {:?}", result.err());

    // Verify file exists
    assert!(test_path.exists(), "GM file was not created");

    // Load from GM file format
    let loaded_result = load_from_gm_file(&test_path);
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
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_load_gm_file_format() {
    let dir = temp_dir();
    let test_path = dir.join("test_gm_format.json");

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

    std::fs::write(&test_path, json_content).unwrap();

    // Load the file
    let result = load_from_gm_file(&test_path);
    assert!(result.is_ok(), "Failed to load GM file: {:?}", result.err());

    // Clean up
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_load_gm_file_empty_variations() {
    let dir = temp_dir();
    let test_path = dir.join("test_empty_variations.json");

    // Create a GM file with no variations
    let json_content = r#"{
  "description": "Empty file",
  "variations": []
}"#;

    std::fs::write(&test_path, json_content).unwrap();

    // Try to load - should fail
    let result = load_from_gm_file(&test_path);
    assert!(result.is_err(), "Should fail when no variations present");

    // Clean up
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_gm_file_minified_variations_format() {
    let dir = temp_dir();
    let test_path = dir.join("test_minified_format.json");

    // Create test tone data
    let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
    values[0][PARAM_MUL] = 5;
    values[0][PARAM_TL] = 30;
    values[ROW_CH][CH_PARAM_ALG] = 3;
    values[ROW_CH][CH_PARAM_FB] = 2;
    values[ROW_CH][CH_PARAM_NOTE] = 60;

    // Save to GM file format
    let result = save_to_gm_file(&test_path, &values, "Test Piano");
    assert!(result.is_ok(), "Failed to save GM file: {:?}", result.err());

    // Read the file content
    let content = std::fs::read_to_string(&test_path).unwrap();

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
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn test_append_to_gm_file() {
    let dir = temp_dir();
    let test_path = dir.join("test_append_gm.json");

    // Create first tone data
    let mut values1 = [[0; GRID_WIDTH]; GRID_HEIGHT];
    values1[0][PARAM_MUL] = 5;
    values1[0][PARAM_TL] = 30;
    values1[ROW_CH][CH_PARAM_ALG] = 3;
    values1[ROW_CH][CH_PARAM_FB] = 2;
    values1[ROW_CH][CH_PARAM_NOTE] = 60;

    // Append first variation (file doesn't exist yet)
    let result = append_to_gm_file(&test_path, &values1, "First Variation");
    assert!(
        result.is_ok(),
        "Failed to append first variation: {:?}",
        result.err()
    );

    // Verify file exists
    assert!(test_path.exists(), "GM file was not created");

    // Read and parse the file
    let content = std::fs::read_to_string(&test_path).unwrap();
    let tone_file: ToneFile = serde_json::from_str(&content).unwrap();
    assert_eq!(tone_file.variations.len(), 1, "Should have 1 variation");
    assert_eq!(tone_file.variations[0].description, "First Variation");

    // Create second tone data with different values
    let mut values2 = [[0; GRID_WIDTH]; GRID_HEIGHT];
    values2[0][PARAM_MUL] = 10;
    values2[0][PARAM_TL] = 50;
    values2[ROW_CH][CH_PARAM_ALG] = 7;
    values2[ROW_CH][CH_PARAM_FB] = 5;
    values2[ROW_CH][CH_PARAM_NOTE] = 72;

    // Append second variation
    let result = append_to_gm_file(&test_path, &values2, "Second Variation");
    assert!(
        result.is_ok(),
        "Failed to append second variation: {:?}",
        result.err()
    );

    // Read and verify we now have 2 variations
    let content = std::fs::read_to_string(&test_path).unwrap();
    let tone_file: ToneFile = serde_json::from_str(&content).unwrap();
    assert_eq!(tone_file.variations.len(), 2, "Should have 2 variations");
    assert_eq!(tone_file.variations[0].description, "First Variation");
    assert_eq!(tone_file.variations[1].description, "Second Variation");

    // Verify the variations have different note numbers
    assert_eq!(tone_file.variations[0].note_number, Some(60));
    assert_eq!(tone_file.variations[1].note_number, Some(72));

    // Load first variation and verify values
    let loaded1 = load_from_gm_file(&test_path).unwrap();
    assert_eq!(loaded1[0][PARAM_MUL], values1[0][PARAM_MUL]);
    assert_eq!(loaded1[0][PARAM_TL], values1[0][PARAM_TL]);

    // Clean up
    std::fs::remove_dir_all(&dir).ok();
}
