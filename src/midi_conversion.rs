//! MIDI to YM2151 pitch conversion utilities
//!
//! This module handles bidirectional conversion between MIDI note numbers
//! and YM2151 Key Code (KC) / Key Fraction (KF) values.

// Import MIDI to KC/KF conversion from smf-to-ym2151log-rust library
pub use smf_to_ym2151log::midi::midi_to_kc_kf;
use smf_to_ym2151log::ym2151::note_table::NOTE_TABLE;

/// Convert YM2151 KC (Key Code) to approximate MIDI note number
///
/// This is an approximate reverse conversion since YM2151 has finer
/// pitch resolution than MIDI's semitone-based system
pub fn kc_to_midi_note(kc: u8) -> u8 {
    // Extract octave (bits 6-4) and note value (bits 3-0)
    let ym_octave = (kc >> 4) & 0x07;
    let ym_note = kc & 0x0F;

    let note_in_octave = NOTE_TABLE.iter().position(|&n| n == ym_note).unwrap_or(0) as u8;
    let adjusted_midi = (ym_octave + 2) * 12 + note_in_octave;

    (adjusted_midi + 1).min(127)
}
