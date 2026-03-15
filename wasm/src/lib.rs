//! WASM bindings for YM2151 tone generation.
//!
//! This crate is a thin `wasm-bindgen` wrapper around [`ym2151-tone-params`].  All
//! logic (random tone generation, register encoding, MIDI pitch conversion)
//! lives in `ym2151-tone-params` as the Single Source of Truth shared with the native
//! TUI application.
//!
//! # Usage from TypeScript
//! ```typescript
//! import init, { generate_random_tone_registers } from './ym2151_wasm.js';
//! await init();
//! const seed = Date.now();
//! const registers = generate_random_tone_registers(seed, 69); // 69 = A4, MIDI note 0–127
//! // registers: hex string e.g. "4000600080001F..."
//! ```

use wasm_bindgen::prelude::*;

/// Generate a random YM2151 tone and return it as a register hex string.
///
/// # Parameters
/// - `seed`: A numeric seed for the random number generator.  Pass `Date.now()`
///   from TypeScript to get a different tone on each call.
/// - `current_note`: MIDI note number (0–127) to embed in the tone data.
///   Pass 69 for A4 (concert pitch) if unsure.
///
/// # Returns
/// A hex string of register address/data pairs (4 chars each, e.g. `"4000..."`).
/// This is the same format used by `editor_rows_to_registers` in the native app.
#[wasm_bindgen]
pub fn generate_random_tone_registers(seed: f64, current_note: u8) -> String {
    let seed_u64 = seed.abs() as u64;
    let tone = ym2151_tone_params::generate_random_tone_with_seed(seed_u64, current_note);
    ym2151_tone_params::editor_rows_to_registers(&tone)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Decode a register hex string back into (alg, fb, ar[4], tl[4]).
    fn decode_registers(regs: &str) -> (u8, u8, [u8; 4], [u8; 4]) {
        let chars: Vec<char> = regs.chars().collect();
        assert!(chars.len().is_multiple_of(4));
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
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(result.len() % 4, 0);
    }

    #[test]
    fn test_generate_random_tone_registers_alg_in_range() {
        let result = generate_random_tone_registers(99999.0, 69);
        let (alg, fb, _, _) = decode_registers(&result);
        assert!(alg <= 7);
        assert!(fb <= 7);
    }

    #[test]
    fn test_generate_random_tone_registers_ar_in_range() {
        let result = generate_random_tone_registers(42.0, 60);
        let (_, _, ar, _) = decode_registers(&result);
        for (op, &v) in ar.iter().enumerate() {
            assert!(
                (5..=31).contains(&v),
                "AR for op {} out of range: {}",
                op,
                v
            );
        }
    }

    #[test]
    fn test_generate_random_tone_registers_carrier_tl_zero() {
        use ym2151_tone_params::{CARRIERS_PER_ALG, MODULATOR_TL_PER_ALG};
        let result = generate_random_tone_registers(7777.0, 69);
        let (alg, _, _, tl) = decode_registers(&result);
        for (op, &v) in tl.iter().enumerate() {
            if CARRIERS_PER_ALG[alg as usize][op] {
                assert_eq!(v, 0);
            } else {
                assert_eq!(v, MODULATOR_TL_PER_ALG[alg as usize]);
            }
        }
    }

    #[test]
    fn test_different_seeds_produce_different_results() {
        let r1 = generate_random_tone_registers(1.0, 69);
        let r2 = generate_random_tone_registers(2.0, 69);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_same_seed_produces_same_result() {
        let r1 = generate_random_tone_registers(100.0, 69);
        let r2 = generate_random_tone_registers(100.0, 69);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_note_is_embedded_in_kc_and_kf_registers() {
        let result = generate_random_tone_registers(1234.0, 69);
        let chars: Vec<char> = result.chars().collect();
        let mut found_kc = false;
        let mut found_kf = false;
        for chunk in chars.chunks(4) {
            let addr = u8::from_str_radix(&chunk[0..2].iter().collect::<String>(), 16).unwrap();
            let data = u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap();
            if (0x28..=0x2F).contains(&addr) {
                found_kc = true;
                assert!(
                    data <= 0x7E,
                    "KC should be in valid YM2151 range (max 0x7E), got 0x{:02X}",
                    data
                );
            }
            if (0x30..=0x37).contains(&addr) {
                found_kf = true;
                assert_eq!(data, 0);
            }
        }
        assert!(found_kc, "KC register (0x28) not found");
        assert!(found_kf, "KF register (0x30) not found");
    }

    #[test]
    fn test_seed_zero_does_not_panic() {
        let result = generate_random_tone_registers(0.0, 60);
        assert!(!result.is_empty());
    }
}
