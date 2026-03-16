//! Unit tests for ui module

use crate::models::*;
use crate::ui::*;
use ratatui::style::Color;

#[test]
fn test_get_key_guide() {
    // Test that all parameters with jump keybindings return the correct letter
    assert_eq!(get_key_guide(PARAM_SM), Some('O'));
    assert_eq!(get_key_guide(PARAM_TL), Some('T'));
    assert_eq!(get_key_guide(PARAM_MUL), Some('M'));
    assert_eq!(get_key_guide(PARAM_AR), Some('A'));
    assert_eq!(get_key_guide(PARAM_D1R), Some('D'));
    assert_eq!(get_key_guide(PARAM_D1L), Some('L'));
    assert_eq!(get_key_guide(PARAM_D2R), Some('S'));
    assert_eq!(get_key_guide(PARAM_RR), Some('R'));
    assert_eq!(get_key_guide(PARAM_DT), Some('U'));
    assert_eq!(get_key_guide(PARAM_DT2), Some('N'));
    assert_eq!(get_key_guide(PARAM_KS), Some('K'));
    assert_eq!(get_key_guide(PARAM_AMS), Some('I'));
}

#[test]
fn test_get_operator_guide() {
    // Test that operator rows return the correct operator number
    assert_eq!(get_operator_guide(0), Some('1')); // O1/M1
    assert_eq!(get_operator_guide(1), Some('2')); // O2/M2
    assert_eq!(get_operator_guide(2), Some('3')); // O3/C1
    assert_eq!(get_operator_guide(3), Some('4')); // O4/C2

    // Test that CH row returns None
    assert_eq!(get_operator_guide(4), None);
    assert_eq!(get_operator_guide(ROW_CH), None);

    // Test out of bounds
    assert_eq!(get_operator_guide(5), None);
}

#[test]
fn test_get_ch_key_guide() {
    // Test that CH row parameters with keybindings return the correct letter
    assert_eq!(get_ch_key_guide(CH_PARAM_ALG), Some('G')); // 'g'/'G' for ALG
    assert_eq!(get_ch_key_guide(CH_PARAM_FB), Some('F')); // 'f'/'F' for FB

    // Test that Note parameter returns None (no keybinding)
    assert_eq!(get_ch_key_guide(CH_PARAM_NOTE), None);

    // Test out of bounds
    assert_eq!(get_ch_key_guide(3), None);
}

#[test]
fn test_get_param_color() {
    // Test operator row colors
    assert_eq!(get_param_color(PARAM_MUL, false), Color::Green);

    // Test TL and D1L are cyan (light blue)
    assert_eq!(get_param_color(PARAM_TL, false), Color::Cyan);
    assert_eq!(get_param_color(PARAM_D1L, false), Color::Cyan);

    // Test envelope parameters are orange
    assert_eq!(get_param_color(PARAM_AR, false), Color::Rgb(255, 165, 0));
    assert_eq!(get_param_color(PARAM_D1R, false), Color::Rgb(255, 165, 0));
    assert_eq!(get_param_color(PARAM_D2R, false), Color::Rgb(255, 165, 0));
    assert_eq!(get_param_color(PARAM_RR, false), Color::Rgb(255, 165, 0));

    // Test other parameters are white
    assert_eq!(get_param_color(PARAM_DT, false), Color::White);
    assert_eq!(get_param_color(PARAM_KS, false), Color::White);
    assert_eq!(get_param_color(PARAM_DT2, false), Color::White);
    assert_eq!(get_param_color(PARAM_AMS, false), Color::White);
    assert_eq!(get_param_color(PARAM_SM, false), Color::White);

    // Test CH row colors - ALG and FB should be green
    assert_eq!(get_param_color(CH_PARAM_ALG, true), Color::Green);
    assert_eq!(get_param_color(CH_PARAM_FB, true), Color::Green);
    assert_eq!(get_param_color(CH_PARAM_NOTE, true), Color::White);
}

#[test]
fn test_get_algorithm_diagram() {
    // Test that each algorithm returns a diagram
    for alg in 0..=7 {
        let diagram = get_algorithm_diagram(alg);
        assert!(
            !diagram.is_empty(),
            "Algorithm {} should have a diagram",
            alg
        );
        assert!(
            diagram[0].starts_with("ALG "),
            "First line should start with 'ALG '"
        );
    }

    // Test specific algorithms
    let alg0 = get_algorithm_diagram(0);
    assert!(
        alg0[0].contains("O1->O2->O3->O4->OUT"),
        "ALG 0 should show cascade"
    );

    let alg7 = get_algorithm_diagram(7);
    assert!(alg7.len() >= 5, "ALG 7 should have at least 5 lines");
    assert!(
        alg7[0].contains("O1->OUT"),
        "ALG 7 should show O1 to output"
    );

    // Test invalid algorithm
    let invalid = get_algorithm_diagram(8);
    assert_eq!(invalid.len(), 1);
    assert_eq!(invalid[0], "Invalid ALG");
}

// ---------------------------------------------------------------------------
// compute_op_envelope_points tests
// ---------------------------------------------------------------------------

/// Build a zeroed `[u8; GRID_WIDTH]` row, then override specific indices.
fn make_op_row(ar: u8, d1r: u8, d1l: u8, d2r: u8, rr: u8, tl: u8) -> [u8; GRID_WIDTH] {
    let mut row = [0u8; GRID_WIDTH];
    row[PARAM_AR] = ar;
    row[PARAM_D1R] = d1r;
    row[PARAM_D1L] = d1l;
    row[PARAM_D2R] = d2r;
    row[PARAM_RR] = rr;
    row[PARAM_TL] = tl;
    row
}

#[test]
fn test_envelope_points_length() {
    // compute_op_envelope_points always returns exactly 6 points
    let row = make_op_row(31, 0, 0, 0, 0, 0);
    let pts = compute_op_envelope_points(&row);
    assert_eq!(pts.len(), 6, "expected 6 envelope points");
}

#[test]
fn test_envelope_starts_and_ends_at_zero() {
    // First point is (0.0, 0.0) and last is (1.0, 0.0) for any parameters
    let row = make_op_row(20, 10, 5, 3, 8, 30);
    let pts = compute_op_envelope_points(&row);
    let (t0, l0) = pts[0];
    let (t_end, l_end) = *pts.last().unwrap();
    assert_eq!(t0, 0.0, "first time should be 0.0");
    assert_eq!(l0, 0.0, "first level should be 0.0");
    assert_eq!(t_end, 1.0, "last time should be 1.0");
    assert_eq!(l_end, 0.0, "last level should be 0.0");
}

#[test]
fn test_envelope_all_levels_in_range() {
    // All level values must be in [0.0, 1.0]
    for ar in [0u8, 15, 31] {
        for d1l in [0u8, 7, 15] {
            for tl in [0u8, 50, 99] {
                let row = make_op_row(ar, 10, d1l, 5, 5, tl);
                let pts = compute_op_envelope_points(&row);
                for (_, level) in &pts {
                    assert!(
                        *level >= 0.0 && *level <= 1.0,
                        "level {level} out of [0,1] for ar={ar} d1l={d1l} tl={tl}"
                    );
                }
            }
        }
    }
}

#[test]
fn test_envelope_tl99_near_silent() {
    // With TL=99 (near silent) the peak should be near 0
    let row = make_op_row(31, 31, 0, 0, 15, 99);
    let pts = compute_op_envelope_points(&row);
    // The second point is the attack peak
    let (_, peak) = pts[1];
    assert!(
        peak < 0.02,
        "TL=99 should produce a near-silent envelope, got peak={peak}"
    );
}

#[test]
fn test_envelope_ar0_gives_zero_peak() {
    // AR=0 means no attack; the peak (second point) should be 0.0
    let row = make_op_row(0, 0, 0, 0, 0, 0);
    let pts = compute_op_envelope_points(&row);
    let (_, peak) = pts[1];
    assert_eq!(peak, 0.0, "AR=0 should give zero attack peak");
}

#[test]
fn test_envelope_ar31_gives_full_peak_at_tl0() {
    // AR=31, TL=0 → peak should equal amplitude (≈ 1.0)
    let row = make_op_row(31, 0, 0, 0, 0, 0);
    let pts = compute_op_envelope_points(&row);
    let (_, peak) = pts[1];
    assert!(
        (peak - 1.0_f64).abs() < 1e-9,
        "AR=31 TL=0 should give peak ≈ 1.0, got {peak}"
    );
}

#[test]
fn test_envelope_d1l15_sustain_at_zero() {
    // D1L=15 means Decay1 targets silence; with D1R=31 (instant) the sustain level should be 0
    let row = make_op_row(31, 31, 15, 0, 0, 0);
    let pts = compute_op_envelope_points(&row);
    // pts[2] = end of Decay1
    let (_, level_after_d1) = pts[2];
    assert!(
        level_after_d1 < 1e-9,
        "D1L=15 D1R=31 should decay to silence after Decay1, got {level_after_d1}"
    );
}

#[test]
fn test_envelope_d1l0_no_decay() {
    // D1L=0 means no decay (sustain at full amplitude);
    // with AR=31 and any D1R, the level after Decay1 should equal the attack peak
    let row = make_op_row(31, 31, 0, 0, 0, 0);
    let pts = compute_op_envelope_points(&row);
    let (_, peak) = pts[1];
    let (_, level_after_d1) = pts[2];
    assert!(
        (level_after_d1 - peak).abs() < 1e-9,
        "D1L=0 should keep level at peak after Decay1: peak={peak} level_after_d1={level_after_d1}"
    );
}

#[test]
fn test_envelope_time_points_are_ascending() {
    // Time values must be non-decreasing
    let row = make_op_row(20, 15, 8, 5, 10, 20);
    let pts = compute_op_envelope_points(&row);
    for w in pts.windows(2) {
        let (t1, _) = w[0];
        let (t2, _) = w[1];
        assert!(
            t2 >= t1,
            "time points must be ascending: {t1} followed by {t2}"
        );
    }
}
