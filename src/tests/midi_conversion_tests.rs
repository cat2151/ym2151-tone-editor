//! Unit tests for midi_conversion module

use crate::midi_conversion::*;

#[test]
fn test_midi_to_kc_kf_middle_c() {
    // MIDI note 60 = middle C (C4)
    let (kc, _kf) = midi_to_kc_kf(60);

    // According to smf-to-ym2151log-rust library:
    // MIDI 60 should map to KC = 0x2E (octave 2, note C = 14/0xE)
    assert_eq!(kc, 0x2E, "MIDI note 60 should map to KC 0x2E");
}

#[test]
fn test_midi_to_kc_kf_various_notes() {
    // Test A4 (MIDI note 69)
    let (kc, _) = midi_to_kc_kf(69);
    assert_eq!(kc, 0x3A, "A4 (MIDI 69) should map to KC 0x3A");

    // Test C5 (MIDI note 72)
    let (kc, _) = midi_to_kc_kf(72);
    assert_eq!(kc, 0x3E, "C5 (MIDI 72) should map to KC 0x3E");

    // Test C3 (MIDI note 48)
    let (kc, _) = midi_to_kc_kf(48);
    assert_eq!(kc, 0x1E, "C3 (MIDI 48) should map to KC 0x1E");
}

#[test]
fn test_midi_to_kc_kf_boundary_values() {
    // Test minimum MIDI note
    let (kc, _) = midi_to_kc_kf(0);
    assert!(kc <= 0x7F, "KC should be within valid range");
    let octave = (kc >> 4) & 0x07;
    assert_eq!(octave, 0, "MIDI note 0 should clamp to octave 0");

    // Test maximum MIDI note
    let (kc, _) = midi_to_kc_kf(127);
    assert!(kc <= 0x7F, "KC should be within valid range");
    let octave = (kc >> 4) & 0x07;
    assert_eq!(octave, 7, "MIDI note 127 should clamp to octave 7");
}

#[test]
fn test_kc_to_midi_note_middle_c() {
    // KC for middle C from the library is 0x2E
    let kc = 0x2E;
    let midi_note = kc_to_midi_note(kc);

    // Should convert back to 60 (middle C)
    assert_eq!(midi_note, 60, "KC 0x2E should convert to MIDI note 60");
}

#[test]
fn test_midi_note_roundtrip() {
    // Test that converting MIDI -> KC -> MIDI gives the same value
    // Note: MIDI notes below 36 get clamped to YM2151 octave 0, so roundtrip only works for MIDI >= 36
    for midi_in in [36, 48, 60, 69, 72, 84, 96] {
        let (kc, _) = midi_to_kc_kf(midi_in);
        let midi_out = kc_to_midi_note(kc);

        assert_eq!(
            midi_in, midi_out,
            "MIDI note {} -> KC 0x{:02X} -> MIDI note {} should roundtrip exactly",
            midi_in, kc, midi_out
        );
    }
}
