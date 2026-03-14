//! Random tone generation module.
//!
//! Implements random tone parameter generation based on web-ym2151 random-tone logic.
//! The configuration is stored internally as fixed values (future: external JSON edit).

use crate::models::*;

/// Parameter range for random value generation
pub struct ParamRange {
    pub min: u8,
    pub max: u8,
}

/// Random tone configuration based on web-ym2151 getDefaultConfig()
pub struct RandomToneConfig {
    pub ar: ParamRange,
    pub d1r: ParamRange,
    pub d2r: ParamRange,
    pub rr: ParamRange,
    pub d1l: ParamRange,
    pub ks: ParamRange,
    pub mul: ParamRange,
    pub dt: ParamRange,
    pub alg: ParamRange,
    pub fb: ParamRange,
}

impl Default for RandomToneConfig {
    fn default() -> Self {
        RandomToneConfig {
            ar: ParamRange { min: 5, max: 31 },
            d1r: ParamRange { min: 0, max: 9 },
            d2r: ParamRange { min: 0, max: 0 },
            rr: ParamRange { min: 0, max: 0 },
            d1l: ParamRange { min: 15, max: 15 },
            ks: ParamRange { min: 0, max: 3 },
            mul: ParamRange { min: 0, max: 15 },
            dt: ParamRange { min: 0, max: 7 },
            alg: ParamRange { min: 0, max: 7 },
            fb: ParamRange { min: 0, max: 7 },
        }
    }
}

/// Carrier operator flags per ALG value (0-7), indexed by [alg][operator_index].
/// Based on web-ym2151 CARRIERS_PER_CON.
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
/// Based on web-ym2151 MODULATOR_TL_PER_CON.
const MODULATOR_TL_PER_ALG: [u8; 8] = [
    0x20, // ALG=0: stage count 4
    0x20, // ALG=1: stage count 4
    0x20, // ALG=2: stage count 4
    0x20, // ALG=3: stage count 4
    0x18, // ALG=4: stage count 3
    0x10, // ALG=5: stage count 2
    0x10, // ALG=6: stage count 2
    0x00, // ALG=7: no external modulators
];

/// Simple LCG pseudo-random number generator
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        use std::collections::hash_map::RandomState;
        use std::hash::BuildHasher;
        let rs = RandomState::new();
        SimpleRng {
            state: rs.hash_one(std::time::SystemTime::now()),
        }
    }

    fn next_u64(&mut self) -> u64 {
        // LCG parameters from Numerical Recipes
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

/// Generate a random tone based on web-ym2151 random-tone logic.
///
/// - Randomizes ALG and FB for the channel row
/// - For each operator, TL is set based on carrier/modulator role (determined by ALG)
/// - Other operator parameters are randomized within config ranges
/// - SM is set to 1 (all slots enabled), DT2 and AMS are set to 0
/// - The Note (MIDI note number) is preserved from `current_note`
pub fn generate_random_tone(config: &RandomToneConfig, current_note: u8) -> ToneData {
    let mut rng = SimpleRng::new();
    let mut values = [[0u8; GRID_WIDTH]; GRID_HEIGHT];

    let alg = rng.range(config.alg.min, config.alg.max).min(7);
    let modulator_tl = MODULATOR_TL_PER_ALG[alg as usize];

    for (op, row) in values.iter_mut().take(4).enumerate() {
        let is_carrier = CARRIERS_PER_ALG[alg as usize][op];

        // SM = 1 (all slots enabled)
        row[PARAM_SM] = 1;

        // TL: carrier = 0, modulator = modulator_tl (capped at PARAM_MAX[PARAM_TL] = 99)
        row[PARAM_TL] = if is_carrier {
            0
        } else {
            modulator_tl.min(PARAM_MAX[PARAM_TL])
        };

        row[PARAM_MUL] = rng.range(config.mul.min, config.mul.max);
        row[PARAM_AR] = rng.range(config.ar.min, config.ar.max);
        row[PARAM_D1R] = rng.range(config.d1r.min, config.d1r.max);
        row[PARAM_D1L] = rng.range(config.d1l.min, config.d1l.max);
        row[PARAM_D2R] = rng.range(config.d2r.min, config.d2r.max);
        row[PARAM_RR] = rng.range(config.rr.min, config.rr.max);
        row[PARAM_DT] = rng.range(config.dt.min, config.dt.max);
        row[PARAM_DT2] = 0; // Not in web-ym2151 random config
        row[PARAM_KS] = rng.range(config.ks.min, config.ks.max);
        row[PARAM_AMS] = 0; // Not in web-ym2151 random config
    }

    values[ROW_CH][CH_PARAM_ALG] = alg;
    values[ROW_CH][CH_PARAM_FB] = rng.range(config.fb.min, config.fb.max);
    values[ROW_CH][CH_PARAM_NOTE] = current_note;

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_tone_values_in_range() {
        let config = RandomToneConfig::default();
        let current_note = 69;
        let values = generate_random_tone(&config, current_note);

        // Note is preserved
        assert_eq!(values[ROW_CH][CH_PARAM_NOTE], current_note);

        // ALG is in range
        let alg = values[ROW_CH][CH_PARAM_ALG];
        assert!(alg <= 7, "ALG should be <= 7, got {}", alg);

        // FB is in range
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
        let config = RandomToneConfig::default();
        let values = generate_random_tone(&config, 69);
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
        let config = RandomToneConfig::default();
        let note = 60;
        let values = generate_random_tone(&config, note);
        assert_eq!(values[ROW_CH][CH_PARAM_NOTE], note);
    }
}
