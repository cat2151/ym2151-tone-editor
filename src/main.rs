mod models;
mod register;
mod file_ops;
mod ui;
mod app;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, MouseEventKind, EnableMouseCapture, DisableMouseCapture},
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

    // Create app state with interactive mode flag
    let mut app = App::new(use_interactive_mode);

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
                // Handle mouse movement to update parameter value
                // Only responds to mouse movement within the terminal
                if mouse.kind == MouseEventKind::Moved {
                    // Get terminal width from the current frame
                    let terminal_width = terminal.size().map(|size| size.width).unwrap_or(80);
                    app.update_value_from_mouse_x(mouse.column, terminal_width);
                }
            }
            _ => {}
        }
    }
}


