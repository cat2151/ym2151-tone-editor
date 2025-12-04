use crate::models::{ToneData, ToneFile};
use crate::register;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::fs;
use std::io;

#[cfg(windows)]
use crate::audio;

/// Default envelope delay for preview playback (seconds)
#[cfg(windows)]
const PREVIEW_ENVELOPE_DELAY: f64 = 0.005;

/// Variation selector state
struct SelectorState {
    list_state: ListState,
    tone_file: ToneFile,
    #[cfg(windows)]
    last_previewed_index: Option<usize>,
}

impl SelectorState {
    fn new(tone_file: ToneFile) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0)); // Start with first item selected
        SelectorState {
            list_state,
            tone_file,
            #[cfg(windows)]
            last_previewed_index: None,
        }
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.tone_file.variations.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        #[cfg(windows)]
        self.preview_current();
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tone_file.variations.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        #[cfg(windows)]
        self.preview_current();
    }

    fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    /// Preview the currently selected variation (Windows only)
    #[cfg(windows)]
    fn preview_current(&mut self) {
        if let Some(idx) = self.selected_index() {
            // Only preview if index has changed to avoid redundant playback
            if self.last_previewed_index == Some(idx) {
                return;
            }
            self.last_previewed_index = Some(idx);

            if idx < self.tone_file.variations.len() {
                let variation = &self.tone_file.variations[idx];

                // Convert registers to ToneData and play
                if let Ok(tone_data) = register::registers_to_editor_rows(&variation.registers) {
                    // Validate tone data before playing
                    if tone_data.len() == crate::models::GRID_HEIGHT
                        && tone_data
                            .iter()
                            .all(|row| row.len() == crate::models::GRID_WIDTH)
                    {
                        // Play the tone using audio module
                        audio::play_tone(&tone_data, true, 0, 0, PREVIEW_ENVELOPE_DELAY);
                    }
                }
            }
        }
    }

    /// Play initial preview for the first selected item
    #[cfg(windows)]
    fn play_initial_preview(&mut self) {
        self.preview_current();
    }
}

/// Render the variation selector UI
fn ui(f: &mut Frame, state: &mut SelectorState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(f.area());

    // Create list items
    let items: Vec<ListItem> = state
        .tone_file
        .variations
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let content = format!("{}: {}", i + 1, v.description);
            ListItem::new(Line::from(Span::raw(content)))
        })
        .collect();

    // Create list widget
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Variation Selector (↑↓/jk: Navigate, Enter: Select, Esc/q: Cancel)"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut state.list_state);

    // Help text
    let help = Paragraph::new(
        "Phase 2: Audio preview plays automatically on cursor movement (Windows only)",
    )
    .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[1]);
}

/// Run the selector event loop
fn run_selector<B: Backend>(
    terminal: &mut ratatui::Terminal<B>,
    state: &mut SelectorState,
) -> io::Result<Option<usize>> {
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            // Only process key press events (not release)
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Down | KeyCode::Char('j') => state.next(),
                    KeyCode::Up | KeyCode::Char('k') => state.previous(),
                    KeyCode::Enter => return Ok(state.selected_index()),
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(None),
                    _ => {}
                }
            }
        }
    }
}

/// Open variation selector using ratatui and load selected variation
/// Reads GM000 file (000_AcousticGrand.json) and displays variations for selection
/// Phase 2: Implements audio preview on cursor movement (Windows only)
pub fn open_variation_selector() -> io::Result<Option<ToneData>> {
    // Load GM000 tone file
    let filename = "tones/general_midi/000_AcousticGrand.json";
    let json_string = fs::read_to_string(filename)?;
    let tone_file: ToneFile = serde_json::from_str(&json_string).map_err(io::Error::other)?;

    if tone_file.variations.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No variations found in tone file",
        ));
    }

    // Create selector state
    let mut state = SelectorState::new(tone_file);

    // Play initial preview for the first item (Windows only)
    #[cfg(windows)]
    state.play_initial_preview();

    // Create terminal for selector
    // Note: main.rs suspends its terminal before calling this function
    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;
    terminal.clear()?;

    // Run selector
    let selected_idx = run_selector(&mut terminal, &mut state)?;

    // Clear terminal before returning
    terminal.clear()?;

    // Handle selection
    match selected_idx {
        Some(idx) if idx < state.tone_file.variations.len() => {
            let variation = &state.tone_file.variations[idx];
            let tone_data = register::registers_to_editor_rows(&variation.registers)?;

            // Validate tone data dimensions
            if tone_data.len() != crate::models::GRID_HEIGHT {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid tone data: incorrect number of rows",
                ));
            }
            for row in tone_data.iter() {
                if row.len() != crate::models::GRID_WIDTH {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid tone data: incorrect row width",
                    ));
                }
            }

            Ok(Some(tone_data))
        }
        _ => Ok(None),
    }
}
