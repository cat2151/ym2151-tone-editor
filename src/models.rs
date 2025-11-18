use serde::{Deserialize, Serialize};

// Grid dimensions for the UI layout
pub const GRID_WIDTH: usize = 11;
pub const GRID_HEIGHT: usize = 5;

// Parameter names for each column
pub const PARAM_NAMES: [&str; GRID_WIDTH] = [
    "DT", "MUL", "TL", "KS", "AR", "D1R", "D1L", "D2R", "RR", "DT2", "AMS"
];

// CH row has 7 parameters: ALG, FB, 4 slot masks, and MIDI note number
pub const CH_PARAM_COUNT: usize = 7;
pub const CH_PARAM_NAMES: [&str; CH_PARAM_COUNT] = ["ALG", "FB", "M1", "C1", "M2", "C2", "Note"];

// Maximum values for each parameter (respecting YM2151 bit ranges)
pub const PARAM_MAX: [u8; GRID_WIDTH] = [
    7,   // DT: 3 bits (0-7)
    15,  // MUL: 4 bits (0-15)
    99,  // TL: 7 bits (0-127, limited to 99 for display)
    3,   // KS: 2 bits (0-3)
    31,  // AR: 5 bits (0-31)
    31,  // D1R: 5 bits (0-31)
    15,  // D1L: 4 bits (0-15)
    15,  // D2R: 4 bits (0-15)
    15,  // RR: 4 bits (0-15)
    3,   // DT2: 2 bits (0-3)
    3    // AMS: 2 bits (0-3)
];

// Maximum values for CH row parameters
pub const CH_PARAM_MAX: [u8; CH_PARAM_COUNT] = [
    7,   // ALG: 3 bits (0-7) - Algorithm
    7,   // FB: 3 bits (0-7) - Feedback
    1,   // M1 MASK: 0 or 1
    1,   // C1 MASK: 0 or 1
    1,   // M2 MASK: 0 or 1
    1,   // C2 MASK: 0 or 1
    127  // MIDI Note Number: 0-127 (60 = middle C)
];

// Row names for operators (display order: M1, C1, M2, C2)
pub const ROW_NAMES: [&str; GRID_HEIGHT] = [
    "M1", "C1", "M2", "C2", "CH"
];

// Display row to data row mapping for operators
// Display shows: M1(row0), C1(row1), M2(row2), C2(row3)
// Internal data: M1(row0), M2(row1), C1(row2), C2(row3)
// So: Display row 0→Data row 0, Display row 1→Data row 2, Display row 2→Data row 1, Display row 3→Data row 3
pub const DISPLAY_ROW_TO_DATA_ROW: [usize; 4] = [0, 2, 1, 3];
// Inverse mapping: Data row to display row
pub const DATA_ROW_TO_DISPLAY_ROW: [usize; 4] = [0, 2, 1, 3];

// Parameter column indices for operator rows (matching PARAM_NAMES order)
pub const PARAM_DT: usize = 0;
pub const PARAM_MUL: usize = 1;
pub const PARAM_TL: usize = 2;
pub const PARAM_KS: usize = 3;
pub const PARAM_AR: usize = 4;
pub const PARAM_D1R: usize = 5;
pub const PARAM_D1L: usize = 6;
pub const PARAM_D2R: usize = 7;
pub const PARAM_RR: usize = 8;
pub const PARAM_DT2: usize = 9;
pub const PARAM_AMS: usize = 10;

// Parameter column indices for CH row (matching CH_PARAM_NAMES order)
// CH_PARAM_NAMES: ALG, FB, M1-mask, C1-mask, M2-mask, C2-mask, Note
pub const CH_PARAM_ALG: usize = 0;
pub const CH_PARAM_FB: usize = 1;
pub const CH_PARAM_M1_MASK: usize = 2;
pub const CH_PARAM_C1_MASK: usize = 3;
pub const CH_PARAM_M2_MASK: usize = 4;
pub const CH_PARAM_C2_MASK: usize = 5;
pub const CH_PARAM_NOTE: usize = 6;

// Row index for channel settings
pub const ROW_CH: usize = 4;

/// Type alias for tone data grid
pub type ToneData = [[u8; GRID_WIDTH]; GRID_HEIGHT];

/// JSON event structure for ym2151-log-play-server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ym2151Event {
    pub time: u32,
    pub addr: String,
    pub data: String,
}

/// JSON log structure for ym2151-log-play-server
#[derive(Serialize, Deserialize, Debug)]
pub struct Ym2151Log {
    pub event_count: usize,
    pub events: Vec<Ym2151Event>,
}