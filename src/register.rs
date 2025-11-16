use std::io;
use crate::models::*;
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

/// Convert tone data to YM2151 register events
/// This generates register writes for the YM2151 chip based on the current tone parameters
pub fn to_ym2151_events(values: &ToneData) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    // YM2151 Register Map:
    // $08: Key On/Off - Note on/off control
    // $20-$27: RL, FB, CON (channel 0-7) - Algorithm/Feedback
    // $28-$2F: KC (Key Code) - Note frequency
    // $30-$37: KF (Key Fraction) - Fine frequency
    // $38-$3F: PMS, AMS (Phase/Amplitude Modulation Sensitivity)
    // $40-$5F: DT1, MUL (Detune/Multiply) - 4 operators x 8 channels
    // $60-$7F: TL (Total Level) - 4 operators x 8 channels
    // $80-$9F: KS, AR (Key Scale/Attack Rate) - 4 operators x 8 channels
    // $A0-$BF: AMS-EN, D1R (Decay 1 Rate) - 4 operators x 8 channels
    // $C0-$DF: DT2, D2R (Decay 2 Rate) - 4 operators x 8 channels
    // $E0-$FF: D1L, RR (Decay 1 Level/Release Rate) - 4 operators x 8 channels

    // We'll use channel 0 for this example
    let channel = 0;

    // For each of 4 operators (M1, M2, C1, C2 in YM2151 terminology)
    // We map our OP1-OP4 to operators
    for op in 0..4 {
        let op_offset = op * 8 + channel; // Operator offset in register map
        
        // DT1 (bits 6-4) and MUL (bits 3-0) - Register $40-$5F
        let dt = values[op][PARAM_DT];
        let mul = values[op][PARAM_MUL];
        let dt_mul = ((dt & 0x07) << 4) | (mul & 0x0F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x40 + op_offset),
            data: format!("0x{:02X}", dt_mul),
        });

        // TL (Total Level) - Register $60-$7F (7 bits)
        let tl = values[op][PARAM_TL];
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x60 + op_offset),
            data: format!("0x{:02X}", tl & 0x7F),
        });

        // KS (bits 7-6) and AR (bits 4-0) - Register $80-$9F
        let ks = values[op][PARAM_KS];
        let ar = values[op][PARAM_AR];
        let ks_ar = ((ks & 0x03) << 6) | (ar & 0x1F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x80 + op_offset),
            data: format!("0x{:02X}", ks_ar),
        });

        // AMS (bits 7-6) and D1R (bits 4-0) - Register $A0-$BF
        let ams = values[op][PARAM_AMS];
        let d1r = values[op][PARAM_D1R];
        let ams_d1r = ((ams & 0x03) << 6) | (d1r & 0x1F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0xA0 + op_offset),
            data: format!("0x{:02X}", ams_d1r),
        });

        // DT2 (bits 7-6) and D2R (bits 3-0) - Register $C0-$DF
        let dt2 = values[op][PARAM_DT2];
        let d2r = values[op][PARAM_D2R];
        let dt2_d2r = ((dt2 & 0x03) << 6) | (d2r & 0x0F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0xC0 + op_offset),
            data: format!("0x{:02X}", dt2_d2r),
        });

        // D1L (bits 7-4) and RR (bits 3-0) - Register $E0-$FF
        let d1l = values[op][PARAM_D1L];
        let rr = values[op][PARAM_RR];
        let d1l_rr = ((d1l & 0x0F) << 4) | (rr & 0x0F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0xE0 + op_offset),
            data: format!("0x{:02X}", d1l_rr),
        });
    }

    // Channel settings: RL, FB, CON (Algorithm) - Register $20-$27
    // Use ALG and FB from CH row
    let alg = values[ROW_CH][CH_PARAM_ALG];
    let fb = values[ROW_CH][CH_PARAM_FB];
    let rl = 0xC0; // Both L and R enabled
    let rl_fb_con = rl | ((fb & 0x07) << 3) | (alg & 0x07);
    events.push(Ym2151Event {
        time: 0,
        addr: format!("0x{:02X}", 0x20 + channel),
        data: format!("0x{:02X}", rl_fb_con),
    });
    
    // Key Code (KC) and Key Fraction (KF) - Use MIDI note number from CH row
    let midi_note = values[ROW_CH][CH_PARAM_NOTE];
    let (kc, kf) = midi_to_kc_kf(midi_note);
    
    // Key Code (KC) - Register $28-$2F
    events.push(Ym2151Event {
        time: 0,
        addr: format!("0x{:02X}", 0x28 + channel),
        data: format!("0x{:02X}", kc),
    });
    
    // Key Fraction (KF) - Register $30-$37 - Fine frequency adjust
    events.push(Ym2151Event {
        time: 0,
        addr: format!("0x{:02X}", 0x30 + channel),
        data: format!("0x{:02X}", kf),
    });
    
    // Note On - Register $08 - Key On with operators based on slot masks
    // Bits 0-2: Channel (0-7)
    // Bits 3-6: Operator enable (M1=bit3, M2=bit4, C1=bit5, C2=bit6)
    // Use slot masks from CH row to determine which operators to enable
    let op1_mask = values[ROW_CH][CH_PARAM_OP1_MASK];
    let op2_mask = values[ROW_CH][CH_PARAM_OP2_MASK];
    let op3_mask = values[ROW_CH][CH_PARAM_OP3_MASK];
    let op4_mask = values[ROW_CH][CH_PARAM_OP4_MASK];
    
    let key_on_data = ((op1_mask & 1) << 3) | ((op2_mask & 1) << 4) 
                    | ((op3_mask & 1) << 5) | ((op4_mask & 1) << 6) | (channel as u8);
    events.push(Ym2151Event {
        time: 0,
        addr: "0x08".to_string(),
        data: format!("0x{:02X}", key_on_data),
    });

    events
}

/// Convert YM2151 events back to tone data
pub fn events_to_tone_data(events: &[Ym2151Event]) -> io::Result<ToneData> {
    let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];

    for event in events {
        // Parse address and data
        let addr = u8::from_str_radix(event.addr.trim_start_matches("0x"), 16)
            .map_err(io::Error::other)?;
        let data = u8::from_str_radix(event.data.trim_start_matches("0x"), 16)
            .map_err(io::Error::other)?;

        // Decode based on register address range
        match addr {
            // DT1/MUL registers (0x40-0x5F)
            0x40..=0x5F => {
                let op = ((addr - 0x40) / 8) as usize;
                if op < 4 {
                    values[op][PARAM_DT] = (data >> 4) & 0x07;
                    values[op][PARAM_MUL] = data & 0x0F;
                }
            }
            // TL registers (0x60-0x7F)
            0x60..=0x7F => {
                let op = ((addr - 0x60) / 8) as usize;
                if op < 4 {
                    values[op][PARAM_TL] = data & 0x7F;
                }
            }
            // KS/AR registers (0x80-0x9F)
            0x80..=0x9F => {
                let op = ((addr - 0x80) / 8) as usize;
                if op < 4 {
                    values[op][PARAM_KS] = (data >> 6) & 0x03;
                    values[op][PARAM_AR] = data & 0x1F;
                }
            }
            // AMS-EN/D1R registers (0xA0-0xBF)
            0xA0..=0xBF => {
                let op = ((addr - 0xA0) / 8) as usize;
                if op < 4 {
                    values[op][PARAM_AMS] = (data >> 6) & 0x03;
                    values[op][PARAM_D1R] = data & 0x1F;
                }
            }
            // DT2/D2R registers (0xC0-0xDF)
            0xC0..=0xDF => {
                let op = ((addr - 0xC0) / 8) as usize;
                if op < 4 {
                    values[op][PARAM_DT2] = (data >> 6) & 0x03;
                    values[op][PARAM_D2R] = data & 0x0F;
                }
            }
            // D1L/RR registers (0xE0-0xFF)
            0xE0..=0xFF => {
                let op = ((addr - 0xE0) / 8) as usize;
                if op < 4 {
                    values[op][PARAM_D1L] = (data >> 4) & 0x0F;
                    values[op][PARAM_RR] = data & 0x0F;
                }
            }
            // RL/FB/CON register (0x20-0x27)
            0x20..=0x27 => {
                // This register contains RL (bit 7-6), FB (bit 5-3), and CON/ALG (bit 2-0)
                // Extract ALG and FB to CH row
                values[ROW_CH][CH_PARAM_ALG] = data & 0x07; // ALG is bits 0-2
                values[ROW_CH][CH_PARAM_FB] = (data >> 3) & 0x07; // FB is bits 3-5
            }
            // Key On register (0x08)
            0x08 => {
                // Bits 3-6 contain operator enable flags
                // Bit 3: OP1, Bit 4: OP2, Bit 5: OP3, Bit 6: OP4
                values[ROW_CH][CH_PARAM_OP1_MASK] = (data >> 3) & 0x01;
                values[ROW_CH][CH_PARAM_OP2_MASK] = (data >> 4) & 0x01;
                values[ROW_CH][CH_PARAM_OP3_MASK] = (data >> 5) & 0x01;
                values[ROW_CH][CH_PARAM_OP4_MASK] = (data >> 6) & 0x01;
            }
            // KC (Key Code) register (0x28-0x2F)
            0x28..=0x2F => {
                // Convert KC back to MIDI note number
                values[ROW_CH][CH_PARAM_NOTE] = kc_to_midi_note(data);
            }
            _ => {}
        }
    }

    Ok(values)
}

/// Convert tone data to JSON string in ym2151-log-play-server format
pub fn to_json_string(values: &ToneData) -> Result<String, serde_json::Error> {
    let events = to_ym2151_events(values);
    let log = Ym2151Log {
        event_count: events.len(),
        events,
    };
    serde_json::to_string_pretty(&log)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ym2151_events() {
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Set some test values
        values[0][PARAM_MUL] = 1;
        values[0][PARAM_TL] = 20;
        values[ROW_CH][CH_PARAM_ALG] = 4;
        values[ROW_CH][CH_PARAM_FB] = 0;
        values[ROW_CH][CH_PARAM_OP1_MASK] = 1;
        values[ROW_CH][CH_PARAM_OP2_MASK] = 1;
        values[ROW_CH][CH_PARAM_OP3_MASK] = 1;
        values[ROW_CH][CH_PARAM_OP4_MASK] = 1;
        
        let events = to_ym2151_events(&values);
        
        // Should have events for:
        // - 4 operators × 6 registers = 24 events
        // - 1 channel register (RL/FB/CON)
        // - 1 Key Code register
        // - 1 Key Fraction register  
        // - 1 Note On register
        // Total = 28 events
        assert_eq!(events.len(), 28);
        
        // Check that events have correct format
        for event in &events {
            assert_eq!(event.time, 0);
            assert!(event.addr.starts_with("0x"));
            assert!(event.data.starts_with("0x"));
        }
        
        // Verify note on event is present
        let note_on_event = events.iter().find(|e| e.addr == "0x08");
        assert!(note_on_event.is_some(), "Note on event should be present");
    }

    #[test]
    fn test_events_to_tone_data() {
        // Create sample events
        let events = vec![
            Ym2151Event {
                time: 0,
                addr: "0x40".to_string(),
                data: "0x12".to_string(), // DT=1, MUL=2
            },
            Ym2151Event {
                time: 0,
                addr: "0x60".to_string(),
                data: "0x1F".to_string(), // TL=31
            },
            Ym2151Event {
                time: 0,
                addr: "0x80".to_string(),
                data: "0x8A".to_string(), // KS=2, AR=10
            },
            Ym2151Event {
                time: 0,
                addr: "0xA0".to_string(),
                data: "0x0C".to_string(), // D1R=12
            },
            Ym2151Event {
                time: 0,
                addr: "0xC0".to_string(),
                data: "0x85".to_string(), // DT2=2, D2R=5
            },
            Ym2151Event {
                time: 0,
                addr: "0xE0".to_string(),
                data: "0x78".to_string(), // D1L=7, RR=8
            },
        ];

        let result = events_to_tone_data(&events);
        assert!(result.is_ok());

        let values = result.unwrap();
        
        // Check operator 1 values
        assert_eq!(values[0][PARAM_DT], 1);
        assert_eq!(values[0][PARAM_MUL], 2);
        assert_eq!(values[0][PARAM_TL], 31);
        assert_eq!(values[0][PARAM_KS], 2);
        assert_eq!(values[0][PARAM_AR], 10);
        assert_eq!(values[0][PARAM_D1R], 12);
        assert_eq!(values[0][PARAM_D1L], 7);
        assert_eq!(values[0][PARAM_D2R], 5);
        assert_eq!(values[0][PARAM_RR], 8);
        assert_eq!(values[0][PARAM_DT2], 2);
    }

    #[test]
    fn test_json_serialization() {
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        values[0][PARAM_MUL] = 1;
        values[ROW_CH][CH_PARAM_ALG] = 4;
        
        let json_result = to_json_string(&values);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        assert!(json_string.contains("event_count"));
        assert!(json_string.contains("events"));
    }

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

    #[test]
    fn test_to_ym2151_events_with_midi_note() {
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Set MIDI note to 72 (C5)
        values[ROW_CH][CH_PARAM_NOTE] = 72;
        values[ROW_CH][CH_PARAM_ALG] = 4;
        values[ROW_CH][CH_PARAM_OP1_MASK] = 1;
        values[ROW_CH][CH_PARAM_OP2_MASK] = 1;
        values[ROW_CH][CH_PARAM_OP3_MASK] = 1;
        values[ROW_CH][CH_PARAM_OP4_MASK] = 1;
        
        let events = to_ym2151_events(&values);
        
        // Find KC event (register 0x28)
        let kc_event = events.iter().find(|e| e.addr == "0x28");
        assert!(kc_event.is_some(), "KC event should be present");
        
        // Verify KC value corresponds to MIDI note 72 (C5)
        let kc_data = kc_event.unwrap().data.trim_start_matches("0x");
        let kc = u8::from_str_radix(kc_data, 16).unwrap();
        // According to smf-to-ym2151log-rust: MIDI 72 (C5) -> KC 0x4E
        assert_eq!(kc, 0x4E, "MIDI note 72 (C5) should map to KC 0x4E");
    }

    #[test]
    fn test_events_to_tone_data_with_kc() {
        // Create events with KC register
        // Using 0x3E which is KC for middle C (MIDI 60)
        let events = vec![
            Ym2151Event {
                time: 0,
                addr: "0x28".to_string(),
                data: "0x3E".to_string(), // KC for middle C (MIDI 60)
            },
        ];
        
        let result = events_to_tone_data(&events);
        assert!(result.is_ok());
        
        let values = result.unwrap();
        
        // Check that MIDI note was extracted
        assert_eq!(values[ROW_CH][CH_PARAM_NOTE], 60, "KC 0x3E should convert to MIDI note 60");
    }
}