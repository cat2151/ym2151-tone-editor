//! WASM bindings for YM2151 tone generation.
//!
//! Exposes `generate_random_tone_registers` for use from TypeScript/JavaScript
//! in browser environments (web-ym2151 and similar projects).
//!
//! # Usage from TypeScript
//! ```typescript
//! import init, { generate_random_tone_registers } from './ym2151_wasm.js';
//! await init();
//! const seed = Date.now();
//! const registers = generate_random_tone_registers(seed);
//! // registers: hex string e.g. "4000600080001F..."
//! ```

use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// YM2151 parameter constants (mirrored from models.rs)
// ---------------------------------------------------------------------------

const GRID_WIDTH: usize = 12;
const GRID_HEIGHT: usize = 5;

// Column indices – order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
const PARAM_SM: usize = 0;
const PARAM_TL: usize = 1;
const PARAM_MUL: usize = 2;
const PARAM_AR: usize = 3;
const PARAM_D1R: usize = 4;
const PARAM_D1L: usize = 5;
const PARAM_D2R: usize = 6;
const PARAM_RR: usize = 7;
const PARAM_DT: usize = 8;
const PARAM_DT2: usize = 9;
const PARAM_KS: usize = 10;
const PARAM_AMS: usize = 11;

// CH row parameter indices – order: ALG, FB, Note
const CH_PARAM_ALG: usize = 0;
const CH_PARAM_FB: usize = 1;
const CH_PARAM_NOTE: usize = 2;

// Row index for channel settings
const ROW_CH: usize = 4;

// Maximum values for each operator parameter
const PARAM_MAX: [u8; GRID_WIDTH] = [
    1,  // SM
    99, // TL
    15, // MUL
    31, // AR
    31, // D1R
    15, // D1L
    15, // D2R
    15, // RR
    7,  // DT
    3,  // DT2
    3,  // KS
    3,  // AMS
];

// YM2151 hardware operator register order: O1, O3, O2, O4
const REG_FROM_O1_O4: [usize; 4] = [0, 2, 1, 3];

// ---------------------------------------------------------------------------
// Random tone configuration (mirrored from random_tone.rs defaults)
// ---------------------------------------------------------------------------

/// Carrier operator flags per ALG value (0-7), indexed by [alg][operator_index].
const CARRIERS_PER_ALG: [[bool; 4]; 8] = [
    [false, false, false, true], // ALG=0: OP4 only
    [false, false, false, true], // ALG=1: OP4 only
    [false, false, false, true], // ALG=2: OP4 only
    [false, false, false, true], // ALG=3: OP4 only
    [false, false, true, true],  // ALG=4: OP3, OP4
    [false, true, true, true],   // ALG=5: OP2, OP3, OP4
    [false, true, true, true],   // ALG=6: OP2, OP3, OP4
    [true, true, true, true],    // ALG=7: all OPs
];

/// Modulator TL value per ALG value (stage_count * 0x08).
const MODULATOR_TL_PER_ALG: [u8; 8] = [
    0x20, // ALG=0
    0x20, // ALG=1
    0x20, // ALG=2
    0x20, // ALG=3
    0x18, // ALG=4
    0x10, // ALG=5
    0x10, // ALG=6
    0x00, // ALG=7
];

// ---------------------------------------------------------------------------
// Seed-based LCG pseudo-random number generator (WASM-compatible)
// ---------------------------------------------------------------------------

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    /// Create a new RNG from an externally supplied seed.
    ///
    /// In a browser, callers should pass `Date.now()` or a similar value to
    /// obtain different tones on each call.
    fn from_seed(seed: u64) -> Self {
        // Mix the seed to avoid trivial fixed points for seed=0 or seed=1.
        let state = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        SimpleRng { state }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn range(&mut self, min: u8, max: u8) -> u8 {
        if min >= max {
            return min;
        }
        let span = (max - min) as u64 + 1;
        min + (self.next_u64() % span) as u8
    }
}

// ---------------------------------------------------------------------------
// Tone data type alias
// ---------------------------------------------------------------------------

type ToneData = [[u8; GRID_WIDTH]; GRID_HEIGHT];

// ---------------------------------------------------------------------------
// Random tone generation
// ---------------------------------------------------------------------------

/// Generate a random tone grid from an externally supplied seed.
///
/// This is the WASM-compatible equivalent of `generate_random_tone` in
/// `random_tone.rs`.  Instead of seeding the RNG from `SystemTime` (which is
/// unavailable in WASM), it uses a caller-supplied `u64` seed.
fn generate_random_tone_with_seed(seed: u64, current_note: u8) -> ToneData {
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
// Register encoding (mirrored from register.rs)
// ---------------------------------------------------------------------------

fn editor_rows_to_registers(values: &ToneData) -> String {
    let mut result = String::new();

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

    // KC (Key Code) – Register $28-$2F
    let midi_note = values[ROW_CH][CH_PARAM_NOTE];
    let kc = midi_to_kc(midi_note);
    push_reg_pair(&mut result, (0x28 + channel) as u8, kc);

    // Key On – Register $08 (all slots enabled, channel 0)
    let sm0 = values[0][PARAM_SM];
    let sm1 = values[1][PARAM_SM];
    let sm2 = values[2][PARAM_SM];
    let sm3 = values[3][PARAM_SM];
    let slot_mask = if sm0 != 0 { 0x08u8 } else { 0 }
        | if sm1 != 0 { 0x10 } else { 0 }
        | if sm2 != 0 { 0x20 } else { 0 }
        | if sm3 != 0 { 0x40 } else { 0 };
    push_reg_pair(&mut result, 0x08, slot_mask | channel as u8);

    result
}

#[inline]
fn push_reg_pair(out: &mut String, addr: u8, data: u8) {
    out.push_str(&format!("{:02X}{:02X}", addr, data));
}

/// Minimal MIDI note → YM2151 KC conversion (mirrored from midi_conversion.rs /
/// smf_to_ym2151log `midi_to_kc_kf`).
///
/// Returns the KC byte for the given MIDI note number (0-127).
fn midi_to_kc(midi_note: u8) -> u8 {
    // YM2151 KC format: bits 6-4 = octave (0-7), bits 3-0 = note in YM2151 encoding.
    // YM2151 uses 14 note values (0-14, skipping 3, 7, 11) to represent 12 semitones.
    const NOTE_MAP: [u8; 12] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14];

    // Adjust MIDI note by -1 to align octaves between MIDI and YM2151 numbering
    let adjusted = if midi_note > 0 { midi_note - 1 } else { 0 };
    let note_in_octave = (adjusted % 12) as usize;
    let ym_octave = ((adjusted / 12) as i8 - 2).clamp(0, 7) as u8;
    let kc_note = NOTE_MAP[note_in_octave];

    (ym_octave << 4) | kc_note
}

// ---------------------------------------------------------------------------
// WASM-exported functions
// ---------------------------------------------------------------------------

/// Generate a random YM2151 tone and return it as a register hex string.
///
/// # Parameters
/// - `seed`: A numeric seed for the random number generator. Pass `Date.now()`
///   from TypeScript to get a different tone on each call.
/// - `current_note`: MIDI note number (0-127) to embed in the tone data.
///   Pass 69 for A4 (concert pitch) if unsure.
///
/// # Returns
/// A hex string of register address/data pairs (4 chars each, e.g. `"4000..."`).
/// This is the same format used by `editor_rows_to_registers` in the native app.
#[wasm_bindgen]
pub fn generate_random_tone_registers(seed: f64, current_note: u8) -> String {
    let seed_u64 = seed.abs() as u64;
    let tone = generate_random_tone_with_seed(seed_u64, current_note);
    editor_rows_to_registers(&tone)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Decode a register hex string back into operator/channel parameters for
    /// assertion purposes.  Returns (alg, fb, [ar; 4], [tl; 4]).
    fn decode_registers(regs: &str) -> (u8, u8, [u8; 4], [u8; 4]) {
        let chars: Vec<char> = regs.chars().collect();
        assert!(
            chars.len().is_multiple_of(4),
            "register string length must be multiple of 4"
        );

        let mut alg = 0u8;
        let mut fb = 0u8;
        let mut ar = [0u8; 4];
        let mut tl = [0u8; 4];

        for chunk in chars.chunks(4) {
            let addr = u8::from_str_radix(&chunk[0..2].iter().collect::<String>(), 16).unwrap();
            let data = u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap();

            match addr {
                0x20..=0x27 => {
                    alg = data & 0x07;
                    fb = (data >> 3) & 0x07;
                }
                0x60..=0x7F => {
                    let reg = ((addr - 0x60) / 8) as usize;
                    if reg < 4 {
                        // REG_FROM_O1_O4 maps op→reg; O1_O4_FROM_REG is the inverse (same values)
                        let op = [0usize, 2, 1, 3][reg];
                        tl[op] = data & 0x7F;
                    }
                }
                0x80..=0x9F => {
                    let reg = ((addr - 0x80) / 8) as usize;
                    if reg < 4 {
                        let op = [0usize, 2, 1, 3][reg];
                        ar[op] = data & 0x1F;
                    }
                }
                _ => {}
            }
        }

        (alg, fb, ar, tl)
    }

    #[test]
    fn test_generate_random_tone_registers_returns_hex_string() {
        let result = generate_random_tone_registers(12345.0, 69);
        assert!(!result.is_empty());
        // All characters must be valid hex digits (uppercase)
        assert!(
            result.chars().all(|c| c.is_ascii_hexdigit()),
            "Expected all hex digits, got: {}",
            result
        );
        // Each register pair is 4 characters (addr 2 + data 2)
        assert_eq!(result.len() % 4, 0);
    }

    #[test]
    fn test_generate_random_tone_registers_alg_in_range() {
        let result = generate_random_tone_registers(99999.0, 69);
        let (alg, fb, _, _) = decode_registers(&result);
        assert!(alg <= 7, "ALG should be <= 7, got {}", alg);
        assert!(fb <= 7, "FB should be <= 7, got {}", fb);
    }

    #[test]
    fn test_generate_random_tone_registers_ar_in_range() {
        let result = generate_random_tone_registers(42.0, 60);
        let (_, _, ar, _) = decode_registers(&result);
        for (op, &v) in ar.iter().enumerate() {
            assert!(
                (5..=31).contains(&v),
                "AR for op {} should be in [5,31], got {}",
                op,
                v
            );
        }
    }

    #[test]
    fn test_generate_random_tone_registers_carrier_tl_zero() {
        let result = generate_random_tone_registers(7777.0, 69);
        let (alg, _, _, tl) = decode_registers(&result);
        for (op, &v) in tl.iter().enumerate() {
            if CARRIERS_PER_ALG[alg as usize][op] {
                assert_eq!(v, 0, "Carrier OP{} TL should be 0 for ALG={}", op + 1, alg);
            } else {
                let expected = MODULATOR_TL_PER_ALG[alg as usize];
                assert_eq!(
                    v,
                    expected,
                    "Modulator OP{} TL should be {} for ALG={}",
                    op + 1,
                    expected,
                    alg
                );
            }
        }
    }

    #[test]
    fn test_different_seeds_produce_different_results() {
        let r1 = generate_random_tone_registers(1.0, 69);
        let r2 = generate_random_tone_registers(2.0, 69);
        assert_ne!(r1, r2, "Different seeds should produce different tones");
    }

    #[test]
    fn test_same_seed_produces_same_result() {
        let r1 = generate_random_tone_registers(100.0, 69);
        let r2 = generate_random_tone_registers(100.0, 69);
        assert_eq!(r1, r2, "Same seed should produce the same tone");
    }

    #[test]
    fn test_note_is_embedded_in_kc_register() {
        // Generate with note=69 (A4) and verify the KC register contains a plausible value
        let result = generate_random_tone_registers(1234.0, 69);
        let chars: Vec<char> = result.chars().collect();
        let mut found_kc = false;
        for chunk in chars.chunks(4) {
            let addr = u8::from_str_radix(&chunk[0..2].iter().collect::<String>(), 16).unwrap();
            if (0x28..=0x2F).contains(&addr) {
                found_kc = true;
                let kc = u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap();
                // MIDI 69 = A4; KC should be non-zero and within valid range
                assert!(
                    kc <= 0x77,
                    "KC should be in valid YM2151 range, got 0x{:02X}",
                    kc
                );
            }
        }
        assert!(found_kc, "KC register (0x28) not found in output");
    }

    #[test]
    fn test_seed_zero_does_not_panic() {
        let result = generate_random_tone_registers(0.0, 60);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_simple_rng_from_seed() {
        let mut rng = SimpleRng::from_seed(42);
        let v1 = rng.next_u64();
        let mut rng2 = SimpleRng::from_seed(42);
        let v2 = rng2.next_u64();
        assert_eq!(v1, v2, "Same seed should produce same RNG sequence");
    }

    #[test]
    fn test_simple_rng_range() {
        let mut rng = SimpleRng::from_seed(999);
        for _ in 0..100 {
            let v = rng.range(5, 31);
            assert!((5..=31).contains(&v), "range(5,31) out of bounds: {}", v);
        }
    }
}
