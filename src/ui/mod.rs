mod helpers;
pub use helpers::*;
mod help;

use crate::{app::App, config::Config, models::*};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};

/// Background color for shortcut key guides
const KEY_GUIDE_BG_COLOR: Color = Color::Rgb(40, 40, 40);

/// Height (in character rows) of the operator envelope canvas.
/// Each row in Braille mode provides 4 pixels of vertical resolution.
const ENVELOPE_CANVAS_HEIGHT: u16 = 6;

/// Terminal row at which the CH parameter values are drawn, assuming the outer
/// block starts at y=0 (full-screen mode).
/// Derivation: border_top(1) + label_offset(1) + op_rows(4) + ch_header(1) = 7.
pub const LAYOUT_CH_ROW_Y: u16 = 7;

/// Colors used to draw the four operator envelopes (O1–O4).
const OP_ENVELOPE_COLORS: [Color; 4] = [Color::Cyan, Color::Green, Color::Yellow, Color::Magenta];

pub fn ui(f: &mut Frame, app: &App, config: &Config) {
    let size = f.area();

    let block = Block::default()
        .title("YM2151 Tone Editor")
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

            // Display guide to the left of the value
            // Show operator number guide in current column, or parameter key guide on current row
            // When cursor is on CH row, show guides on the last operator row the cursor was on
            let is_current_row = app.cursor_y == display_row;
            let is_current_col = app.cursor_x == col;
            let show_guide_for_ch_row =
                app.cursor_y == ROW_CH && display_row == app.last_operator_row;

            let line = if is_current_col {
                // In current column, show operator number guide
                if let Some(op_guide) = get_operator_guide(display_row) {
                    let op_guide_style =
                        Style::default().fg(Color::DarkGray).bg(KEY_GUIDE_BG_COLOR);
                    Line::from(vec![
                        Span::styled(op_guide.to_string(), op_guide_style),
                        Span::styled(format!("{:2}", value), value_style),
                    ])
                } else {
                    // No guide for non-operator rows in current column
                    Line::from(Span::styled(format!(" {:2}", value), value_style))
                }
            } else if let Some(key_guide) = get_key_guide(col) {
                if is_current_row || show_guide_for_ch_row {
                    // Show parameter key guide on current row (for non-current columns)
                    // or on last operator row when cursor is on CH row
                    let key_guide_style =
                        Style::default().fg(Color::DarkGray).bg(KEY_GUIDE_BG_COLOR);
                    Line::from(vec![
                        Span::styled(key_guide.to_string(), key_guide_style),
                        Span::styled(format!("{:2}", value), value_style),
                    ])
                } else {
                    // No guide on non-current rows in non-current columns
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

        let value_style = if app.cursor_x == col && app.cursor_y == ROW_CH {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            let color = get_param_color(col, true);
            Style::default().fg(color)
        };

        // Display guide to the left of the value on the CH row
        // ALG and FB guides are always shown because 'g'/'G' and 'f'/'F' can jump to them from anywhere
        let line = if let Some(key_guide) = get_ch_key_guide(col) {
            let key_guide_style = Style::default().fg(Color::DarkGray).bg(KEY_GUIDE_BG_COLOR);
            Line::from(vec![
                Span::styled(key_guide.to_string(), key_guide_style),
                Span::styled(format!("{:2}", value), value_style),
            ])
        } else {
            // No guide for parameters without keybindings
            Line::from(Span::styled(format!(" {:2}", value), value_style))
        };

        let paragraph = Paragraph::new(line);
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

    // Draw envelope canvas below keyboard if there is enough vertical space.
    // The canvas needs ENVELOPE_CANVAS_HEIGHT character rows + 1 gap row.
    let envelope_y = penta_keyboard_y + 1;
    // Reserve 1 row at the bottom for keybind hints and 1 row for border.
    let available_for_envelope = size.height.saturating_sub(2).saturating_sub(envelope_y);

    // On Windows, prefer the sixel waveform when it has been generated.
    // The sixel is printed by event_loop.rs after each terminal draw,
    // so we only need to skip the braille canvas here to avoid overlap.
    #[cfg(windows)]
    let has_sixel_waveform = app
        .sixel_waveform
        .lock()
        .ok()
        .map(|g| g.is_some())
        .unwrap_or(false);
    #[cfg(not(windows))]
    let has_sixel_waveform = false;

    if available_for_envelope >= ENVELOPE_CANVAS_HEIGHT && !has_sixel_waveform {
        let envelope_area = Rect {
            x: inner.x,
            y: envelope_y,
            width: inner.width,
            height: ENVELOPE_CANVAS_HEIGHT,
        };
        draw_envelope_canvas(f, app, envelope_area);
    }

    // Draw keybind hints at the bottom of the screen (left-aligned)
    help::draw_keybind_hints(f, app, config, inner);
}

/// Draw operator envelope shapes for all 4 OPs into `area` using ratatui's Braille Canvas.
///
/// Each operator's ADSR-like envelope is rendered as a line-chart using a distinct colour:
/// - O1: Cyan, O2: Green, O3: Yellow, O4: Magenta.
///
/// The canvas title shows each operator's role (C = Carrier, M = Modulator) derived
/// from the current algorithm value, so the user can immediately see which operators
/// contribute to the output and which are modulators.
///
/// A vertical dark-gray marker is drawn at x = [`ENVELOPE_NOTE_OFF_T`] to indicate the note-off point,
/// visually separating the sustain phase from the release phase.
///
/// Operators whose slot-mask (SM) is 0 are drawn in dark-gray to indicate they are muted.
///
/// The x-axis represents normalised time (note-on → note-off → release).
/// The y-axis represents normalised amplitude (0 = silent, 1 = max).
fn draw_envelope_canvas(f: &mut Frame, app: &App, area: Rect) {
    // Build all envelope point-sets before the closure (avoids capturing `app` by ref inside FnMut).
    let envelope_points: Vec<Vec<(f64, f64)>> = (0..4)
        .map(|op| compute_op_envelope_points(&app.values[op]))
        .collect();
    let ops_enabled: [bool; 4] = std::array::from_fn(|op| app.values[op][PARAM_SM] != 0);

    // Build title with per-operator carrier (C) / modulator (M) role labels.
    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let operator_roles = get_operator_roles_for_alg(alg_value);
    let role_label = |op: usize| if operator_roles[op] { "C" } else { "M" };
    let title = format!(
        "Env O1({})=Cy O2({})=Gn O3({})=Ye O4({})=Mg",
        role_label(0),
        role_label(1),
        role_label(2),
        role_label(3),
    );

    let canvas = Canvas::default()
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .marker(Marker::Braille)
        .x_bounds([0.0, 1.0])
        .y_bounds([0.0, 1.0])
        .paint(move |ctx| {
            // Draw note-off marker at ENVELOPE_NOTE_OFF_T to show where sustain ends and release begins.
            ctx.draw(&CanvasLine {
                x1: ENVELOPE_NOTE_OFF_T,
                y1: 0.0,
                x2: ENVELOPE_NOTE_OFF_T,
                y2: 1.0,
                color: Color::DarkGray,
            });
            // Draw operator envelope polylines on top of the marker.
            for (op, points) in envelope_points.iter().enumerate() {
                let color = if ops_enabled[op] {
                    OP_ENVELOPE_COLORS[op]
                } else {
                    Color::DarkGray
                };
                for segment in points.windows(2) {
                    let (x1, y1) = segment[0];
                    let (x2, y2) = segment[1];
                    ctx.draw(&CanvasLine {
                        x1,
                        y1,
                        x2,
                        y2,
                        color,
                    });
                }
            }
        });

    f.render_widget(canvas, area);
}

fn draw_virtual_pentatonic_keyboard_at_y(f: &mut Frame, app: &App, inner: Rect, keyboard_y: u16) {
    let center_note = 60;
    let width = inner.width as i16;
    const PENTA_INTERVALS: [i16; 5] = [0, 2, 4, 7, 9];
    const PENTA_LABELS: [&str; 5] = ["C", "D", "E", "G", "A"];

    let center_x = width / 2;
    #[cfg(windows)]
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
        #[cfg(windows)]
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
