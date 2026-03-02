//! Unit tests for app module - ADSR, MUL, and SM parameter shortcuts

use crate::app::*;
use crate::models::DEFAULT_ENVELOPE_DELAY_SECONDS;
use crate::models::*;

#[test]
fn test_jump_to_ar_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial AR value
    app.values[0][PARAM_AR] = 10;

    // Jump to AR and increase
    app.jump_to_ar_and_increase();

    // Verify cursor moved to AR column
    assert_eq!(app.cursor_x, PARAM_AR, "Cursor should move to AR column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify AR value increased
    assert_eq!(
        app.values[0][PARAM_AR], 11,
        "AR should increase from 10 to 11"
    );
}

#[test]
fn test_jump_to_ar_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1 (O2/M2), column 5
    app.cursor_x = 5;
    app.cursor_y = 1;

    // Set initial AR value for row 1 (O2/M2)
    app.values[1][PARAM_AR] = 15;

    // Jump to AR and decrease
    app.jump_to_ar_and_decrease();

    // Verify cursor moved to AR column
    assert_eq!(app.cursor_x, PARAM_AR, "Cursor should move to AR column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify AR value decreased
    assert_eq!(
        app.values[1][PARAM_AR], 14,
        "AR should decrease from 15 to 14"
    );
}

#[test]
fn test_jump_to_d1r_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2 (O3/C1), column 1
    app.cursor_x = 1;
    app.cursor_y = 2;

    // Set initial D1R value for row 2 (O3/C1)
    app.values[2][PARAM_D1R] = 8;

    // Jump to D1R and increase
    app.jump_to_d1r_and_increase();

    // Verify cursor moved to D1R column
    assert_eq!(app.cursor_x, PARAM_D1R, "Cursor should move to D1R column");
    assert_eq!(app.cursor_y, 2, "Cursor should stay on same row");

    // Verify D1R value increased
    assert_eq!(
        app.values[2][PARAM_D1R], 9,
        "D1R should increase from 8 to 9"
    );
}

#[test]
fn test_jump_to_d1r_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 3
    app.cursor_x = 7;
    app.cursor_y = 3;

    // Set initial D1R value for display row 3 (data row 3)
    app.values[3][PARAM_D1R] = 20;

    // Jump to D1R and decrease
    app.jump_to_d1r_and_decrease();

    // Verify cursor moved to D1R column
    assert_eq!(app.cursor_x, PARAM_D1R, "Cursor should move to D1R column");
    assert_eq!(app.cursor_y, 3, "Cursor should stay on same row");

    // Verify D1R value decreased
    assert_eq!(
        app.values[3][PARAM_D1R], 19,
        "D1R should decrease from 20 to 19"
    );
}

#[test]
fn test_jump_to_d2r_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 2;
    app.cursor_y = 0;

    // Set initial D2R value
    app.values[0][PARAM_D2R] = 5;

    // Jump to D2R and increase
    app.jump_to_d2r_and_increase();

    // Verify cursor moved to D2R column
    assert_eq!(app.cursor_x, PARAM_D2R, "Cursor should move to D2R column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify D2R value increased
    assert_eq!(
        app.values[0][PARAM_D2R], 6,
        "D2R should increase from 5 to 6"
    );
}

#[test]
fn test_jump_to_d2r_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1 (O2/M2)
    app.cursor_x = 8;
    app.cursor_y = 1;

    // Set initial D2R value for row 1 (O2/M2)
    app.values[1][PARAM_D2R] = 12;

    // Jump to D2R and decrease
    app.jump_to_d2r_and_decrease();

    // Verify cursor moved to D2R column
    assert_eq!(app.cursor_x, PARAM_D2R, "Cursor should move to D2R column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify D2R value decreased
    assert_eq!(
        app.values[1][PARAM_D2R], 11,
        "D2R should decrease from 12 to 11"
    );
}

#[test]
fn test_jump_to_rr_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2 (O3/C1)
    app.cursor_x = 3;
    app.cursor_y = 2;

    // Set initial RR value for row 2 (O3/C1)
    app.values[2][PARAM_RR] = 7;

    // Jump to RR and increase
    app.jump_to_rr_and_increase();

    // Verify cursor moved to RR column
    assert_eq!(app.cursor_x, PARAM_RR, "Cursor should move to RR column");
    assert_eq!(app.cursor_y, 2, "Cursor should stay on same row");

    // Verify RR value increased
    assert_eq!(app.values[2][PARAM_RR], 8, "RR should increase from 7 to 8");
}

#[test]
fn test_jump_to_rr_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 3
    app.cursor_x = 1;
    app.cursor_y = 3;

    // Set initial RR value for display row 3 (data row 3)
    app.values[3][PARAM_RR] = 9;

    // Jump to RR and decrease
    app.jump_to_rr_and_decrease();

    // Verify cursor moved to RR column
    assert_eq!(app.cursor_x, PARAM_RR, "Cursor should move to RR column");
    assert_eq!(app.cursor_y, 3, "Cursor should stay on same row");

    // Verify RR value decreased
    assert_eq!(app.values[3][PARAM_RR], 8, "RR should decrease from 9 to 8");
}

#[test]
fn test_adsr_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 0;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use ADSR shortcuts on CH row - they should be ignored
    app.jump_to_ar_and_increase();
    assert_eq!(
        app.values, initial_values,
        "AR shortcut should not modify values on CH row"
    );

    app.jump_to_d1r_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "D1R shortcut should not modify values on CH row"
    );

    app.jump_to_d2r_and_increase();
    assert_eq!(
        app.values, initial_values,
        "D2R shortcut should not modify values on CH row"
    );

    app.jump_to_rr_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "RR shortcut should not modify values on CH row"
    );

    // Cursor should move to the parameter column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
}

#[test]
fn test_jump_to_mul_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial MUL value
    app.values[0][PARAM_MUL] = 5;

    // Jump to MUL and increase
    app.jump_to_mul_and_increase();

    // Verify cursor moved to MUL column
    assert_eq!(app.cursor_x, PARAM_MUL, "Cursor should move to MUL column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify MUL value increased
    assert_eq!(
        app.values[0][PARAM_MUL], 6,
        "MUL should increase from 5 to 6"
    );
}

#[test]
fn test_jump_to_mul_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1 (O2/M2), column 5
    app.cursor_x = 5;
    app.cursor_y = 1;

    // Set initial MUL value for row 1 (O2/M2)
    app.values[1][PARAM_MUL] = 10;

    // Jump to MUL and decrease
    app.jump_to_mul_and_decrease();

    // Verify cursor moved to MUL column
    assert_eq!(app.cursor_x, PARAM_MUL, "Cursor should move to MUL column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify MUL value decreased
    assert_eq!(
        app.values[1][PARAM_MUL], 9,
        "MUL should decrease from 10 to 9"
    );
}

#[test]
fn test_jump_to_mul_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set MUL to max value (15)
    app.values[0][PARAM_MUL] = PARAM_MAX[PARAM_MUL];

    // Jump to MUL and try to increase
    app.jump_to_mul_and_increase();

    // Verify MUL value did not exceed max
    assert_eq!(
        app.values[0][PARAM_MUL], PARAM_MAX[PARAM_MUL],
        "MUL should not exceed max value (15)"
    );
}

#[test]
fn test_jump_to_mul_clamps_to_min() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2
    app.cursor_x = 5;
    app.cursor_y = 2;

    // Set MUL to min value for display row 2 (data row 1)
    app.values[1][PARAM_MUL] = 0;

    // Jump to MUL and try to decrease
    app.jump_to_mul_and_decrease();

    // Verify MUL value did not go below min
    assert_eq!(
        app.values[1][PARAM_MUL], 0,
        "MUL should not go below min (0)"
    );
}

#[test]
fn test_mul_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 0;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use MUL shortcuts on CH row - they should be ignored
    app.jump_to_mul_and_increase();
    assert_eq!(
        app.values, initial_values,
        "MUL shortcut should not modify values on CH row"
    );

    app.jump_to_mul_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "MUL shortcut should not modify values on CH row"
    );

    // Cursor should move to the MUL column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
    assert_eq!(app.cursor_x, PARAM_MUL, "Cursor should move to MUL column");
}

#[test]
fn test_jump_to_sm_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0, column 5
    app.cursor_x = 5;
    app.cursor_y = 0;

    // Set initial SM value
    app.values[0][PARAM_SM] = 0;

    // Jump to SM and increase
    app.jump_to_sm_and_increase();

    // Verify cursor moved to SM column
    assert_eq!(app.cursor_x, PARAM_SM, "Cursor should move to SM column");
    assert_eq!(app.cursor_y, 0, "Cursor should stay on same row");

    // Verify SM value increased
    assert_eq!(app.values[0][PARAM_SM], 1, "SM should increase from 0 to 1");
}

#[test]
fn test_jump_to_sm_and_decrease() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 1, column 3
    app.cursor_x = 3;
    app.cursor_y = 1;

    // Set initial SM value for row 1
    app.values[1][PARAM_SM] = 1;

    // Jump to SM and decrease
    app.jump_to_sm_and_decrease();

    // Verify cursor moved to SM column
    assert_eq!(app.cursor_x, PARAM_SM, "Cursor should move to SM column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify SM value decreased
    assert_eq!(app.values[1][PARAM_SM], 0, "SM should decrease from 1 to 0");
}

#[test]
fn test_jump_to_sm_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 0
    app.cursor_x = 5;
    app.cursor_y = 0;

    // Set SM to max value (1)
    app.values[0][PARAM_SM] = PARAM_MAX[PARAM_SM];

    // Jump to SM and try to increase
    app.jump_to_sm_and_increase();

    // Verify SM value did not exceed max
    assert_eq!(
        app.values[0][PARAM_SM], PARAM_MAX[PARAM_SM],
        "SM should not exceed max value (1)"
    );
}

#[test]
fn test_jump_to_sm_clamps_to_min() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to operator row 2
    app.cursor_x = 5;
    app.cursor_y = 2;

    // Set SM to min value for row 2
    app.values[2][PARAM_SM] = 0;

    // Jump to SM and try to decrease
    app.jump_to_sm_and_decrease();

    // Verify SM value did not go below min
    assert_eq!(app.values[2][PARAM_SM], 0, "SM should not go below min (0)");
}

#[test]
fn test_sm_shortcuts_ignore_ch_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set cursor to CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;

    // Store initial values
    let initial_values = app.values;

    // Try to use SM shortcuts on CH row - they should be ignored
    app.jump_to_sm_and_increase();
    assert_eq!(
        app.values, initial_values,
        "SM shortcut should not modify values on CH row"
    );

    app.jump_to_sm_and_decrease();
    assert_eq!(
        app.values, initial_values,
        "SM shortcut should not modify values on CH row"
    );

    // Cursor should move to the SM column, but stay on CH row
    assert_eq!(app.cursor_y, ROW_CH, "Cursor should stay on CH row");
    assert_eq!(app.cursor_x, PARAM_SM, "Cursor should move to SM column");
}
