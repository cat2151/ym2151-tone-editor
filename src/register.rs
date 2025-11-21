use crate::midi_conversion::{kc_to_midi_note, midi_to_kc_kf};
use crate::models::*;
use std::io;

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
            time: 0.0,
            addr: format!("0x{:02X}", 0x40 + op_offset),
            data: format!("0x{:02X}", dt_mul),
        });

        // TL (Total Level) - Register $60-$7F (7 bits)
        let tl = values[data_row][PARAM_TL];
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0x60 + op_offset),
            data: format!("0x{:02X}", tl & 0x7F),
        });

        // KS (bits 7-6) and AR (bits 4-0) - Register $80-$9F
        let ks = values[data_row][PARAM_KS];
        let ar = values[data_row][PARAM_AR];
        let ks_ar = ((ks & 0x03) << 6) | (ar & 0x1F);
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0x80 + op_offset),
            data: format!("0x{:02X}", ks_ar),
        });

        // AMS (bits 7-6) and D1R (bits 4-0) - Register $A0-$BF
        let ams = values[data_row][PARAM_AMS];
        let d1r = values[data_row][PARAM_D1R];
        let ams_d1r = ((ams & 0x03) << 6) | (d1r & 0x1F);
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0xA0 + op_offset),
            data: format!("0x{:02X}", ams_d1r),
        });

        // DT2 (bits 7-6) and D2R (bits 3-0) - Register $C0-$DF
        let dt2 = values[data_row][PARAM_DT2];
        let d2r = values[data_row][PARAM_D2R];
        let dt2_d2r = ((dt2 & 0x03) << 6) | (d2r & 0x0F);
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0xC0 + op_offset),
            data: format!("0x{:02X}", dt2_d2r),
        });

        // D1L (bits 7-4) and RR (bits 3-0) - Register $E0-$FF
        let d1l = values[data_row][PARAM_D1L];
        let rr = values[data_row][PARAM_RR];
        let d1l_rr = ((d1l & 0x0F) << 4) | (rr & 0x0F);
        events.push(Ym2151Event {
            time: 0.0,
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
        time: 0.0,
        addr: format!("0x{:02X}", 0x20 + channel),
        data: format!("0x{:02X}", rl_fb_con),
    });

    // Key Code (KC) and Key Fraction (KF) - Use MIDI note number from CH row
    let midi_note = values[ROW_CH][CH_PARAM_NOTE];
    let (kc, kf) = midi_to_kc_kf(midi_note);

    // Key Code (KC) - Register $28-$2F
    events.push(Ym2151Event {
        time: 0.0,
        addr: format!("0x{:02X}", 0x28 + channel),
        data: format!("0x{:02X}", kc),
    });

    // Key Fraction (KF) - Register $30-$37 - Fine frequency adjust
    events.push(Ym2151Event {
        time: 0.0,
        addr: format!("0x{:02X}", 0x30 + channel),
        data: format!("0x{:02X}", kf),
    });

    // Note On - Register $08 - Key On with operators based on slot masks
    // Bits 0-2: Channel (0-7)
    // Bits 3-6: Operator enable (M1=bit3, C1=bit4, M2=bit5, C2=bit6)
    // YM2151 hardware uses non-sequential bit order: M1, C1, M2, C2
    // Use slot masks from operator rows (SM parameter at PARAM_SM)
    let m1_mask = values[0][PARAM_SM]; // M1 is data row 0
    let m2_mask = values[1][PARAM_SM]; // M2 is data row 1
    let c1_mask = values[2][PARAM_SM]; // C1 is data row 2
    let c2_mask = values[3][PARAM_SM]; // C2 is data row 3

    // Correct bit mapping: M1→bit3, C1→bit4, M2→bit5, C2→bit6
    let key_on_data = ((m1_mask & 1) << 3)
        | ((c1_mask & 1) << 4)
        | ((m2_mask & 1) << 5)
        | ((c2_mask & 1) << 6)
        | (channel as u8);
    events.push(Ym2151Event {
        time: 0.0,
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
                values[0][PARAM_SM] = (data >> 3) & 0x01; // M1 is data row 0
                values[2][PARAM_SM] = (data >> 4) & 0x01; // C1 is data row 2
                values[1][PARAM_SM] = (data >> 5) & 0x01; // M2 is data row 1
                values[3][PARAM_SM] = (data >> 6) & 0x01; // C2 is data row 3
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
///   etc.
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
    if !chars.len().is_multiple_of(4) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Registers string length must be a multiple of 4",
        ));
    }

    for chunk in chars.chunks(4) {
        let addr_str: String = chunk[0..2].iter().collect();
        let data_str: String = chunk[2..4].iter().collect();

        // Parse hex values
        let addr = u8::from_str_radix(&addr_str, 16).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid address hex: {}", e),
            )
        })?;
        let data = u8::from_str_radix(&data_str, 16).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid data hex: {}", e),
            )
        })?;

        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", addr),
            data: format!("0x{:02X}", data),
        });
    }

    // Convert events to tone data using existing function
    events_to_tone_data(&events)
}
