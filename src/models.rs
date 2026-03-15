use serde::{Deserialize, Serialize};

// Re-export platform-independent constants and types from ym2151-tone-params (Single Source of Truth).
// Both this crate and the WASM crate depend on ym2151-tone-params; these re-exports keep
// existing call-sites (`use crate::models::*`) working without any change.
pub use ym2151_tone_params::ToneData;
pub use ym2151_tone_params::{
    CH_PARAM_ALG, CH_PARAM_FB, CH_PARAM_NOTE, GRID_HEIGHT, GRID_WIDTH, PARAM_AMS, PARAM_AR,
    PARAM_D1L, PARAM_D1R, PARAM_D2R, PARAM_DT, PARAM_DT2, PARAM_KS, PARAM_MAX, PARAM_MUL, PARAM_RR,
    PARAM_SM, PARAM_TL, ROW_CH,
};

// Parameter names for each column (UI display only; not needed by ym2151-tone-params)
pub const PARAM_NAMES: [&str; GRID_WIDTH] = [
    "SM", "TL", "MUL", "AR", "D1R", "D1L", "D2R", "RR", "DT", "DT2", "KS", "AMS",
];

// CH row has 3 parameters: ALG, FB, and MIDI note number
pub const CH_PARAM_COUNT: usize = 3;
pub const CH_PARAM_NAMES: [&str; CH_PARAM_COUNT] = ["ALG", "FB", "Note"];

// Maximum values for CH row parameters
pub const CH_PARAM_MAX: [u8; CH_PARAM_COUNT] = [
    7,   // ALG: 3 bits (0-7) - Algorithm
    7,   // FB: 3 bits (0-7) - Feedback
    127, // MIDI Note Number: 0-127 (60 = middle C)
];

// Row names for operators
pub const ROW_NAMES: [&str; GRID_HEIGHT] = ["O1", "O2", "O3", "O4", "CH"];

/// Default envelope delay in seconds before tone parameters are set
/// This is the default value used when no configuration is provided
pub const DEFAULT_ENVELOPE_DELAY_SECONDS: f64 = 0.01;

/// JSON event structure for ym2151-log-play-server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ym2151Event {
    pub time: f64,
    pub addr: String,
    pub data: String,
}

/// JSON log structure for ym2151-log-play-server
#[derive(Serialize, Deserialize, Debug)]
pub struct Ym2151Log {
    pub events: Vec<Ym2151Event>,
}

/// Tone variation structure for General MIDI tone files
/// Represents a single tone variation with optional MML or note number for playback
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToneVariation {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_number: Option<u8>,
    pub registers: String,
}

/// Tone file structure for General MIDI tone files
/// Contains a description and array of tone variations
#[derive(Serialize, Deserialize, Debug)]
pub struct ToneFile {
    pub description: String,
    pub variations: Vec<ToneVariation>,
}
