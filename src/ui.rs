use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::{models::*, app::App};

/// Get ASCII art diagram for YM2151 algorithm (0-7)
/// Returns a vector of strings, one per line of the diagram
/// Uses M1, C1, M2, C2 notation (M=Modulator, C=Carrier)
pub fn get_algorithm_diagram(alg: u8) -> Vec<&'static str> {
    match alg {
        0 => vec![
            "ALG 0: M1->C1->M2->C2->OUT",
            "       (Pure FM cascade)",
        ],
        1 => vec![
            "ALG 1: M1->C1-+",
            "       M2-----+->C2->OUT",
            "       (Parallel mod)",
        ],
        2 => vec![
            "ALG 2: M1-+",
            "       C1-+->M2->C2->OUT",
            "       (Fork cascade)",
        ],
        3 => vec![
            "ALG 3: M1->C1->C2->OUT",
            "       M2--------->OUT",
            "       (Cascade+carrier)",
        ],
        4 => vec![
            "ALG 4: M1->C1->OUT",
            "       M2->C2->OUT",
            "       (Two FM pairs)",
        ],
        5 => vec![
            "ALG 5: M1->C1->OUT",
            "       M1->M2->OUT",
            "       M1->C2->OUT",
            "       (Fan out)",
        ],
        6 => vec![
            "ALG 6: M1->C1->OUT",
            "       M2------>OUT",
            "       C2------>OUT",
            "       (Cascade+carriers)",
        ],
        7 => vec![
            "ALG 7: M1->OUT",
            "       C1->OUT",
            "       M2->OUT",
            "       C2->OUT",
            "       (Additive)",
        ],
        _ => vec!["Invalid ALG"],
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    let block = Block::default()
        .title("YM2151 Tone Editor (hjkl/wasd:move, q/e:dec/inc, mouse:move to change value, ESC:quit)")
        .borders(Borders::ALL);
    let inner = block.inner(size);
    f.render_widget(block, size);

    // Calculate cell dimensions
    let cell_width = 4; // 2 digits + spacing
    let cell_height = 1;
    let label_offset = 1; // Space for parameter name labels
    let row_label_width = 4; // Width for row labels (e.g., "OP1 ")

    // Draw parameter names (column headers) for operator rows
    for col in 0..GRID_WIDTH {
        let x = inner.x + row_label_width + (col as u16 * cell_width);
        let y = inner.y;

        let area = Rect {
            x,
            y,
            width: cell_width,
            height: cell_height,
        };

        let param_name = PARAM_NAMES[col];
        let paragraph = Paragraph::new(Span::styled(
            param_name,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    // Draw grid values with row labels for operators (rows 0-3)
    // Display order: M1, C1, M2, C2
    for display_row in 0..4 {
        let data_row = DISPLAY_ROW_TO_DATA_ROW[display_row];
        
        // Draw row label (operator name)
        let row_label_area = Rect {
            x: inner.x,
            y: inner.y + label_offset + display_row as u16,
            width: row_label_width,
            height: cell_height,
        };
        let row_name = ROW_NAMES[display_row];
        let row_label = Paragraph::new(Span::styled(
            row_name,
            Style::default().fg(Color::Yellow),
        ));
        f.render_widget(row_label, row_label_area);

        // Draw values
        for col in 0..GRID_WIDTH {
            let value = app.values[data_row][col];
            let x = inner.x + row_label_width + (col as u16 * cell_width);
            let y = inner.y + label_offset + display_row as u16;

            let area = Rect {
                x,
                y,
                width: cell_width,
                height: cell_height,
            };

            let style = if app.cursor_x == col && app.cursor_y == display_row {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let text = format!("{:2}", value);
            let paragraph = Paragraph::new(Span::styled(text, style));
            f.render_widget(paragraph, area);
        }
    }

    // Draw CH row header (parameter names for CH row)
    let ch_header_y = inner.y + label_offset + 4;
    for col in 0..CH_PARAM_COUNT {
        let x = inner.x + row_label_width + (col as u16 * cell_width);

        let area = Rect {
            x,
            y: ch_header_y,
            width: cell_width,
            height: cell_height,
        };

        let param_name = CH_PARAM_NAMES[col];
        let paragraph = Paragraph::new(Span::styled(
            param_name,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    // Draw CH row (row 4) with ALG, FB, and slot masks
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

    // Draw all CH row values (ALG, FB, 4 slot masks, and MIDI note number)
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
            Style::default()
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
        if y < size.height - 1 { // Make sure we don't draw outside the terminal
            let area = Rect {
                x: inner.x,
                y,
                width: inner.width,
                height: 1,
            };
            let paragraph = Paragraph::new(Span::styled(
                *line,
                Style::default().fg(Color::Green),
            ));
            f.render_widget(paragraph, area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_algorithm_diagram() {
        // Test that each algorithm returns a diagram
        for alg in 0..=7 {
            let diagram = get_algorithm_diagram(alg);
            assert!(!diagram.is_empty(), "Algorithm {} should have a diagram", alg);
            assert!(diagram[0].starts_with("ALG "), "First line should start with 'ALG '");
        }
        
        // Test specific algorithms
        let alg0 = get_algorithm_diagram(0);
        assert!(alg0[0].contains("M1->C1->M2->C2->OUT"), "ALG 0 should show cascade");
        
        let alg7 = get_algorithm_diagram(7);
        assert!(alg7.len() >= 5, "ALG 7 should have at least 5 lines");
        assert!(alg7[0].contains("M1->OUT"), "ALG 7 should show M1 to output");
        
        // Test invalid algorithm
        let invalid = get_algorithm_diagram(8);
        assert_eq!(invalid.len(), 1);
        assert_eq!(invalid[0], "Invalid ALG");
    }
}