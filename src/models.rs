use serde::{Deserialize, Serialize};

// Grid dimensions for the UI layout
pub const GRID_WIDTH: usize = 11;
pub const GRID_HEIGHT: usize = 5;

// Parameter names for each column
pub const PARAM_NAMES: [&str; GRID_WIDTH] = [
    "DT", "MUL", "TL", "KS", "AR", "D1R", "D1L", "D2R", "RR", "DT2", "AMS"
];

// CH row has 6 parameters: ALG, FB, and 4 slot masks
pub const CH_PARAM_COUNT: usize = 6;
pub const CH_PARAM_NAMES: [&str; CH_PARAM_COUNT] = ["ALG", "FB", "OP1", "OP2", "OP3", "OP4"];

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
    7,  // ALG: 3 bits (0-7) - Algorithm
    7,  // FB: 3 bits (0-7) - Feedback
    1,  // OP1 MASK: 0 or 1
    1,  // OP2 MASK: 0 or 1
    1,  // OP3 MASK: 0 or 1
    1   // OP4 MASK: 0 or 1
];

// Row names for operators
pub const ROW_NAMES: [&str; GRID_HEIGHT] = [
    "OP1", "OP2", "OP3", "OP4", "CH "
];

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
pub const CH_PARAM_ALG: usize = 0;
pub const CH_PARAM_FB: usize = 1;
pub const CH_PARAM_OP1_MASK: usize = 2;
pub const CH_PARAM_OP2_MASK: usize = 3;
pub const CH_PARAM_OP3_MASK: usize = 4;
pub const CH_PARAM_OP4_MASK: usize = 5;

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