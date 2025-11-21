//! Windows audio integration for YM2151 tone editor
//!
//! This module handles real-time audio feedback through the ym2151-log-play-server.
//! It supports two modes:
//! - Legacy JSON mode: Sends complete tone data as JSON for each change
//! - Interactive mode: Sends complete tone data as JSON using play_json_interactive API

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
    log_verbose("send_all_registers: Converting tone data to JSON");

    // Get JSON string of current tone data
    let json_string = match register::to_json_string(values) {
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
/// Sends the complete tone data as JSON to interactive mode
#[cfg(windows)]
fn send_interactive_update(values: &ToneData, _cursor_x: usize, _cursor_y: usize) {
    log_verbose("send_interactive_update: Converting tone data to JSON");

    // Get JSON string of current tone data
    let json_string = match register::to_json_string(values) {
        Ok(json) => json,
        Err(_) => {
            log_verbose("send_interactive_update: Failed to convert to JSON");
            return;
        }
    };

    log_verbose("send_interactive_update: Sending JSON to interactive mode");

    // Send JSON content to interactive mode
    let _ = ym2151_log_play_server::client::play_json_interactive(&json_string);

    log_verbose("send_interactive_update: JSON sent successfully");
}

/// Cleanup - stop interactive mode if active
#[cfg(windows)]
pub fn cleanup_interactive_mode() {
    log_verbose("cleanup_interactive_mode: Stopping interactive mode");
    let _ = ym2151_log_play_server::client::stop_interactive();
    log_verbose("cleanup_interactive_mode: Interactive mode stopped");
}
