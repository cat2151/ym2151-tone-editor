use crate::app::App;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub(super) fn draw_keybind_hints(f: &mut Frame, app: &App, inner: Rect) {
    // Bottom line inside the inner area (inside the block border)
    let inner_bottom = inner.y + inner.height.saturating_sub(1);
    if inner.height == 0 {
        return;
    }

    if app.show_help {
        draw_help_dialog(f, inner);
    } else {
        // Brief hint on the last line of the inner area
        let area = Rect {
            x: inner.x,
            y: inner_bottom,
            width: inner.width,
            height: 1,
        };
        let paragraph = Paragraph::new(Span::styled(
            "?:help | hjkl/wasd:move  q/e:dec/inc  H:history  ESC:quit",
            Style::default().fg(Color::DarkGray),
        ));
        f.render_widget(paragraph, area);
    }
}

/// Render a centered help dialog with key bindings grouped by category.
fn draw_help_dialog(f: &mut Frame, inner: Rect) {
    // Group definitions: (header, lines...)
    let groups: &[(&str, &[&str])] = &[
        (
            " Navigation ",
            &[
                "hjkl / wasd  : Move cursor",
                "1 - 4        : Jump to OP row",
            ],
        ),
        (
            " Value Edit ",
            &[
                "q / e        : Decrease / Increase",
                ". / ,        : +1 / -1",
                "> / <        : +10 / -10",
                "Home / End   : Max / Min",
            ],
        ),
        (
            " Operator Parameters ",
            &[
                "a/A : AR    d/D : D1R   s/S : D2R   r/R : RR",
                "t/T : TL    m/M : MUL   l/L : D1L",
                "u/U : DT    n/N : DT2   k/K : KS",
                "i/I : AMS   o/O : SM",
            ],
        ),
        (
            " Channel Parameters ",
            &["f/F : FB    g/G : ALG   j/J : Note"],
        ),
        (
            " App ",
            &[
                "Space / p    : Play",
                "F5           : Random tone",
                "Ctrl+s       : Save",
                "Ctrl+o       : Open / Select file",
                "H            : History",
                "?            : Close this help",
                "ESC          : Quit",
            ],
        ),
    ];

    // Build content lines: group header + key lines, separated by blank lines between groups.
    // A footer note clarifies that these are the default keybinds (may differ if TOML overrides exist).
    let mut content_lines: Vec<Line> = Vec::new();
    let header_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default().fg(Color::Cyan);
    let note_style = Style::default().fg(Color::DarkGray);

    for (i, (group_header, lines)) in groups.iter().enumerate() {
        if i > 0 {
            content_lines.push(Line::from(""));
        }
        content_lines.push(Line::from(Span::styled(*group_header, header_style)));
        for line in *lines {
            content_lines.push(Line::from(Span::styled(*line, key_style)));
        }
    }
    content_lines.push(Line::from(""));
    content_lines.push(Line::from(Span::styled(
        "(default keybinds — may differ if ym2151-tone-editor.toml overrides exist)",
        note_style,
    )));

    // Compute dialog width from the longest content line + 2 for left/right borders
    let max_content_width = content_lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16;
    let dialog_width: u16 = max_content_width + 2;
    // +2 for top and bottom border lines
    let dialog_height: u16 = content_lines.len() as u16 + 2;

    // Center the dialog within the inner area
    let x = inner
        .x
        .saturating_add(inner.width.saturating_sub(dialog_width) / 2);
    let y = inner
        .y
        .saturating_add(inner.height.saturating_sub(dialog_height) / 2);
    let width = dialog_width.min(inner.width);
    let height = dialog_height.min(inner.height);

    let dialog_area = Rect {
        x,
        y,
        width,
        height,
    };

    // Clear the background behind the dialog
    f.render_widget(Clear, dialog_area);

    let block = Block::default()
        .title(Span::styled(
            " Help ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().bg(Color::Rgb(20, 20, 40)));

    let paragraph = Paragraph::new(Text::from(content_lines))
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, dialog_area);
}
