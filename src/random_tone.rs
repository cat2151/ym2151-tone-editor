//! Random tone generation module.
//!
//! Thin native wrapper around `ym2151_tone_params::generate_random_tone_with_seed`.
//! The core logic (parameter ranges, RNG algorithm, carrier/modulator rules)
//! lives in `ym2151-tone-params` as the Single Source of Truth shared with the WASM crate.

use crate::models::ToneData;

/// Generate a random YM2151 tone using a platform-native random seed.
///
/// The actual generation logic (parameter ranges, LCG RNG, carrier/modulator TL
/// rules) lives in `ym2151_tone_params::generate_random_tone_with_seed`, which is the
/// Single Source of Truth shared with the WASM bindings.
///
/// - `current_note`: MIDI note number (0–127) preserved in the channel row.
pub fn generate_random_tone(current_note: u8) -> ToneData {
    use std::collections::hash_map::RandomState;
    use std::hash::BuildHasher;
    let rs = RandomState::new();
    let seed: u64 = rs.hash_one(std::time::SystemTime::now());
    ym2151_tone_params::generate_random_tone_with_seed(seed, current_note)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use ym2151_tone_params::{CARRIERS_PER_ALG, MODULATOR_TL_PER_ALG};

    #[test]
    fn test_generate_random_tone_values_in_range() {
        let current_note = 69;
        let values = generate_random_tone(current_note);

        assert_eq!(values[ROW_CH][CH_PARAM_NOTE], current_note);

        let alg = values[ROW_CH][CH_PARAM_ALG];
        assert!(alg <= 7, "ALG should be <= 7, got {}", alg);

        let fb = values[ROW_CH][CH_PARAM_FB];
        assert!(fb <= 7, "FB should be <= 7, got {}", fb);

        for (op, row) in values.iter().take(4).enumerate() {
            assert_eq!(row[PARAM_SM], 1, "SM should be 1 for op {}", op);
            assert!(row[PARAM_TL] <= 99, "TL should be <= 99 for op {}", op);
            assert!(row[PARAM_MUL] <= 15, "MUL should be <= 15 for op {}", op);
            assert!(row[PARAM_AR] >= 5, "AR should be >= 5 for op {}", op);
            assert!(row[PARAM_AR] <= 31, "AR should be <= 31 for op {}", op);
            assert!(row[PARAM_D1R] <= 9, "D1R should be <= 9 for op {}", op);
            assert_eq!(row[PARAM_D1L], 15, "D1L should be 15 for op {}", op);
            assert_eq!(row[PARAM_D2R], 0, "D2R should be 0 for op {}", op);
            assert_eq!(row[PARAM_RR], 0, "RR should be 0 for op {}", op);
            assert!(row[PARAM_DT] <= 7, "DT should be <= 7 for op {}", op);
            assert_eq!(row[PARAM_DT2], 0, "DT2 should be 0 for op {}", op);
            assert!(row[PARAM_KS] <= 3, "KS should be <= 3 for op {}", op);
            assert_eq!(row[PARAM_AMS], 0, "AMS should be 0 for op {}", op);
        }
    }

    #[test]
    fn test_carrier_operators_have_tl_zero() {
        let values = generate_random_tone(69);
        let alg = values[ROW_CH][CH_PARAM_ALG] as usize;

        for (op, row) in values.iter().take(4).enumerate() {
            if CARRIERS_PER_ALG[alg][op] {
                assert_eq!(
                    row[PARAM_TL],
                    0,
                    "Carrier OP{} should have TL=0 for ALG={}",
                    op + 1,
                    alg
                );
            } else {
                let expected_tl = MODULATOR_TL_PER_ALG[alg];
                assert_eq!(
                    row[PARAM_TL],
                    expected_tl,
                    "Modulator OP{} should have TL=0x{:02X} for ALG={}",
                    op + 1,
                    expected_tl,
                    alg
                );
            }
        }
    }

    #[test]
    fn test_note_is_preserved() {
        let note = 60;
        let values = generate_random_tone(note);
        assert_eq!(values[ROW_CH][CH_PARAM_NOTE], note);
    }
}
