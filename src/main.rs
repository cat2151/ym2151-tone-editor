mod app;
#[cfg(windows)]
mod audio;
mod config;
mod file_ops;
mod midi_conversion;
mod models;
mod register;
#[cfg(test)]
mod tests;
mod ui;

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

/// Global verbose logging flag
static VERBOSE_LOGGING: Mutex<bool> = Mutex::new(false);

/// Log a message to ym2151-tone-editor.log if verbose logging is enabled
pub fn log_verbose(message: &str) {
    if let Ok(enabled) = VERBOSE_LOGGING.lock() {
        if *enabled {
            drop(enabled); // Release lock before file I/O
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("ym2151-tone-editor.log")
            {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let _ = writeln!(file, "[{}] {}", timestamp, message);
            }
        }
    }
}

/// Enable verbose logging
pub fn enable_verbose_logging() {
    if let Ok(mut enabled) = VERBOSE_LOGGING.lock() {
        *enabled = true;
    }
}

use app::App;
use config::KeybindsConfig;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::io;

/// Convert KeyCode and KeyModifiers to a key string for config lookup
fn key_to_string(code: KeyCode, modifiers: KeyModifiers) -> Option<String> {
    match code {
        KeyCode::Char(c) => {
            // Handle CTRL+SHIFT modifier (for CTRL+SHIFT+1,2,3,4)
            if modifiers.contains(KeyModifiers::CONTROL) && modifiers.contains(KeyModifiers::SHIFT)
            {
                Some(format!("Ctrl+Shift+{}", c))
            }
            // Handle CTRL modifier (for CTRL+1,2,3,4)
            else if modifiers.contains(KeyModifiers::CONTROL) {
                Some(format!("Ctrl+{}", c))
            }
            // Handle SHIFT modifier for special characters
            else if modifiers.contains(KeyModifiers::SHIFT) {
                // For shifted characters, return the character as-is
                Some(c.to_string())
            } else {
                Some(c.to_string())
            }
        }
        KeyCode::Left => Some("Left".to_string()),
        KeyCode::Right => Some("Right".to_string()),
        KeyCode::Up => Some("Up".to_string()),
        KeyCode::Down => Some("Down".to_string()),
        KeyCode::Home => Some("Home".to_string()),
        KeyCode::End => Some("End".to_string()),
        KeyCode::PageUp => Some("PageUp".to_string()),
        KeyCode::PageDown => Some("PageDown".to_string()),
        KeyCode::Esc => Some("Esc".to_string()),
        _ => None,
    }
}

fn main() -> Result<(), io::Error> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let use_interactive_mode = args
        .iter()
        .any(|arg| arg == "--use-client-interactive-mode-access");
    let value_by_mouse_move = args.iter().any(|arg| arg == "--value-by-mouse-move");
    let verbose = args.iter().any(|arg| arg == "--verbose");

    // Enable verbose logging if requested
    if verbose {
        enable_verbose_logging();
        log_verbose("Verbose logging enabled");
    }

    // Load keybinds configuration
    let keybinds_config = KeybindsConfig::load_or_default();

    // Ensure server is running (Windows only)
    #[cfg(windows)]
    {
        if let Err(e) = ym2151_log_play_server::client::ensure_server_ready("cat-play-mml") {
            eprintln!("⚠️  Warning: Failed to ensure server is ready: {}", e);
            eprintln!("   Live audio feedback may not be available.");
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state with interactive mode flag and mouse mode flag
    let mut app = App::new(use_interactive_mode, value_by_mouse_move);

    // Main loop
    let res = run_app(&mut terminal, &mut app, &keybinds_config);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    keybinds_config: &KeybindsConfig,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            ui::ui(f, app);
        })?;

        match event::read()? {
            Event::Key(key) => {
                // Only process key press and repeat events, ignore release events
                // This follows crossterm/ratatui best practices for avoiding duplicate
                // actions while still supporting key repeat functionality
                if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
                    // Convert key to string for config lookup
                    if let Some(key_string) = key_to_string(key.code, key.modifiers) {
                        // Look up action in config
                        if let Some(action) = keybinds_config.get_action(&key_string) {
                            use config::Action;
                            match action {
                                Action::DecreaseValue => app.decrease_value(),
                                Action::IncreaseValue => app.increase_value(),
                                Action::SetValueToMax => app.set_value_to_max(),
                                Action::SetValueToMin => app.set_value_to_min(),
                                Action::SetValueToRandom => app.set_value_to_random(),
                                Action::IncreaseValueBy1 => app.increase_value_by(1),
                                Action::IncreaseValueBy2 => app.increase_value_by(2),
                                Action::IncreaseValueBy3 => app.increase_value_by(3),
                                Action::IncreaseValueBy4 => app.increase_value_by(4),
                                Action::IncreaseValueBy5 => app.increase_value_by(5),
                                Action::IncreaseValueBy6 => app.increase_value_by(6),
                                Action::IncreaseValueBy7 => app.increase_value_by(7),
                                Action::IncreaseValueBy8 => app.increase_value_by(8),
                                Action::IncreaseValueBy9 => app.increase_value_by(9),
                                Action::IncreaseValueBy10 => app.increase_value_by(10),
                                Action::DecreaseValueBy1 => app.decrease_value_by(1),
                                Action::DecreaseValueBy2 => app.decrease_value_by(2),
                                Action::DecreaseValueBy3 => app.decrease_value_by(3),
                                Action::DecreaseValueBy4 => app.decrease_value_by(4),
                                Action::DecreaseValueBy5 => app.decrease_value_by(5),
                                Action::DecreaseValueBy6 => app.decrease_value_by(6),
                                Action::DecreaseValueBy7 => app.decrease_value_by(7),
                                Action::DecreaseValueBy8 => app.decrease_value_by(8),
                                Action::DecreaseValueBy9 => app.decrease_value_by(9),
                                Action::DecreaseValueBy10 => app.decrease_value_by(10),
                                Action::PlayCurrentTone => app.play_current_tone(),
                                Action::IncreaseFb => app.increase_fb(),
                                Action::DecreaseFb => app.decrease_fb(),
                                Action::MoveCursorLeft => app.move_cursor_left(),
                                Action::MoveCursorRight => app.move_cursor_right(),
                                Action::MoveCursorUp => app.move_cursor_up(),
                                Action::MoveCursorDown => app.move_cursor_down(),
                                Action::JumpToOp1AndIncrease => {
                                    app.jump_to_operator_and_increase(0)
                                }
                                Action::JumpToOp2AndIncrease => {
                                    app.jump_to_operator_and_increase(1)
                                }
                                Action::JumpToOp3AndIncrease => {
                                    app.jump_to_operator_and_increase(2)
                                }
                                Action::JumpToOp4AndIncrease => {
                                    app.jump_to_operator_and_increase(3)
                                }
                                Action::JumpToOp1AndDecrease => {
                                    app.jump_to_operator_and_decrease(0)
                                }
                                Action::JumpToOp2AndDecrease => {
                                    app.jump_to_operator_and_decrease(1)
                                }
                                Action::JumpToOp3AndDecrease => {
                                    app.jump_to_operator_and_decrease(2)
                                }
                                Action::JumpToOp4AndDecrease => {
                                    app.jump_to_operator_and_decrease(3)
                                }
                                Action::JumpToArAndIncrease => app.jump_to_ar_and_increase(),
                                Action::JumpToD1rAndIncrease => app.jump_to_d1r_and_increase(),
                                Action::JumpToD2rAndIncrease => app.jump_to_d2r_and_increase(),
                                Action::JumpToRrAndIncrease => app.jump_to_rr_and_increase(),
                                Action::JumpToArAndDecrease => app.jump_to_ar_and_decrease(),
                                Action::JumpToD1rAndDecrease => app.jump_to_d1r_and_decrease(),
                                Action::JumpToD2rAndDecrease => app.jump_to_d2r_and_decrease(),
                                Action::JumpToRrAndDecrease => app.jump_to_rr_and_decrease(),
                                Action::Exit => {
                                    // Save tone data to JSON before exiting
                                    app.save_to_json()?;
                                    // Stop interactive mode if active (Windows only)
                                    #[cfg(windows)]
                                    app.cleanup();
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
            Event::Mouse(mouse) => {
                if app.value_by_mouse_move {
                    // Legacy mode: Handle mouse movement to update parameter value
                    // Only responds to mouse movement within the terminal
                    if mouse.kind == MouseEventKind::Moved {
                        // Get terminal width from the current frame
                        let terminal_width = terminal.size().map(|size| size.width).unwrap_or(80);
                        app.update_value_from_mouse_x(mouse.column, terminal_width);
                    }
                } else {
                    // Default mode: Handle mouse wheel events at mouse pointer position
                    match mouse.kind {
                        MouseEventKind::ScrollUp => {
                            // Move cursor to mouse position and increase value
                            app.move_cursor_to_mouse_position(mouse.column, mouse.row);
                            app.increase_value();
                        }
                        MouseEventKind::ScrollDown => {
                            // Move cursor to mouse position and decrease value
                            app.move_cursor_to_mouse_position(mouse.column, mouse.row);
                            app.decrease_value();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
