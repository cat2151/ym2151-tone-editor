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
