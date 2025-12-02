//! Unit tests for app module

use crate::app::*;
use crate::models::*;

#[test]
fn test_cursor_movement() {
    let mut app = App::new(false, false);

    // Test initial position
    assert_eq!(app.cursor_x, 0);
    assert_eq!(app.cursor_y, 0);

    // Test move right
    app.move_cursor_right();
    assert_eq!(app.cursor_x, 1);
    assert_eq!(app.cursor_y, 0);

    // Test move down
    app.move_cursor_down();
    assert_eq!(app.cursor_x, 1);
    assert_eq!(app.cursor_y, 1);

    // Test move left
    app.move_cursor_left();
    assert_eq!(app.cursor_x, 0);
    assert_eq!(app.cursor_y, 1);

    // Test move up
    app.move_cursor_up();
    assert_eq!(app.cursor_x, 0);
    assert_eq!(app.cursor_y, 0);

    // Test boundary: can't move left from 0,0
    app.move_cursor_left();
    assert_eq!(app.cursor_x, 0);
    app.move_cursor_up();
    assert_eq!(app.cursor_y, 0);

    // Test boundary: move to max position
    for _ in 0..GRID_WIDTH {
        app.move_cursor_right();
    }
    assert_eq!(app.cursor_x, GRID_WIDTH - 1);

    for _ in 0..GRID_HEIGHT {
        app.move_cursor_down();
    }
    assert_eq!(app.cursor_y, GRID_HEIGHT - 1);
    // After moving down to CH row, cursor_x should be clamped to CH_PARAM_COUNT - 1
    assert_eq!(app.cursor_x, CH_PARAM_COUNT - 1);

    // Test can't exceed boundaries on CH row
    app.move_cursor_right();
    assert_eq!(app.cursor_x, CH_PARAM_COUNT - 1);
    app.move_cursor_down();
    assert_eq!(app.cursor_y, GRID_HEIGHT - 1);
}

#[test]
fn test_increase_decrease_value() {
    let mut app = App::new(false, false);

    // Move cursor to a parameter with a wider range (e.g., TL at index 1)
    app.cursor_x = PARAM_TL;

    // Store initial value
    let initial_value = app.values[0][PARAM_TL];

    // Increase value
    app.increase_value();
    assert_eq!(app.values[0][PARAM_TL], initial_value + 1);

    // Decrease value
    app.decrease_value();
    assert_eq!(app.values[0][PARAM_TL], initial_value);

    // Test boundary: decrease at 0 should not go negative
    app.values[0][PARAM_TL] = 0;
    app.decrease_value();
    assert_eq!(app.values[0][PARAM_TL], 0);

    // Test boundary: increase at max should not exceed
    app.cursor_x = PARAM_SM; // SM parameter
    app.values[0][PARAM_SM] = PARAM_MAX[PARAM_SM]; // Set to max (1)
    app.increase_value();
    assert_eq!(app.values[0][PARAM_SM], PARAM_MAX[PARAM_SM]);
}

#[test]
fn test_ch_row_cursor_restriction() {
    let mut app = App::new(false, false);

    // Start on an operator row, move to the right edge
    app.cursor_y = 0;
    app.cursor_x = 9; // Last column (DT2)

    // Move down to CH row - cursor should be clamped
    app.move_cursor_down();
    app.move_cursor_down();
    app.move_cursor_down();
    app.move_cursor_down();

    assert_eq!(app.cursor_y, ROW_CH);
    assert_eq!(
        app.cursor_x,
        CH_PARAM_COUNT - 1,
        "Cursor should be clamped to last CH column"
    );

    // Try to move right - should not move
    app.move_cursor_right();
    assert_eq!(
        app.cursor_x,
        CH_PARAM_COUNT - 1,
        "Cursor should not move beyond CH columns"
    );

    // Move to column 0
    app.cursor_x = 0;

    // Move up to operator row - cursor should stay at 0
    app.move_cursor_up();
    assert_eq!(app.cursor_y, 3);
    assert_eq!(app.cursor_x, 0, "Cursor x should remain valid");
}

#[test]
fn test_update_value_from_mouse_x() {
    let mut app = App::new(false, false);
    let terminal_width = 120;
    app.cursor_x = PARAM_DT; // DT has max value of 7
    app.cursor_y = 0;

    // Test mouse at left boundary of middle third (x=40) should give min value (0)
    app.update_value_from_mouse_x(40, terminal_width);
    assert_eq!(
        app.values[0][PARAM_DT], 0,
        "Left boundary of middle third should give min value"
    );

    // Test mouse at right boundary of middle third (x=80) should give max value
    app.update_value_from_mouse_x(80, terminal_width);
    assert_eq!(
        app.values[0][PARAM_DT], 7,
        "Right boundary of middle third should give max value"
    );

    // Test mouse at center of middle third (x=60) should give approximately half of max
    app.update_value_from_mouse_x(60, terminal_width);
    let middle_value = app.values[0][PARAM_DT];
    assert!(
        (3..=4).contains(&middle_value),
        "Center of middle third should give ~half of max value, got {}",
        middle_value
    );

    // Test with different parameter (MUL has max value of 15)
    app.cursor_x = PARAM_MUL;
    app.update_value_from_mouse_x(40, terminal_width);
    assert_eq!(
        app.values[0][PARAM_MUL], 0,
        "Left boundary of middle third should give min value for MUL"
    );

    app.update_value_from_mouse_x(80, terminal_width);
    assert_eq!(
        app.values[0][PARAM_MUL], 15,
        "Right boundary of middle third should give max value for MUL"
    );

    // Test edge case: mouse left of middle third (left side) should set to min (0)
    app.cursor_x = PARAM_DT;
    app.update_value_from_mouse_x(20, terminal_width); // Left third
    assert_eq!(
        app.values[0][PARAM_DT], 0,
        "Mouse left of middle third should set to min value (0)"
    );

    // Test edge case: mouse right of middle third (right side) should set to max
    app.update_value_from_mouse_x(100, terminal_width); // Right third
    assert_eq!(
        app.values[0][PARAM_DT], 7,
        "Mouse right of middle third should set to max value (7)"
    );
}

#[test]
fn test_update_value_from_mouse_x_zero_width() {
    let mut app = App::new(false, false);
    let terminal_width = 0;
    app.cursor_x = PARAM_DT;
    app.cursor_y = 0;

    let initial_value = app.values[0][PARAM_DT];

    // Should not crash or change value when terminal_width is 0
    app.update_value_from_mouse_x(50, terminal_width);
    assert_eq!(
        app.values[0][PARAM_DT], initial_value,
        "Value should not change when terminal_width is 0"
    );
}

#[test]
fn test_set_value_to_max() {
    let mut app = App::new(false, false);

    // Test with operator row parameter (DT, max = 7)
    app.cursor_x = PARAM_DT;
    app.cursor_y = 0;
    app.values[0][PARAM_DT] = 3;

    app.set_value_to_max();
    assert_eq!(
        app.values[0][PARAM_DT], 7,
        "DT should be set to max value (7)"
    );

    // Test with different parameter (MUL, max = 15)
    app.cursor_x = PARAM_MUL;
    app.values[0][PARAM_MUL] = 5;

    app.set_value_to_max();
    assert_eq!(
        app.values[0][PARAM_MUL], 15,
        "MUL should be set to max value (15)"
    );

    // Test with CH row parameter (ALG, max = 7)
    app.cursor_y = ROW_CH;
    app.cursor_x = CH_PARAM_ALG;
    app.values[ROW_CH][CH_PARAM_ALG] = 2;

    app.set_value_to_max();
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 7,
        "ALG should be set to max value (7)"
    );

    // Test with CH row FB (max = 7)
    app.cursor_x = CH_PARAM_FB;
    app.values[ROW_CH][CH_PARAM_FB] = 1;

    app.set_value_to_max();
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_FB], 7,
        "FB should be set to max value (7)"
    );
}

#[test]
fn test_set_value_to_min() {
    let mut app = App::new(false, false);

    // Test with operator row parameter
    app.cursor_x = PARAM_DT;
    app.cursor_y = 0;
    app.values[0][PARAM_DT] = 5;

    app.set_value_to_min();
    assert_eq!(
        app.values[0][PARAM_DT], 0,
        "DT should be set to min value (0)"
    );

    // Test with different parameter
    app.cursor_x = PARAM_MUL;
    app.values[0][PARAM_MUL] = 10;

    app.set_value_to_min();
    assert_eq!(
        app.values[0][PARAM_MUL], 0,
        "MUL should be set to min value (0)"
    );

    // Test with CH row parameter
    app.cursor_y = ROW_CH;
    app.cursor_x = CH_PARAM_ALG;
    app.values[ROW_CH][CH_PARAM_ALG] = 5;

    app.set_value_to_min();
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 0,
        "ALG should be set to min value (0)"
    );
}

#[test]
fn test_set_value_to_random() {
    let mut app = App::new(false, false);

    // Test with operator row parameter (DT, max = 7)
    app.cursor_x = PARAM_DT;
    app.cursor_y = 0;

    app.set_value_to_random();
    let random_value = app.values[0][PARAM_DT];
    assert!(
        random_value <= 7,
        "Random DT value should be <= 7, got {}",
        random_value
    );

    // Test with different parameter (TL, max = 99)
    app.cursor_x = PARAM_TL;

    app.set_value_to_random();
    let random_value = app.values[0][PARAM_TL];
    assert!(
        random_value <= 99,
        "Random TL value should be <= 99, got {}",
        random_value
    );

    // Test with CH row parameter (ALG, max = 7)
    app.cursor_y = ROW_CH;
    app.cursor_x = CH_PARAM_ALG;

    app.set_value_to_random();
    let random_value = app.values[ROW_CH][CH_PARAM_ALG];
    assert!(
        random_value <= 7,
        "Random ALG value should be <= 7, got {}",
        random_value
    );

    // Test that calling multiple times produces values in valid range
    for _ in 0..10 {
        app.cursor_x = PARAM_MUL;
        app.cursor_y = 0;
        app.set_value_to_random();
        let random_value = app.values[0][PARAM_MUL];
        assert!(
            random_value <= 15,
            "Random MUL value should be <= 15, got {}",
            random_value
        );
    }
}

#[test]
fn test_update_value_from_mouse_x_left_right_edges() {
    let mut app = App::new(false, false);
    let terminal_width = 120;

    // Test with DT parameter (max value = 7)
    app.cursor_x = PARAM_DT;
    app.cursor_y = 0;

    // Far left (x=0) should set to min (0)
    app.update_value_from_mouse_x(0, terminal_width);
    assert_eq!(
        app.values[0][PARAM_DT], 0,
        "Far left should set to min value (0)"
    );

    // Far right (x=119) should set to max (7)
    app.update_value_from_mouse_x(119, terminal_width);
    assert_eq!(
        app.values[0][PARAM_DT], 7,
        "Far right should set to max value (7)"
    );

    // Just before left boundary (x=39) should set to min (0)
    app.update_value_from_mouse_x(39, terminal_width);
    assert_eq!(
        app.values[0][PARAM_DT], 0,
        "Just before left boundary should set to min value (0)"
    );

    // Just after right boundary (x=81) should set to max (7)
    app.update_value_from_mouse_x(81, terminal_width);
    assert_eq!(
        app.values[0][PARAM_DT], 7,
        "Just after right boundary should set to max value (7)"
    );

    // Test with TL parameter (max value = 99)
    app.cursor_x = PARAM_TL;

    // Far left should set to min (0)
    app.update_value_from_mouse_x(0, terminal_width);
    assert_eq!(
        app.values[0][PARAM_TL], 0,
        "Far left should set to min value (0) for TL"
    );

    // Far right should set to max (99)
    app.update_value_from_mouse_x(119, terminal_width);
    assert_eq!(
        app.values[0][PARAM_TL], 99,
        "Far right should set to max value (99) for TL"
    );

    // Test with CH row parameters
    app.cursor_y = ROW_CH;
    app.cursor_x = CH_PARAM_ALG; // ALG has max value of 7

    // Far left should set to min (0)
    app.update_value_from_mouse_x(0, terminal_width);
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 0,
        "Far left should set to min value (0) for CH ALG"
    );

    // Far right should set to max (7)
    app.update_value_from_mouse_x(119, terminal_width);
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 7,
        "Far right should set to max value (7) for CH ALG"
    );
}

#[test]
fn test_move_cursor_to_mouse_position() {
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
fn test_increase_value_by() {
    let mut app = App::new(false, false);

    // Test with TL parameter (max = 99)
    app.cursor_x = PARAM_TL;
    app.cursor_y = 0;
    app.values[0][PARAM_TL] = 10;

    // Increase by 5
    app.increase_value_by(5);
    assert_eq!(
        app.values[0][PARAM_TL], 15,
        "TL should increase from 10 to 15"
    );

    // Increase by 10
    app.increase_value_by(10);
    assert_eq!(
        app.values[0][PARAM_TL], 25,
        "TL should increase from 15 to 25"
    );

    // Test boundary: increase near max should clamp
    app.values[0][PARAM_TL] = 95;
    app.increase_value_by(10);
    assert_eq!(app.values[0][PARAM_TL], 99, "TL should clamp to max (99)");

    // Test at max: should not change
    app.values[0][PARAM_TL] = 99;
    app.increase_value_by(5);
    assert_eq!(app.values[0][PARAM_TL], 99, "TL should remain at max (99)");
}

#[test]
fn test_decrease_value_by() {
    let mut app = App::new(false, false);

    // Test with TL parameter
    app.cursor_x = PARAM_TL;
    app.cursor_y = 0;
    app.values[0][PARAM_TL] = 50;

    // Decrease by 5
    app.decrease_value_by(5);
    assert_eq!(
        app.values[0][PARAM_TL], 45,
        "TL should decrease from 50 to 45"
    );

    // Decrease by 10
    app.decrease_value_by(10);
    assert_eq!(
        app.values[0][PARAM_TL], 35,
        "TL should decrease from 45 to 35"
    );

    // Test boundary: decrease near min should clamp to 0
    app.values[0][PARAM_TL] = 5;
    app.decrease_value_by(10);
    assert_eq!(app.values[0][PARAM_TL], 0, "TL should clamp to min (0)");

    // Test at min: should not change
    app.values[0][PARAM_TL] = 0;
    app.decrease_value_by(5);
    assert_eq!(app.values[0][PARAM_TL], 0, "TL should remain at min (0)");
}

#[test]
fn test_increase_value_by_with_different_parameters() {
    let mut app = App::new(false, false);

    // Test with DT parameter (max = 7)
    app.cursor_x = PARAM_DT;
    app.cursor_y = 0;
    app.values[0][PARAM_DT] = 2;

    app.increase_value_by(3);
    assert_eq!(app.values[0][PARAM_DT], 5, "DT should increase from 2 to 5");

    // Test clamping to max
    app.increase_value_by(5);
    assert_eq!(app.values[0][PARAM_DT], 7, "DT should clamp to max (7)");

    // Test with MUL parameter (max = 15)
    app.cursor_x = PARAM_MUL;
    app.values[0][PARAM_MUL] = 8;

    app.increase_value_by(9);
    assert_eq!(app.values[0][PARAM_MUL], 15, "MUL should clamp to max (15)");

    // Test with CH row parameter (ALG, max = 7)
    app.cursor_y = ROW_CH;
    app.cursor_x = CH_PARAM_ALG;
    app.values[ROW_CH][CH_PARAM_ALG] = 3;

    app.increase_value_by(2);
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 5,
        "ALG should increase from 3 to 5"
    );

    app.increase_value_by(10);
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_ALG], 7,
        "ALG should clamp to max (7)"
    );
}

#[test]
fn test_decrease_value_by_with_different_parameters() {
    let mut app = App::new(false, false);

    // Test with DT parameter
    app.cursor_x = PARAM_DT;
    app.cursor_y = 0;
    app.values[0][PARAM_DT] = 7;

    app.decrease_value_by(3);
    assert_eq!(app.values[0][PARAM_DT], 4, "DT should decrease from 7 to 4");

    // Test clamping to min
    app.decrease_value_by(10);
    assert_eq!(app.values[0][PARAM_DT], 0, "DT should clamp to min (0)");

    // Test with AR parameter (max = 31)
    app.cursor_x = PARAM_AR;
    app.values[0][PARAM_AR] = 25;

    app.decrease_value_by(9);
    assert_eq!(
        app.values[0][PARAM_AR], 16,
        "AR should decrease from 25 to 16"
    );

    // Test with CH row parameter (FB, max = 7)
    app.cursor_y = ROW_CH;
    app.cursor_x = CH_PARAM_FB;
    app.values[ROW_CH][CH_PARAM_FB] = 6;

    app.decrease_value_by(4);
    assert_eq!(
        app.values[ROW_CH][CH_PARAM_FB], 2,
        "FB should decrease from 6 to 2"
    );
}

#[test]
fn test_increase_value_by_amount_10() {
    let mut app = App::new(false, false);

    // Test with TL parameter which has max of 99 (supports +10 without clamping)
    app.cursor_x = PARAM_TL;
    app.cursor_y = 0;
    app.values[0][PARAM_TL] = 20;

    // Increase by 10 (simulating '0' key)
    app.increase_value_by(10);
    assert_eq!(
        app.values[0][PARAM_TL], 30,
        "TL should increase from 20 to 30"
    );

    // Increase by 10 again
    app.increase_value_by(10);
    assert_eq!(
        app.values[0][PARAM_TL], 40,
        "TL should increase from 30 to 40"
    );

    // Test near max - should clamp
    app.values[0][PARAM_TL] = 92;
    app.increase_value_by(10);
    assert_eq!(app.values[0][PARAM_TL], 99, "TL should clamp to max (99)");
}

#[test]
fn test_decrease_value_by_amount_10() {
    let mut app = App::new(false, false);

    // Test with TL parameter
    app.cursor_x = PARAM_TL;
    app.cursor_y = 0;
    app.values[0][PARAM_TL] = 50;

    // Decrease by 10 (simulating 'Shift+0' key)
    app.decrease_value_by(10);
    assert_eq!(
        app.values[0][PARAM_TL], 40,
        "TL should decrease from 50 to 40"
    );

    // Decrease by 10 again
    app.decrease_value_by(10);
    assert_eq!(
        app.values[0][PARAM_TL], 30,
        "TL should decrease from 40 to 30"
    );

    // Test near min - should clamp to 0
    app.values[0][PARAM_TL] = 7;
    app.decrease_value_by(10);
    assert_eq!(app.values[0][PARAM_TL], 0, "TL should clamp to min (0)");
}

#[test]
fn test_jump_to_op1_and_increase() {
    let mut app = App::new(false, false);

    // Start at a different position (OP3, column 5)
    app.cursor_x = 5;
    app.cursor_y = 2; // M2
    app.values[0][5] = 10; // Set OP1 column 5 to 10

    // Jump to OP1 and increase
    app.jump_to_operator_and_increase(0);

    // Verify cursor moved to OP1 (M1), column 5 preserved
    assert_eq!(app.cursor_y, 0, "Cursor should move to OP1 row (M1)");
    assert_eq!(app.cursor_x, 5, "Cursor column should be preserved");

    // Verify value increased
    assert_eq!(
        app.values[0][5], 11,
        "Value at OP1 column 5 should increase from 10 to 11"
    );
}

#[test]
fn test_jump_to_op2_and_increase() {
    let mut app = App::new(false, false);

    // Start at a different position
    app.cursor_x = 2;
    app.cursor_y = 0; // M1
    app.values[2][2] = 5; // Set OP2 (C1 - data row 2) column 2 to 5

    // Jump to OP2 (C1 - display row 1) and increase
    app.jump_to_operator_and_increase(1);

    // Verify cursor moved to OP2 (C1)
    assert_eq!(app.cursor_y, 1, "Cursor should move to OP2 row (C1)");
    assert_eq!(app.cursor_x, 2, "Cursor column should be preserved");

    // Verify value increased
    assert_eq!(
        app.values[2][2], 6,
        "Value at OP2 column 2 should increase from 5 to 6"
    );
}

#[test]
fn test_jump_to_op3_and_increase() {
    let mut app = App::new(false, false);

    // Start at CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;
    app.values[1][1] = 20; // Set OP3 (M2 - data row 1) column 1 to 20

    // Jump to OP3 (M2 - display row 2) and increase
    app.jump_to_operator_and_increase(2);

    // Verify cursor moved to OP3 (M2)
    assert_eq!(app.cursor_y, 2, "Cursor should move to OP3 row (M2)");
    assert_eq!(app.cursor_x, 1, "Cursor column should be preserved");

    // Verify value increased
    assert_eq!(
        app.values[1][1], 21,
        "Value at OP3 column 1 should increase from 20 to 21"
    );
}

#[test]
fn test_jump_to_op4_and_increase() {
    let mut app = App::new(false, false);

    // Start at a different position
    app.cursor_x = 3;
    app.cursor_y = 1; // C1
    app.values[3][3] = 15; // Set OP4 (C2 - data row 3) column 3 to 15

    // Jump to OP4 (C2 - display row 3) and increase
    app.jump_to_operator_and_increase(3);

    // Verify cursor moved to OP4 (C2)
    assert_eq!(app.cursor_y, 3, "Cursor should move to OP4 row (C2)");
    assert_eq!(app.cursor_x, 3, "Cursor column should be preserved");

    // Verify value increased
    assert_eq!(
        app.values[3][3], 16,
        "Value at OP4 column 3 should increase from 15 to 16"
    );
}

#[test]
fn test_jump_to_op1_and_decrease() {
    let mut app = App::new(false, false);

    // Start at a different position
    app.cursor_x = 4;
    app.cursor_y = 3; // C2
    app.values[0][4] = 10; // Set OP1 column 4 to 10

    // Jump to OP1 and decrease
    app.jump_to_operator_and_decrease(0);

    // Verify cursor moved to OP1 (M1)
    assert_eq!(app.cursor_y, 0, "Cursor should move to OP1 row (M1)");
    assert_eq!(app.cursor_x, 4, "Cursor column should be preserved");

    // Verify value decreased
    assert_eq!(
        app.values[0][4], 9,
        "Value at OP1 column 4 should decrease from 10 to 9"
    );
}

#[test]
fn test_jump_to_op2_and_decrease() {
    let mut app = App::new(false, false);

    // Start at a different position
    app.cursor_x = 6;
    app.cursor_y = 2; // M2
    app.values[2][6] = 8; // Set OP2 (C1 - data row 2) column 6 to 8

    // Jump to OP2 (C1 - display row 1) and decrease
    app.jump_to_operator_and_decrease(1);

    // Verify cursor moved to OP2 (C1)
    assert_eq!(app.cursor_y, 1, "Cursor should move to OP2 row (C1)");
    assert_eq!(app.cursor_x, 6, "Cursor column should be preserved");

    // Verify value decreased
    assert_eq!(
        app.values[2][6], 7,
        "Value at OP2 column 6 should decrease from 8 to 7"
    );
}

#[test]
fn test_jump_and_increase_clamps_to_max() {
    let mut app = App::new(false, false);

    // Set OP1 SM (column 0, max=1) to max value
    app.cursor_x = 0;
    app.cursor_y = 2; // M2
    app.values[0][0] = 1; // SM max is 1

    // Jump to OP1 and try to increase
    app.jump_to_operator_and_increase(0);

    // Verify value did not exceed max
    assert_eq!(app.values[0][0], 1, "SM should not exceed max value (1)");
}

#[test]
fn test_jump_and_decrease_clamps_to_min() {
    let mut app = App::new(false, false);

    // Set OP1 column 1 to min value
    app.cursor_x = 1;
    app.cursor_y = 3; // C2
    app.values[0][1] = 0;

    // Jump to OP1 and try to decrease
    app.jump_to_operator_and_decrease(0);

    // Verify value did not go below min
    assert_eq!(app.values[0][1], 0, "Value should not go below min (0)");
}

#[test]
fn test_jump_from_ch_row_clamps_cursor_x() {
    let mut app = App::new(false, false);

    // Start at CH row which has only 3 columns
    // Place cursor at column 2 (last column in CH row)
    app.cursor_x = 2;
    app.cursor_y = ROW_CH;

    // Set a value at OP1 column 2
    app.values[0][2] = 5;

    // Jump to OP1 - cursor_x should remain valid
    app.jump_to_operator_and_increase(0);

    // Verify cursor position is valid
    assert_eq!(app.cursor_y, 0, "Cursor should move to OP1");
    assert_eq!(
        app.cursor_x, 2,
        "Cursor column 2 should be valid for operator rows"
    );
    assert_eq!(app.values[0][2], 6, "Value should increase");
}

#[test]
fn test_rapid_operator_switching() {
    let mut app = App::new(false, false);

    // Set initial values for column 3 on all operators
    app.values[0][3] = 10; // OP1 (M1)
    app.values[2][3] = 15; // OP2 (C1)
    app.values[1][3] = 20; // OP3 (M2)
    app.values[3][3] = 25; // OP4 (C2)

    // Start at column 3
    app.cursor_x = 3;
    app.cursor_y = 0;

    // Jump to OP4 and increase
    app.jump_to_operator_and_increase(3);
    assert_eq!(app.cursor_y, 3, "Should jump to OP4");
    assert_eq!(app.values[3][3], 26, "OP4 value should increase");

    // Jump to OP1 and decrease
    app.jump_to_operator_and_decrease(0);
    assert_eq!(app.cursor_y, 0, "Should jump to OP1");
    assert_eq!(app.values[0][3], 9, "OP1 value should decrease");

    // Jump to OP2 and increase
    app.jump_to_operator_and_increase(1);
    assert_eq!(app.cursor_y, 1, "Should jump to OP2");
    assert_eq!(app.values[2][3], 16, "OP2 value should increase");

    // Jump to OP3 and decrease
    app.jump_to_operator_and_decrease(2);
    assert_eq!(app.cursor_y, 2, "Should jump to OP3");
    assert_eq!(app.values[1][3], 19, "OP3 value should decrease");
}

#[test]
fn test_jump_to_ar_and_increase() {
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

    // Set cursor to operator row 1, column 5
    app.cursor_x = 5;
    app.cursor_y = 1;

    // Set initial AR value for display row 1 (data row 2)
    app.values[2][PARAM_AR] = 15;

    // Jump to AR and decrease
    app.jump_to_ar_and_decrease();

    // Verify cursor moved to AR column
    assert_eq!(app.cursor_x, PARAM_AR, "Cursor should move to AR column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify AR value decreased
    assert_eq!(
        app.values[2][PARAM_AR], 14,
        "AR should decrease from 15 to 14"
    );
}

#[test]
fn test_jump_to_d1r_and_increase() {
    let mut app = App::new(false, false);

    // Set cursor to operator row 2, column 1
    app.cursor_x = 1;
    app.cursor_y = 2;

    // Set initial D1R value for display row 2 (data row 1)
    app.values[1][PARAM_D1R] = 8;

    // Jump to D1R and increase
    app.jump_to_d1r_and_increase();

    // Verify cursor moved to D1R column
    assert_eq!(app.cursor_x, PARAM_D1R, "Cursor should move to D1R column");
    assert_eq!(app.cursor_y, 2, "Cursor should stay on same row");

    // Verify D1R value increased
    assert_eq!(
        app.values[1][PARAM_D1R], 9,
        "D1R should increase from 8 to 9"
    );
}

#[test]
fn test_jump_to_d1r_and_decrease() {
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

    // Set cursor to operator row 1
    app.cursor_x = 8;
    app.cursor_y = 1;

    // Set initial D2R value for display row 1 (data row 2)
    app.values[2][PARAM_D2R] = 12;

    // Jump to D2R and decrease
    app.jump_to_d2r_and_decrease();

    // Verify cursor moved to D2R column
    assert_eq!(app.cursor_x, PARAM_D2R, "Cursor should move to D2R column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify D2R value decreased
    assert_eq!(
        app.values[2][PARAM_D2R], 11,
        "D2R should decrease from 12 to 11"
    );
}

#[test]
fn test_jump_to_rr_and_increase() {
    let mut app = App::new(false, false);

    // Set cursor to operator row 2
    app.cursor_x = 3;
    app.cursor_y = 2;

    // Set initial RR value for display row 2 (data row 1)
    app.values[1][PARAM_RR] = 7;

    // Jump to RR and increase
    app.jump_to_rr_and_increase();

    // Verify cursor moved to RR column
    assert_eq!(app.cursor_x, PARAM_RR, "Cursor should move to RR column");
    assert_eq!(app.cursor_y, 2, "Cursor should stay on same row");

    // Verify RR value increased
    assert_eq!(app.values[1][PARAM_RR], 8, "RR should increase from 7 to 8");
}

#[test]
fn test_jump_to_rr_and_decrease() {
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

    // Set cursor to operator row 1, column 5
    app.cursor_x = 5;
    app.cursor_y = 1;

    // Set initial MUL value for display row 1 (data row 2)
    app.values[2][PARAM_MUL] = 10;

    // Jump to MUL and decrease
    app.jump_to_mul_and_decrease();

    // Verify cursor moved to MUL column
    assert_eq!(app.cursor_x, PARAM_MUL, "Cursor should move to MUL column");
    assert_eq!(app.cursor_y, 1, "Cursor should stay on same row");

    // Verify MUL value decreased
    assert_eq!(
        app.values[2][PARAM_MUL], 9,
        "MUL should decrease from 10 to 9"
    );
}

#[test]
fn test_jump_to_mul_clamps_to_max() {
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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

#[test]
fn test_jump_to_tl_and_increase() {
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
    let mut app = App::new(false, false);

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
