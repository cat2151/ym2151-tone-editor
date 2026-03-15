use crate::models::ToneData;
use crate::register;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::io;

#[cfg(windows)]
use crate::audio;

/// Default envelope delay for preview playback (seconds)
#[cfg(windows)]
const PREVIEW_ENVELOPE_DELAY: f64 = 0.005;

/// Maximum number of history entries shown (one per a-z key)
const MAX_HISTORY_DISPLAY: usize = 26;

/// Maximum number of register string characters shown per entry in the list
const REGISTER_DISPLAY_LENGTH: usize = 32;

/// Render the history selector UI
fn draw_ui(f: &mut Frame, entries: &[String], selected_index: Option<usize>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),    // List
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Create list items: one per history entry with a-z label
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(i, registers)| {
            let key_label = (b'a' + i as u8) as char;
            // Show abbreviated register string (first REGISTER_DISPLAY_LENGTH chars)
            let short = if registers.len() > REGISTER_DISPLAY_LENGTH {
                format!("{}...", &registers[..REGISTER_DISPLAY_LENGTH])
            } else {
                registers.clone()
            };
            let content = format!("{}: {}", key_label, short);
            let style = if selected_index == Some(i) {
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("History ({} entries)", entries.len())),
    );

    f.render_widget(list, chunks[0]);

    // Help text
    let help_text = if cfg!(windows) {
        "a-z: play and load tone  Shift+F: add to favorites  ESC: back  (audio preview on Windows)"
    } else {
        "a-z: load tone  Shift+F: add to favorites  ESC: back"
    };
    let help =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[1]);
}

/// Open the history selector screen.
/// Returns the selected `ToneData` if a history entry was chosen, or `None` if ESC was pressed.
/// Playing from history does NOT save to history.
pub fn open_history_selector(
    #[cfg_attr(not(windows), allow(unused_variables))] use_interactive_mode: bool,
) -> io::Result<Option<ToneData>> {
    let history = match crate::history::load_history() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Warning: could not load history: {}", e);
            Vec::new()
        }
    };

    // Limit to MAX_HISTORY_DISPLAY entries (a-z)
    let entries: Vec<String> = history.into_iter().take(MAX_HISTORY_DISPLAY).collect();

    if entries.is_empty() {
        return Ok(None);
    }

    // Create terminal for selector
    // Note: the caller (event_loop.rs) suspends its terminal before calling this function
    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;
    terminal.clear()?;

    // Track selected entry: (index, tone_data)
    let mut selected: Option<(usize, ToneData)> = None;

    loop {
        let entries_ref = &entries;
        let sel_index = selected.as_ref().map(|(i, _)| *i);
        terminal.draw(|f| draw_ui(f, entries_ref, sel_index))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char('F') => {
                        // Add the currently selected tone to favorites
                        if let Some((_, ref tone_data)) = selected {
                            if let Err(e) = crate::favorites::save_to_favorites(tone_data) {
                                eprintln!("Warning: could not save to favorites: {}", e);
                            }
                        }
                    }
                    KeyCode::Char(c) if c.is_ascii_lowercase() => {
                        let index = (c as u8 - b'a') as usize;
                        if index < entries.len() {
                            if let Ok(tone_data) =
                                register::registers_to_editor_rows(&entries[index])
                            {
                                // Play without saving to history (Windows only)
                                #[cfg(windows)]
                                audio::play_tone(
                                    &tone_data,
                                    use_interactive_mode,
                                    0,
                                    0,
                                    PREVIEW_ENVELOPE_DELAY,
                                );
                                selected = Some((index, tone_data));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    terminal.clear()?;

    // Return the last selected tone (if any)
    Ok(selected.map(|(_, tone_data)| tone_data))
}
