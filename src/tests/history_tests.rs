//! Unit tests for history module

use crate::history::*;
use crate::models::*;
use crate::register;

fn temp_history_path() -> std::path::PathBuf {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("ym2151_history_test_{}.json", ts))
}

#[test]
fn test_history_file_path_ends_with_history_tone_json() {
    if let Some(path) = history_file_path() {
        assert!(
            path.ends_with("history_tone.json"),
            "path should end with history_tone.json, got: {:?}",
            path
        );
    }
    // If config_local_dir() returns None (some CI environments), we just skip the assertion
}

#[test]
fn test_save_to_history_creates_file() {
    let path = temp_history_path();
    let values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];

    let result = save_to_history_at_path(&path, &values);
    assert!(
        result.is_ok(),
        "save_to_history_at_path failed: {:?}",
        result.err()
    );
    assert!(path.exists(), "history file was not created");

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_history_stores_registers_string() {
    let path = temp_history_path();
    let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values[0][PARAM_SM] = 1;
    values[0][PARAM_MUL] = 3;
    values[ROW_CH][CH_PARAM_ALG] = 4;

    save_to_history_at_path(&path, &values).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let history: Vec<String> = serde_json::from_str(&content).unwrap();

    assert_eq!(history.len(), 1);
    let expected_registers = register::editor_rows_to_registers(&values);
    assert_eq!(history[0], expected_registers);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_history_newest_first() {
    let path = temp_history_path();

    let mut values1 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values1[0][PARAM_MUL] = 1;

    let mut values2 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values2[0][PARAM_MUL] = 2;

    save_to_history_at_path(&path, &values1).unwrap();
    save_to_history_at_path(&path, &values2).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let history: Vec<String> = serde_json::from_str(&content).unwrap();

    assert_eq!(history.len(), 2);
    // Newest (values2) should be first
    let expected2 = register::editor_rows_to_registers(&values2);
    let expected1 = register::editor_rows_to_registers(&values1);
    assert_eq!(history[0], expected2);
    assert_eq!(history[1], expected1);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_history_deduplicates_replayed_tone() {
    let path = temp_history_path();

    let mut values1 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values1[0][PARAM_MUL] = 1;

    let mut values2 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values2[0][PARAM_MUL] = 2;

    // Save values1, then values2, then replay values1
    save_to_history_at_path(&path, &values1).unwrap();
    save_to_history_at_path(&path, &values2).unwrap();
    save_to_history_at_path(&path, &values1).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let history: Vec<String> = serde_json::from_str(&content).unwrap();

    // values1 should be at the front; no duplicate; total length = 2
    let expected1 = register::editor_rows_to_registers(&values1);
    let expected2 = register::editor_rows_to_registers(&values2);
    assert_eq!(history.len(), 2, "Replayed tone should not duplicate");
    assert_eq!(history[0], expected1, "Replayed tone should be at front");
    assert_eq!(history[1], expected2);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_history_corrupted_file_returns_error() {
    let path = temp_history_path();

    // Write invalid JSON to simulate corruption
    std::fs::write(&path, b"not valid json").unwrap();

    let values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    let result = save_to_history_at_path(&path, &values);

    assert!(
        result.is_err(),
        "Corrupted history file should return an error"
    );
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_history_limits_to_26_entries() {
    let path = temp_history_path();

    for i in 0u8..30 {
        let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
        values[0][PARAM_MUL] = i % 16; // MUL 0-15
        values[0][PARAM_TL] = i % 100; // TL unique per iteration
        save_to_history_at_path(&path, &values).unwrap();
    }

    let content = std::fs::read_to_string(&path).unwrap();
    let history: Vec<String> = serde_json::from_str(&content).unwrap();

    assert_eq!(history.len(), 26, "History should be limited to 26 entries");

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_history_format_is_compact_registers_per_line() {
    let path = temp_history_path();

    let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values[0][PARAM_SM] = 1;
    values[ROW_CH][CH_PARAM_ALG] = 2;

    save_to_history_at_path(&path, &values).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();

    // The entire history should be on a single line (compact JSON format)
    assert_eq!(
        content.lines().count(),
        1,
        "History file should be a single compact JSON line, got {} lines",
        content.lines().count()
    );

    // The compact registers string should appear directly in the content
    let registers = register::editor_rows_to_registers(&values);
    assert!(
        content.contains(&registers),
        "registers string '{}' should appear in the compact file content",
        registers
    );

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_load_history_at_path_returns_empty_when_no_file() {
    let path = temp_history_path();
    // Ensure the file does not exist
    assert!(!path.exists());

    let history = load_history_at_path(&path).unwrap();
    assert!(
        history.is_empty(),
        "Should return empty Vec when file does not exist"
    );
}

#[test]
fn test_load_history_at_path_returns_saved_entries() {
    let path = temp_history_path();

    let mut values1 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values1[0][PARAM_MUL] = 3;

    let mut values2 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values2[0][PARAM_MUL] = 7;

    save_to_history_at_path(&path, &values1).unwrap();
    save_to_history_at_path(&path, &values2).unwrap();

    let history = load_history_at_path(&path).unwrap();

    assert_eq!(history.len(), 2);
    // Newest entry (values2) should be first
    let expected2 = register::editor_rows_to_registers(&values2);
    let expected1 = register::editor_rows_to_registers(&values1);
    assert_eq!(history[0], expected2);
    assert_eq!(history[1], expected1);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_migrate_history_from_roaming_copies_when_new_missing() {
    let legacy_path = temp_history_path();
    let new_path = temp_history_path();

    let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values[0][PARAM_MUL] = 5;
    save_to_history_at_path(&legacy_path, &values).unwrap();

    // new_path does not exist yet; simulate migration
    migrate_history_from_roaming_at_paths(&legacy_path, &new_path);

    assert!(new_path.exists(), "migration should copy file to new path");
    let history = load_history_at_path(&new_path).unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0], register::editor_rows_to_registers(&values));

    std::fs::remove_file(&legacy_path).ok();
    std::fs::remove_file(&new_path).ok();
}

#[test]
fn test_migrate_history_from_roaming_skips_when_new_exists() {
    let legacy_path = temp_history_path();
    let new_path = temp_history_path();

    let mut legacy_values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    legacy_values[0][PARAM_MUL] = 3;
    save_to_history_at_path(&legacy_path, &legacy_values).unwrap();

    let mut new_values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    new_values[0][PARAM_MUL] = 7;
    save_to_history_at_path(&new_path, &new_values).unwrap();

    // new_path already exists; migration should be a no-op
    migrate_history_from_roaming_at_paths(&legacy_path, &new_path);

    let history = load_history_at_path(&new_path).unwrap();
    assert_eq!(
        history.len(),
        1,
        "existing new file should not be overwritten"
    );
    assert_eq!(history[0], register::editor_rows_to_registers(&new_values));

    std::fs::remove_file(&legacy_path).ok();
    std::fs::remove_file(&new_path).ok();
}

#[test]
fn test_migrate_history_from_roaming_noop_when_legacy_missing() {
    let legacy_path = temp_history_path();
    let new_path = temp_history_path();

    // Neither file exists; migration should be a no-op and not create new_path
    migrate_history_from_roaming_at_paths(&legacy_path, &new_path);

    assert!(
        !new_path.exists(),
        "new path should not be created when legacy is also missing"
    );
}

#[test]
fn test_load_history_at_path_corrupted_file_returns_error() {
    let path = temp_history_path();
    std::fs::write(&path, b"not valid json").unwrap();

    let result = load_history_at_path(&path);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);

    std::fs::remove_file(&path).ok();
}
