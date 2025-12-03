use std::fs;

/// Test that GM file can be loaded and parsed
#[test]
fn test_gm_file_exists_and_valid() {
    let filename = "tones/general_midi/000_AcousticGrand.json";
    let json_string = fs::read_to_string(filename).expect("GM file should exist");
    let tone_file: crate::models::ToneFile =
        serde_json::from_str(&json_string).expect("GM file should be valid JSON");

    assert!(!tone_file.variations.is_empty(), "GM file should have at least one variation");
    assert_eq!(tone_file.description, "Acoustic Grand Piano");
}

/// Test that variations can be converted to ToneData
#[test]
fn test_variation_to_tone_data_conversion() {
    let filename = "tones/general_midi/000_AcousticGrand.json";
    let json_string = fs::read_to_string(filename).expect("GM file should exist");
    let tone_file: crate::models::ToneFile =
        serde_json::from_str(&json_string).expect("GM file should be valid JSON");

    assert!(!tone_file.variations.is_empty());

    // Test converting first variation to ToneData
    let variation = &tone_file.variations[0];
    let tone_data = crate::register::registers_to_editor_rows(&variation.registers)
        .expect("Should convert registers to tone data");

    // Verify it's a valid ToneData structure
    assert_eq!(tone_data.len(), crate::models::GRID_HEIGHT);
    for row in tone_data.iter() {
        assert_eq!(row.len(), crate::models::GRID_WIDTH);
    }
}

/// Test that OpenVariationSelector action is properly defined
#[test]
fn test_open_variation_selector_action_exists() {
    use crate::config::{Action, Config};

    let config = Config::default();
    
    // Verify CTRL+O is mapped to OpenVariationSelector
    assert_eq!(
        config.get_action("Ctrl+o"),
        Some(&Action::OpenVariationSelector),
        "CTRL+O should be mapped to OpenVariationSelector action"
    );
}
