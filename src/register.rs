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

    // YM2151 hardware operator register order: M1, M2, C1, C2 (OP1, OP2, OP3, OP4)
    // We display as: M1, C1, M2, C2 (user-friendly order)
    // Internal data order: M1, M2, C1, C2 (rows 0, 1, 2, 3)
    // Mapping: Data M1(row0)→slot0, Data M2(row1)→slot1, Data C1(row2)→slot2, Data C2(row3)→slot3
    const DATA_ROW_TO_SLOT: [usize; 4] = [0, 1, 2, 3];
    
    // For each of 4 operators in data order (M1, M2, C1, C2)
    for data_row in 0..4 {
        let hw_slot = DATA_ROW_TO_SLOT[data_row]; // Convert data row to hardware slot
        let op_offset = hw_slot * 8 + channel; // Operator offset in register map
        
        // DT1 (bits 6-4) and MUL (bits 3-0) - Register $40-$5F
        let dt = values[data_row][PARAM_DT];
        let mul = values[data_row][PARAM_MUL];
        let dt_mul = ((dt & 0x07) << 4) | (mul & 0x0F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x40 + op_offset),
            data: format!("0x{:02X}", dt_mul),
        });

        // TL (Total Level) - Register $60-$7F (7 bits)
        let tl = values[data_row][PARAM_TL];
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x60 + op_offset),
            data: format!("0x{:02X}", tl & 0x7F),
        });

        // KS (bits 7-6) and AR (bits 4-0) - Register $80-$9F
        let ks = values[data_row][PARAM_KS];
        let ar = values[data_row][PARAM_AR];
        let ks_ar = ((ks & 0x03) << 6) | (ar & 0x1F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x80 + op_offset),
            data: format!("0x{:02X}", ks_ar),
        });

        // AMS (bits 7-6) and D1R (bits 4-0) - Register $A0-$BF
        let ams = values[data_row][PARAM_AMS];
        let d1r = values[data_row][PARAM_D1R];
        let ams_d1r = ((ams & 0x03) << 6) | (d1r & 0x1F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0xA0 + op_offset),
            data: format!("0x{:02X}", ams_d1r),
        });

        // DT2 (bits 7-6) and D2R (bits 3-0) - Register $C0-$DF
        let dt2 = values[data_row][PARAM_DT2];
        let d2r = values[data_row][PARAM_D2R];
        let dt2_d2r = ((dt2 & 0x03) << 6) | (d2r & 0x0F);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0xC0 + op_offset),
            data: format!("0x{:02X}", dt2_d2r),
        });

        // D1L (bits 7-4) and RR (bits 3-0) - Register $E0-$FF
        let d1l = values[data_row][PARAM_D1L];
        let rr = values[data_row][PARAM_RR];
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
    // Bits 3-6: Operator enable (slot 0-3)
    // YM2151 hardware slot order: M1=slot0, M2=slot1, C1=slot2, C2=slot3
    // Key On bit mapping: M1→bit3, M2→bit4, C1→bit5, C2→bit6
    // Use slot masks from operator rows (SM parameter at PARAM_SM)
    let m1_mask = values[0][PARAM_SM];  // M1 is data row 0
    let m2_mask = values[1][PARAM_SM];  // M2 is data row 1
    let c1_mask = values[2][PARAM_SM];  // C1 is data row 2
    let c2_mask = values[3][PARAM_SM];  // C2 is data row 3
    
    // Correct bit mapping: M1→bit3, M2→bit4, C1→bit5, C2→bit6
    let key_on_data = ((m1_mask & 1) << 3) | ((m2_mask & 1) << 4) 
                    | ((c1_mask & 1) << 5) | ((c2_mask & 1) << 6) | (channel as u8);
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

    // Inverse mapping: hardware slot → data row
    // Hardware slots: M1(0), M2(1), C1(2), C2(3)
    // Data rows: M1(0), M2(1), C1(2), C2(3)
    // Mapping: slot 0→row0, slot 1→row1, slot 2→row2, slot 3→row3
    const SLOT_TO_DATA_ROW: [usize; 4] = [0, 1, 2, 3];

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
                let hw_slot = ((addr - 0x40) / 8) as usize;
                if hw_slot < 4 {
                    let data_row = SLOT_TO_DATA_ROW[hw_slot];
                    values[data_row][PARAM_DT] = (data >> 4) & 0x07;
                    values[data_row][PARAM_MUL] = data & 0x0F;
                }
            }
            // TL registers (0x60-0x7F)
            0x60..=0x7F => {
                let hw_slot = ((addr - 0x60) / 8) as usize;
                if hw_slot < 4 {
                    let data_row = SLOT_TO_DATA_ROW[hw_slot];
                    values[data_row][PARAM_TL] = data & 0x7F;
                }
            }
            // KS/AR registers (0x80-0x9F)
            0x80..=0x9F => {
                let hw_slot = ((addr - 0x80) / 8) as usize;
                if hw_slot < 4 {
                    let data_row = SLOT_TO_DATA_ROW[hw_slot];
                    values[data_row][PARAM_KS] = (data >> 6) & 0x03;
                    values[data_row][PARAM_AR] = data & 0x1F;
                }
            }
            // AMS-EN/D1R registers (0xA0-0xBF)
            0xA0..=0xBF => {
                let hw_slot = ((addr - 0xA0) / 8) as usize;
                if hw_slot < 4 {
                    let data_row = SLOT_TO_DATA_ROW[hw_slot];
                    values[data_row][PARAM_AMS] = (data >> 6) & 0x03;
                    values[data_row][PARAM_D1R] = data & 0x1F;
                }
            }
            // DT2/D2R registers (0xC0-0xDF)
            0xC0..=0xDF => {
                let hw_slot = ((addr - 0xC0) / 8) as usize;
                if hw_slot < 4 {
                    let data_row = SLOT_TO_DATA_ROW[hw_slot];
                    values[data_row][PARAM_DT2] = (data >> 6) & 0x03;
                    values[data_row][PARAM_D2R] = data & 0x0F;
                }
            }
            // D1L/RR registers (0xE0-0xFF)
            0xE0..=0xFF => {
                let hw_slot = ((addr - 0xE0) / 8) as usize;
                if hw_slot < 4 {
                    let data_row = SLOT_TO_DATA_ROW[hw_slot];
                    values[data_row][PARAM_D1L] = (data >> 4) & 0x0F;
                    values[data_row][PARAM_RR] = data & 0x0F;
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
                // YM2151 hardware slot order: M1=slot0, M2=slot1, C1=slot2, C2=slot3
                // Bit 3: M1, Bit 4: M2, Bit 5: C1, Bit 6: C2
                // Store these in the SM parameter of each operator row
                values[0][PARAM_SM] = (data >> 3) & 0x01;  // M1 is data row 0
                values[1][PARAM_SM] = (data >> 4) & 0x01;  // M2 is data row 1
                values[2][PARAM_SM] = (data >> 5) & 0x01;  // C1 is data row 2
                values[3][PARAM_SM] = (data >> 6) & 0x01;  // C2 is data row 3
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
        values[0][PARAM_SM] = 1;
        values[1][PARAM_SM] = 1;
        values[2][PARAM_SM] = 1;
        values[3][PARAM_SM] = 1;
        values[ROW_CH][CH_PARAM_ALG] = 4;
        values[ROW_CH][CH_PARAM_FB] = 0;
        
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
        values[0][PARAM_SM] = 1;
        values[1][PARAM_SM] = 1;
        values[2][PARAM_SM] = 1;
        values[3][PARAM_SM] = 1;
        
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

    #[test]
    fn test_slot_mask_bit_order() {
        // Test that slot masks use correct YM2151 bit order: M1, M2, C1, C2
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Enable only M2
        values[0][PARAM_SM] = 0;  // M1
        values[1][PARAM_SM] = 1;  // M2 should map to bit 4
        values[2][PARAM_SM] = 0;  // C1
        values[3][PARAM_SM] = 0;  // C2
        values[ROW_CH][CH_PARAM_ALG] = 4;
        
        let events = to_ym2151_events(&values);
        
        // Find the Key On event (register 0x08)
        let key_on_event = events.iter().find(|e| e.addr == "0x08");
        assert!(key_on_event.is_some(), "Key On event should be present");
        
        let key_on_data = key_on_event.unwrap().data.trim_start_matches("0x");
        let data = u8::from_str_radix(key_on_data, 16).unwrap();
        
        // M2 should be at bit 4, so data should be 0b00010000 | channel = 0x10
        assert_eq!(data, 0x10, "M2 should map to bit 4 (0x10)");
        
        // Test C1
        values[1][PARAM_SM] = 0;  // M2
        values[2][PARAM_SM] = 1;  // C1 should map to bit 5
        
        let events = to_ym2151_events(&values);
        let key_on_event = events.iter().find(|e| e.addr == "0x08");
        let key_on_data = key_on_event.unwrap().data.trim_start_matches("0x");
        let data = u8::from_str_radix(key_on_data, 16).unwrap();
        
        // C1 should be at bit 5, so data should be 0b00100000 | channel = 0x20
        assert_eq!(data, 0x20, "C1 should map to bit 5 (0x20)");
    }

    #[test]
    fn test_slot_mask_roundtrip() {
        // Test that slot masks roundtrip correctly through events
        let mut values_original = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Set a specific pattern: M1=1, C1=1, M2=0, C2=0
        values_original[0][PARAM_SM] = 1;  // M1
        values_original[1][PARAM_SM] = 0;  // M2
        values_original[2][PARAM_SM] = 1;  // C1
        values_original[3][PARAM_SM] = 0;  // C2
        values_original[ROW_CH][CH_PARAM_ALG] = 4;
        
        // Convert to events and back
        let events = to_ym2151_events(&values_original);
        let values_roundtrip = events_to_tone_data(&events).unwrap();
        
        // Verify slot masks are preserved
        assert_eq!(values_roundtrip[0][PARAM_SM], 1, "M1 mask should roundtrip");
        assert_eq!(values_roundtrip[2][PARAM_SM], 1, "C1 mask should roundtrip");
        assert_eq!(values_roundtrip[1][PARAM_SM], 0, "M2 mask should roundtrip");
        assert_eq!(values_roundtrip[3][PARAM_SM], 0, "C2 mask should roundtrip");
    }

    #[test]
    fn test_operator_register_order() {
        // Test that data rows map to correct hardware slots: 
        // Data row 0 (M1)→slot0, Data row 1 (M2)→slot1, Data row 2 (C1)→slot2, Data row 3 (C2)→slot3
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Set unique MUL values for each data row to identify them
        values[0][PARAM_MUL] = 1;  // Data row 0 (M1) should go to slot 0
        values[1][PARAM_MUL] = 2;  // Data row 1 (M2) should go to slot 1
        values[2][PARAM_MUL] = 3;  // Data row 2 (C1) should go to slot 2
        values[3][PARAM_MUL] = 4;  // Data row 3 (C2) should go to slot 3
        
        let events = to_ym2151_events(&values);
        
        // Check DT1/MUL registers (0x40-0x5F)
        // Register 0x40 (slot 0, channel 0) should have M1's MUL=1
        let m1_event = events.iter().find(|e| e.addr == "0x40");
        assert!(m1_event.is_some(), "M1 register should be present");
        let data = u8::from_str_radix(m1_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(data & 0x0F, 1, "Register 0x40 (slot 0) should have M1's MUL=1");
        
        // Register 0x48 (slot 1, channel 0) should have M2's MUL=2
        let m2_event = events.iter().find(|e| e.addr == "0x48");
        assert!(m2_event.is_some(), "M2 register should be present");
        let data = u8::from_str_radix(m2_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(data & 0x0F, 2, "Register 0x48 (slot 1) should have M2's MUL=2");
        
        // Register 0x50 (slot 2, channel 0) should have C1's MUL=3
        let c1_event = events.iter().find(|e| e.addr == "0x50");
        assert!(c1_event.is_some(), "C1 register should be present");
        let data = u8::from_str_radix(c1_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(data & 0x0F, 3, "Register 0x50 (slot 2) should have C1's MUL=3");
        
        // Register 0x58 (slot 3, channel 0) should have C2's MUL=4
        let c2_event = events.iter().find(|e| e.addr == "0x58");
        assert!(c2_event.is_some(), "C2 register should be present");
        let data = u8::from_str_radix(c2_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(data & 0x0F, 4, "Register 0x58 (slot 3) should have C2's MUL=4");
    }

    #[test]
    fn test_operator_order_roundtrip() {
        // Test that operator values roundtrip correctly with the mapping
        let mut values_original = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Set distinct values for each data row
        for row in 0..4 {
            values_original[row][PARAM_MUL] = (row + 1) as u8;
            values_original[row][PARAM_TL] = (row * 10) as u8;
            values_original[row][PARAM_AR] = (row * 5) as u8;
        }
        values_original[ROW_CH][CH_PARAM_ALG] = 4;
        
        // Convert to events and back
        let events = to_ym2151_events(&values_original);
        let values_roundtrip = events_to_tone_data(&events).unwrap();
        
        // Verify all operator values are preserved
        let row_names = ["M1", "M2", "C1", "C2"];
        for row in 0..4 {
            assert_eq!(values_roundtrip[row][PARAM_MUL], values_original[row][PARAM_MUL], 
                "{} MUL should roundtrip correctly", row_names[row]);
            assert_eq!(values_roundtrip[row][PARAM_TL], values_original[row][PARAM_TL], 
                "{} TL should roundtrip correctly", row_names[row]);
            assert_eq!(values_roundtrip[row][PARAM_AR], values_original[row][PARAM_AR], 
                "{} AR should roundtrip correctly", row_names[row]);
        }
    }

    #[test]
    fn test_issue_59_alg4_operator_order() {
        // Issue #59: In ALG4, rows 2 and 3 (M2 and C2) were swapped
        // This test verifies that the operators are correctly mapped to hardware slots
        // ALG4: M1->C1->OUT, M2->C2->OUT (two independent FM pairs)
        
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Set algorithm to 4
        values[ROW_CH][CH_PARAM_ALG] = 4;
        
        // Set distinct MUL values for each operator to track them
        // MUL is 4 bits (0-15), so use values in that range
        values[0][PARAM_MUL] = 1;   // M1 (data row 0)
        values[1][PARAM_MUL] = 2;   // M2 (data row 1)  
        values[2][PARAM_MUL] = 3;   // C1 (data row 2)
        values[3][PARAM_MUL] = 4;   // C2 (data row 3)
        
        // Enable all operators via slot masks
        values[0][PARAM_SM] = 1;
        values[1][PARAM_SM] = 1;
        values[2][PARAM_SM] = 1;
        values[3][PARAM_SM] = 1;
        
        let events = to_ym2151_events(&values);
        
        // Verify correct hardware slot mapping:
        // M1 (data row 0) should map to hardware slot 0 (register 0x40)
        let m1_event = events.iter().find(|e| e.addr == "0x40");
        assert!(m1_event.is_some());
        let m1_data = u8::from_str_radix(m1_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(m1_data & 0x0F, 1, "M1 should be at slot 0 with MUL=1");
        
        // M2 (data row 1) should map to hardware slot 1 (register 0x48)
        // This was incorrectly mapped to slot 2 (0x50) before the fix
        let m2_event = events.iter().find(|e| e.addr == "0x48");
        assert!(m2_event.is_some());
        let m2_data = u8::from_str_radix(m2_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(m2_data & 0x0F, 2, "M2 should be at slot 1 with MUL=2");
        
        // C1 (data row 2) should map to hardware slot 2 (register 0x50)
        // This was incorrectly mapped to slot 1 (0x48) before the fix
        let c1_event = events.iter().find(|e| e.addr == "0x50");
        assert!(c1_event.is_some());
        let c1_data = u8::from_str_radix(c1_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(c1_data & 0x0F, 3, "C1 should be at slot 2 with MUL=3");
        
        // C2 (data row 3) should map to hardware slot 3 (register 0x58)
        let c2_event = events.iter().find(|e| e.addr == "0x58");
        assert!(c2_event.is_some());
        let c2_data = u8::from_str_radix(c2_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(c2_data & 0x0F, 4, "C2 should be at slot 3 with MUL=4");
        
        // Verify slot masks are correctly set
        // Key On register (0x08) should have all operators enabled
        // Bits: M1=bit3, M2=bit4, C1=bit5, C2=bit6
        let key_on_event = events.iter().find(|e| e.addr == "0x08");
        assert!(key_on_event.is_some());
        let key_on_data = u8::from_str_radix(key_on_event.unwrap().data.trim_start_matches("0x"), 16).unwrap();
        
        // Extract operator enable bits
        let m1_enabled = (key_on_data >> 3) & 1;
        let m2_enabled = (key_on_data >> 4) & 1;
        let c1_enabled = (key_on_data >> 5) & 1;
        let c2_enabled = (key_on_data >> 6) & 1;
        
        assert_eq!(m1_enabled, 1, "M1 should be enabled (bit 3)");
        assert_eq!(m2_enabled, 1, "M2 should be enabled (bit 4)");
        assert_eq!(c1_enabled, 1, "C1 should be enabled (bit 5)");
        assert_eq!(c2_enabled, 1, "C2 should be enabled (bit 6)");
    }
}