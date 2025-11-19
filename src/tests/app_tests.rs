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
        assert_eq!(app.cursor_x, CH_PARAM_COUNT - 1, "Cursor should be clamped to last CH column");
        
        // Try to move right - should not move
        app.move_cursor_right();
        assert_eq!(app.cursor_x, CH_PARAM_COUNT - 1, "Cursor should not move beyond CH columns");
        
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
        assert_eq!(app.values[0][PARAM_DT], 0, "Left boundary of middle third should give min value");
        
        // Test mouse at right boundary of middle third (x=80) should give max value
        app.update_value_from_mouse_x(80, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 7, "Right boundary of middle third should give max value");
        
        // Test mouse at center of middle third (x=60) should give approximately half of max
        app.update_value_from_mouse_x(60, terminal_width);
        let middle_value = app.values[0][PARAM_DT];
        assert!(middle_value >= 3 && middle_value <= 4, "Center of middle third should give ~half of max value, got {}", middle_value);
        
        // Test with different parameter (MUL has max value of 15)
        app.cursor_x = PARAM_MUL;
        app.update_value_from_mouse_x(40, terminal_width);
        assert_eq!(app.values[0][PARAM_MUL], 0, "Left boundary of middle third should give min value for MUL");
        
        app.update_value_from_mouse_x(80, terminal_width);
        assert_eq!(app.values[0][PARAM_MUL], 15, "Right boundary of middle third should give max value for MUL");
        
        // Test edge case: mouse left of middle third (left side) should set to min (0)
        app.cursor_x = PARAM_DT;
        app.update_value_from_mouse_x(20, terminal_width); // Left third
        assert_eq!(app.values[0][PARAM_DT], 0, "Mouse left of middle third should set to min value (0)");
        
        // Test edge case: mouse right of middle third (right side) should set to max
        app.update_value_from_mouse_x(100, terminal_width); // Right third
        assert_eq!(app.values[0][PARAM_DT], 7, "Mouse right of middle third should set to max value (7)");
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
        assert_eq!(app.values[0][PARAM_DT], initial_value, "Value should not change when terminal_width is 0");
    }

    #[test]
    fn test_set_value_to_max() {
        let mut app = App::new(false, false);
        
        // Test with operator row parameter (DT, max = 7)
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        app.values[0][PARAM_DT] = 3;
        
        app.set_value_to_max();
        assert_eq!(app.values[0][PARAM_DT], 7, "DT should be set to max value (7)");
        
        // Test with different parameter (MUL, max = 15)
        app.cursor_x = PARAM_MUL;
        app.values[0][PARAM_MUL] = 5;
        
        app.set_value_to_max();
        assert_eq!(app.values[0][PARAM_MUL], 15, "MUL should be set to max value (15)");
        
        // Test with CH row parameter (ALG, max = 7)
        app.cursor_y = ROW_CH;
        app.cursor_x = CH_PARAM_ALG;
        app.values[ROW_CH][CH_PARAM_ALG] = 2;
        
        app.set_value_to_max();
        assert_eq!(app.values[ROW_CH][CH_PARAM_ALG], 7, "ALG should be set to max value (7)");
        
        // Test with CH row FB (max = 7)
        app.cursor_x = CH_PARAM_FB;
        app.values[ROW_CH][CH_PARAM_FB] = 1;
        
        app.set_value_to_max();
        assert_eq!(app.values[ROW_CH][CH_PARAM_FB], 7, "FB should be set to max value (7)");
    }

    #[test]
    fn test_set_value_to_min() {
        let mut app = App::new(false, false);
        
        // Test with operator row parameter
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        app.values[0][PARAM_DT] = 5;
        
        app.set_value_to_min();
        assert_eq!(app.values[0][PARAM_DT], 0, "DT should be set to min value (0)");
        
        // Test with different parameter
        app.cursor_x = PARAM_MUL;
        app.values[0][PARAM_MUL] = 10;
        
        app.set_value_to_min();
        assert_eq!(app.values[0][PARAM_MUL], 0, "MUL should be set to min value (0)");
        
        // Test with CH row parameter
        app.cursor_y = ROW_CH;
        app.cursor_x = CH_PARAM_ALG;
        app.values[ROW_CH][CH_PARAM_ALG] = 5;
        
        app.set_value_to_min();
        assert_eq!(app.values[ROW_CH][CH_PARAM_ALG], 0, "ALG should be set to min value (0)");
    }

    #[test]
    fn test_set_value_to_random() {
        let mut app = App::new(false, false);
        
        // Test with operator row parameter (DT, max = 7)
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        
        app.set_value_to_random();
        let random_value = app.values[0][PARAM_DT];
        assert!(random_value <= 7, "Random DT value should be <= 7, got {}", random_value);
        
        // Test with different parameter (TL, max = 99)
        app.cursor_x = PARAM_TL;
        
        app.set_value_to_random();
        let random_value = app.values[0][PARAM_TL];
        assert!(random_value <= 99, "Random TL value should be <= 99, got {}", random_value);
        
        // Test with CH row parameter (ALG, max = 7)
        app.cursor_y = ROW_CH;
        app.cursor_x = CH_PARAM_ALG;
        
        app.set_value_to_random();
        let random_value = app.values[ROW_CH][CH_PARAM_ALG];
        assert!(random_value <= 7, "Random ALG value should be <= 7, got {}", random_value);
        
        // Test that calling multiple times produces values in valid range
        for _ in 0..10 {
            app.cursor_x = PARAM_MUL;
            app.cursor_y = 0;
            app.set_value_to_random();
            let random_value = app.values[0][PARAM_MUL];
            assert!(random_value <= 15, "Random MUL value should be <= 15, got {}", random_value);
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
        assert_eq!(app.values[0][PARAM_DT], 0, "Far left should set to min value (0)");
        
        // Far right (x=119) should set to max (7)
        app.update_value_from_mouse_x(119, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 7, "Far right should set to max value (7)");
        
        // Just before left boundary (x=39) should set to min (0)
        app.update_value_from_mouse_x(39, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 0, "Just before left boundary should set to min value (0)");
        
        // Just after right boundary (x=81) should set to max (7)
        app.update_value_from_mouse_x(81, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 7, "Just after right boundary should set to max value (7)");
        
        // Test with TL parameter (max value = 99)
        app.cursor_x = PARAM_TL;
        
        // Far left should set to min (0)
        app.update_value_from_mouse_x(0, terminal_width);
        assert_eq!(app.values[0][PARAM_TL], 0, "Far left should set to min value (0) for TL");
        
        // Far right should set to max (99)
        app.update_value_from_mouse_x(119, terminal_width);
        assert_eq!(app.values[0][PARAM_TL], 99, "Far right should set to max value (99) for TL");
        
        // Test with CH row parameters
        app.cursor_y = ROW_CH;
        app.cursor_x = CH_PARAM_ALG; // ALG has max value of 7
        
        // Far left should set to min (0)
        app.update_value_from_mouse_x(0, terminal_width);
        assert_eq!(app.values[ROW_CH][CH_PARAM_ALG], 0, "Far left should set to min value (0) for CH ALG");
        
        // Far right should set to max (7)
        app.update_value_from_mouse_x(119, terminal_width);
        assert_eq!(app.values[ROW_CH][CH_PARAM_ALG], 7, "Far right should set to max value (7) for CH ALG");
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
        assert_eq!(app.cursor_x, prev_x, "Cursor X should not change when clicking outside valid CH columns");
        assert_eq!(app.cursor_y, prev_y, "Cursor Y should not change when clicking outside valid CH columns");
        
        // Test clicking in row label area (should be ignored)
        let prev_x = app.cursor_x;
        let prev_y = app.cursor_y;
        app.move_cursor_to_mouse_position(3, 2); // In row label area
        assert_eq!(app.cursor_x, prev_x, "Cursor should not move when clicking in row label area");
        assert_eq!(app.cursor_y, prev_y, "Cursor should not move when clicking in row label area");
        
        // Test clicking in header area (should be ignored)
        let prev_x = app.cursor_x;
        let prev_y = app.cursor_y;
        app.move_cursor_to_mouse_position(5, 1); // In header area
        assert_eq!(app.cursor_x, prev_x, "Cursor should not move when clicking in header area");
        assert_eq!(app.cursor_y, prev_y, "Cursor should not move when clicking in header area");
        
        // Test clicking on CH header row (y=6, should be ignored)
        let prev_x = app.cursor_x;
        let prev_y = app.cursor_y;
        app.move_cursor_to_mouse_position(5, 6); // CH header row
        assert_eq!(app.cursor_x, prev_x, "Cursor should not move when clicking on CH header row");
        assert_eq!(app.cursor_y, prev_y, "Cursor should not move when clicking on CH header row");
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
        assert_eq!(app.values, values_before, "Values should not be modified by play_current_tone");
        assert_eq!(app.cursor_x, cursor_x_before, "Cursor X should not be modified by play_current_tone");
        assert_eq!(app.cursor_y, cursor_y_before, "Cursor Y should not be modified by play_current_tone");
        
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
        assert_eq!(app.cursor_x, CH_PARAM_FB, "Cursor X should move to FB column");
        assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should move to CH row");
        
        // Verify FB value increased
        assert_eq!(app.values[ROW_CH][CH_PARAM_FB], 4, "FB should increase from 3 to 4");
        
        // Test boundary: increase at max should not exceed
        app.values[ROW_CH][CH_PARAM_FB] = CH_PARAM_MAX[CH_PARAM_FB]; // Set to max (7)
        app.increase_fb();
        assert_eq!(app.values[ROW_CH][CH_PARAM_FB], CH_PARAM_MAX[CH_PARAM_FB], "FB should not exceed max value (7)");
        
        // Verify cursor still at FB position
        assert_eq!(app.cursor_x, CH_PARAM_FB, "Cursor X should remain at FB column");
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
        assert_eq!(app.cursor_x, CH_PARAM_FB, "Cursor X should move to FB column");
        assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should move to CH row");
        
        // Verify FB value decreased
        assert_eq!(app.values[ROW_CH][CH_PARAM_FB], 4, "FB should decrease from 5 to 4");
        
        // Test boundary: decrease at 0 should not go negative
        app.values[ROW_CH][CH_PARAM_FB] = 0;
        app.decrease_fb();
        assert_eq!(app.values[ROW_CH][CH_PARAM_FB], 0, "FB should not go below 0");
        
        // Verify cursor still at FB position
        assert_eq!(app.cursor_x, CH_PARAM_FB, "Cursor X should remain at FB column");
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
        assert_eq!(app.cursor_x, CH_PARAM_FB, "Cursor X should move to FB column from operator row");
        assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should move to CH row from operator row");
        
        // Verify FB value increased
        assert_eq!(app.values[ROW_CH][CH_PARAM_FB], 3, "FB should increase from 2 to 3");
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
        assert_eq!(app.cursor_x, CH_PARAM_FB, "Cursor X should move to FB column from operator row");
        assert_eq!(app.cursor_y, ROW_CH, "Cursor Y should move to CH row from operator row");
        
        // Verify FB value decreased
        assert_eq!(app.values[ROW_CH][CH_PARAM_FB], 5, "FB should decrease from 6 to 5");
    }
