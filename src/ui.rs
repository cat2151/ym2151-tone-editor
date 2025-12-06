use crate::{app::App, models::*};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Background color for shortcut key guides
const KEY_GUIDE_BG_COLOR: Color = Color::Rgb(40, 40, 40);

pub fn get_operator_roles_for_alg(alg: u8) -> [bool; 4] {
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

/// Get ASCII art diagram for YM2151 algorithm (0-7)
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

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    let block = Block::default()
        .title(
            "YM2151 Tone Editor (hjkl/wasd:move, q/e:dec/inc, mouse wheel:change value, ESC:quit)",
        )
        .borders(Borders::ALL);
    let inner = block.inner(size);
    f.render_widget(block, size);

    // Calculate cell dimensions
    let cell_width = 4; // 2 digits + spacing
    let cell_height = 1;
    let label_offset = 1; // Space for parameter name labels
    let row_label_width = 4; // Width for row labels (e.g., "OP1 ")

    // Draw parameter names (column headers) for operator rows
    for (col, param_name) in PARAM_NAMES.iter().enumerate().take(GRID_WIDTH) {
        let x = inner.x + row_label_width + (col as u16 * cell_width);
        let y = inner.y;

        let area = Rect {
            x,
            y,
            width: cell_width,
            height: 1,
        };

        let color = get_param_color(col, false);
        let paragraph = Paragraph::new(Span::styled(
            *param_name,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let operator_roles = get_operator_roles_for_alg(alg_value);
    // Draw grid values with row labels for operators (rows 0-3)
    for display_row in 0..4 {
        let slot_mask_enabled = app.values[display_row][PARAM_SM] != 0;
        // Draw row label (operator name)
        let row_label_area = Rect {
            x: inner.x,
            y: inner.y + label_offset + display_row as u16,
            width: row_label_width,
            height: cell_height,
        };
        let row_name = ROW_NAMES[display_row];
        let row_label_color = if slot_mask_enabled {
            if operator_roles[display_row] {
                Color::White
            } else {
                Color::Green
            }
        } else {
            Color::DarkGray
        };
        let row_label =
            Paragraph::new(Span::styled(row_name, Style::default().fg(row_label_color)));
        f.render_widget(row_label, row_label_area);
        // Draw values
        for col in 0..GRID_WIDTH {
            let value = app.values[display_row][col];
            let x = inner.x + row_label_width + (col as u16 * cell_width);
            let y = inner.y + label_offset + display_row as u16;
            let area = Rect {
                x,
                y,
                width: cell_width,
                height: cell_height,
            };
            let value_style = if app.cursor_x == col && app.cursor_y == display_row {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                let color = if slot_mask_enabled {
                    if operator_roles[display_row] {
                        Color::White
                    } else {
                        Color::Green
                    }
                } else {
                    Color::DarkGray
                };
                Style::default().fg(color)
            };

            // Display key guide letter to the left of the value if available
            // Only show key guide on the currently edited operator row
            let is_current_row = app.cursor_y == display_row;
            let line = if let Some(key_guide) = get_key_guide(col) {
                if is_current_row {
                    // Show key guide with darker color and background on current row
                    let key_guide_style =
                        Style::default().fg(Color::DarkGray).bg(KEY_GUIDE_BG_COLOR);
                    Line::from(vec![
                        Span::styled(key_guide.to_string(), key_guide_style),
                        Span::styled(format!("{:2}", value), value_style),
                    ])
                } else {
                    // No key guide on non-current rows
                    Line::from(Span::styled(format!(" {:2}", value), value_style))
                }
            } else {
                Line::from(Span::styled(format!(" {:2}", value), value_style))
            };
            let paragraph = Paragraph::new(line);
            f.render_widget(paragraph, area);
        }
    }

    // Draw CH row header (parameter names for CH row)
    let ch_header_y = inner.y + label_offset + 4;
    for (col, ch_param_name) in CH_PARAM_NAMES.iter().enumerate().take(CH_PARAM_COUNT) {
        let x = inner.x + row_label_width + (col as u16 * cell_width);

        let area = Rect {
            x,
            y: ch_header_y,
            width: cell_width,
            height: 1,
        };

        let color = get_param_color(col, true);
        let paragraph = Paragraph::new(Span::styled(
            *ch_param_name,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    // Draw CH row (row 4) with ALG, FB, and MIDI note number
    let ch_row_y = inner.y + label_offset + 5;

    // Draw row label (CH)
    let row_label_area = Rect {
        x: inner.x,
        y: ch_row_y,
        width: row_label_width,
        height: cell_height,
    };
    let row_label = Paragraph::new(Span::styled(
        ROW_NAMES[ROW_CH],
        Style::default().fg(Color::Yellow),
    ));
    f.render_widget(row_label, row_label_area);

    // Draw all CH row values (ALG, FB, and MIDI note number)
    for col in 0..CH_PARAM_COUNT {
        let value = app.values[ROW_CH][col];
        let x = inner.x + row_label_width + (col as u16 * cell_width);

        let area = Rect {
            x,
            y: ch_row_y,
            width: cell_width,
            height: cell_height,
        };

        let style = if app.cursor_x == col && app.cursor_y == ROW_CH {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            let color = get_param_color(col, true);
            Style::default().fg(color)
        };

        let text = format!("{:2}", value);
        let paragraph = Paragraph::new(Span::styled(text, style));
        f.render_widget(paragraph, area);
    }

    // Draw algorithm diagram below the CH row
    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let diagram = get_algorithm_diagram(alg_value);
    let diagram_start_y = ch_row_y + 2; // Leave one line of space

    for (i, line) in diagram.iter().enumerate() {
        let y = diagram_start_y + i as u16;
        if y < size.height - 1 {
            // Make sure we don't draw outside the terminal
            let area = Rect {
                x: inner.x,
                y,
                width: inner.width,
                height: 1,
            };
            let paragraph = Paragraph::new(Span::styled(*line, Style::default().fg(Color::Green)));
            f.render_widget(paragraph, area);
        }
    }

    let penta_keyboard_y = diagram_start_y + diagram.len() as u16 + 1;
    // Only draw keyboard if it fits within terminal bounds
    if penta_keyboard_y < size.height - 1 {
        draw_virtual_pentatonic_keyboard_at_y(f, app, inner, penta_keyboard_y);
    }
}

fn draw_virtual_pentatonic_keyboard_at_y(f: &mut Frame, app: &App, inner: Rect, keyboard_y: u16) {
    let center_note = 60;
    let width = inner.width as i16;
    const PENTA_INTERVALS: [i16; 5] = [0, 2, 4, 7, 9];
    const PENTA_LABELS: [&str; 5] = ["C", "D", "E", "G", "A"];

    let center_x = width / 2;
    let mut hovered_note: Option<u8> = None;
    for x in 0..width {
        let rel = x - center_x;
        let octave = rel.div_euclid(5);
        let penta_idx = rel.rem_euclid(5);
        let note = center_note as i16 + octave * 12 + PENTA_INTERVALS[penta_idx as usize];
        if !(0..=127).contains(&note) {
            continue;
        }
        let label = PENTA_LABELS[penta_idx as usize];
        let area = Rect {
            x: inner.x + x as u16,
            y: keyboard_y,
            width: 1,
            height: 1,
        };
        let is_hovered = match app.hovered_penta_x {
            Some(hx) => hx == x as usize,
            None => false,
        };
        if is_hovered {
            hovered_note = Some(note as u8);
        }
        let style = if is_hovered {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan)
        };
        let paragraph = Paragraph::new(Span::styled(label, style));
        f.render_widget(paragraph, area);
    }

    #[cfg(windows)]
    if let Some(note_num) = hovered_note {
        use crate::audio;
        let mut preview_values = app.values;
        preview_values[ROW_CH][CH_PARAM_NOTE] = note_num;
        audio::play_tone(
            &preview_values,
            app.use_interactive_mode,
            CH_PARAM_NOTE,
            ROW_CH,
            app.envelope_delay_seconds,
        );
    }
}
