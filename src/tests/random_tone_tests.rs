//! Unit tests for randomize_tone() in app module

use crate::app::App;
use crate::models::DEFAULT_ENVELOPE_DELAY_SECONDS;
use crate::models::*;

#[test]
fn test_randomize_tone_values_in_range() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);
    app.randomize_tone();

    // ALG in range
    assert!(app.values[ROW_CH][CH_PARAM_ALG] <= 7, "ALG should be <= 7");

    // FB in range
    assert!(app.values[ROW_CH][CH_PARAM_FB] <= 7, "FB should be <= 7");

    for (op, row) in app.values.iter().take(4).enumerate() {
        assert_eq!(row[PARAM_SM], 1, "SM should be 1 for op {}", op);
        assert!(row[PARAM_TL] <= 99, "TL should be <= 99 for op {}", op);
        assert!(row[PARAM_AR] >= 5, "AR should be >= 5 for op {}", op);
        assert!(row[PARAM_AR] <= 31, "AR should be <= 31 for op {}", op);
        assert!(row[PARAM_D1R] <= 9, "D1R should be <= 9 for op {}", op);
        assert_eq!(row[PARAM_D1L], 15, "D1L should be 15 for op {}", op);
        assert_eq!(row[PARAM_D2R], 0, "D2R should be 0 for op {}", op);
        assert_eq!(row[PARAM_RR], 0, "RR should be 0 for op {}", op);
        assert!(row[PARAM_DT] <= 7, "DT should be <= 7 for op {}", op);
        assert_eq!(row[PARAM_DT2], 0, "DT2 should be 0 for op {}", op);
        assert!(row[PARAM_KS] <= 3, "KS should be <= 3 for op {}", op);
        assert_eq!(row[PARAM_AMS], 0, "AMS should be 0 for op {}", op);
    }
}

#[test]
fn test_randomize_tone_preserves_note() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);
    let original_note = 72u8; // C5
    app.values[ROW_CH][CH_PARAM_NOTE] = original_note;
    app.randomize_tone();

    assert_eq!(
        app.values[ROW_CH][CH_PARAM_NOTE], original_note,
        "Note should be preserved after randomize_tone"
    );
}

#[test]
fn test_randomize_tone_cursor_unchanged() {
    let mut app = App::new(false, false, DEFAULT_ENVELOPE_DELAY_SECONDS);
    app.cursor_x = 3;
    app.cursor_y = 1;
    app.randomize_tone();

    assert_eq!(app.cursor_x, 3, "cursor_x should be unchanged");
    assert_eq!(app.cursor_y, 1, "cursor_y should be unchanged");
}
