//! Platform-independent YM2151 FM tone parameter model and register encoding.
//!
//! This crate is the **Single Source of Truth** for the parameter-level YM2151
//! logic shared between the native TUI application (`ym2151-tone-editor`) and the
//! WASM bindings (`ym2151-wasm`).  It covers FM tone data structures, constants,
//! random tone generation, MIDI pitch conversion, and register hex encoding.
//!
//! It has **no external dependencies** and compiles for any target including
//! `wasm32-unknown-unknown`.

use core::fmt::Write as _;

// ---------------------------------------------------------------------------
// Grid dimensions
// ---------------------------------------------------------------------------

/// Number of parameter columns in the tone grid (one per operator parameter).
pub const GRID_WIDTH: usize = 12;

/// Number of rows in the tone grid (four operators + one channel row).
pub const GRID_HEIGHT: usize = 5;

// ---------------------------------------------------------------------------
// Operator parameter column indices
// (order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Channel-row parameter column indices
// ---------------------------------------------------------------------------

pub const CH_PARAM_ALG: usize = 0;
pub const CH_PARAM_FB: usize = 1;
pub const CH_PARAM_NOTE: usize = 2;

/// Row index for the channel settings row (below the four operator rows).
pub const ROW_CH: usize = 4;

// ---------------------------------------------------------------------------
// Parameter maximum values (respecting YM2151 bit-field widths)
// (order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS)
// ---------------------------------------------------------------------------

pub const PARAM_MAX: [u8; GRID_WIDTH] = [
    1,  // SM (SlotMask): 0 or 1
    99, // TL: 7 bits (0-127, displayed limit 99)
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

// ---------------------------------------------------------------------------
// Tone data type
// ---------------------------------------------------------------------------

/// A 5×12 grid of `u8` values representing all tone parameters.
///
/// Rows 0-3 hold operator parameters; row 4 (`ROW_CH`) holds channel settings.
pub type ToneData = [[u8; GRID_WIDTH]; GRID_HEIGHT];

// ---------------------------------------------------------------------------
// Hardware register layout constants
// ---------------------------------------------------------------------------

/// Maps the user-facing operator index (O1-O4) to the YM2151 hardware register
/// slot index.  The hardware order is O1, O3, O2, O4.
pub const REG_FROM_O1_O4: [usize; 4] = [0, 2, 1, 3];

// ---------------------------------------------------------------------------
// Random tone configuration (based on web-ym2151 `getDefaultConfig`)
// ---------------------------------------------------------------------------

/// Which operators are carriers for each ALG value (0-7).
pub const CARRIERS_PER_ALG: [[bool; 4]; 8] = [
    [false, false, false, true], // ALG=0: OP4 only
    [false, false, false, true], // ALG=1: OP4 only
    [false, false, false, true], // ALG=2: OP4 only
    [false, false, false, true], // ALG=3: OP4 only
    [false, false, true, true],  // ALG=4: OP3, OP4
    [false, true, true, true],   // ALG=5: OP2, OP3, OP4
    [false, true, true, true],   // ALG=6: OP2, OP3, OP4
    [true, true, true, true],    // ALG=7: all OPs
];

/// Modulator TL value per ALG value (`stage_count × 0x08`).
pub const MODULATOR_TL_PER_ALG: [u8; 8] = [
    0x20, // ALG=0: 4 stages
    0x20, // ALG=1: 4 stages
    0x20, // ALG=2: 4 stages
    0x20, // ALG=3: 4 stages
    0x18, // ALG=4: 3 stages
    0x10, // ALG=5: 2 stages
    0x10, // ALG=6: 2 stages
    0x00, // ALG=7: no external modulators
];

// ---------------------------------------------------------------------------
// Seed-based LCG pseudo-random number generator
// ---------------------------------------------------------------------------

/// A simple Linear Congruential Generator seeded by the caller.
///
/// This design avoids any dependency on `std::time::SystemTime` (unavailable
/// in bare WASM) while still being reproducible for testing.
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    /// Create a new RNG from an externally supplied seed.
    ///
    /// In a browser context, pass `Date.now()` to get a different tone on each
    /// call.  In the native app, pass a value derived from `SystemTime`.
    pub fn from_seed(seed: u64) -> Self {
        // Mix the seed to avoid trivial fixed-points for seed=0 or seed=1.
        let state = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        SimpleRng { state }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    pub fn range(&mut self, min: u8, max: u8) -> u8 {
        if min >= max {
            return min;
        }
        let span = (max - min) as u64 + 1;
        min + (self.next_u64() % span) as u8
    }
}

// ---------------------------------------------------------------------------
// Random tone generation
// ---------------------------------------------------------------------------

/// Generate a random YM2151 tone from a caller-supplied seed.
///
/// The parameter ranges are fixed to the defaults from `web-ym2151`
/// `getDefaultConfig()`:
///
/// | Param | Range |
/// |-------|-------|
/// | ALG   | 0–7   |
/// | FB    | 0–7   |
/// | AR    | 5–31  |
/// | D1R   | 0–9   |
/// | MUL   | 0–15  |
/// | DT    | 0–7   |
/// | KS    | 0–3   |
///
/// All other parameters are set to fixed values (D1L=15, D2R=0, RR=0, DT2=0,
/// AMS=0, SM=1).  Carrier TL is always 0; modulator TL is determined by ALG.
///
/// # Parameters
/// - `seed`: An externally supplied 64-bit seed.  Different seeds produce
///   different tones; the same seed always produces the same tone.
/// - `current_note`: MIDI note number (0–127) stored in the channel row.
pub fn generate_random_tone_with_seed(seed: u64, current_note: u8) -> ToneData {
    let mut rng = SimpleRng::from_seed(seed);
    let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];

    let alg = rng.range(0, 7);
    let modulator_tl = MODULATOR_TL_PER_ALG[alg as usize];

    for (op, row) in values.iter_mut().take(4).enumerate() {
        let is_carrier = CARRIERS_PER_ALG[alg as usize][op];

        row[PARAM_SM] = 1;
        row[PARAM_TL] = if is_carrier {
            0
        } else {
            modulator_tl.min(PARAM_MAX[PARAM_TL])
        };
        row[PARAM_MUL] = rng.range(0, 15);
        row[PARAM_AR] = rng.range(5, 31);
        row[PARAM_D1R] = rng.range(0, 9);
        row[PARAM_D1L] = 15;
        row[PARAM_D2R] = 0;
        row[PARAM_RR] = 0;
        row[PARAM_DT] = rng.range(0, 7);
        row[PARAM_DT2] = 0;
        row[PARAM_KS] = rng.range(0, 3);
        row[PARAM_AMS] = 0;
    }

    values[ROW_CH][CH_PARAM_ALG] = alg;
    values[ROW_CH][CH_PARAM_FB] = rng.range(0, 7);
    values[ROW_CH][CH_PARAM_NOTE] = current_note;

    values
}

// ---------------------------------------------------------------------------
// MIDI → YM2151 pitch conversion
// ---------------------------------------------------------------------------

/// Convert a MIDI note number (0–127) to a YM2151 `(KC, KF)` pair.
///
/// The conversion mirrors `smf_to_ym2151log::midi::midi_to_kc_kf`, including
/// the octave offset and the 14-value YM2151 note encoding.  `KF` is always 0
/// (no fine tuning), matching the native app behaviour.
pub fn midi_to_kc_kf(midi_note: u8) -> (u8, u8) {
    // YM2151 encodes 12 semitones as 14 values (0–14, skipping 3, 7, 11).
    const NOTE_MAP: [u8; 12] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14];

    // Subtract 1 to align MIDI octave numbering with YM2151 octave numbering.
    let adjusted = if midi_note > 0 { midi_note - 1 } else { 0 };
    let note_in_octave = (adjusted % 12) as usize;
    let ym_octave = ((adjusted / 12) as i8 - 2).clamp(0, 7) as u8;
    let kc_note = NOTE_MAP[note_in_octave];

    ((ym_octave << 4) | kc_note, 0)
}

// ---------------------------------------------------------------------------
// Register hex encoding
// ---------------------------------------------------------------------------

/// Encode a tone grid as a register hex string.
///
/// The output is a sequence of `AADD` pairs (2-digit uppercase hex address
/// followed by 2-digit uppercase hex data) that can be sent directly to the
/// YM2151 chip or consumed by `web-ym2151`.  This is the same format produced
/// by `editor_rows_to_registers` in the native app.
pub fn editor_rows_to_registers(values: &ToneData) -> String {
    // Pre-allocate: 6 operator regs × 4 ops + RL/FB/CON + KC + KF + KEY_ON = 28 pairs × 4 chars
    let mut result = String::with_capacity(28 * 4);

    let channel: usize = 0;

    for row_id in 0..4 {
        let op_offset = REG_FROM_O1_O4[row_id] * 8 + channel;

        // DT1 (bits 6-4) and MUL (bits 3-0) – Register $40-$5F
        let dt_mul = ((values[row_id][PARAM_DT] & 0x07) << 4) | (values[row_id][PARAM_MUL] & 0x0F);
        push_reg_pair(&mut result, (0x40 + op_offset) as u8, dt_mul);

        // TL – Register $60-$7F
        push_reg_pair(
            &mut result,
            (0x60 + op_offset) as u8,
            values[row_id][PARAM_TL] & 0x7F,
        );

        // KS (bits 7-6) and AR (bits 4-0) – Register $80-$9F
        let ks_ar = ((values[row_id][PARAM_KS] & 0x03) << 6) | (values[row_id][PARAM_AR] & 0x1F);
        push_reg_pair(&mut result, (0x80 + op_offset) as u8, ks_ar);

        // AMS (bits 7-6) and D1R (bits 4-0) – Register $A0-$BF
        let ams_d1r =
            ((values[row_id][PARAM_AMS] & 0x03) << 6) | (values[row_id][PARAM_D1R] & 0x1F);
        push_reg_pair(&mut result, (0xA0 + op_offset) as u8, ams_d1r);

        // DT2 (bits 7-6) and D2R (bits 3-0) – Register $C0-$DF
        let dt2_d2r =
            ((values[row_id][PARAM_DT2] & 0x03) << 6) | (values[row_id][PARAM_D2R] & 0x0F);
        push_reg_pair(&mut result, (0xC0 + op_offset) as u8, dt2_d2r);

        // D1L (bits 7-4) and RR (bits 3-0) – Register $E0-$FF
        let d1l_rr = ((values[row_id][PARAM_D1L] & 0x0F) << 4) | (values[row_id][PARAM_RR] & 0x0F);
        push_reg_pair(&mut result, (0xE0 + op_offset) as u8, d1l_rr);
    }

    // RL/FB/CON – Register $20-$27
    let alg = values[ROW_CH][CH_PARAM_ALG];
    let fb = values[ROW_CH][CH_PARAM_FB];
    let rl_fb_con = 0xC0u8 | ((fb & 0x07) << 3) | (alg & 0x07);
    push_reg_pair(&mut result, (0x20 + channel) as u8, rl_fb_con);

    // KC (Key Code) and KF (Key Fraction) – Registers $28-$2F and $30-$37
    let midi_note = values[ROW_CH][CH_PARAM_NOTE];
    let (kc, kf) = midi_to_kc_kf(midi_note);
    push_reg_pair(&mut result, (0x28 + channel) as u8, kc);
    push_reg_pair(&mut result, (0x30 + channel) as u8, kf);

    // Key On – Register $08
    let slot_mask = if values[0][PARAM_SM] != 0 { 0x08u8 } else { 0 }
        | if values[1][PARAM_SM] != 0 { 0x10 } else { 0 }
        | if values[2][PARAM_SM] != 0 { 0x20 } else { 0 }
        | if values[3][PARAM_SM] != 0 { 0x40 } else { 0 };
    push_reg_pair(&mut result, 0x08, slot_mask | channel as u8);

    result
}

#[inline]
fn push_reg_pair(out: &mut String, addr: u8, data: u8) {
    write!(out, "{:02X}{:02X}", addr, data).unwrap();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
