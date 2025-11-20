mod models;
mod midi_conversion;
mod register;
mod file_ops;
mod ui;
mod app;
#[cfg(windows)]
mod audio;
#[cfg(test)]
mod tests;

use std::sync::Mutex;
use std::fs::OpenOptions;
use std::io::Write;

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

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::env;
use app::App;

fn main() -> Result<(), io::Error> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let use_interactive_mode = args.iter().any(|arg| arg == "--use-client-interactive-mode-access");
    let value_by_mouse_move = args.iter().any(|arg| arg == "--value-by-mouse-move");
    let verbose = args.iter().any(|arg| arg == "--verbose");
    
    // Enable verbose logging if requested
    if verbose {
        enable_verbose_logging();
        log_verbose("Verbose logging enabled");
    }

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
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
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
                    match key.code {
                        // Value modification keys
                        KeyCode::Char('q') | KeyCode::PageDown => app.decrease_value(),
                        KeyCode::Char('e') | KeyCode::PageUp => app.increase_value(),
                        KeyCode::Home => app.set_value_to_max(),
                        KeyCode::End => app.set_value_to_min(),
                        KeyCode::Char('r') | KeyCode::Char('R') => app.set_value_to_random(),
                        
                        // Number keys for quick value adjustment
                        // Keys 1-9: increase by 1-9, key 0: increase by 10
                        // With SHIFT: decrease by the same amount
                        KeyCode::Char('1') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(1);
                            } else {
                                app.increase_value_by(1);
                            }
                        }
                        KeyCode::Char('2') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(2);
                            } else {
                                app.increase_value_by(2);
                            }
                        }
                        KeyCode::Char('3') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(3);
                            } else {
                                app.increase_value_by(3);
                            }
                        }
                        KeyCode::Char('4') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(4);
                            } else {
                                app.increase_value_by(4);
                            }
                        }
                        KeyCode::Char('5') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(5);
                            } else {
                                app.increase_value_by(5);
                            }
                        }
                        KeyCode::Char('6') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(6);
                            } else {
                                app.increase_value_by(6);
                            }
                        }
                        KeyCode::Char('7') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(7);
                            } else {
                                app.increase_value_by(7);
                            }
                        }
                        KeyCode::Char('8') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(8);
                            } else {
                                app.increase_value_by(8);
                            }
                        }
                        KeyCode::Char('9') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(9);
                            } else {
                                app.increase_value_by(9);
                            }
                        }
                        KeyCode::Char('0') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_value_by(10);
                            } else {
                                app.increase_value_by(10);
                            }
                        }
                        
                        // Play current tone without parameter changes
                        KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Char(' ') => app.play_current_tone(),
                        
                        // FB (Feedback) shortcuts
                        KeyCode::Char('f') | KeyCode::Char('F') => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.decrease_fb();
                            } else {
                                app.increase_fb();
                            }
                        }
                        
                        // Cursor movement keys (hjkl/aswd + arrow keys)
                        KeyCode::Char('h') | KeyCode::Char('a') | KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Char('j') | KeyCode::Char('s') | KeyCode::Down => app.move_cursor_down(),
                        KeyCode::Char('k') | KeyCode::Char('w') | KeyCode::Up => app.move_cursor_up(),
                        KeyCode::Char('l') | KeyCode::Char('d') | KeyCode::Right => app.move_cursor_right(),
                        
                        KeyCode::Esc => {
                            // Save tone data to JSON before exiting
                            app.save_to_json()?;
                            // Stop interactive mode if active (Windows only)
                            #[cfg(windows)]
                            app.cleanup();
                            return Ok(());
                        }
                        _ => {}
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


