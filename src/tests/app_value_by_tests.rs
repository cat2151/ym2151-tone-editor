//! Unit tests for app module - bulk value changes and operator jump shortcuts

use crate::app::*;
use crate::models::DEFAULT_ENVELOPE_DELAY_SECONDS;
use crate::models::*;

#[test]
fn test_increase_value_by() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Start at a different position
    app.cursor_x = 2;
    app.cursor_y = 0; // O1/M1
    app.values[1][2] = 5; // Set OP2 (O2/M2 - row 1) column 2 to 5

    // Jump to OP2 (O2/M2 - row 1) and increase
    app.jump_to_operator_and_increase(1);

    // Verify cursor moved to OP2 (O2/M2)
    assert_eq!(app.cursor_y, 1, "Cursor should move to OP2 row (O2/M2)");
    assert_eq!(app.cursor_x, 2, "Cursor column should be preserved");

    // Verify value increased
    assert_eq!(
        app.values[1][2], 6,
        "Value at OP2 column 2 should increase from 5 to 6"
    );
}

#[test]
fn test_jump_to_op3_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Start at CH row
    app.cursor_x = 1;
    app.cursor_y = ROW_CH;
    app.values[2][1] = 20; // Set OP3 (O3/C1 - row 2) column 1 to 20

    // Jump to OP3 (O3/C1 - row 2) and increase
    app.jump_to_operator_and_increase(2);

    // Verify cursor moved to OP3 (O3/C1)
    assert_eq!(app.cursor_y, 2, "Cursor should move to OP3 row (O3/C1)");
    assert_eq!(app.cursor_x, 1, "Cursor column should be preserved");

    // Verify value increased
    assert_eq!(
        app.values[2][1], 21,
        "Value at OP3 column 1 should increase from 20 to 21"
    );
}

#[test]
fn test_jump_to_op4_and_increase() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Start at a different position
    app.cursor_x = 6;
    app.cursor_y = 2; // O3/C1
    app.values[1][6] = 8; // Set OP2 (O2/M2 - row 1) column 6 to 8

    // Jump to OP2 (O2/M2 - row 1) and decrease
    app.jump_to_operator_and_decrease(1);

    // Verify cursor moved to OP2 (O2/M2)
    assert_eq!(app.cursor_y, 1, "Cursor should move to OP2 row (O2/M2)");
    assert_eq!(app.cursor_x, 6, "Cursor column should be preserved");

    // Verify value decreased
    assert_eq!(
        app.values[1][6], 7,
        "Value at OP2 column 6 should decrease from 8 to 7"
    );
}

#[test]
fn test_jump_and_increase_clamps_to_max() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

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
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);

    // Set initial values for column 3 on all operators
    app.values[0][3] = 10; // OP1 (O1/M1 - row 0)
    app.values[1][3] = 15; // OP2 (O2/M2 - row 1)
    app.values[2][3] = 20; // OP3 (O3/C1 - row 2)
    app.values[3][3] = 25; // OP4 (O4/C2 - row 3)

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
    assert_eq!(app.values[1][3], 16, "OP2 value should increase");

    // Jump to OP3 and decrease
    app.jump_to_operator_and_decrease(2);
    assert_eq!(app.cursor_y, 2, "Should jump to OP3");
    assert_eq!(app.values[2][3], 19, "OP3 value should decrease");
}
