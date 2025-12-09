use crate::midi_conversion::{kc_to_midi_note, midi_to_kc_kf};
use crate::models::*;
use std::io;

/// Helper function to add KEY_ON register to events
/// This is platform-independent and used for generating register data.
/// Note: A similar function exists in audio.rs for Windows with additional logging.
fn add_key_on(values: &ToneData, events: &mut Vec<Ym2151Event>) {
    let channel = 0; // We use channel 0

    // Calculate slot mask based on which operators are enabled
    let sm0 = values[0][PARAM_SM];
    let sm1 = values[1][PARAM_SM];
    let sm2 = values[2][PARAM_SM];
    let sm3 = values[3][PARAM_SM];

    let slot_mask = if sm0 != 0 { 0x08 } else { 0 }
        | if sm1 != 0 { 0x10 } else { 0 }
        | if sm2 != 0 { 0x20 } else { 0 }
        | if sm3 != 0 { 0x40 } else { 0 };

    let key_on = slot_mask | channel as u8;
    events.push(Ym2151Event {
        time: 0.0,
        addr: "0x08".to_string(),
        data: format!("0x{:02X}", key_on),
    });
}

// YM2151 hardware operator register order: O1, O3, O2, O4
// We display as: O1, O2, O3, O4 (reordered for user-friendly layout)
pub const REG_FROM_O1_O4: [usize; 4] = [0, 2, 1, 3];
pub const O1_O4_FROM_REG: [usize; 4] = [0, 2, 1, 3]; // 内容は同じだが、可読性を優先し、別名で定義

/// Helper function to generate D2R=15 register events for envelope reset
/// Preserves DT2 values while setting D2R to maximum decay rate (15)
/// Used to reset envelope amplitude to 0 before next note (issue #115)
#[cfg(windows)]
pub fn generate_d2r_15_events(values: &ToneData, channel: usize, time: f64) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    for row_id in 0..4 {
        let reg = REG_FROM_O1_O4[row_id];
        let op_offset = reg * 8 + channel;

        // Get current DT2 value to preserve it
        let dt2 = values[row_id][PARAM_DT2];
        // Set D2R to 15 (maximum decay rate)
        let dt2_d2r = ((dt2 & 0x03) << 6) | 0x0F;

        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0xC0 + op_offset),
            data: format!("0x{:02X}", dt2_d2r),
        });
    }

    events
}

/// Helper function to generate complete ADSR envelope reset events
/// Sets AR=31, D1R=31, D1L=15, D2R=15, RR=15 for all operators
/// Preserves DT2, KS, AMS values while setting envelope parameters to maximum
/// Used to reset envelope amplitude to 0 before next note (issue #151)
#[cfg(windows)]
pub fn generate_full_envelope_reset_events(
    values: &ToneData,
    channel: usize,
    time: f64,
) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    for row_id in 0..4 {
        let reg = REG_FROM_O1_O4[row_id];
        let op_offset = reg * 8 + channel;

        // AR=31 with KS preserved - Register $80-$9F
        let ks = values[row_id][PARAM_KS];
        let ks_ar = ((ks & 0x03) << 6) | 0x1F; // AR=31 (max)
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0x80 + op_offset),
            data: format!("0x{:02X}", ks_ar),
        });

        // D1R=31 with AMS preserved - Register $A0-$BF
        let ams = values[row_id][PARAM_AMS];
        let ams_d1r = ((ams & 0x03) << 6) | 0x1F; // D1R=31 (max)
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0xA0 + op_offset),
            data: format!("0x{:02X}", ams_d1r),
        });

        // D2R=15 with DT2 preserved - Register $C0-$DF
        let dt2 = values[row_id][PARAM_DT2];
        let dt2_d2r = ((dt2 & 0x03) << 6) | 0x0F; // D2R=15 (max)
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0xC0 + op_offset),
            data: format!("0x{:02X}", dt2_d2r),
        });

        // D1L=15, RR=15 - Register $E0-$FF
        let d1l_rr = 0xFF; // D1L=15 (max), RR=15 (max)
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0xE0 + op_offset),
            data: format!("0x{:02X}", d1l_rr),
        });
    }

    events
}

/// Convert tone data to YM2151 register events
/// This generates register writes for the YM2151 chip based on the current tone parameters
pub fn editor_rows_to_ym2151_events(editor_rows: &ToneData) -> Vec<Ym2151Event> {
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

    for row_id in 0..4 {
        let op_offset = REG_FROM_O1_O4[row_id] * 8 + channel; // Operator offset in register map

        // DT1 (bits 6-4) and MUL (bits 3-0) - Register $40-$5F
        let dt = editor_rows[row_id][PARAM_DT];
        let mul = editor_rows[row_id][PARAM_MUL];
        let dt_mul = ((dt & 0x07) << 4) | (mul & 0x0F);
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0x40 + op_offset),
            data: format!("0x{:02X}", dt_mul),
        });

        // TL (Total Level) - Register $60-$7F (7 bits)
        let tl = editor_rows[row_id][PARAM_TL];
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0x60 + op_offset),
            data: format!("0x{:02X}", tl & 0x7F),
        });

        // KS (bits 7-6) and AR (bits 4-0) - Register $80-$9F
        let ks = editor_rows[row_id][PARAM_KS];
        let ar = editor_rows[row_id][PARAM_AR];
        let ks_ar = ((ks & 0x03) << 6) | (ar & 0x1F);
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0x80 + op_offset),
            data: format!("0x{:02X}", ks_ar),
        });

        // AMS (bits 7-6) and D1R (bits 4-0) - Register $A0-$BF
        let ams = editor_rows[row_id][PARAM_AMS];
        let d1r = editor_rows[row_id][PARAM_D1R];
        let ams_d1r = ((ams & 0x03) << 6) | (d1r & 0x1F);
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0xA0 + op_offset),
            data: format!("0x{:02X}", ams_d1r),
        });

        // DT2 (bits 7-6) and D2R (bits 3-0) - Register $C0-$DF
        let dt2 = editor_rows[row_id][PARAM_DT2];
        let d2r = editor_rows[row_id][PARAM_D2R];
        let dt2_d2r = ((dt2 & 0x03) << 6) | (d2r & 0x0F);
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0xC0 + op_offset),
            data: format!("0x{:02X}", dt2_d2r),
        });

        // D1L (bits 7-4) and RR (bits 3-0) - Register $E0-$FF
        let d1l = editor_rows[row_id][PARAM_D1L];
        let rr = editor_rows[row_id][PARAM_RR];
        let d1l_rr = ((d1l & 0x0F) << 4) | (rr & 0x0F);
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0xE0 + op_offset),
            data: format!("0x{:02X}", d1l_rr),
        });
    }

    // Channel settings: RL, FB, CON (Algorithm) - Register $20-$27
    // Use ALG and FB from CH row
    let alg = editor_rows[ROW_CH][CH_PARAM_ALG];
    let fb = editor_rows[ROW_CH][CH_PARAM_FB];
    let rl = 0xC0; // Both L and R enabled
    let rl_fb_con = rl | ((fb & 0x07) << 3) | (alg & 0x07);
    events.push(Ym2151Event {
        time: 0.0,
        addr: format!("0x{:02X}", 0x20 + channel),
        data: format!("0x{:02X}", rl_fb_con),
    });

    // Key Code (KC) and Key Fraction (KF) - Use MIDI note number from CH row
    let midi_note = editor_rows[ROW_CH][CH_PARAM_NOTE];
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

    add_key_on(editor_rows, &mut events);

    events
}

/// Convert YM2151 events back to tone data
pub fn json_events_to_editor_rows(events: &[Ym2151Event]) -> io::Result<ToneData> {
    let mut editor_rows = [[0; GRID_WIDTH]; GRID_HEIGHT];

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
                let reg = ((addr - 0x40) / 8) as usize;
                if reg < 4 {
                    let data_row = O1_O4_FROM_REG[reg];
                    editor_rows[data_row][PARAM_DT] = (data >> 4) & 0x07;
                    editor_rows[data_row][PARAM_MUL] = data & 0x0F;
                }
            }
            // TL registers (0x60-0x7F)
            0x60..=0x7F => {
                let reg = ((addr - 0x60) / 8) as usize;
                if reg < 4 {
                    let data_row = O1_O4_FROM_REG[reg];
                    editor_rows[data_row][PARAM_TL] = data & 0x7F;
                }
            }
            // KS/AR registers (0x80-0x9F)
            0x80..=0x9F => {
                let reg = ((addr - 0x80) / 8) as usize;
                if reg < 4 {
                    let data_row = O1_O4_FROM_REG[reg];
                    editor_rows[data_row][PARAM_KS] = (data >> 6) & 0x03;
                    editor_rows[data_row][PARAM_AR] = data & 0x1F;
                }
            }
            // AMS-EN/D1R registers (0xA0-0xBF)
            0xA0..=0xBF => {
                let reg = ((addr - 0xA0) / 8) as usize;
                if reg < 4 {
                    let data_row = O1_O4_FROM_REG[reg];
                    editor_rows[data_row][PARAM_AMS] = (data >> 6) & 0x03;
                    editor_rows[data_row][PARAM_D1R] = data & 0x1F;
                }
            }
            // DT2/D2R registers (0xC0-0xDF)
            0xC0..=0xDF => {
                let reg = ((addr - 0xC0) / 8) as usize;
                if reg < 4 {
                    let data_row = O1_O4_FROM_REG[reg];
                    editor_rows[data_row][PARAM_DT2] = (data >> 6) & 0x03;
                    editor_rows[data_row][PARAM_D2R] = data & 0x0F;
                }
            }
            // D1L/RR registers (0xE0-0xFF)
            0xE0..=0xFF => {
                let reg = ((addr - 0xE0) / 8) as usize;
                if reg < 4 {
                    let data_row = O1_O4_FROM_REG[reg];
                    editor_rows[data_row][PARAM_D1L] = (data >> 4) & 0x0F;
                    editor_rows[data_row][PARAM_RR] = data & 0x0F;
                }
            }
            // RL/FB/CON register (0x20-0x27)
            0x20..=0x27 => {
                // This register contains RL (bit 7-6), FB (bit 5-3), and CON/ALG (bit 2-0)
                // Extract ALG and FB to CH row
                editor_rows[ROW_CH][CH_PARAM_ALG] = data & 0x07; // ALG is bits 0-2
                editor_rows[ROW_CH][CH_PARAM_FB] = (data >> 3) & 0x07; // FB is bits 3-5
            }
            // Key On register (0x08)
            0x08 => {
                // Bits 3-6 contain operator enable flags
                // Store these in the SM parameter of each operator row
                editor_rows[0][PARAM_SM] = (data >> 3) & 0x01;
                editor_rows[1][PARAM_SM] = (data >> 4) & 0x01;
                editor_rows[2][PARAM_SM] = (data >> 5) & 0x01;
                editor_rows[3][PARAM_SM] = (data >> 6) & 0x01;
            }
            // KC (Key Code) register (0x28-0x2F)
            0x28..=0x2F => {
                // Convert KC back to MIDI note number
                editor_rows[ROW_CH][CH_PARAM_NOTE] = kc_to_midi_note(data);
            }
            _ => {}
        }
    }

    Ok(editor_rows)
}

/// Convert tone data to YM2151 register events with envelope reset
/// This is used for audio preview to prevent envelope continuation across notes (issue #115, issue #151)
#[cfg(windows)]
pub fn editor_rows_to_ym2151_events_with_envelope_reset(
    editor_rows: &ToneData,
    envelope_delay_seconds: f64,
) -> Vec<Ym2151Event> {
    let mut events = Vec::new();
    let channel = 0;

    // Step 1: Set full ADSR envelope reset (AR=31, D1R=31, D1L=15, D2R=15, RR=15) for all operators at time 0.0
    events.extend(generate_full_envelope_reset_events(
        editor_rows,
        channel,
        0.0,
    ));

    // Step 2: KEY_OFF at time 0.0
    events.push(Ym2151Event {
        time: 0.0,
        addr: "0x08".to_string(),
        data: format!("0x{:02X}", channel),
    });

    // Step 3: Wait for configured delay, then set tone parameters and KEY_ON
    // Add all the normal register events with the configured delay
    let mut tone_events = editor_rows_to_ym2151_events(editor_rows);
    for event in &mut tone_events {
        event.time = envelope_delay_seconds; // All tone settings and KEY_ON happen after configured delay
    }
    events.extend(tone_events);

    events
}

/// Convert tone data to JSON string in ym2151-log-play-server format
pub fn to_json_string(values: &ToneData) -> Result<String, serde_json::Error> {
    let events = editor_rows_to_ym2151_events(values);
    let log = Ym2151Log { events };
    serde_json::to_string_pretty(&log)
}

/// Convert tone data to JSON string with envelope reset for audio preview
#[cfg(windows)]
pub fn to_json_string_with_envelope_reset(
    values: &ToneData,
    envelope_delay_seconds: f64,
) -> Result<String, serde_json::Error> {
    let events = editor_rows_to_ym2151_events_with_envelope_reset(values, envelope_delay_seconds);
    let log = Ym2151Log { events };
    serde_json::to_string_pretty(&log)
}

/// Convert tone data to registers hex string format
/// Format: pairs of address (2 hex chars) + data (2 hex chars)
/// Example: "204F204C364037808003812D" represents 3 register writes:
/// - Register 0x20 = 0x4F
/// - Register 0x20 = 0x4C (this example shows duplicate addresses are allowed)
/// - Register 0x36 = 0x40
///   etc.
pub fn editor_rows_to_registers(values: &ToneData) -> String {
    let events = editor_rows_to_ym2151_events(values);
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
pub fn registers_to_editor_rows(registers: &str) -> io::Result<ToneData> {
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
    json_events_to_editor_rows(&events)
}
