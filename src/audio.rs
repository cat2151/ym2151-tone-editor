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
pub fn play_tone(
    values: &ToneData,
    use_interactive_mode: bool,
    cursor_x: usize,
    cursor_y: usize,
    envelope_delay_seconds: f64,
) {
    if use_interactive_mode {
        log_verbose(&format!(
            "play_tone: Interactive mode - cursor_x={}, cursor_y={}",
            cursor_x, cursor_y
        ));
        send_interactive_update(values, cursor_x, cursor_y, envelope_delay_seconds);
    } else {
        send_json_update(values, envelope_delay_seconds);
    }
}

/// Send full JSON update (legacy mode)
/// Converts tone data to JSON and sends via named pipe to server
#[cfg(windows)]
fn send_json_update(values: &ToneData, envelope_delay_seconds: f64) {
    // Get JSON string of current tone data with envelope reset
    let json_string =
        match register::to_json_string_with_envelope_reset(values, envelope_delay_seconds) {
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
fn send_interactive_update(
    values: &ToneData,
    cursor_x: usize,
    cursor_y: usize,
    envelope_delay_seconds: f64,
) {
    if cursor_y == ROW_CH {
        // Channel parameter changed - send specific channel register(s)
        log_verbose(&format!(
            "send_interactive_update: Sending channel register for param {}",
            cursor_x
        ));
        send_channel_register_for_param(values, cursor_x, envelope_delay_seconds);
    } else {
        // Operator parameter changed - send specific operator register
        let data_row = cursor_y;
        log_verbose(&format!(
            "send_interactive_update: Sending operator register for data_row={}, param={}",
            data_row, cursor_x
        ));
        send_operator_register_for_param(values, data_row, cursor_x, envelope_delay_seconds);
    }
}

/// Send all tone registers after envelope reset when any operator parameter changes
/// This ensures that after envelope reset (AR=31, D1R=31, D1L=15, D2R=15, RR=15),
/// all parameters are restored to their tone values before KEY_ON (fixes issue #156)
#[cfg(windows)]
fn send_operator_register_for_param(
    values: &ToneData,
    data_row: usize,
    param_index: usize,
    envelope_delay_seconds: f64,
) {
    let channel: u8 = 0; // We use channel 0

    log_verbose(&format!(
        "send_operator_register_for_param: data_row={}, param_index={} ({})",
        data_row,
        param_index,
        if param_index < PARAM_NAMES.len() {
            PARAM_NAMES[param_index]
        } else {
            "unknown"
        }
    ));

    let mut events = Vec::new();

    // Step 1: Envelope reset and KEY_OFF at time 0.0
    add_key_off(&mut events, channel, values);

    // Step 2: Restore all tone parameters at envelope_delay_seconds
    // Generate all register events with the tone values
    let mut tone_events = register::editor_rows_to_ym2151_events(values);
    
    // Set all tone events to happen after envelope delay
    for event in &mut tone_events {
        event.time = envelope_delay_seconds;
    }
    
    log_verbose(&format!(
        "  restoring all {} tone registers at time {}",
        tone_events.len(),
        envelope_delay_seconds
    ));
    
    events.extend(tone_events);

    if events.is_empty() {
        return;
    }

    // Create JSON with envelope reset + all tone parameters
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

/// Send all tone registers after envelope reset when any channel parameter changes
/// This ensures that after envelope reset (AR=31, D1R=31, D1L=15, D2R=15, RR=15),
/// all parameters are restored to their tone values before KEY_ON (fixes issue #156)
#[cfg(windows)]
fn send_channel_register_for_param(
    values: &ToneData,
    param_index: usize,
    envelope_delay_seconds: f64,
) {
    let channel = 0;

    log_verbose(&format!(
        "send_channel_register_for_param: param_index={} ({})",
        param_index,
        if param_index < CH_PARAM_NAMES.len() {
            CH_PARAM_NAMES[param_index]
        } else {
            "unknown"
        }
    ));

    let mut events = Vec::new();

    // Step 1: Envelope reset and KEY_OFF at time 0.0
    add_key_off(&mut events, channel, values);

    // Step 2: Restore all tone parameters at envelope_delay_seconds
    // Generate all register events with the tone values
    let mut tone_events = register::editor_rows_to_ym2151_events(values);
    
    // Set all tone events to happen after envelope delay
    for event in &mut tone_events {
        event.time = envelope_delay_seconds;
    }
    
    log_verbose(&format!(
        "  restoring all {} tone registers at time {}",
        tone_events.len(),
        envelope_delay_seconds
    ));
    
    events.extend(tone_events);

    if events.is_empty() {
        return;
    }

    // Create JSON with envelope reset + all tone parameters
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
/// Sets AR=31, D1R=31, D1L=15, D2R=15, RR=15 for all operators before KEY_OFF to reset envelope to 0
/// This prevents envelope continuation across notes (issue #115, issue #151)
#[cfg(windows)]
fn add_key_off(events: &mut Vec<Ym2151Event>, channel: u8, values: &ToneData) {
    // Set full ADSR envelope reset parameters for all operators to decay envelope to 0 before next note
    // AR=31, D1R=31, D1L=15, D2R=15, RR=15
    let envelope_events =
        register::generate_full_envelope_reset_events(values, channel as usize, 0.0);

    for event in &envelope_events {
        log_verbose(&format!(
            "  operator register: addr={}, data={} (full ADSR envelope reset)",
            event.addr, event.data
        ));
    }
    events.extend(envelope_events);

    log_verbose(&format!(
        "  channel register: addr=0x08, data=0x{:02X} (KEY_OFF)",
        channel
    ));
    events.push(Ym2151Event {
        time: 0.0,
        addr: "0x08".to_string(),
        data: format!("0x{:02X}", channel), // KEY_OFF: no slot mask, just channel
    });
}

/// Helper function to add KEY_ON register to events
#[cfg(windows)]
pub fn add_key_on(values: &ToneData, events: &mut Vec<Ym2151Event>, envelope_delay_seconds: f64) {
    let channel = 0; // We use channel 0

    // Calculate slot mask based on which operators are enabled
    // YM2151 KEY_ON register (0x08) uses hardware slot order: M1, C1, M2, C2 (bits 3-6)
    // which corresponds to display rows: 0, 2, 1, 3 (using REG_FROM_O1_O4 mapping)
    let sm0 = values[0][PARAM_SM]; // M1 -> bit 3
    let sm1 = values[1][PARAM_SM]; // M2 -> bit 5 (hardware slot 2)
    let sm2 = values[2][PARAM_SM]; // C1 -> bit 4 (hardware slot 1)
    let sm3 = values[3][PARAM_SM]; // C2 -> bit 6

    let slot_mask = if sm0 != 0 { 0x08 } else { 0 }  // M1 at bit 3
        | if sm2 != 0 { 0x10 } else { 0 }            // C1 at bit 4 (row 2)
        | if sm1 != 0 { 0x20 } else { 0 }            // M2 at bit 5 (row 1)
        | if sm3 != 0 { 0x40 } else { 0 };           // C2 at bit 6

    let key_on = slot_mask | channel as u8;
    log_verbose(&format!(
        "  channel register: addr=0x08, data=0x{:02X} (KEY_ON, slot_mask=0x{:02X})",
        key_on, slot_mask
    ));
    events.push(Ym2151Event {
        time: envelope_delay_seconds,
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
