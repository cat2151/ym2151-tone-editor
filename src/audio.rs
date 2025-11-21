//! Windows audio integration for YM2151 tone editor
//!
//! This module handles real-time audio feedback through the ym2151-log-play-server.
//! It sends complete tone data as JSON for each change.

#[cfg(windows)]
use crate::models::*;
#[cfg(windows)]
use crate::register;

/// Play current tone data through audio server
/// Converts tone data to JSON and sends via named pipe to server
#[cfg(windows)]
pub fn play_tone(values: &ToneData) {
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
