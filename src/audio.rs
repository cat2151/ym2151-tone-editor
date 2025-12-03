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

#[cfg(windows)]
fn handle_server_send_error(context: &str, err: &dyn std::fmt::Display) {
    let msg = format!("{}: サーバー送信失敗: {}", context, err);
    eprintln!("{}", msg);
    log_verbose(&msg);
    std::process::exit(1);
}

/// Log a verbose message (imported from main)
#[cfg(windows)]
pub(crate) fn log_verbose(message: &str) {
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
    // Get JSON string of current tone data with envelope reset
    let json_string = match register::to_json_string_with_envelope_reset(values) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("send_json_update: JSON変換失敗: {}", e);
            std::process::exit(1);
        }
    };

    // サーバーへJSON送信（失敗時はprintして即終了）
    match ym2151_log_play_server::client::send_json(&json_string) {
        Ok(_) => {}
        Err(e) => handle_server_send_error("send_json_update", &e),
    }
}

/// Initialize interactive mode
/// Starts continuous audio streaming on the server and sends initial tone data
#[cfg(windows)]
pub fn init_interactive_mode(values: &ToneData) -> Result<(), Box<dyn std::error::Error>> {
    let is_interactive =
        match ym2151_log_play_server::client::get_interactive_mode_state_with_retry() {
            Ok(val) => val,
            Err(e) => {
                let msg = format!(
                    "get_interactive_mode_state_with_retry: サーバー状態取得失敗: {}",
                    e
                );
                eprintln!("{}", msg);
                log_verbose(&msg);
                std::process::exit(1);
            }
        };

    if !is_interactive {
        log_verbose("init_interactive_mode: Starting interactive mode");
        ym2151_log_play_server::client::start_interactive()?;
    }

    log_verbose("init_interactive_mode: sending all registers");
    send_all_registers(values);
    log_verbose("init_interactive_mode: Initialization complete");

    Ok(())
}

/// Send all register values in interactive mode
/// This initializes the YM2151 chip with the current tone data
#[cfg(windows)]
fn send_all_registers(values: &ToneData) {
    // Get all YM2151 events for the current tone
    let events = register::editor_rows_to_ym2151_events(values);

    log_verbose(&format!(
        "send_all_registers: Creating JSON with {} register writes",
        events.len()
    ));

    // Create minimal JSON with these events
    let log = Ym2151Log { events };

    let json_string = match serde_json::to_string(&log) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("send_all_registers: JSON変換失敗: {}", e);
            std::process::exit(1);
        }
    };

    log_verbose("send_all_registers: Sending JSON to interactive mode");

    // インタラクティブモードへJSON送信（失敗時はprintして即終了）
    match ym2151_log_play_server::client::play_json_interactive(&json_string) {
        Ok(_) => log_verbose("send_all_registers: JSON sent successfully"),
        Err(e) => handle_server_send_error("send_all_registers", &e),
    }
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
    let channel: u8 = 0; // We use channel 0
    let reg = register::REG_FROM_O1_O4[data_row];
    let op_offset = reg * 8 + channel as usize;

    // let _ = ym2151_log_play_server::client::clear_schedule();

    let mut events = Vec::new();

    add_key_off(&mut events, channel, values);

    // Determine which register(s) to send based on the edited parameter
    match param_index {
        PARAM_SM => {
            // SM affects KEY_ON register, which is handled at the end
            // No additional register needed here
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

    add_key_on(values, &mut events);

    if events.is_empty() {
        return;
    }

    // Create minimal JSON with only the affected register(s)
    let log = Ym2151Log { events };

    let json_string = match serde_json::to_string(&log) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("send_operator_register_for_param: JSON変換失敗: {}", e);
            std::process::exit(1);
        }
    };

    // インタラクティブモードへJSON送信（失敗時はprintして即終了）
    match ym2151_log_play_server::client::play_json_interactive(&json_string) {
        Ok(_) => {}
        Err(e) => handle_server_send_error("send_operator_register_for_param", &e),
    }
}

/// Send only the specific channel register(s) affected by a parameter change
#[cfg(windows)]
fn send_channel_register_for_param(values: &ToneData, param_index: usize) {
    let channel = 0;

    // let _ = ym2151_log_play_server::client::clear_schedule();

    let mut events = Vec::new();

    add_key_off(&mut events, channel, values);

    // Determine which register(s) to send based on the edited parameter
    match param_index {
        CH_PARAM_ALG | CH_PARAM_FB => {
            // RL, FB, CON (Algorithm) - Register $20-$27 (shared register)
            let alg = values[ROW_CH][CH_PARAM_ALG];
            let fb = values[ROW_CH][CH_PARAM_FB];
            let rl_fb_con = 0xC0 | ((fb & 0x07) << 3) | (alg & 0x07);
            log_verbose(&format!(
                "  channel register: addr=0x{:02X}, data=0x{:02X} (ALG={}, FB={})",
                0x20 + channel,
                rl_fb_con,
                alg,
                fb
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x20 + channel),
                data: format!("0x{:02X}", rl_fb_con),
            });
        }
        CH_PARAM_NOTE => {
            // MIDI note affects KC, KF
            let midi_note = values[ROW_CH][CH_PARAM_NOTE];
            let (kc, kf) = midi_to_kc_kf(midi_note);

            // KC - Register $28-$2F
            log_verbose(&format!(
                "  channel register: addr=0x{:02X}, data=0x{:02X} (KC from MIDI note={})",
                0x28 + channel,
                kc,
                midi_note
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x28 + channel),
                data: format!("0x{:02X}", kc),
            });

            // KF - Register $30-$37
            log_verbose(&format!(
                "  channel register: addr=0x{:02X}, data=0x{:02X} (KF from MIDI note={})",
                0x30 + channel,
                kf,
                midi_note
            ));
            events.push(Ym2151Event {
                time: 0.0,
                addr: format!("0x{:02X}", 0x30 + channel),
                data: format!("0x{:02X}", kf),
            });
        }
        _ => {
            log_verbose(&format!(
                "send_channel_register_for_param: Unknown parameter index {}",
                param_index
            ));
            return;
        }
    }

    add_key_on(values, &mut events);

    if events.is_empty() {
        return;
    }

    // Create minimal JSON with only the affected register(s)
    let log = Ym2151Log { events };

    let json_string = match serde_json::to_string(&log) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("send_channel_register_for_param: JSON変換失敗: {}", e);
            std::process::exit(1);
        }
    };

    // インタラクティブモードへJSON送信（失敗時はprintして即終了）
    match ym2151_log_play_server::client::play_json_interactive(&json_string) {
        Ok(_) => {}
        Err(e) => handle_server_send_error("send_channel_register_for_param", &e),
    }
}

/// Helper function to add KEY_OFF (note-off) register to events
/// Sets D2R=15 for all operators before KEY_OFF to reset envelope to 0
#[cfg(windows)]
fn add_key_off(events: &mut Vec<Ym2151Event>, channel: u8, values: &ToneData) {
    // Set D2R=15 for all operators to decay envelope to 0 before next note
    // This prevents envelope continuation across notes (issue #115)
    for row_id in 0..4 {
        let reg = register::REG_FROM_O1_O4[row_id];
        let op_offset = reg * 8 + channel as usize;
        
        // Get current DT2 value from values to preserve it
        let dt2 = values[row_id][PARAM_DT2];
        // Set D2R to 15 (maximum decay rate)
        let dt2_d2r = ((dt2 & 0x03) << 6) | 0x0F;
        
        log_verbose(&format!(
            "  operator register: addr=0x{:02X}, data=0x{:02X} (DT2={}, D2R=15 for envelope reset)",
            0xC0 + op_offset as u8,
            dt2_d2r,
            dt2
        ));
        events.push(Ym2151Event {
            time: 0.0,
            addr: format!("0x{:02X}", 0xC0 + op_offset as u8),
            data: format!("0x{:02X}", dt2_d2r),
        });
    }
    
    log_verbose(&format!(
        "  channel register: addr=0x08, data=0x{:02X} (KEY_OFF)",
        channel
    ));
    events.push(Ym2151Event {
        time: 0.005, // Wait 5ms for envelope to decay to 0
        addr: "0x08".to_string(),
        data: format!("0x{:02X}", channel), // KEY_OFF: no slot mask, just channel
    });
}

/// Helper function to add KEY_ON register to events
#[cfg(windows)]
pub fn add_key_on(values: &ToneData, events: &mut Vec<Ym2151Event>) {
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
