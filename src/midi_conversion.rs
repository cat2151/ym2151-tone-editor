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
    let octave = (kc >> 4) & 0x07;
    let ym_note = kc & 0x0F;
    
    // Find which note in NOTE_TABLE matches ym_note
    let note_in_octave = NOTE_TABLE.iter()
        .position(|&n| n == ym_note)
        .unwrap_or(0) as u8;
    
    // Reverse the adjustment: add 1 back, then calculate MIDI note
    // Formula from midi_to_kc_kf: adjusted_midi = midi_note - 1
    // So: midi_note = adjusted_midi + 1
    let adjusted_midi = (octave + 1) * 12 + note_in_octave;
    let midi_note = (adjusted_midi + 1).min(127);
    
    midi_note
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_to_kc_kf_middle_c() {
        // MIDI note 60 = middle C (C4)
        let (kc, _kf) = midi_to_kc_kf(60);
        
        // According to smf-to-ym2151log-rust library:
        // MIDI 60 should map to KC = 0x3E (octave 3, note C = 14/0xE)
        assert_eq!(kc, 0x3E, "MIDI note 60 should map to KC 0x3E");
    }

    #[test]
    fn test_midi_to_kc_kf_various_notes() {
        // Test A4 (MIDI note 69)
        let (kc, _) = midi_to_kc_kf(69);
        assert_eq!(kc, 0x4A, "A4 (MIDI 69) should map to KC 0x4A");
        
        // Test C5 (MIDI note 72)
        let (kc, _) = midi_to_kc_kf(72);
        assert_eq!(kc, 0x4E, "C5 (MIDI 72) should map to KC 0x4E");
        
        // Test C3 (MIDI note 48)
        let (kc, _) = midi_to_kc_kf(48);
        assert_eq!(kc, 0x2E, "C3 (MIDI 48) should map to KC 0x2E");
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
        // KC for middle C from the library is 0x3E
        let kc = 0x3E;
        let midi_note = kc_to_midi_note(kc);
        
        // Should convert back to 60 (middle C)
        assert_eq!(midi_note, 60, "KC 0x3E should convert to MIDI note 60");
    }

    #[test]
    fn test_midi_note_roundtrip() {
        // Test that converting MIDI -> KC -> MIDI gives the same value
        for midi_in in [24, 36, 48, 60, 69, 72, 84, 96] {
            let (kc, _) = midi_to_kc_kf(midi_in);
            let midi_out = kc_to_midi_note(kc);
            
            assert_eq!(midi_in, midi_out, 
                "MIDI note {} -> KC 0x{:02X} -> MIDI note {} should roundtrip exactly", 
                midi_in, kc, midi_out);
        }
    }
}
