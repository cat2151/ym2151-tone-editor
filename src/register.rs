use std::io;
use crate::models::*;
use crate::midi_conversion::{kc_to_midi_note, midi_to_kc_kf};

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

    // YM2151 hardware operator register order: M1, M2, C1, C2 (sequential slots 0, 1, 2, 3)
    // We display as: M1, C1, M2, C2 (reordered for user-friendly layout)
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
    // Bits 3-6: Operator enable (M1=bit3, C1=bit4, M2=bit5, C2=bit6)
    // YM2151 hardware uses non-sequential bit order: M1, C1, M2, C2
    // Use slot masks from operator rows (SM parameter at PARAM_SM)
    let m1_mask = values[0][PARAM_SM];  // M1 is data row 0
    let m2_mask = values[1][PARAM_SM];  // M2 is data row 1
    let c1_mask = values[2][PARAM_SM];  // C1 is data row 2
    let c2_mask = values[3][PARAM_SM];  // C2 is data row 3
    
    // Correct bit mapping: M1→bit3, C1→bit4, M2→bit5, C2→bit6
    let key_on_data = ((m1_mask & 1) << 3) | ((c1_mask & 1) << 4) 
                    | ((m2_mask & 1) << 5) | ((c2_mask & 1) << 6) | (channel as u8);
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
                // YM2151 hardware uses bit order: M1, C1, M2, C2
                // Bit 3: M1, Bit 4: C1, Bit 5: M2, Bit 6: C2
                // Store these in the SM parameter of each operator row
                values[0][PARAM_SM] = (data >> 3) & 0x01;  // M1 is data row 0
                values[2][PARAM_SM] = (data >> 4) & 0x01;  // C1 is data row 2
                values[1][PARAM_SM] = (data >> 5) & 0x01;  // M2 is data row 1
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

/// Convert tone data to registers hex string format
/// Format: pairs of address (2 hex chars) + data (2 hex chars)
/// Example: "204F204C364037808003812D" represents 3 register writes:
/// - Register 0x20 = 0x4F
/// - Register 0x20 = 0x4C (this example shows duplicate addresses are allowed)
/// - Register 0x36 = 0x40
/// etc.
pub fn tone_data_to_registers(values: &ToneData) -> String {
    let events = to_ym2151_events(values);
    let mut result = String::new();
    
    for event in events {
        // Remove "0x" prefix from addr and data
        let addr_hex = event.addr.trim_start_matches("0x");
        let data_hex = event.data.trim_start_matches("0x");
        result.push_str(addr_hex);
        result.push_str(data_hex);
    }
    
    result
}

/// Convert registers hex string to tone data
/// Format: pairs of address (2 hex chars) + data (2 hex chars)
/// Example: "204F204C364037808003812D"
pub fn registers_to_tone_data(registers: &str) -> io::Result<ToneData> {
    // Parse hex string into events
    let mut events = Vec::new();
    
    // Process pairs of address+data (4 characters each)
    let chars: Vec<char> = registers.chars().collect();
    if chars.len() % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Registers string length must be a multiple of 4"
        ));
    }
    
    for chunk in chars.chunks(4) {
        let addr_str: String = chunk[0..2].iter().collect();
        let data_str: String = chunk[2..4].iter().collect();
        
        // Parse hex values
        let addr = u8::from_str_radix(&addr_str, 16)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid address hex: {}", e)))?;
        let data = u8::from_str_radix(&data_str, 16)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid data hex: {}", e)))?;
        
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", addr),
            data: format!("0x{:02X}", data),
        });
    }
    
    // Convert events to tone data using existing function
    events_to_tone_data(&events)
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
        // Test that slot masks use correct YM2151 bit order: M1, C1, M2, C2
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Enable only M2
        values[0][PARAM_SM] = 0;  // M1
        values[1][PARAM_SM] = 1;  // M2 should map to bit 5
        values[2][PARAM_SM] = 0;  // C1
        values[3][PARAM_SM] = 0;  // C2
        values[ROW_CH][CH_PARAM_ALG] = 4;
        
        let events = to_ym2151_events(&values);
        
        // Find the Key On event (register 0x08)
        let key_on_event = events.iter().find(|e| e.addr == "0x08");
        assert!(key_on_event.is_some(), "Key On event should be present");
        
        let key_on_data = key_on_event.unwrap().data.trim_start_matches("0x");
        let data = u8::from_str_radix(key_on_data, 16).unwrap();
        
        // M2 should be at bit 5, so data should be 0b00100000 | channel = 0x20
        assert_eq!(data, 0x20, "M2 should map to bit 5 (0x20)");
        
        // Test C1
        values[1][PARAM_SM] = 0;  // M2
        values[2][PARAM_SM] = 1;  // C1 should map to bit 4
        
        let events = to_ym2151_events(&values);
        let key_on_event = events.iter().find(|e| e.addr == "0x08");
        let key_on_data = key_on_event.unwrap().data.trim_start_matches("0x");
        let data = u8::from_str_radix(key_on_data, 16).unwrap();
        
        // C1 should be at bit 4, so data should be 0b00010000 | channel = 0x10
        assert_eq!(data, 0x10, "C1 should map to bit 4 (0x10)");
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
    fn test_alg4_carrier_mapping() {
        // Verify that for Algorithm 4, C1 (data row 2) and C2 (data row 3) act as carriers
        // and M1 (data row 0) and M2 (data row 1) act as modulators
        // This test ensures that the fix for issue #57 is correct
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Set ALG=4 (two FM pairs: M1->C1->OUT and M2->C2->OUT)
        values[ROW_CH][CH_PARAM_ALG] = 4;
        
        // Set unique MUL values to identify operators
        values[0][PARAM_MUL] = 1;  // M1 (data row 0) - modulator
        values[1][PARAM_MUL] = 2;  // M2 (data row 1) - modulator
        values[2][PARAM_MUL] = 3;  // C1 (data row 2) - carrier
        values[3][PARAM_MUL] = 4;  // C2 (data row 3) - carrier
        
        let events = to_ym2151_events(&values);
        
        // Verify hardware slot mapping (YM2151 hardware: M1=slot0, M2=slot1, C1=slot2, C2=slot3)
        // For ALG4, slots 2 (C1) and 3 (C2) should be carriers
        
        // Check that M1 (data row 0) maps to slot 0
        let m1_event = events.iter().find(|e| e.addr == "0x40").unwrap();
        let m1_data = u8::from_str_radix(m1_event.data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(m1_data & 0x0F, 1, "M1 should have MUL=1 at slot 0");
        
        // Check that M2 (data row 1) maps to slot 1
        let m2_event = events.iter().find(|e| e.addr == "0x48").unwrap();
        let m2_data = u8::from_str_radix(m2_event.data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(m2_data & 0x0F, 2, "M2 should have MUL=2 at slot 1");
        
        // Check that C1 (data row 2) maps to slot 2 - this is a carrier in ALG4
        let c1_event = events.iter().find(|e| e.addr == "0x50").unwrap();
        let c1_data = u8::from_str_radix(c1_event.data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(c1_data & 0x0F, 3, "C1 (carrier) should have MUL=3 at slot 2");
        
        // Check that C2 (data row 3) maps to slot 3 - this is a carrier in ALG4
        let c2_event = events.iter().find(|e| e.addr == "0x58").unwrap();
        let c2_data = u8::from_str_radix(c2_event.data.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(c2_data & 0x0F, 4, "C2 (carrier) should have MUL=4 at slot 3");
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
    fn test_tone_data_to_registers() {
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
        
        let registers = tone_data_to_registers(&values);
        
        // Should be a hex string with pairs of address+data (4 chars per register write)
        // We have 28 events, so 28 * 4 = 112 characters
        assert_eq!(registers.len(), 112, "Registers string should have 112 characters (28 events * 4 chars)");
        
        // All characters should be valid hex
        assert!(registers.chars().all(|c| c.is_ascii_hexdigit()), "All characters should be hex digits");
    }

    #[test]
    fn test_registers_to_tone_data() {
        // Create a simple test case
        let mut values_original = [[0; GRID_WIDTH]; GRID_HEIGHT];
        values_original[0][PARAM_MUL] = 5;
        values_original[0][PARAM_TL] = 30;
        values_original[ROW_CH][CH_PARAM_ALG] = 3;
        values_original[ROW_CH][CH_PARAM_FB] = 2;
        
        // Convert to registers string
        let registers = tone_data_to_registers(&values_original);
        
        // Convert back to tone data
        let values_result = registers_to_tone_data(&registers).unwrap();
        
        // Verify key values are preserved
        assert_eq!(values_result[0][PARAM_MUL], values_original[0][PARAM_MUL], "MUL should roundtrip correctly");
        assert_eq!(values_result[0][PARAM_TL], values_original[0][PARAM_TL], "TL should roundtrip correctly");
        assert_eq!(values_result[ROW_CH][CH_PARAM_ALG], values_original[ROW_CH][CH_PARAM_ALG], "ALG should roundtrip correctly");
        assert_eq!(values_result[ROW_CH][CH_PARAM_FB], values_original[ROW_CH][CH_PARAM_FB], "FB should roundtrip correctly");
    }

    #[test]
    fn test_registers_to_tone_data_roundtrip() {
        // Test a more complete roundtrip with various parameter values
        let mut values_original = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Set different values for each operator
        for row in 0..4 {
            values_original[row][PARAM_SM] = 1;
            values_original[row][PARAM_MUL] = (row + 1) as u8;
            values_original[row][PARAM_TL] = (row * 10) as u8;
            values_original[row][PARAM_AR] = (row * 5) as u8;
            values_original[row][PARAM_D1R] = (row * 3) as u8;
            values_original[row][PARAM_D1L] = (row * 2) as u8;
        }
        values_original[ROW_CH][CH_PARAM_ALG] = 5;
        values_original[ROW_CH][CH_PARAM_FB] = 3;
        
        // Convert to registers and back
        let registers = tone_data_to_registers(&values_original);
        let values_roundtrip = registers_to_tone_data(&registers).unwrap();
        
        // Verify all important values are preserved
        for row in 0..4 {
            assert_eq!(values_roundtrip[row][PARAM_MUL], values_original[row][PARAM_MUL], "Row {} MUL should roundtrip", row);
            assert_eq!(values_roundtrip[row][PARAM_TL], values_original[row][PARAM_TL], "Row {} TL should roundtrip", row);
            assert_eq!(values_roundtrip[row][PARAM_AR], values_original[row][PARAM_AR], "Row {} AR should roundtrip", row);
            assert_eq!(values_roundtrip[row][PARAM_D1R], values_original[row][PARAM_D1R], "Row {} D1R should roundtrip", row);
            assert_eq!(values_roundtrip[row][PARAM_D1L], values_original[row][PARAM_D1L], "Row {} D1L should roundtrip", row);
        }
        assert_eq!(values_roundtrip[ROW_CH][CH_PARAM_ALG], values_original[ROW_CH][CH_PARAM_ALG], "ALG should roundtrip");
        assert_eq!(values_roundtrip[ROW_CH][CH_PARAM_FB], values_original[ROW_CH][CH_PARAM_FB], "FB should roundtrip");
    }

    #[test]
    fn test_registers_invalid_length() {
        // Test with invalid length (not a multiple of 4)
        let result = registers_to_tone_data("204F2");
        assert!(result.is_err(), "Should error on invalid length");
    }

    #[test]
    fn test_registers_invalid_hex() {
        // Test with invalid hex characters
        let result = registers_to_tone_data("GGGG");
        assert!(result.is_err(), "Should error on invalid hex characters");
    }
}
