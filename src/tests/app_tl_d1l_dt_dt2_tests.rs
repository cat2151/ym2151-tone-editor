//! Unit tests for app module - TL, D1L, DT, and DT2 parameter shortcuts

use crate::app::*;
use crate::models::DEFAULT_ENVELOPE_DELAY_SECONDS;
use crate::models::*;

#[test]
fn test_jump_to_tl_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 5
    app.cursor_x = 5;
    app.cursor_y = 0;

    // Set initial TL value
    app.values[0][PARAM_TL] = 50;

    // Jump to TL and increase
    app.jump_to_tl_and_increase();

    // Verify cursor moved to TL column
    assert_eq!(app.cursor_x, PARAM_TL, "Cursor should move to TL column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify TL value increased
    assert_eq!(
        app.values[0][PARAM_TL], 51,
        "TL should increase from 50 to 51"
    );
}

#[test]
fn test_jump_to_tl_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1, column 3
    app.cursor_x = 3;
    app.cursor_y = 1;

    // Set initial TL value for row 1
    app.values[1][PARAM_TL] = 75;

    // Jump to TL and decrease
    app.jump_to_tl_and_decrease();

    // Verify cursor moved to TL column
    assert_eq!(app.cursor_x, PARAM_TL, "Cursor should move to TL column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify TL value decreased
    assert_eq!(
        app.values[1][PARAM_TL], 74,
        "TL should decrease from 75 to 74"
    );
}

#[test]
fn test_jump_to_tl_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 5;
    app.cursor_y = 0;

    // Set TL to max value (99)
    app.values[0][PARAM_TL] = PARAM_MAX[PARAM_TL];

    // Jump to TL and try to increase
    app.jump_to_tl_and_increase();

    // Verify TL value did not exceed max
    assert_eq!(
        app.values[0][PARAM_TL], PARAM_MAX[PARAM_TL],
        "TL should not exceed max value (99)"
    );
}

#[test]
fn test_jump_to_tl_clamps_to_min() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2
    app.cursor_x = 5;
    app.cursor_y = 2;

    // Set TL to min value for row 2
    app.values[2][PARAM_TL] = 0;

    // Jump to TL and try to decrease
    app.jump_to_tl_and_decrease();

    // Verify TL value did not go below min
    assert_eq!(app.values[2][PARAM_TL], 0, "TL should not go below min (0)");
}

#[test]
fn test_tl_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use TL shortcuts on CH row - they should be ignored
    app.jump_to_tl_and_increase();
    assert_eq!(
        app.values, initial_values,
        "TL shortcut should not modify values on CH row"
    );

    app.jump_to_tl_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "TL shortcut should not modify values on CH row"
    );

    // Cursor should move to the TL column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
    assert_eq!(app.cursor_x, PARAM_TL, "Cursor should move to TL column");
}

#[test]
fn test_jump_to_d1l_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial D1L value
    app.values[0][PARAM_D1L] = 7;

    // Jump to D1L and increase
    app.jump_to_d1l_and_increase();

    // Verify cursor moved to D1L column
    assert_eq!(app.cursor_x, PARAM_D1L, "Cursor should move to D1L column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify D1L value increased
    assert_eq!(
        app.values[0][PARAM_D1L], 8,
        "D1L should increase from 7 to 8"
    );
}

#[test]
fn test_jump_to_d1l_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1, column 3
    app.cursor_x = 3;
    app.cursor_y = 1;

    // Set initial D1L value for row 1
    app.values[1][PARAM_D1L] = 10;

    // Jump to D1L and decrease
    app.jump_to_d1l_and_decrease();

    // Verify cursor moved to D1L column
    assert_eq!(app.cursor_x, PARAM_D1L, "Cursor should move to D1L column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify D1L value decreased
    assert_eq!(
        app.values[1][PARAM_D1L], 9,
        "D1L should decrease from 10 to 9"
    );
}

#[test]
fn test_jump_to_d1l_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set D1L to max value (15)
    app.values[0][PARAM_D1L] = PARAM_MAX[PARAM_D1L];

    // Jump to D1L and try to increase
    app.jump_to_d1l_and_increase();

    // Verify D1L value did not exceed max
    assert_eq!(
        app.values[0][PARAM_D1L], PARAM_MAX[PARAM_D1L],
        "D1L should not exceed max value (15)"
    );
}

#[test]
fn test_jump_to_d1l_clamps_to_min() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2
    app.cursor_x = 0;
    app.cursor_y = 2;

    // Set D1L to min value for row 2
    app.values[2][PARAM_D1L] = 0;

    // Jump to D1L and try to decrease
    app.jump_to_d1l_and_decrease();

    // Verify D1L value did not go below min
    assert_eq!(
        app.values[2][PARAM_D1L], 0,
        "D1L should not go below min (0)"
    );
}

#[test]
fn test_d1l_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use D1L shortcuts on CH row - they should be ignored
    app.jump_to_d1l_and_increase();
    assert_eq!(
        app.values, initial_values,
        "D1L shortcut should not modify values on CH row"
    );

    app.jump_to_d1l_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "D1L shortcut should not modify values on CH row"
    );

    // Cursor should move to the D1L column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
    assert_eq!(app.cursor_x, PARAM_D1L, "Cursor should move to D1L column");
}

#[test]
fn test_jump_to_dt_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial DT value
    app.values[0][PARAM_DT] = 3;

    // Jump to DT and increase
    app.jump_to_dt_and_increase();

    // Verify cursor moved to DT column
    assert_eq!(app.cursor_x, PARAM_DT, "Cursor should move to DT column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify DT value increased
    assert_eq!(app.values[0][PARAM_DT], 4, "DT should increase from 3 to 4");
}

#[test]
fn test_jump_to_dt_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1, column 3
    app.cursor_x = 3;
    app.cursor_y = 1;

    // Set initial DT value for row 1
    app.values[1][PARAM_DT] = 5;

    // Jump to DT and decrease
    app.jump_to_dt_and_decrease();

    // Verify cursor moved to DT column
    assert_eq!(app.cursor_x, PARAM_DT, "Cursor should move to DT column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify DT value decreased
    assert_eq!(app.values[1][PARAM_DT], 4, "DT should decrease from 5 to 4");
}

#[test]
fn test_jump_to_dt_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set DT to max value (7)
    app.values[0][PARAM_DT] = PARAM_MAX[PARAM_DT];

    // Jump to DT and try to increase
    app.jump_to_dt_and_increase();

    // Verify DT value did not exceed max
    assert_eq!(
        app.values[0][PARAM_DT], PARAM_MAX[PARAM_DT],
        "DT should not exceed max value (7)"
    );
}

#[test]
fn test_jump_to_dt_clamps_to_min() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2
    app.cursor_x = 0;
    app.cursor_y = 2;

    // Set DT to min value for row 2
    app.values[2][PARAM_DT] = 0;

    // Jump to DT and try to decrease
    app.jump_to_dt_and_decrease();

    // Verify DT value did not go below min
    assert_eq!(app.values[2][PARAM_DT], 0, "DT should not go below min (0)");
}

#[test]
fn test_dt_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use DT shortcuts on CH row - they should be ignored
    app.jump_to_dt_and_increase();
    assert_eq!(
        app.values, initial_values,
        "DT shortcut should not modify values on CH row"
    );

    app.jump_to_dt_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "DT shortcut should not modify values on CH row"
    );

    // Cursor should move to the DT column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
    assert_eq!(app.cursor_x, PARAM_DT, "Cursor should move to DT column");
}

#[test]
fn test_jump_to_dt2_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial DT2 value
    app.values[0][PARAM_DT2] = 1;

    // Jump to DT2 and increase
    app.jump_to_dt2_and_increase();

    // Verify cursor moved to DT2 column
    assert_eq!(app.cursor_x, PARAM_DT2, "Cursor should move to DT2 column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify DT2 value increased
    assert_eq!(
        app.values[0][PARAM_DT2], 2,
        "DT2 should increase from 1 to 2"
    );
}

#[test]
fn test_jump_to_dt2_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1, column 3
    app.cursor_x = 3;
    app.cursor_y = 1;

    // Set initial DT2 value for row 1
    app.values[1][PARAM_DT2] = 2;

    // Jump to DT2 and decrease
    app.jump_to_dt2_and_decrease();

    // Verify cursor moved to DT2 column
    assert_eq!(app.cursor_x, PARAM_DT2, "Cursor should move to DT2 column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify DT2 value decreased
    assert_eq!(
        app.values[1][PARAM_DT2], 1,
        "DT2 should decrease from 2 to 1"
    );
}

#[test]
fn test_jump_to_dt2_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set DT2 to max value (3)
    app.values[0][PARAM_DT2] = PARAM_MAX[PARAM_DT2];

    // Jump to DT2 and try to increase
    app.jump_to_dt2_and_increase();

    // Verify DT2 value did not exceed max
    assert_eq!(
        app.values[0][PARAM_DT2], PARAM_MAX[PARAM_DT2],
        "DT2 should not exceed max value (3)"
    );
}

#[test]
fn test_jump_to_dt2_clamps_to_min() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2
    app.cursor_x = 0;
    app.cursor_y = 2;

    // Set DT2 to min value for row 2
    app.values[2][PARAM_DT2] = 0;

    // Jump to DT2 and try to decrease
    app.jump_to_dt2_and_decrease();

    // Verify DT2 value did not go below min
    assert_eq!(
        app.values[2][PARAM_DT2], 0,
        "DT2 should not go below min (0)"
    );
}

#[test]
fn test_dt2_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use DT2 shortcuts on CH row - they should be ignored
    app.jump_to_dt2_and_increase();
    assert_eq!(
        app.values, initial_values,
        "DT2 shortcut should not modify values on CH row"
    );

    app.jump_to_dt2_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "DT2 shortcut should not modify values on CH row"
    );

    // Cursor should move to the DT2 column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
    assert_eq!(app.cursor_x, PARAM_DT2, "Cursor should move to DT2 column");
}
