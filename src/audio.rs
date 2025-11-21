//! Windows audio integration for YM2151 tone editor
//!
//! This module handles real-time audio feedback through the ym2151-log-play-server.
//! It supports two modes:
//! - Legacy JSON mode: Sends complete tone data as JSON for each change
//! - Interactive mode: Sends minimal JSON with only affected registers using play_json_interactive API

#[cfg(windows)]
use crate::midi_conversion::midi_to_kc_kf;
#[cfg(windows)]
use crate::models::*;
#[cfg(windows)]
use crate::register;

/// Log a verbose message (imported from main)
#[cfg(windows)]
fn log_verbose(message: &str) {
    crate::log_verbose(message);
}

/// Play current tone data through audio server
/// Routes to either interactive mode or legacy JSON mode based on settings
#[cfg(windows)]
pub fn play_tone(values: &ToneData, use_interactive_mode: bool, cursor_x: usize, cursor_y: usize) {
    if use_interactive_mode {
        log_verbose(&format!(
            "play_tone: Interactive mode - cursor_x={}, cursor_y={}",
            cursor_x, cursor_y
        ));
        send_interactive_update(values, cursor_x, cursor_y);
    } else {
        send_json_update(values);
    }
}

/// Send full JSON update (legacy mode)
/// Converts tone data to JSON and sends via named pipe to server
#[cfg(windows)]
fn send_json_update(values: &ToneData) {
    // Get JSON string of current tone data
    let json_string = match register::to_json_string(values) {
        Ok(json) => json,
        Err(_) => return, // Silently fail if JSON conversion fails
    };

    // Send JSON content to server via named pipe
    // Using the ym2151-log-play-server client library with send_json
    // Automatically chooses optimal method (direct or file-based) based on size
    let _ = ym2151_log_play_server::client::send_json(&json_string);

    // Silently ignore errors - server should be auto-started at app launch
}

/// Initialize interactive mode
/// Starts continuous audio streaming on the server and sends initial tone data
#[cfg(windows)]
pub fn init_interactive_mode(values: &ToneData) -> Result<(), Box<dyn std::error::Error>> {
    log_verbose("init_interactive_mode: Starting interactive mode");

    // Start interactive mode on the server
    ym2151_log_play_server::client::start_interactive()?;

    log_verbose("init_interactive_mode: Interactive mode started, sending all registers");

    // Send all current register values to initialize the tone
    send_all_registers(values);

    log_verbose("init_interactive_mode: Initialization complete");

    Ok(())
}

/// Send all register values in interactive mode
/// This initializes the YM2151 chip with the current tone data
#[cfg(windows)]
fn send_all_registers(values: &ToneData) {
    // Get all YM2151 events for the current tone
    let events = register::to_ym2151_events(values);

    log_verbose(&format!(
        "send_all_registers: Creating JSON with {} register writes",
        events.len()
    ));

    // Create minimal JSON with these events
    let log = Ym2151Log {
        event_count: events.len(),
        events,
    };

    let json_string = match serde_json::to_string(&log) {
        Ok(json) => json,
        Err(_) => {
            log_verbose("send_all_registers: Failed to convert to JSON");
            return;
        }
    };

    log_verbose("send_all_registers: Sending JSON to interactive mode");

    // Send JSON content to interactive mode
    let _ = ym2151_log_play_server::client::play_json_interactive(&json_string);

    log_verbose("send_all_registers: JSON sent successfully");
}

/// Send interactive update for a single parameter change
/// Only sends the specific register(s) affected by the edited parameter
#[cfg(windows)]
fn send_interactive_update(values: &ToneData, cursor_x: usize, cursor_y: usize) {
    if cursor_y == ROW_CH {
        // Channel parameter changed - send specific channel register(s)
        log_verbose(&format!(
            "send_interactive_update: Sending channel register for param {}",
            cursor_x
        ));
        send_channel_register_for_param(values, cursor_x);
    } else {
        // Operator parameter changed - send specific operator register
        let data_row = cursor_y;
        log_verbose(&format!(
            "send_interactive_update: Sending operator register for data_row={}, param={}",
            data_row, cursor_x
        ));
        send_operator_register_for_param(values, data_row, cursor_x);
    }
}

/// Send only the specific operator register(s) affected by a parameter change
#[cfg(windows)]
fn send_operator_register_for_param(values: &ToneData, data_row: usize, param_index: usize) {
    let channel = 0; // We use channel 0
    const DATA_ROW_TO_SLOT: [usize; 4] = [0, 1, 2, 3];
    let hw_slot = DATA_ROW_TO_SLOT[data_row];
    let op_offset = hw_slot * 8 + channel;

    let mut events = Vec::new();

    // Determine which register(s) to send based on the edited parameter
    match param_index {
        PARAM_SM => {
            // SM affects KEY_ON register, which is a channel register
            // Send it from here for consistency
            send_key_on_register(values, &mut events);
        }
        PARAM_TL => {
            // TL - Register $60-$7F (standalone)
            let tl = values[data_row][PARAM_TL];
            log_verbose(&format!(
                "  operator register: addr=0x{:02X}, data=0x{:02X} (TL={})",
                0x60 + op_offset as u8,
                tl & 0x7F,
                tl
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x60 + op_offset as u8),
                data: format!("0x{:02X}", tl & 0x7F),
            });
        }
        PARAM_MUL | PARAM_DT => {
            // DT1 and MUL - Register $40-$5F (shared register)
            let dt = values[data_row][PARAM_DT];
            let mul = values[data_row][PARAM_MUL];
            let dt_mul = ((dt & 0x07) << 4) | (mul & 0x0F);
            log_verbose(&format!(
                "  operator register: addr=0x{:02X}, data=0x{:02X} (DT={}, MUL={})",
                0x40 + op_offset as u8,
                dt_mul,
                dt,
                mul
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x40 + op_offset as u8),
                data: format!("0x{:02X}", dt_mul),
            });
        }
        PARAM_AR | PARAM_KS => {
            // KS and AR - Register $80-$9F (shared register)
            let ks = values[data_row][PARAM_KS];
            let ar = values[data_row][PARAM_AR];
            let ks_ar = ((ks & 0x03) << 6) | (ar & 0x1F);
            log_verbose(&format!(
                "  operator register: addr=0x{:02X}, data=0x{:02X} (KS={}, AR={})",
                0x80 + op_offset as u8,
                ks_ar,
                ks,
                ar
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x80 + op_offset as u8),
                data: format!("0x{:02X}", ks_ar),
            });
        }
        PARAM_D1R | PARAM_AMS => {
            // AMS and D1R - Register $A0-$BF (shared register)
            let ams = values[data_row][PARAM_AMS];
            let d1r = values[data_row][PARAM_D1R];
            let ams_d1r = ((ams & 0x03) << 6) | (d1r & 0x1F);
            log_verbose(&format!(
                "  operator register: addr=0x{:02X}, data=0x{:02X} (AMS={}, D1R={})",
                0xA0 + op_offset as u8,
                ams_d1r,
                ams,
                d1r
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0xA0 + op_offset as u8),
                data: format!("0x{:02X}", ams_d1r),
            });
        }
        PARAM_D2R | PARAM_DT2 => {
            // DT2 and D2R - Register $C0-$DF (shared register)
            let dt2 = values[data_row][PARAM_DT2];
            let d2r = values[data_row][PARAM_D2R];
            let dt2_d2r = ((dt2 & 0x03) << 6) | (d2r & 0x1F);
            log_verbose(&format!(
                "  operator register: addr=0x{:02X}, data=0x{:02X} (DT2={}, D2R={})",
                0xC0 + op_offset as u8,
                dt2_d2r,
                dt2,
                d2r
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0xC0 + op_offset as u8),
                data: format!("0x{:02X}", dt2_d2r),
            });
        }
        PARAM_RR | PARAM_D1L => {
            // D1L and RR - Register $E0-$FF (shared register)
            let d1l = values[data_row][PARAM_D1L];
            let rr = values[data_row][PARAM_RR];
            let d1l_rr = ((d1l & 0x0F) << 4) | (rr & 0x0F);
            log_verbose(&format!(
                "  operator register: addr=0x{:02X}, data=0x{:02X} (D1L={}, RR={})",
                0xE0 + op_offset as u8,
                d1l_rr,
                d1l,
                rr
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0xE0 + op_offset as u8),
                data: format!("0x{:02X}", d1l_rr),
            });
        }
        _ => {
            log_verbose(&format!(
                "send_operator_register_for_param: Unknown parameter index {}",
                param_index
            ));
            return;
        }
    }

    if events.is_empty() {
        return;
    }

    // Create minimal JSON with only the affected register(s)
    let log = Ym2151Log {
        event_count: events.len(),
        events,
    };

    let json_string = match serde_json::to_string(&log) {
        Ok(json) => json,
        Err(_) => {
            log_verbose("send_operator_register_for_param: Failed to convert to JSON");
            return;
        }
    };

    log_verbose(&format!(
        "send_operator_register_for_param: Sending JSON with {} event(s)",
        log.event_count
    ));

    // Send JSON content to interactive mode
    let _ = ym2151_log_play_server::client::play_json_interactive(&json_string);
}

/// Send only the specific channel register(s) affected by a parameter change
#[cfg(windows)]
fn send_channel_register_for_param(values: &ToneData, param_index: usize) {
    let channel = 0; // We use channel 0
    let mut events = Vec::new();

    // Determine which register(s) to send based on the edited parameter
    match param_index {
        CH_PARAM_ALG | CH_PARAM_FB => {
            // RL, FB, CON (Algorithm) - Register $20-$27 (shared register)
            let alg = values[ROW_CH][CH_PARAM_ALG];
            let fb = values[ROW_CH][CH_PARAM_FB];
            let rl_fb_con = 0xC0 | ((fb & 0x07) << 3) | (alg & 0x07);
            log_verbose(&format!(
                "  channel register: addr=0x{:02X}, data=0x{:02X} (ALG={}, FB={})",
                0x20 + channel as u8,
                rl_fb_con,
                alg,
                fb
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x20 + channel as u8),
                data: format!("0x{:02X}", rl_fb_con),
            });
        }
        CH_PARAM_NOTE => {
            // MIDI note affects KC, KF, and KEY_ON
            let midi_note = values[ROW_CH][CH_PARAM_NOTE];
            let (kc, kf) = midi_to_kc_kf(midi_note);

            // KC - Register $28-$2F
            log_verbose(&format!(
                "  channel register: addr=0x{:02X}, data=0x{:02X} (KC from MIDI note={})",
                0x28 + channel as u8,
                kc,
                midi_note
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x28 + channel as u8),
                data: format!("0x{:02X}", kc),
            });

            // KF - Register $30-$37
            log_verbose(&format!(
                "  channel register: addr=0x{:02X}, data=0x{:02X} (KF from MIDI note={})",
                0x30 + channel as u8,
                kf,
                midi_note
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x30 + channel as u8),
                data: format!("0x{:02X}", kf),
            });

            // Also send KEY_ON to retrigger the note
            send_key_on_register(values, &mut events);
        }
        _ => {
            log_verbose(&format!(
                "send_channel_register_for_param: Unknown parameter index {}",
                param_index
            ));
            return;
        }
    }

    if events.is_empty() {
        return;
    }

    // Create minimal JSON with only the affected register(s)
    let log = Ym2151Log {
        event_count: events.len(),
        events,
    };

    let json_string = match serde_json::to_string(&log) {
        Ok(json) => json,
        Err(_) => {
            log_verbose("send_channel_register_for_param: Failed to convert to JSON");
            return;
        }
    };

    log_verbose(&format!(
        "send_channel_register_for_param: Sending JSON with {} event(s)",
        log.event_count
    ));

    // Send JSON content to interactive mode
    let _ = ym2151_log_play_server::client::play_json_interactive(&json_string);
}

/// Helper function to add KEY_ON register to events
#[cfg(windows)]
fn send_key_on_register(values: &ToneData, events: &mut Vec<Ym2151Event>) {
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
    log_verbose(&format!(
        "  channel register: addr=0x08, data=0x{:02X} (KEY_ON, slot_mask=0x{:02X})",
        key_on, slot_mask
    ));
    events.push(Ym2151Event {
        time: 0.0,
        addr: "0x08".to_string(),
        data: format!("0x{:02X}", key_on),
    });
}

/// Cleanup - stop interactive mode if active
#[cfg(windows)]
pub fn cleanup_interactive_mode() {
    log_verbose("cleanup_interactive_mode: Stopping interactive mode");
    let _ = ym2151_log_play_server::client::stop_interactive();
    log_verbose("cleanup_interactive_mode: Interactive mode stopped");
}
