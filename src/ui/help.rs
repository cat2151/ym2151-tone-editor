use crate::app::App;
use crate::config::{Action, Config};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// Sort priority for a key string: named keys (len>1) first, then lowercase, uppercase, symbols.
fn key_sort_priority(k: &str) -> (u8, String) {
    let lower = k.to_ascii_lowercase();
    if k.chars().count() != 1 {
        // Named keys: Left, Space, Ctrl+s, F5, etc.
        (0, lower)
    } else {
        let c = k.chars().next().unwrap();
        if c.is_ascii_lowercase() {
            (1, lower)
        } else if c.is_ascii_uppercase() {
            (2, lower)
        } else {
            (3, k.to_string())
        }
    }
}

/// Returns all keys bound to `action`, sorted by user-friendly display order.
/// Named keys come first, then lowercase, then uppercase, then symbols.
fn keys_for_action(config: &Config, action: &Action) -> Vec<String> {
    let mut keys: Vec<String> = config
        .keybinds
        .iter()
        .filter(|(_, a)| *a == action)
        .map(|(k, _)| k.clone())
        .collect();
    keys.sort_by_key(|k| key_sort_priority(k));
    keys
}

/// Formats a list of key strings as "k1/k2/..." or "(unbound)" if empty.
fn fmt_keys(keys: Vec<String>) -> String {
    if keys.is_empty() {
        "(unbound)".to_string()
    } else {
        keys.join("/")
    }
}

/// Formats all keys for `action` as "k1/k2/..." or "(unbound)".
fn fmt_action_keys(config: &Config, action: &Action) -> String {
    fmt_keys(keys_for_action(config, action))
}

/// Formats an inc/dec shortcut pair as "inc/dec : PARAM".
fn fmt_param(config: &Config, inc: &Action, dec: &Action, name: &str) -> String {
    let i = fmt_action_keys(config, inc);
    let d = fmt_action_keys(config, dec);
    format!("{}/{} : {}", i, d, name)
}

pub(super) fn draw_keybind_hints(f: &mut Frame, app: &App, config: &Config, inner: Rect) {
    if inner.height == 0 {
        return;
    }

    // Draw help dialog first so the bottom hint is painted on top of it.
    if app.show_help {
        draw_help_dialog(f, inner, config);
    }

    // Always show ?:help at the bottom-left of the screen (rendered last so it is never
    // overwritten by the dialog's Clear widget even on small terminals).
    let inner_bottom = inner.y + inner.height.saturating_sub(1);
    let help_key = fmt_action_keys(config, &Action::ToggleHelp);
    let area = Rect {
        x: inner.x,
        y: inner_bottom,
        width: inner.width,
        height: 1,
    };
    let paragraph = Paragraph::new(Span::styled(
        format!("{}:help", help_key),
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(paragraph, area);
}

/// Render a centered help dialog with key bindings grouped by category.
fn draw_help_dialog(f: &mut Frame, inner: Rect, config: &Config) {
    // Navigation section — generated from config
    let left = fmt_action_keys(config, &Action::MoveCursorLeft);
    let right = fmt_action_keys(config, &Action::MoveCursorRight);
    let up = fmt_action_keys(config, &Action::MoveCursorUp);
    let down = fmt_action_keys(config, &Action::MoveCursorDown);
    let nav_move = format!("{} / {} / {} / {} : Move cursor", left, right, up, down);

    // OP-row jump shortcuts — generated from config (inc and dec variants)
    let nav_op_row = format!(
        "{}   {}   {}   {}",
        fmt_param(
            config,
            &Action::JumpToOp1AndIncrease,
            &Action::JumpToOp1AndDecrease,
            "OP1"
        ),
        fmt_param(
            config,
            &Action::JumpToOp2AndIncrease,
            &Action::JumpToOp2AndDecrease,
            "OP2"
        ),
        fmt_param(
            config,
            &Action::JumpToOp3AndIncrease,
            &Action::JumpToOp3AndDecrease,
            "OP3"
        ),
        fmt_param(
            config,
            &Action::JumpToOp4AndIncrease,
            &Action::JumpToOp4AndDecrease,
            "OP4"
        ),
    );

    // Operator parameter shortcuts — generated from config
    let op_line1 = format!(
        "{}   {}   {}   {}",
        fmt_param(
            config,
            &Action::JumpToArAndIncrease,
            &Action::JumpToArAndDecrease,
            "AR"
        ),
        fmt_param(
            config,
            &Action::JumpToD1rAndIncrease,
            &Action::JumpToD1rAndDecrease,
            "D1R"
        ),
        fmt_param(
            config,
            &Action::JumpToD2rAndIncrease,
            &Action::JumpToD2rAndDecrease,
            "D2R"
        ),
        fmt_param(
            config,
            &Action::JumpToRrAndIncrease,
            &Action::JumpToRrAndDecrease,
            "RR"
        ),
    );
    let op_line2 = format!(
        "{}   {}   {}",
        fmt_param(
            config,
            &Action::JumpToTlAndIncrease,
            &Action::JumpToTlAndDecrease,
            "TL"
        ),
        fmt_param(
            config,
            &Action::JumpToMulAndIncrease,
            &Action::JumpToMulAndDecrease,
            "MUL"
        ),
        fmt_param(
            config,
            &Action::JumpToD1lAndIncrease,
            &Action::JumpToD1lAndDecrease,
            "D1L"
        ),
    );
    let op_line3 = format!(
        "{}   {}   {}",
        fmt_param(
            config,
            &Action::JumpToDtAndIncrease,
            &Action::JumpToDtAndDecrease,
            "DT"
        ),
        fmt_param(
            config,
            &Action::JumpToDt2AndIncrease,
            &Action::JumpToDt2AndDecrease,
            "DT2"
        ),
        fmt_param(
            config,
            &Action::JumpToKsAndIncrease,
            &Action::JumpToKsAndDecrease,
            "KS"
        ),
    );
    let op_line4 = format!(
        "{}   {}",
        fmt_param(
            config,
            &Action::JumpToAmsAndIncrease,
            &Action::JumpToAmsAndDecrease,
            "AMS"
        ),
        fmt_param(
            config,
            &Action::JumpToSmAndIncrease,
            &Action::JumpToSmAndDecrease,
            "SM"
        ),
    );

    // Channel parameter shortcuts — generated from config
    let ch_line = format!(
        "{}   {}   {}",
        fmt_param(config, &Action::IncreaseFb, &Action::DecreaseFb, "FB"),
        fmt_param(config, &Action::IncreaseAlg, &Action::DecreaseAlg, "ALG"),
        fmt_param(
            config,
            &Action::JumpToNoteAndIncrease,
            &Action::JumpToNoteAndDecrease,
            "Note"
        ),
    );

    // Value edit shortcuts — generated from config
    let dec_inc = format!(
        "{} / {} : Decrease / Increase",
        fmt_action_keys(config, &Action::DecreaseValue),
        fmt_action_keys(config, &Action::IncreaseValue),
    );
    let plus1_minus1 = format!(
        "{} / {} : +1 / -1",
        fmt_action_keys(config, &Action::IncreaseValueBy1),
        fmt_action_keys(config, &Action::DecreaseValueBy1),
    );
    let plus10_minus10 = format!(
        "{} / {} : +10 / -10",
        fmt_action_keys(config, &Action::IncreaseValueBy10),
        fmt_action_keys(config, &Action::DecreaseValueBy10),
    );
    let max_min = format!(
        "{} / {} : Max / Min",
        fmt_action_keys(config, &Action::SetValueToMax),
        fmt_action_keys(config, &Action::SetValueToMin),
    );

    // App shortcuts — generated from config
    let play = format!(
        "{} : Play",
        fmt_action_keys(config, &Action::PlayCurrentTone)
    );
    let random = format!(
        "{} : Random tone",
        fmt_action_keys(config, &Action::RandomizeTone)
    );
    let save = format!(
        "{} : Save",
        fmt_action_keys(config, &Action::SaveToGmVariations)
    );
    let open = format!(
        "{} : Open / Select file",
        fmt_action_keys(config, &Action::OpenVariationSelector)
    );
    let history = format!(
        "{} : History",
        fmt_action_keys(config, &Action::OpenHistorySelector)
    );
    let close_help = format!(
        "{} : Close this help",
        fmt_action_keys(config, &Action::ToggleHelp)
    );
    let quit = format!("{} : Quit", fmt_action_keys(config, &Action::Exit));

    let groups: Vec<(&str, Vec<String>)> = vec![
        (" Navigation ", vec![nav_move, nav_op_row]),
        (
            " Value Edit ",
            vec![dec_inc, plus1_minus1, plus10_minus10, max_min],
        ),
        (
            " Operator Parameters ",
            vec![op_line1, op_line2, op_line3, op_line4],
        ),
        (" Channel Parameters ", vec![ch_line]),
        (
            " App ",
            vec![play, random, save, open, history, close_help, quit],
        ),
    ];

    // Build content lines: group header + key lines, separated by blank lines between groups.
    // A footer note clarifies that these are the current keybinds (from config or defaults).
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
        for line in lines {
            content_lines.push(Line::from(Span::styled(line.as_str(), key_style)));
        }
    }
    content_lines.push(Line::from(""));
    content_lines.push(Line::from(Span::styled(
        "(keybinds shown above reflect current config — override via ym2151-tone-editor.toml)",
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
