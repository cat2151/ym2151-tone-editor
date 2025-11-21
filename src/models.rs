use serde::{Deserialize, Serialize};

// Grid dimensions for the UI layout
pub const GRID_WIDTH: usize = 12;
pub const GRID_HEIGHT: usize = 5;

// Parameter names for each column
// New order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
pub const PARAM_NAMES: [&str; GRID_WIDTH] = [
    "SM", "TL", "MUL", "AR", "D1R", "D1L", "D2R", "RR", "DT", "DT2", "KS", "AMS",
];

// CH row has 3 parameters: ALG, FB, and MIDI note number
pub const CH_PARAM_COUNT: usize = 3;
pub const CH_PARAM_NAMES: [&str; CH_PARAM_COUNT] = ["ALG", "FB", "Note"];

// Maximum values for each parameter (respecting YM2151 bit ranges)
// New order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
pub const PARAM_MAX: [u8; GRID_WIDTH] = [
    1,  // SM (SlotMask): 0 or 1
    99, // TL: 7 bits (0-127, limited to 99 for display)
    15, // MUL: 4 bits (0-15)
    31, // AR: 5 bits (0-31)
    31, // D1R: 5 bits (0-31)
    15, // D1L: 4 bits (0-15)
    15, // D2R: 4 bits (0-15)
    15, // RR: 4 bits (0-15)
    7,  // DT: 3 bits (0-7)
    3,  // DT2: 2 bits (0-3)
    3,  // KS: 2 bits (0-3)
    3,  // AMS: 2 bits (0-3)
];

// Maximum values for CH row parameters
pub const CH_PARAM_MAX: [u8; CH_PARAM_COUNT] = [
    7,   // ALG: 3 bits (0-7) - Algorithm
    7,   // FB: 3 bits (0-7) - Feedback
    127, // MIDI Note Number: 0-127 (60 = middle C)
];

// Row names for operators (display order: M1, C1, M2, C2)
pub const ROW_NAMES: [&str; GRID_HEIGHT] = ["M1", "C1", "M2", "C2", "CH"];

// Display row to data row mapping for operators
// Display shows: M1(row0), C1(row1), M2(row2), C2(row3)
// Internal data: M1(row0), M2(row1), C1(row2), C2(row3)
// So: Display row 0→Data row 0, Display row 1→Data row 2, Display row 2→Data row 1, Display row 3→Data row 3
pub const DISPLAY_ROW_TO_DATA_ROW: [usize; 4] = [0, 2, 1, 3];

// Parameter column indices for operator rows (matching PARAM_NAMES order)
// New order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
pub const PARAM_SM: usize = 0;
pub const PARAM_TL: usize = 1;
pub const PARAM_MUL: usize = 2;
pub const PARAM_AR: usize = 3;
pub const PARAM_D1R: usize = 4;
pub const PARAM_D1L: usize = 5;
pub const PARAM_D2R: usize = 6;
pub const PARAM_RR: usize = 7;
pub const PARAM_DT: usize = 8;
pub const PARAM_DT2: usize = 9;
pub const PARAM_KS: usize = 10;
pub const PARAM_AMS: usize = 11;

// Parameter column indices for CH row (matching CH_PARAM_NAMES order)
pub const CH_PARAM_ALG: usize = 0;
pub const CH_PARAM_FB: usize = 1;
pub const CH_PARAM_NOTE: usize = 2;

// Row index for channel settings
pub const ROW_CH: usize = 4;

/// Type alias for tone data grid
pub type ToneData = [[u8; GRID_WIDTH]; GRID_HEIGHT];

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
    pub event_count: usize,
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
