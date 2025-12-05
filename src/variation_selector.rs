use crate::models::{ToneData, ToneFile};
use crate::register;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
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

/// Represents a filtered variation item with its original index and score
struct FilteredItem {
    original_index: usize,
    description: String,
    score: i64,
}

/// Variation selector state
struct SelectorState {
    list_state: ListState,
    tone_file: ToneFile,
    search_query: String,
    filtered_items: Vec<FilteredItem>,
    matcher: SkimMatcherV2,
    #[cfg(windows)]
    last_previewed_index: Option<usize>,
}

impl SelectorState {
    fn new(tone_file: ToneFile) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0)); // Start with first item selected

        let matcher = SkimMatcherV2::default();
        let filtered_items = Self::create_unfiltered_items(&tone_file);

        SelectorState {
            list_state,
            tone_file,
            search_query: String::new(),
            filtered_items,
            matcher,
            #[cfg(windows)]
            last_previewed_index: None,
        }
    }

    /// Create unfiltered items from tone file variations
    fn create_unfiltered_items(tone_file: &ToneFile) -> Vec<FilteredItem> {
        tone_file
            .variations
            .iter()
            .enumerate()
            .map(|(i, v)| FilteredItem {
                original_index: i,
                description: v.description.clone(),
                score: 0,
            })
            .collect()
    }

    /// Update filtered items based on search query
    fn update_filter(&mut self) {
        if self.search_query.is_empty() {
            // No filter - show all items
            self.filtered_items = Self::create_unfiltered_items(&self.tone_file);
        } else {
            // Apply fuzzy matching
            let mut scored_items: Vec<FilteredItem> = self
                .tone_file
                .variations
                .iter()
                .enumerate()
                .filter_map(|(i, v)| {
                    self.matcher
                        .fuzzy_match(&v.description, &self.search_query)
                        .map(|score| FilteredItem {
                            original_index: i,
                            description: v.description.clone(),
                            score,
                        })
                })
                .collect();

            // Sort by score (higher is better)
            scored_items.sort_by(|a, b| b.score.cmp(&a.score));
            self.filtered_items = scored_items;
        }

        // Reset selection to first item if list is not empty
        if !self.filtered_items.is_empty() {
            self.list_state.select(Some(0));
            #[cfg(windows)]
            {
                self.last_previewed_index = None;
                self.preview_current();
            }
        } else {
            self.list_state.select(None);
        }
    }

    fn next(&mut self) {
        if self.filtered_items.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_items.len() - 1 {
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
        if self.filtered_items.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_items.len() - 1
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

    fn selected_original_index(&self) -> Option<usize> {
        self.list_state
            .selected()
            .and_then(|i| self.filtered_items.get(i).map(|item| item.original_index))
    }

    /// Add a character to the search query
    fn add_char(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filter();
    }

    /// Remove the last character from the search query
    fn backspace(&mut self) {
        self.search_query.pop();
        self.update_filter();
    }

    /// Clear the search query
    fn clear_search(&mut self) {
        self.search_query.clear();
        self.update_filter();
    }

    /// Preview the currently selected variation (Windows only)
    #[cfg(windows)]
    fn preview_current(&mut self) {
        if let Some(original_idx) = self.selected_original_index() {
            // Only preview if index has changed to avoid redundant playback
            if self.last_previewed_index == Some(original_idx) {
                return;
            }
            self.last_previewed_index = Some(original_idx);

            if original_idx < self.tone_file.variations.len() {
                let variation = &self.tone_file.variations[original_idx];

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
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(3),    // List
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Search input field
    let search_display = if state.search_query.is_empty() {
        "Type to search...".to_string()
    } else {
        state.search_query.clone()
    };
    let search_widget = Paragraph::new(search_display)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Search ({} matches)", state.filtered_items.len())),
        )
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(search_widget, chunks[0]);

    // Create list items from filtered results
    let items: Vec<ListItem> = state
        .filtered_items
        .iter()
        .map(|item| {
            let content = format!("{}: {}", item.original_index + 1, item.description);
            ListItem::new(Line::from(Span::raw(content)))
        })
        .collect();

    // Create list widget
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(
            "Variations (↑↓/jk: Navigate, Enter: Select, Esc/q: Cancel, Ctrl+U: Clear search)",
        ))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[1], &mut state.list_state);

    // Help text
    let help_text = if cfg!(windows) {
        "Type to filter variations. Audio preview plays on cursor movement (Windows)."
    } else {
        "Type to filter variations. Audio preview available on Windows only."
    };
    let help =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[2]);
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
                    KeyCode::Down | KeyCode::Char('j')
                        if !key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        state.next();
                    }
                    KeyCode::Up | KeyCode::Char('k')
                        if !key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        state.previous();
                    }
                    KeyCode::Enter => {
                        return Ok(state.selected_original_index());
                    }
                    KeyCode::Esc | KeyCode::Char('q')
                        if !key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        return Ok(None);
                    }
                    KeyCode::Backspace => {
                        state.backspace();
                    }
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.clear_search();
                    }
                    KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        state.add_char(c);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Open variation selector using ratatui with fuzzy search and load selected variation
/// Reads GM000 file (000_AcousticGrand.json) and displays variations for selection
/// Features:
/// - Real-time fuzzy search filtering (type to search)
/// - Audio preview on cursor movement (Windows only)
/// - Keyboard navigation (↑↓/jk for movement, Enter to select, Esc/q to cancel)
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
