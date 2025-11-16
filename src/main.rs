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
use app::App;

fn main() -> Result<(), io::Error> {
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

    // Create app state
    let mut app = App::new();

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
            // Update terminal height for mouse position calculations
            app.terminal_height = f.area().height;
            ui::ui(f, app);
        })?;

        match event::read()? {
            Event::Key(key) => {
                // Only process key press and repeat events, ignore release events
                // This follows crossterm/ratatui best practices for avoiding duplicate
                // actions while still supporting key repeat functionality
                if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
                    match key.code {
                        KeyCode::Char('q') => app.decrease_value(),
                        KeyCode::Char('e') => app.increase_value(),
                        KeyCode::Char('h') | KeyCode::Char('a') => app.move_cursor_left(),
                        KeyCode::Char('j') | KeyCode::Char('s') => app.move_cursor_down(),
                        KeyCode::Char('k') | KeyCode::Char('w') => app.move_cursor_up(),
                        KeyCode::Char('l') | KeyCode::Char('d') => app.move_cursor_right(),
                        KeyCode::Esc => {
                            // Save tone data to JSON before exiting
                            app.save_to_json()?;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
            Event::Mouse(mouse) => {
                // Handle mouse movement to update parameter value
                if mouse.kind == MouseEventKind::Moved {
                    app.update_value_from_mouse_y(mouse.row);
                }
            }
            _ => {}
        }
    }
}


