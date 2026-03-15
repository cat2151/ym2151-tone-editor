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
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // SimpleRng
    // ------------------------------------------------------------------

    #[test]
    fn test_simple_rng_deterministic() {
        let mut a = SimpleRng::from_seed(42);
        let mut b = SimpleRng::from_seed(42);
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn test_simple_rng_range_in_bounds() {
        let mut rng = SimpleRng::from_seed(999);
        for _ in 0..100 {
            let v = rng.range(5, 31);
            assert!((5..=31).contains(&v));
        }
    }

    #[test]
    fn test_simple_rng_range_equal_min_max() {
        let mut rng = SimpleRng::from_seed(1);
        assert_eq!(rng.range(7, 7), 7);
    }

    // ------------------------------------------------------------------
    // midi_to_kc_kf
    // ------------------------------------------------------------------

    #[test]
    fn test_midi_to_kc_kf_kf_always_zero() {
        for note in 0u8..=127 {
            let (_, kf) = midi_to_kc_kf(note);
            assert_eq!(kf, 0, "KF must always be 0 for MIDI note {}", note);
        }
    }

    #[test]
    fn test_midi_to_kc_kf_middle_c() {
        let (kc, _) = midi_to_kc_kf(60);
        // Middle C (MIDI 60) → adjusted=59 → octave=4, note=11 → YM2151 octave=(4-2)=2, kc_note=NOTE_MAP[11]=14
        assert_eq!(
            kc,
            (2 << 4) | 14,
            "Unexpected KC for middle C: 0x{:02X}",
            kc
        );
    }

    #[test]
    fn test_midi_to_kc_kf_a4() {
        let (kc, _) = midi_to_kc_kf(69);
        // A4 (MIDI 69) → adjusted=68 → octave=5, note=8 → YM2151 octave=(5-2)=3, kc_note=NOTE_MAP[8]=10
        assert_eq!(kc, (3 << 4) | 10, "Unexpected KC for A4: 0x{:02X}", kc);
    }

    #[test]
    fn test_midi_to_kc_kf_kc_within_valid_range() {
        for note in 0u8..=127 {
            let (kc, _) = midi_to_kc_kf(note);
            // YM2151 KC max: octave=7 (bits 6-4), note=14 (bits 3-0) → 0x7E
            assert!(
                kc <= 0x7E,
                "KC out of valid YM2151 range for MIDI {}: 0x{:02X}",
                note,
                kc
            );
        }
    }

    // ------------------------------------------------------------------
    // generate_random_tone_with_seed
    // ------------------------------------------------------------------

    #[test]
    fn test_generate_random_tone_with_seed_deterministic() {
        let a = generate_random_tone_with_seed(42, 69);
        let b = generate_random_tone_with_seed(42, 69);
        assert_eq!(a, b);
    }

    #[test]
    fn test_generate_random_tone_with_seed_different_seeds() {
        let a = generate_random_tone_with_seed(1, 69);
        let b = generate_random_tone_with_seed(2, 69);
        assert_ne!(a, b);
    }

    #[test]
    fn test_generate_random_tone_with_seed_note_preserved() {
        let tone = generate_random_tone_with_seed(0, 60);
        assert_eq!(tone[ROW_CH][CH_PARAM_NOTE], 60);
    }

    #[test]
    fn test_generate_random_tone_with_seed_alg_in_range() {
        let tone = generate_random_tone_with_seed(12345, 69);
        assert!(tone[ROW_CH][CH_PARAM_ALG] <= 7);
        assert!(tone[ROW_CH][CH_PARAM_FB] <= 7);
    }

    #[test]
    fn test_generate_random_tone_with_seed_carrier_tl_zero() {
        let tone = generate_random_tone_with_seed(7777, 69);
        let alg = tone[ROW_CH][CH_PARAM_ALG] as usize;
        for (op, row) in tone.iter().take(4).enumerate() {
            if CARRIERS_PER_ALG[alg][op] {
                assert_eq!(
                    row[PARAM_TL],
                    0,
                    "Carrier OP{} TL must be 0 for ALG={}",
                    op + 1,
                    alg
                );
            } else {
                let expected = MODULATOR_TL_PER_ALG[alg];
                assert_eq!(
                    row[PARAM_TL],
                    expected,
                    "Modulator OP{} TL mismatch for ALG={}",
                    op + 1,
                    alg
                );
            }
        }
    }

    #[test]
    fn test_generate_random_tone_with_seed_ar_in_range() {
        let tone = generate_random_tone_with_seed(42, 60);
        for (op, row) in tone.iter().take(4).enumerate() {
            assert!(
                (5..=31).contains(&row[PARAM_AR]),
                "AR out of range for OP{}: {}",
                op + 1,
                row[PARAM_AR]
            );
        }
    }

    // ------------------------------------------------------------------
    // editor_rows_to_registers
    // ------------------------------------------------------------------

    #[test]
    fn test_editor_rows_to_registers_hex_format() {
        let tone = generate_random_tone_with_seed(12345, 69);
        let regs = editor_rows_to_registers(&tone);
        assert!(!regs.is_empty());
        assert_eq!(
            regs.len() % 4,
            0,
            "Register string must be multiple of 4 chars"
        );
        assert!(
            regs.chars().all(|c| c.is_ascii_hexdigit()),
            "All chars must be hex digits: {}",
            regs
        );
    }

    #[test]
    fn test_editor_rows_to_registers_includes_kc_and_kf() {
        let tone = generate_random_tone_with_seed(1234, 69);
        let regs = editor_rows_to_registers(&tone);
        let chars: Vec<char> = regs.chars().collect();
        let mut found_kc = false;
        let mut found_kf = false;
        for chunk in chars.chunks(4) {
            let addr = u8::from_str_radix(&chunk[0..2].iter().collect::<String>(), 16).unwrap();
            let data = u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap();
            if (0x28..=0x2F).contains(&addr) {
                found_kc = true;
                assert!(data <= 0x7E, "KC out of range: 0x{:02X}", data);
            }
            if (0x30..=0x37).contains(&addr) {
                found_kf = true;
                assert_eq!(data, 0, "KF must be 0, got 0x{:02X}", data);
            }
        }
        assert!(found_kc, "KC register (0x28) missing from output");
        assert!(found_kf, "KF register (0x30) missing from output");
    }

    #[test]
    fn test_editor_rows_to_registers_deterministic() {
        let tone = generate_random_tone_with_seed(999, 60);
        let r1 = editor_rows_to_registers(&tone);
        let r2 = editor_rows_to_registers(&tone);
        assert_eq!(r1, r2);
    }
}
