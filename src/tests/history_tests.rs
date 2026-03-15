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
    // If config_dir() returns None (some CI environments), we just skip the assertion
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
fn test_save_to_history_skips_duplicate_latest() {
    let path = temp_history_path();
    let values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];

    save_to_history_at_path(&path, &values).unwrap();
    save_to_history_at_path(&path, &values).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let history: Vec<String> = serde_json::from_str(&content).unwrap();

    assert_eq!(history.len(), 1, "Duplicate entry should not be added");

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_history_limits_to_20_entries() {
    let path = temp_history_path();

    for i in 0u8..25 {
        let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
        values[0][PARAM_MUL] = i % 16; // MUL 0-15
        values[0][PARAM_TL] = i; // TL 0-24 (unique per iteration)
        save_to_history_at_path(&path, &values).unwrap();
    }

    let content = std::fs::read_to_string(&path).unwrap();
    let history: Vec<String> = serde_json::from_str(&content).unwrap();

    assert_eq!(history.len(), 20, "History should be limited to 20 entries");

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
}
