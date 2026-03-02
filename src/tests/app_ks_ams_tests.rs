//! Unit tests for app module - KS and AMS parameter shortcuts

use crate::app::*;
use crate::models::DEFAULT_ENVELOPE_DELAY_SECONDS;
use crate::models::*;

#[test]
fn test_jump_to_ks_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial KS value
    app.values[0][PARAM_KS] = 1;

    // Jump to KS and increase
    app.jump_to_ks_and_increase();

    // Verify cursor moved to KS column
    assert_eq!(app.cursor_x, PARAM_KS, "Cursor should move to KS column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify KS value increased
    assert_eq!(app.values[0][PARAM_KS], 2, "KS should increase from 1 to 2");
}

#[test]
fn test_jump_to_ks_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1, column 3
    app.cursor_x = 3;
    app.cursor_y = 1;

    // Set initial KS value for row 1
    app.values[1][PARAM_KS] = 2;

    // Jump to KS and decrease
    app.jump_to_ks_and_decrease();

    // Verify cursor moved to KS column
    assert_eq!(app.cursor_x, PARAM_KS, "Cursor should move to KS column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify KS value decreased
    assert_eq!(app.values[1][PARAM_KS], 1, "KS should decrease from 2 to 1");
}

#[test]
fn test_jump_to_ks_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set KS to max value (3)
    app.values[0][PARAM_KS] = PARAM_MAX[PARAM_KS];

    // Jump to KS and try to increase
    app.jump_to_ks_and_increase();

    // Verify KS value did not exceed max
    assert_eq!(
        app.values[0][PARAM_KS], PARAM_MAX[PARAM_KS],
        "KS should not exceed max value (3)"
    );
}

#[test]
fn test_jump_to_ks_clamps_to_min() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2
    app.cursor_x = 0;
    app.cursor_y = 2;

    // Set KS to min value for row 2
    app.values[2][PARAM_KS] = 0;

    // Jump to KS and try to decrease
    app.jump_to_ks_and_decrease();

    // Verify KS value did not go below min
    assert_eq!(app.values[2][PARAM_KS], 0, "KS should not go below min (0)");
}

#[test]
fn test_ks_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use KS shortcuts on CH row - they should be ignored
    app.jump_to_ks_and_increase();
    assert_eq!(
        app.values, initial_values,
        "KS shortcut should not modify values on CH row"
    );

    app.jump_to_ks_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "KS shortcut should not modify values on CH row"
    );

    // Cursor should move to the KS column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
    assert_eq!(app.cursor_x, PARAM_KS, "Cursor should move to KS column");
}

#[test]
fn test_jump_to_ams_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial AMS value
    app.values[0][PARAM_AMS] = 1;

    // Jump to AMS and increase
    app.jump_to_ams_and_increase();

    // Verify cursor moved to AMS column
    assert_eq!(app.cursor_x, PARAM_AMS, "Cursor should move to AMS column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify AMS value increased
    assert_eq!(
        app.values[0][PARAM_AMS], 2,
        "AMS should increase from 1 to 2"
    );
}

#[test]
fn test_jump_to_ams_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1, column 3
    app.cursor_x = 3;
    app.cursor_y = 1;

    // Set initial AMS value for row 1
    app.values[1][PARAM_AMS] = 2;

    // Jump to AMS and decrease
    app.jump_to_ams_and_decrease();

    // Verify cursor moved to AMS column
    assert_eq!(app.cursor_x, PARAM_AMS, "Cursor should move to AMS column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify AMS value decreased
    assert_eq!(
        app.values[1][PARAM_AMS], 1,
        "AMS should decrease from 2 to 1"
    );
}

#[test]
fn test_jump_to_ams_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set AMS to max value (3)
    app.values[0][PARAM_AMS] = PARAM_MAX[PARAM_AMS];

    // Jump to AMS and try to increase
    app.jump_to_ams_and_increase();

    // Verify AMS value did not exceed max
    assert_eq!(
        app.values[0][PARAM_AMS], PARAM_MAX[PARAM_AMS],
        "AMS should not exceed max value (3)"
    );
}

#[test]
fn test_jump_to_ams_clamps_to_min() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2
    app.cursor_x = 0;
    app.cursor_y = 2;

    // Set AMS to min value for row 2
    app.values[2][PARAM_AMS] = 0;

    // Jump to AMS and try to decrease
    app.jump_to_ams_and_decrease();

    // Verify AMS value did not go below min
    assert_eq!(
        app.values[2][PARAM_AMS], 0,
        "AMS should not go below min (0)"
    );
}

#[test]
fn test_ams_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use AMS shortcuts on CH row - they should be ignored
    app.jump_to_ams_and_increase();
    assert_eq!(
        app.values, initial_values,
        "AMS shortcut should not modify values on CH row"
    );

    app.jump_to_ams_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "AMS shortcut should not modify values on CH row"
    );

    // Cursor should move to the AMS column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
    assert_eq!(app.cursor_x, PARAM_AMS, "Cursor should move to AMS column");
}
