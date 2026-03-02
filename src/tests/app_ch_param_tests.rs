//! Unit tests for app module - cursor to mouse position, play tone, and CH parameter shortcuts

use crate::app::*;
use crate::models::DEFAULT_ENVELOPE_DELAY_SECONDS;
use crate::models::*;

#[test]
fn test_move_cursor_to_mouse_position() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Test moving cursor to operator row (M1, row 0)
    // UI layout: row_label_width=4, cell_width=4, inner_x=1, inner_y=1, label_offset=1
    // First cell starts at x=5 (1+4), operator rows start at y=2 (1+1+0)
    app.move_cursor_to_mouse_position(5, 2); // M1, column 0
    assert_eq!(app.cursor_x, 0);
    assert_eq!(app.cursor_y, 0);

    // Test moving cursor to different column
    app.move_cursor_to_mouse_position(9, 2); // M1, column 1
    assert_eq!(app.cursor_x, 1);
    assert_eq!(app.cursor_y, 0);

    // Test moving cursor to C1 row (display row 1, y=3)
    app.move_cursor_to_mouse_position(5, 3); // C1, column 0
    assert_eq!(app.cursor_x, 0);
    assert_eq!(app.cursor_y, 1);

    // Test moving cursor to CH row (y=7: 1+1+5)
    app.move_cursor_to_mouse_position(5, 7); // CH row, column 0
    assert_eq!(app.cursor_x, 0);
    assert_eq!(app.cursor_y, ROW_CH);

    // Test moving cursor to CH row, column 1
    app.move_cursor_to_mouse_position(9, 7); // CH row, column 1
    assert_eq!(app.cursor_x, 1);
    assert_eq!(app.cursor_y, ROW_CH);

    // Test clicking outside valid columns for CH row (should be ignored)
    let prev_x = app.cursor_x;
    let prev_y = app.cursor_y;
    app.move_cursor_to_mouse_position(25, 7); // CH row, column 5 (out of bounds, CH has only 3 columns)
    assert_eq!(
        app.cursor_x, prev_x,
        "Cursor X should not change when clicking outside valid CH columns"
    );
    assert_eq!(
        app.cursor_y, prev_y,
        "Cursor Y should not change when clicking outside valid CH columns"
    );

    // Test clicking in row label area (should be ignored)
    let prev_x = app.cursor_x;
    let prev_y = app.cursor_y;
    app.move_cursor_to_mouse_position(3, 2); // In row label area
    assert_eq!(
        app.cursor_x, prev_x,
        "Cursor should not move when clicking in row label area"
    );
    assert_eq!(
        app.cursor_y, prev_y,
        "Cursor should not move when clicking in row label area"
    );

    // Test clicking in header area (should be ignored)
    let prev_x = app.cursor_x;
    let prev_y = app.cursor_y;
    app.move_cursor_to_mouse_position(5, 1); // In header area
    assert_eq!(
        app.cursor_x, prev_x,
        "Cursor should not move when clicking in header area"
    );
    assert_eq!(
        app.cursor_y, prev_y,
        "Cursor should not move when clicking in header area"
    );

    // Test clicking on CH header row (y=6, should be ignored)
    let prev_x = app.cursor_x;
    let prev_y = app.cursor_y;
    app.move_cursor_to_mouse_position(5, 6); // CH header row
    assert_eq!(
        app.cursor_x, prev_x,
        "Cursor should not move when clicking on CH header row"
    );
    assert_eq!(
        app.cursor_y, prev_y,
        "Cursor should not move when clicking on CH header row"
    );
}

#[test]
fn test_play_current_tone_does_not_modify_values() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set specific values
    app.cursor_x = PARAM_DT;
    app.cursor_y = 0;
    app.values[0][PARAM_DT] = 3;
    app.values[0][PARAM_MUL] = 5;
    app.values[0][PARAM_TL] = 20;

    // Store all values before playing
    let values_before = app.values;
    let cursor_x_before = app.cursor_x;
    let cursor_y_before = app.cursor_y;

    // Play current tone
    app.play_current_tone();

    // Verify that no values were modified
    assert_eq!(
        app.values, values_before,
        "Values should not be modified by play_current_tone"
    );
    assert_eq!(
        app.cursor_x, cursor_x_before,
        "Cursor X should not be modified by play_current_tone"
    );
    assert_eq!(
        app.cursor_y, cursor_y_before,
        "Cursor Y should not be modified by play_current_tone"
    );

    // Verify specific values are unchanged
    assert_eq!(app.values[0][PARAM_DT], 3, "DT should remain unchanged");
    assert_eq!(app.values[0][PARAM_MUL], 5, "MUL should remain unchanged");
    assert_eq!(app.values[0][PARAM_TL], 20, "TL should remain unchanged");
}

#[test]
fn test_increase_fb() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set initial cursor position somewhere else
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial FB value
    app.values[ROW_CH][CH_PARAM_FB] = 3;

    // Call increase_fb
    app.increase_fb();

    // Verify cursor moved to FB position
    assert_eq!(
        app.cursor_x, CH_PARAM_FB,
        "Cursor X should move to FB column"
    );
    assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should move to CH row");

    // Verify FB value increased
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_FB], 4,
        "FB should increase from 3 to 4"
    );

    // Test boundary: increase at max should not exceed
    app.values[ROW_CH][CH_PARAM_FB] = CH_PARAM_MAX[CH_PARAM_FB]; // Set to max (7)
    app.increase_fb();
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_FB], CH_PARAM_MAX[CH_PARAM_FB],
        "FB should not exceed max value (7)"
    );

    // Verify cursor still at FB position
    assert_eq!(
        app.cursor_x, CH_PARAM_FB,
        "Cursor X should remain at FB column"
    );
    assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should remain at CH row");
}

#[test]
fn test_decrease_fb() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set initial cursor position somewhere else
    app.cursor_x = 5;
    app.cursor_y = 2;

    // Set initial FB value
    app.values[ROW_CH][CH_PARAM_FB] = 5;

    // Call decrease_fb
    app.decrease_fb();

    // Verify cursor moved to FB position
    assert_eq!(
        app.cursor_x, CH_PARAM_FB,
        "Cursor X should move to FB column"
    );
    assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should move to CH row");

    // Verify FB value decreased
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_FB], 4,
        "FB should decrease from 5 to 4"
    );

    // Test boundary: decrease at 0 should not go negative
    app.values[ROW_CH][CH_PARAM_FB] = 0;
    app.decrease_fb();
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_FB], 0,
        "FB should not go below 0"
    );

    // Verify cursor still at FB position
    assert_eq!(
        app.cursor_x, CH_PARAM_FB,
        "Cursor X should remain at FB column"
    );
    assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should remain at CH row");
}

#[test]
fn test_increase_fb_moves_cursor_from_operator_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Start with cursor on operator row
    app.cursor_x = PARAM_MUL;
    app.cursor_y = 1; // C1 row

    // Set initial FB value
    app.values[ROW_CH][CH_PARAM_FB] = 2;

    // Call increase_fb
    app.increase_fb();

    // Verify cursor moved to FB position
    assert_eq!(
        app.cursor_x, CH_PARAM_FB,
        "Cursor X should move to FB column from operator row"
    );
    assert_eq!(
        app.cursor_y, ROW_CH,
        "Cursor Y should move to CH row from operator row"
    );

    // Verify FB value increased
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_FB], 3,
        "FB should increase from 2 to 3"
    );
}

#[test]
fn test_decrease_fb_moves_cursor_from_operator_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Start with cursor on operator row
    app.cursor_x = PARAM_AR;
    app.cursor_y = 3; // C2 row

    // Set initial FB value
    app.values[ROW_CH][CH_PARAM_FB] = 6;

    // Call decrease_fb
    app.decrease_fb();

    // Verify cursor moved to FB position
    assert_eq!(
        app.cursor_x, CH_PARAM_FB,
        "Cursor X should move to FB column from operator row"
    );
    assert_eq!(
        app.cursor_y, ROW_CH,
        "Cursor Y should move to CH row from operator row"
    );

    // Verify FB value decreased
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_FB], 5,
        "FB should decrease from 6 to 5"
    );
}

#[test]
fn test_increase_alg() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set initial cursor position somewhere else
    app.cursor_x = 0;
    app.cursor_y = 0;

    // Set initial ALG value
    app.values[ROW_CH][CH_PARAM_ALG] = 3;

    // Call increase_alg
    app.increase_alg();

    // Verify cursor moved to ALG position
    assert_eq!(
        app.cursor_x, CH_PARAM_ALG,
        "Cursor X should move to ALG column"
    );
    assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should move to CH row");

    // Verify ALG value increased
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 4,
        "ALG should increase from 3 to 4"
    );

    // Test boundary: increase at max should not exceed
    app.values[ROW_CH][CH_PARAM_ALG] = CH_PARAM_MAX[CH_PARAM_ALG]; // Set to max (7)
    app.increase_alg();
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], CH_PARAM_MAX[CH_PARAM_ALG],
        "ALG should not exceed max value (7)"
    );

    // Verify cursor still at ALG position
    assert_eq!(
        app.cursor_x, CH_PARAM_ALG,
        "Cursor X should remain at ALG column"
    );
    assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should remain at CH row");
}

#[test]
fn test_decrease_alg() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set initial cursor position somewhere else
    app.cursor_x = 5;
    app.cursor_y = 2;

    // Set initial ALG value
    app.values[ROW_CH][CH_PARAM_ALG] = 5;

    // Call decrease_alg
    app.decrease_alg();

    // Verify cursor moved to ALG position
    assert_eq!(
        app.cursor_x, CH_PARAM_ALG,
        "Cursor X should move to ALG column"
    );
    assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should move to CH row");

    // Verify ALG value decreased
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 4,
        "ALG should decrease from 5 to 4"
    );

    // Test boundary: decrease at 0 should not go negative
    app.values[ROW_CH][CH_PARAM_ALG] = 0;
    app.decrease_alg();
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 0,
        "ALG should not go below 0"
    );

    // Verify cursor still at ALG position
    assert_eq!(
        app.cursor_x, CH_PARAM_ALG,
        "Cursor X should remain at ALG column"
    );
    assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should remain at CH row");
}

#[test]
fn test_increase_alg_moves_cursor_from_operator_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Start with cursor on operator row
    app.cursor_x = PARAM_MUL;
    app.cursor_y = 1; // C1 row

    // Set initial ALG value
    app.values[ROW_CH][CH_PARAM_ALG] = 2;

    // Call increase_alg
    app.increase_alg();

    // Verify cursor moved to ALG position
    assert_eq!(
        app.cursor_x, CH_PARAM_ALG,
        "Cursor X should move to ALG column from operator row"
    );
    assert_eq!(
        app.cursor_y, ROW_CH,
        "Cursor Y should move to CH row from operator row"
    );

    // Verify ALG value increased
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 3,
        "ALG should increase from 2 to 3"
    );
}

#[test]
fn test_decrease_alg_moves_cursor_from_operator_row() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Start with cursor on operator row
    app.cursor_x = PARAM_AR;
    app.cursor_y = 3; // C2 row

    // Set initial ALG value
    app.values[ROW_CH][CH_PARAM_ALG] = 6;

    // Call decrease_alg
    app.decrease_alg();

    // Verify cursor moved to ALG position
    assert_eq!(
        app.cursor_x, CH_PARAM_ALG,
        "Cursor X should move to ALG column from operator row"
    );
    assert_eq!(
        app.cursor_y, ROW_CH,
        "Cursor Y should move to CH row from operator row"
    );

    // Verify ALG value decreased
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 5,
        "ALG should decrease from 6 to 5"
    );
}
