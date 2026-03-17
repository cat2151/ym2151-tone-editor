//! Unit tests for event_loop module

use crate::event_loop::key_to_string;
use crossterm::event::{KeyCode, KeyModifiers};

#[test]
fn test_space_maps_to_space_string() {
    let result = key_to_string(KeyCode::Char(' '), KeyModifiers::NONE);
    assert_eq!(result, Some("Space".to_string()));
}

#[test]
fn test_shift_space_maps_to_space_string() {
    let result = key_to_string(KeyCode::Char(' '), KeyModifiers::SHIFT);
    assert_eq!(result, Some("Space".to_string()));
}

#[test]
fn test_regular_char_maps_to_itself() {
    let result = key_to_string(KeyCode::Char('a'), KeyModifiers::NONE);
    assert_eq!(result, Some("a".to_string()));
}

#[test]
fn test_function_key_f5_maps_to_f5_string() {
    let result = key_to_string(KeyCode::F(5), KeyModifiers::NONE);
    assert_eq!(result, Some("F5".to_string()));
}

#[test]
fn test_function_key_maps_generically() {
    let result = key_to_string(KeyCode::F(1), KeyModifiers::NONE);
    assert_eq!(result, Some("F1".to_string()));
    let result = key_to_string(KeyCode::F(12), KeyModifiers::NONE);
    assert_eq!(result, Some("F12".to_string()));
}

#[test]
fn test_question_mark_shift_slash_maps_to_question_mark() {
    // On most keyboard layouts, '?' is Shift+/ and crossterm delivers it as Char('?') with SHIFT
    let result = key_to_string(KeyCode::Char('?'), KeyModifiers::SHIFT);
    assert_eq!(result, Some("?".to_string()));
}

#[test]
fn test_question_mark_no_modifier_maps_to_question_mark() {
    let result = key_to_string(KeyCode::Char('?'), KeyModifiers::NONE);
    assert_eq!(result, Some("?".to_string()));
}
