//! Unit tests for favorites module

use crate::favorites::*;
use crate::models::*;
use crate::register;

fn temp_favorites_path() -> std::path::PathBuf {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("ym2151_favorites_test_{}.json", ts))
}

#[test]
fn test_favorites_file_path_ends_with_favorites_json() {
    if let Some(path) = favorites_file_path() {
        assert!(
            path.ends_with("favorites.json"),
            "path should end with favorites.json, got: {:?}",
            path
        );
    }
    // If config_dir() returns None (some CI environments), we just skip the assertion
}

#[test]
fn test_save_to_favorites_creates_file() {
    let path = temp_favorites_path();
    let values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];

    let result = save_to_favorites_at_path(&path, &values);
    assert!(
        result.is_ok(),
        "save_to_favorites_at_path failed: {:?}",
        result.err()
    );
    assert!(path.exists(), "favorites file was not created");

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_favorites_stores_registers_string() {
    let path = temp_favorites_path();
    let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values[0][PARAM_SM] = 1;
    values[0][PARAM_MUL] = 3;
    values[ROW_CH][CH_PARAM_ALG] = 4;

    save_to_favorites_at_path(&path, &values).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let favorites: Vec<String> = serde_json::from_str(&content).unwrap();

    assert_eq!(favorites.len(), 1);
    let expected_registers = register::editor_rows_to_registers(&values);
    assert_eq!(favorites[0], expected_registers);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_favorites_newest_first() {
    let path = temp_favorites_path();

    let mut values1 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values1[0][PARAM_MUL] = 1;

    let mut values2 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values2[0][PARAM_MUL] = 2;

    save_to_favorites_at_path(&path, &values1).unwrap();
    save_to_favorites_at_path(&path, &values2).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let favorites: Vec<String> = serde_json::from_str(&content).unwrap();

    assert_eq!(favorites.len(), 2);
    // Newest (values2) should be first
    let expected2 = register::editor_rows_to_registers(&values2);
    let expected1 = register::editor_rows_to_registers(&values1);
    assert_eq!(favorites[0], expected2);
    assert_eq!(favorites[1], expected1);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_favorites_deduplicates_same_tone() {
    let path = temp_favorites_path();

    let mut values1 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values1[0][PARAM_MUL] = 1;

    let mut values2 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values2[0][PARAM_MUL] = 2;

    // Save values1, then values2, then add values1 again
    save_to_favorites_at_path(&path, &values1).unwrap();
    save_to_favorites_at_path(&path, &values2).unwrap();
    save_to_favorites_at_path(&path, &values1).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let favorites: Vec<String> = serde_json::from_str(&content).unwrap();

    // values1 should be at the front; no duplicate; total length = 2
    let expected1 = register::editor_rows_to_registers(&values1);
    let expected2 = register::editor_rows_to_registers(&values2);
    assert_eq!(favorites.len(), 2, "Duplicate tone should not appear twice");
    assert_eq!(favorites[0], expected1, "Re-added tone should be at front");
    assert_eq!(favorites[1], expected2);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_favorites_corrupted_file_returns_error() {
    let path = temp_favorites_path();

    // Write invalid JSON to simulate corruption
    std::fs::write(&path, b"not valid json").unwrap();

    let values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    let result = save_to_favorites_at_path(&path, &values);

    assert!(
        result.is_err(),
        "Corrupted favorites file should return an error"
    );
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_save_to_favorites_limits_to_20_entries() {
    let path = temp_favorites_path();

    for i in 0u8..25 {
        let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
        values[0][PARAM_MUL] = i % 16; // MUL 0-15
        values[0][PARAM_TL] = i % 100; // TL unique per iteration
        save_to_favorites_at_path(&path, &values).unwrap();
    }

    let content = std::fs::read_to_string(&path).unwrap();
    let favorites: Vec<String> = serde_json::from_str(&content).unwrap();

    assert_eq!(
        favorites.len(),
        20,
        "Favorites should be limited to 20 entries"
    );

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_favorites_format_is_compact_json() {
    let path = temp_favorites_path();

    let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values[0][PARAM_SM] = 1;
    values[ROW_CH][CH_PARAM_ALG] = 2;

    save_to_favorites_at_path(&path, &values).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();

    // The entire favorites should be on a single line (compact JSON format)
    assert_eq!(
        content.lines().count(),
        1,
        "Favorites file should be a single compact JSON line, got {} lines",
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
fn test_load_favorites_at_path_returns_empty_when_no_file() {
    let path = temp_favorites_path();
    // Ensure the file does not exist
    assert!(!path.exists());

    let favorites = load_favorites_at_path(&path).unwrap();
    assert!(
        favorites.is_empty(),
        "Should return empty Vec when file does not exist"
    );
}

#[test]
fn test_load_favorites_at_path_returns_saved_entries() {
    let path = temp_favorites_path();

    let mut values1 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values1[0][PARAM_MUL] = 3;

    let mut values2 = [[0u8; GRID_WIDTH]; GRID_HEIGHT];
    values2[0][PARAM_MUL] = 7;

    save_to_favorites_at_path(&path, &values1).unwrap();
    save_to_favorites_at_path(&path, &values2).unwrap();

    let favorites = load_favorites_at_path(&path).unwrap();

    assert_eq!(favorites.len(), 2);
    // Newest entry (values2) should be first
    let expected2 = register::editor_rows_to_registers(&values2);
    let expected1 = register::editor_rows_to_registers(&values1);
    assert_eq!(favorites[0], expected2);
    assert_eq!(favorites[1], expected1);

    std::fs::remove_file(&path).ok();
}

#[test]
fn test_load_favorites_at_path_corrupted_file_returns_error() {
    let path = temp_favorites_path();
    std::fs::write(&path, b"not valid json").unwrap();

    let result = load_favorites_at_path(&path);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);

    std::fs::remove_file(&path).ok();
}
