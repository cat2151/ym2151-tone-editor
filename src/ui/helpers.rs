use crate::models::*;
use ratatui::style::Color;

pub(crate) fn get_operator_roles_for_alg(alg: u8) -> [bool; 4] {
    match alg {
        0 => [false, false, false, true], // O4のみキャリア
        1 => [false, false, false, true], // O4のみキャリア
        2 => [false, false, false, true], // O4のみキャリア
        3 => [false, true, false, true],  // O2,O4キャリア
        4 => [false, true, false, true],  // O2,O4キャリア
        5 => [false, true, true, true],   // O2,O3,O4キャリア
        6 => [false, true, true, true],   // O2,O3,O4キャリア
        7 => [true, true, true, true],    // 全キャリア
        _ => [false, false, false, false],
    }
}

/// Get the keybinding guide letter for a parameter column
/// Returns the uppercase letter if there's a jump keybinding for that parameter
/// Based on default keybindings from config.rs
pub(crate) fn get_key_guide(col: usize) -> Option<char> {
    match col {
        PARAM_SM => Some('O'),  // 'o'/'O' for SM (Slot Mask)
        PARAM_TL => Some('T'),  // 't'/'T' for TL (Total Level)
        PARAM_MUL => Some('M'), // 'm'/'M' for MUL
        PARAM_AR => Some('A'),  // 'a'/'A' for AR (Attack Rate)
        PARAM_D1R => Some('D'), // 'd'/'D' for D1R (Decay 1 Rate)
        PARAM_D1L => Some('L'), // 'l'/'L' for D1L (Decay 1 Level)
        PARAM_D2R => Some('S'), // 's'/'S' for D2R (Decay 2 Rate / Sustain Rate)
        PARAM_RR => Some('R'),  // 'r'/'R' for RR (Release Rate)
        PARAM_DT => Some('U'),  // 'u'/'U' for DT (Detune 1)
        PARAM_DT2 => Some('N'), // 'n'/'N' for DT2 (Detune 2)
        PARAM_KS => Some('K'),  // 'k'/'K' for KS (Key Scaling)
        PARAM_AMS => Some('I'), // 'i'/'I' for AMS (Amplitude Modulation Sensitivity)
        _ => None,
    }
}

/// Get the operator number guide for a specific row
/// Returns the operator number ('1'-'4') for operator rows (0-3)
/// Returns None for the CH row
/// Based on operator jump keybindings: '1'-'4' to jump to operators
pub(crate) fn get_operator_guide(row: usize) -> Option<char> {
    match row {
        0 => Some('1'), // O1/M1 - Operator 1
        1 => Some('2'), // O2/M2 - Operator 2
        2 => Some('3'), // O3/C1 - Operator 3
        3 => Some('4'), // O4/C2 - Operator 4
        _ => None,      // CH row has no operator number
    }
}

/// Get the keybinding guide letter for a CH row parameter column
/// Returns the uppercase letter if there's a keybinding for that parameter
/// Based on default keybindings from config.rs
pub(crate) fn get_ch_key_guide(col: usize) -> Option<char> {
    match col {
        CH_PARAM_ALG => Some('G'), // 'g'/'G' for ALG (Algorithm)
        CH_PARAM_FB => Some('F'),  // 'f'/'F' for FB (Feedback)
        _ => None,
    }
}

/// Get the color for a parameter based on its column index and row
/// Returns the color to use for both the parameter name and value
pub(crate) fn get_param_color(col: usize, is_ch_row: bool) -> Color {
    if is_ch_row {
        // CH row colors
        match col {
            CH_PARAM_ALG | CH_PARAM_FB => Color::Green, // ALG and FB: Green (same as MUL)
            _ => Color::White,
        }
    } else {
        // Operator row colors
        match col {
            PARAM_MUL => Color::Green,           // MUL: Green
            PARAM_TL | PARAM_D1L => Color::Cyan, // TL and D1L: Light Blue (Cyan)
            PARAM_AR | PARAM_D1R | PARAM_D2R | PARAM_RR => Color::Rgb(255, 165, 0), // Envelope params: Orange
            _ => Color::White,                                                      // Others: White
        }
    }
}

/// Compute key points for a YM2151 operator envelope visualization.
///
/// Returns a `Vec<(f64, f64)>` of `(time, level)` pairs, where:
/// - Time is in `[0.0, 1.0]`: 0.0 = note-on, 0.70 = note-off, 1.0 = end.
/// - Level is in `[0.0, 1.0]`: 0.0 = silent, 1.0 = maximum amplitude.
///
/// The shape is a linear approximation of the four-phase YM2151 envelope:
/// **Attack → Decay1 → Sustain/Decay2 → Release**.
///
/// Parameters used from `row`:
/// - `PARAM_AR`  (0–31): attack rate – higher = faster rise.
/// - `PARAM_D1R` (0–31): decay-1 rate – higher = faster fall toward `D1L`.
/// - `PARAM_D1L` (0–15): decay-1 level – 0 = sustain at full, 15 = decay to silence.
/// - `PARAM_D2R` (0–15): decay-2 rate – continued fall during sustain phase.
/// - `PARAM_RR`  (0–15): release rate – higher = faster fall after note-off.
/// - `PARAM_TL`  (0–99): total level – 0 = loudest, 99 = near silent.
pub(crate) fn compute_op_envelope_points(row: &[u8; GRID_WIDTH]) -> Vec<(f64, f64)> {
    let ar = row[PARAM_AR] as f64; // 0–PARAM_MAX[PARAM_AR]
    let d1r = row[PARAM_D1R] as f64; // 0–PARAM_MAX[PARAM_D1R]
    let d1l = row[PARAM_D1L] as f64; // 0–PARAM_MAX[PARAM_D1L]
    let d2r = row[PARAM_D2R] as f64; // 0–PARAM_MAX[PARAM_D2R]
    let rr = row[PARAM_RR] as f64; // 0–PARAM_MAX[PARAM_RR]
    let tl = row[PARAM_TL] as f64; // 0–PARAM_MAX[PARAM_TL]

    let ar_max = PARAM_MAX[PARAM_AR] as f64;
    let d1r_max = PARAM_MAX[PARAM_D1R] as f64;
    let d1l_max = PARAM_MAX[PARAM_D1L] as f64;
    let d2r_max = PARAM_MAX[PARAM_D2R] as f64;
    let rr_max = PARAM_MAX[PARAM_RR] as f64;
    let tl_max = PARAM_MAX[PARAM_TL] as f64;

    // Overall amplitude based on TL.
    // In YM2151, TL=0 means maximum output (no attenuation), TL=127 (99 in this editor's scale)
    // means near-silent (maximum attenuation).  The formula maps: TL=0 → 1.0 (full), TL=99 → ~0.0.
    let amplitude = (1.0 - tl / tl_max).clamp(0.0, 1.0);

    // Fixed time divisions (x axis):
    //   [0.00, 0.15]  Attack phase
    //   [0.15, 0.45]  Decay-1 phase
    //   [0.45, 0.70]  Sustain / Decay-2 phase  (note still held)
    //   [0.70, 1.00]  Release phase (note off)
    let t_attack_end = 0.15_f64;
    let t_decay1_end = 0.45_f64;
    let t_noteoff = 0.70_f64;
    let t_end = 1.00_f64;

    // Level at end of attack: AR=ar_max → reaches full amplitude; AR=0 → stays at 0.
    let attack_peak = amplitude * (ar / ar_max);

    // Target sustain level after Decay-1: D1L=0 → sustain at full; D1L=d1l_max → silence.
    let sustain_target = amplitude * (1.0 - d1l / d1l_max).max(0.0);

    // Level at end of Decay-1: D1R=d1r_max → fully reaches sustain_target; D1R=0 → stays at peak.
    let level_end_d1 = (attack_peak - (attack_peak - sustain_target) * (d1r / d1r_max)).clamp(
        sustain_target.min(attack_peak),
        attack_peak.max(sustain_target),
    );

    // Level at note-off (after Decay-2 during sustain):
    // D2R=0 → no change; D2R=d2r_max → drops by up to ~50% of the current level.
    // The 0.5 cap keeps the visualisation readable: a full D2R=d2r_max halves the level
    // over the fixed sustain window rather than driving it to zero (which would make
    // D2R and D1L visually indistinguishable for the viewer).
    let level_at_noteoff = (level_end_d1 * (1.0 - d2r / d2r_max * 0.5)).max(0.0);

    // Release: always ends at 0.0.  The line slope from (t_noteoff, level_at_noteoff)
    // to (t_end, 0.0) implicitly reflects the rate; RR=rr_max makes it steep.
    // Add a midpoint to show the RR effect: high RR → mostly gone by midpoint.
    let t_release_mid = (t_noteoff + t_end) * 0.5;
    // RR=0 → halfway through release, level is still ~100% of level_at_noteoff;
    // RR=rr_max → level is ~0% at the midpoint.
    let level_release_mid = level_at_noteoff * (1.0 - rr / rr_max);

    vec![
        (0.0, 0.0),
        (t_attack_end, attack_peak),
        (t_decay1_end, level_end_d1),
        (t_noteoff, level_at_noteoff),
        (t_release_mid, level_release_mid),
        (t_end, 0.0),
    ]
}

/// Compute the terminal row at which the envelope display area starts.
///
/// This mirrors the layout logic in [`ui`] so that callers (e.g. the event
/// loop) can position sixel graphics at exactly the same location without
/// duplicating the layout constants.
#[cfg_attr(not(windows), allow(dead_code))]
pub fn compute_envelope_area_y(alg_value: u8) -> u16 {
    // inner.y(1) + label_offset(1) + 5 operator/CH rows
    let ch_row_y: u16 = 7;
    let diagram_start_y = ch_row_y + 2;
    let diagram_len = get_algorithm_diagram(alg_value).len() as u16;
    let penta_keyboard_y = diagram_start_y + diagram_len + 1;
    penta_keyboard_y + 1 // envelope_y
}
/// Returns a vector of strings, one per line of the diagram
/// Uses O1, O2, O3, O4 notation
pub fn get_algorithm_diagram(alg: u8) -> Vec<&'static str> {
    match alg {
        0 => vec!["ALG 0: O1->O2->O3->O4->OUT", "       (Pure FM cascade)"],
        1 => vec![
            "ALG 1: O1->O2-+",
            "       O3-----+->O4->OUT",
            "       (Parallel mod)",
        ],
        2 => vec![
            "ALG 2: O1-+",
            "       O2-+->O3->O4->OUT",
            "       (Fork cascade)",
        ],
        3 => vec![
            "ALG 3: O1->O2->O4->OUT",
            "       O3--------->OUT",
            "       (Cascade+carrier)",
        ],
        4 => vec![
            "ALG 4: O1->O2->OUT",
            "       O3->O4->OUT",
            "       (Two FM pairs)",
        ],
        5 => vec![
            "ALG 5: O1->O2->OUT",
            "       O1->O3->OUT",
            "       O1->O4->OUT",
            "       (Fan out)",
        ],
        6 => vec![
            "ALG 6: O1->O2->OUT",
            "       O3------>OUT",
            "       O4------>OUT",
            "       (Cascade+carriers)",
        ],
        7 => vec![
            "ALG 7: O1->OUT",
            "       O2->OUT",
            "       O3->OUT",
            "       O4->OUT",
            "       (Additive)",
        ],
        _ => vec!["Invalid ALG"],
    }
}
