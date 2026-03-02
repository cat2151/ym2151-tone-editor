//! Unit tests for register module - roundtrip, format, and envelope reset

use crate::models::*;
use crate::register::*;

#[test]
fn test_tone_data_to_registers() {
    let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];

    // Set some test values
    values[0][PARAM_MUL] = 1;
    values[0][PARAM_TL] = 20;
    values[0][PARAM_SM] = 1;
    values[1][PARAM_SM] = 1;
    values[2][PARAM_SM] = 1;
    values[3][PARAM_SM] = 1;
    values[ROW_CH][CH_PARAM_ALG] = 4;
    values[ROW_CH][CH_PARAM_FB] = 0;

    let registers = editor_rows_to_registers(&values);

    // Should be a hex string with pairs of address+data (4 chars per register write)
    // We have 28 events, so 28 * 4 = 112 characters
    assert_eq!(
        registers.len(),
        112,
        "Registers string should have 112 characters (28 events * 4 chars)"
    );

    // All characters should be valid hex
    assert!(
        registers.chars().all(|c| c.is_ascii_hexdigit()),
        "All characters should be hex digits"
    );
}

#[test]
fn test_registers_to_tone_data() {
    // Create a simple test case
    let mut values_original = [[0; GRID_WIDTH]; GRID_HEIGHT];
    values_original[0][PARAM_MUL] = 5;
    values_original[0][PARAM_TL] = 30;
    values_original[ROW_CH][CH_PARAM_ALG] = 3;
    values_original[ROW_CH][CH_PARAM_FB] = 2;

    // Convert to registers string
    let registers = editor_rows_to_registers(&values_original);

    // Convert back to tone data
    let values_result = registers_to_editor_rows(&registers).unwrap();

    // Verify key values are preserved
    assert_eq!(
        values_result[0][PARAM_MUL], values_original[0][PARAM_MUL],
        "MUL should roundtrip correctly"
    );
    assert_eq!(
        values_result[0][PARAM_TL], values_original[0][PARAM_TL],
        "TL should roundtrip correctly"
    );
    assert_eq!(
        values_result[ROW_CH][CH_PARAM_ALG], values_original[ROW_CH][CH_PARAM_ALG],
        "ALG should roundtrip correctly"
    );
    assert_eq!(
        values_result[ROW_CH][CH_PARAM_FB], values_original[ROW_CH][CH_PARAM_FB],
        "FB should roundtrip correctly"
    );
}

#[test]
#[allow(clippy::needless_range_loop)]
fn test_registers_to_tone_data_roundtrip() {
    // Test a more complete roundtrip with various parameter values
    let mut values_original = [[0; GRID_WIDTH]; GRID_HEIGHT];

    // Set different values for each operator
    for row in 0..4 {
        values_original[row][PARAM_SM] = 1;
        values_original[row][PARAM_MUL] = (row + 1) as u8;
        values_original[row][PARAM_TL] = (row * 10) as u8;
        values_original[row][PARAM_AR] = (row * 5) as u8;
        values_original[row][PARAM_D1R] = (row * 3) as u8;
        values_original[row][PARAM_D1L] = (row * 2) as u8;
    }
    values_original[ROW_CH][CH_PARAM_ALG] = 5;
    values_original[ROW_CH][CH_PARAM_FB] = 3;

    // Convert to registers and back
    let registers = editor_rows_to_registers(&values_original);
    let values_roundtrip = registers_to_editor_rows(&registers).unwrap();

    // Verify all important values are preserved
    for row in 0..4 {
        assert_eq!(
            values_roundtrip[row][PARAM_MUL], values_original[row][PARAM_MUL],
            "Row {} MUL should roundtrip",
            row
        );
        assert_eq!(
            values_roundtrip[row][PARAM_TL], values_original[row][PARAM_TL],
            "Row {} TL should roundtrip",
            row
        );
        assert_eq!(
            values_roundtrip[row][PARAM_AR], values_original[row][PARAM_AR],
            "Row {} AR should roundtrip",
            row
        );
        assert_eq!(
            values_roundtrip[row][PARAM_D1R], values_original[row][PARAM_D1R],
            "Row {} D1R should roundtrip",
            row
        );
        assert_eq!(
            values_roundtrip[row][PARAM_D1L], values_original[row][PARAM_D1L],
            "Row {} D1L should roundtrip",
            row
        );
    }
    assert_eq!(
        values_roundtrip[ROW_CH][CH_PARAM_ALG], values_original[ROW_CH][CH_PARAM_ALG],
        "ALG should roundtrip"
    );
    assert_eq!(
        values_roundtrip[ROW_CH][CH_PARAM_FB], values_original[ROW_CH][CH_PARAM_FB],
        "FB should roundtrip"
    );
}

#[test]
fn test_registers_invalid_length() {
    // Test with invalid length (not a multiple of 4)
    let result = registers_to_editor_rows("204F2");
    assert!(result.is_err(), "Should error on invalid length");
}

#[test]
fn test_registers_invalid_hex() {
    // Test with invalid hex characters
    let result = registers_to_editor_rows("GGGG");
    assert!(result.is_err(), "Should error on invalid hex characters");
}

#[test]
#[cfg(windows)]
fn test_envelope_reset_events() {
    use crate::register::editor_rows_to_ym2151_events_with_envelope_reset;

    let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];

    // Set some ADSR values different from max to test they get overridden
    values[0][PARAM_AR] = 10;
    values[0][PARAM_D1R] = 5;
    values[0][PARAM_D1L] = 3;
    values[0][PARAM_D2R] = 5;
    values[0][PARAM_RR] = 7;

    values[1][PARAM_AR] = 15;
    values[1][PARAM_D1R] = 10;
    values[1][PARAM_D1L] = 8;
    values[1][PARAM_D2R] = 7;
    values[1][PARAM_RR] = 9;

    values[2][PARAM_AR] = 20;
    values[2][PARAM_D1R] = 15;
    values[2][PARAM_D1L] = 10;
    values[2][PARAM_D2R] = 3;
    values[2][PARAM_RR] = 5;

    values[3][PARAM_AR] = 25;
    values[3][PARAM_D1R] = 20;
    values[3][PARAM_D1L] = 12;
    values[3][PARAM_D2R] = 9;
    values[3][PARAM_RR] = 11;

    // Set DT2, KS, AMS values to verify they are preserved
    values[0][PARAM_DT2] = 1;
    values[0][PARAM_KS] = 2;
    values[0][PARAM_AMS] = 1;

    values[1][PARAM_DT2] = 2;
    values[1][PARAM_KS] = 3;
    values[1][PARAM_AMS] = 2;

    values[2][PARAM_DT2] = 0;
    values[2][PARAM_KS] = 1;
    values[2][PARAM_AMS] = 0;

    values[3][PARAM_DT2] = 3;
    values[3][PARAM_KS] = 0;
    values[3][PARAM_AMS] = 3;

    // Enable all operators
    values[0][PARAM_SM] = 1;
    values[1][PARAM_SM] = 1;
    values[2][PARAM_SM] = 1;
    values[3][PARAM_SM] = 1;

    values[ROW_CH][CH_PARAM_ALG] = 4;
    values[ROW_CH][CH_PARAM_FB] = 0;

    let events =
        editor_rows_to_ym2151_events_with_envelope_reset(&values, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Verify full ADSR envelope reset events at time 0.0
    use std::collections::HashSet;
    let channel = 0;

    // Check AR=31 events (registers 0x80-0x9F)
    let expected_ar_addrs: HashSet<String> = (0..4)
        .map(|row_id| {
            let reg = crate::register::REG_FROM_O1_O4[row_id];
            let op_offset = reg * 8 + channel;
            format!("0x{:02X}", 0x80 + op_offset)
        })
        .collect();

    let ar_events: Vec<_> = events
        .iter()
        .filter(|e| expected_ar_addrs.contains(&e.addr) && e.time == 0.0)
        .collect();

    assert_eq!(
        ar_events.len(),
        4,
        "Should have exactly 4 AR register writes at time 0.0"
    );

    // Check first AR event to verify AR=31 and KS is preserved
    let first_ar = ar_events[0];
    let data = u8::from_str_radix(first_ar.data.trim_start_matches("0x"), 16).unwrap();
    assert_eq!(
        data & 0x1F,
        0x1F,
        "AR should be set to 31 for envelope reset"
    );
    assert_eq!((data >> 6) & 0x03, 2, "KS should be preserved as 2");

    // Check D1R=31 events (registers 0xA0-0xBF)
    let expected_d1r_addrs: HashSet<String> = (0..4)
        .map(|row_id| {
            let reg = crate::register::REG_FROM_O1_O4[row_id];
            let op_offset = reg * 8 + channel;
            format!("0x{:02X}", 0xA0 + op_offset)
        })
        .collect();

    let d1r_events: Vec<_> = events
        .iter()
        .filter(|e| expected_d1r_addrs.contains(&e.addr) && e.time == 0.0)
        .collect();

    assert_eq!(
        d1r_events.len(),
        4,
        "Should have exactly 4 D1R register writes at time 0.0"
    );

    // Check first D1R event to verify D1R=31 and AMS is preserved
    let first_d1r = d1r_events[0];
    let data = u8::from_str_radix(first_d1r.data.trim_start_matches("0x"), 16).unwrap();
    assert_eq!(
        data & 0x1F,
        0x1F,
        "D1R should be set to 31 for envelope reset"
    );
    assert_eq!((data >> 6) & 0x03, 1, "AMS should be preserved as 1");

    // Check D2R=15 events (registers 0xC0-0xDF)
    let expected_d2r_addrs: HashSet<String> = (0..4)
        .map(|row_id| {
            let reg = crate::register::REG_FROM_O1_O4[row_id];
            let op_offset = reg * 8 + channel;
            format!("0x{:02X}", 0xC0 + op_offset)
        })
        .collect();

    let d2r_events: Vec<_> = events
        .iter()
        .filter(|e| expected_d2r_addrs.contains(&e.addr) && e.time == 0.0)
        .collect();

    assert_eq!(
        d2r_events.len(),
        4,
        "Should have exactly 4 D2R register writes at time 0.0"
    );

    // Check first D2R event to verify D2R=15 and DT2 is preserved
    let first_d2r = d2r_events[0];
    let data = u8::from_str_radix(first_d2r.data.trim_start_matches("0x"), 16).unwrap();
    assert_eq!(
        data & 0x0F,
        0x0F,
        "D2R should be set to 15 for envelope reset"
    );
    assert_eq!((data >> 6) & 0x03, 1, "DT2 should be preserved as 1");

    // Check D1L=15, RR=15 events (registers 0xE0-0xFF)
    let expected_rr_addrs: HashSet<String> = (0..4)
        .map(|row_id| {
            let reg = crate::register::REG_FROM_O1_O4[row_id];
            let op_offset = reg * 8 + channel;
            format!("0x{:02X}", 0xE0 + op_offset)
        })
        .collect();

    let rr_events: Vec<_> = events
        .iter()
        .filter(|e| expected_rr_addrs.contains(&e.addr) && e.time == 0.0)
        .collect();

    assert_eq!(
        rr_events.len(),
        4,
        "Should have exactly 4 D1L/RR register writes at time 0.0"
    );

    // Check first D1L/RR event to verify both are set to 15
    let first_rr = rr_events[0];
    let data = u8::from_str_radix(first_rr.data.trim_start_matches("0x"), 16).unwrap();
    assert_eq!(
        data & 0x0F,
        0x0F,
        "RR should be set to 15 for envelope reset"
    );
    assert_eq!(
        (data >> 4) & 0x0F,
        0x0F,
        "D1L should be set to 15 for envelope reset"
    );

    // Verify KEY_OFF event at time 0.0
    let key_off_events: Vec<_> = events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time == 0.0)
        .collect();
    assert_eq!(
        key_off_events.len(),
        1,
        "Should have exactly one KEY_OFF event at time 0.0"
    );
    assert_eq!(
        key_off_events[0].data, "0x00",
        "KEY_OFF should be for channel 0"
    );

    // Verify tone settings and KEY_ON are at the default envelope delay time
    let delayed_events: Vec<_> = events
        .iter()
        .filter(|e| e.time == DEFAULT_ENVELOPE_DELAY_SECONDS)
        .collect();
    assert!(
        !delayed_events.is_empty(),
        "Should have events at time {} (tone settings and KEY_ON)",
        DEFAULT_ENVELOPE_DELAY_SECONDS
    );

    // Verify KEY_ON is at the default envelope delay time
    let key_on_events: Vec<_> = events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time == DEFAULT_ENVELOPE_DELAY_SECONDS)
        .collect();
    assert_eq!(
        key_on_events.len(),
        1,
        "Should have exactly one KEY_ON event at time {}",
        DEFAULT_ENVELOPE_DELAY_SECONDS
    );

    // Verify total event count: envelope reset events + normal events
    let normal_events = crate::register::editor_rows_to_ym2151_events(&values);
    // Full envelope reset: 4 operators × 4 registers (AR, D1R, D2R, D1L/RR) = 16 events
    let expected_total = 16 + 1 + normal_events.len(); // 16 ADSR + 1 KEY_OFF + normal events
    assert_eq!(
        events.len(),
        expected_total,
        "Should have correct total number of events (16 ADSR + 1 KEY_OFF + {} normal = {})",
        normal_events.len(),
        expected_total
    );
}
