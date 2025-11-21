//! Unit tests for ui module

use crate::models::*;
use crate::ui::*;
use ratatui::style::Color;

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
        alg0[0].contains("M1->C1->M2->C2->OUT"),
        "ALG 0 should show cascade"
    );

    let alg7 = get_algorithm_diagram(7);
    assert!(alg7.len() >= 5, "ALG 7 should have at least 5 lines");
    assert!(
        alg7[0].contains("M1->OUT"),
        "ALG 7 should show M1 to output"
    );

    // Test invalid algorithm
    let invalid = get_algorithm_diagram(8);
    assert_eq!(invalid.len(), 1);
    assert_eq!(invalid[0], "Invalid ALG");
}
