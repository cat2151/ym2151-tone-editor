mod app;
mod app_init;
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
mod variation_selector;

use crate::models::{CH_PARAM_ALG, ROW_CH};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

/// Global verbose logging flag
static VERBOSE_LOGGING: Mutex<bool> = Mutex::new(false);

static LOG_FILENAME: &str = "ym2151-tone-editor.log";

/// Log a message to ym2151-tone-editor.log if verbose logging is enabled
pub fn log_verbose(message: &str) {
    if let Ok(enabled) = VERBOSE_LOGGING.lock() {
        if *enabled {
            drop(enabled); // Release lock before file I/O
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(LOG_FILENAME)
            {
                use chrono::Local;
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
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
use clap::{Arg, Command};
use config::Config;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
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
    let matches = Command::new("ym2151-tone-editor")
        .version("0.1.0")
        .about("YM2151 FM音色エディタ")
        .arg(
            Arg::new("legacy_play_mode")
                .long("legacy-play-mode")
                .help("Windows限定: ym2151-log-play-serverを使わないレガシープレイモードで起動")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("value-by-mouse-move")
                .long("value-by-mouse-move")
                .help("マウス移動で値変更するレガシーモードを有効化")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .help("詳細なログ出力を有効化")
                .action(clap::ArgAction::SetTrue),
        )
        .after_help("例: ym2151-tone-editor --verbose")
        .get_matches();

    let legacy_play_mode = matches.get_flag("legacy_play_mode");
    let value_by_mouse_move = matches.get_flag("value-by-mouse-move");
    let verbose = matches.get_flag("verbose");

    if verbose {
        enable_verbose_logging();
        log_verbose("Verbose logging enabled");
    }
    #[cfg(windows)]
    ym2151_log_play_server::client::init_client(verbose);

    let config = Config::load_or_default();
    let use_interactive_mode = !legacy_play_mode;

    #[cfg(windows)]
    {
        if use_interactive_mode {
            if let Err(e) = ym2151_log_play_server::client::ensure_server_ready("cat-play-mml") {
                eprintln!("⚠️  Warning: Failed to ensure server is ready: {}", e);
                eprintln!("   Live audio feedback may not be available.");
            }
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(
        use_interactive_mode,
        value_by_mouse_move,
        config.audio.envelope_delay_seconds,
    );

    #[cfg(windows)]
    {
        if use_interactive_mode {
            if let Err(e) = audio::init_interactive_mode(&app.values) {
                eprintln!("⚠️  Warning: Failed to start interactive mode: {}", e);
            }
        }
    }

    let res = run_app(&mut terminal, &mut app, &config);

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

/// Handle variation selector action by suspending TUI, running selector, and restoring state
/// Returns Ok(()) if successful, Err if terminal operations fail
fn handle_open_variation_selector<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    // Suspend terminal UI to allow variation selector to take over
    let mut stdout = io::stdout();
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    // Run variation selector
    let selection_result = crate::variation_selector::open_variation_selector();

    // Restore terminal UI first
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    terminal.clear()?;

    // Process selection result after UI is restored
    match selection_result {
        Ok(Some(tone_data)) => {
            app.values = tone_data;
            #[cfg(windows)]
            {
                if app.use_interactive_mode {
                    // Play the loaded tone with current cursor position
                    audio::play_tone(
                        &app.values,
                        app.use_interactive_mode,
                        app.cursor_x,
                        app.cursor_y,
                        app.envelope_delay_seconds,
                    );
                }
            }
        }
        Ok(None) => {
            // User cancelled selection, do nothing
        }
        Err(e) => {
            eprintln!("Error loading variation: {}", e);
        }
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    config: &Config,
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
                        if let Some(action) = config.get_action(&key_string) {
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
                                Action::IncreaseAlg => app.increase_alg(),
                                Action::DecreaseAlg => app.decrease_alg(),
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
                                Action::JumpToMulAndIncrease => app.jump_to_mul_and_increase(),
                                Action::JumpToMulAndDecrease => app.jump_to_mul_and_decrease(),
                                Action::JumpToSmAndIncrease => app.jump_to_sm_and_increase(),
                                Action::JumpToSmAndDecrease => app.jump_to_sm_and_decrease(),
                                Action::JumpToTlAndIncrease => app.jump_to_tl_and_increase(),
                                Action::JumpToTlAndDecrease => app.jump_to_tl_and_decrease(),
                                Action::JumpToD1lAndIncrease => app.jump_to_d1l_and_increase(),
                                Action::JumpToD1lAndDecrease => app.jump_to_d1l_and_decrease(),
                                Action::JumpToDtAndIncrease => app.jump_to_dt_and_increase(),
                                Action::JumpToDtAndDecrease => app.jump_to_dt_and_decrease(),
                                Action::JumpToDt2AndIncrease => app.jump_to_dt2_and_increase(),
                                Action::JumpToDt2AndDecrease => app.jump_to_dt2_and_decrease(),
                                Action::JumpToKsAndIncrease => app.jump_to_ks_and_increase(),
                                Action::JumpToKsAndDecrease => app.jump_to_ks_and_decrease(),
                                Action::JumpToAmsAndIncrease => app.jump_to_ams_and_increase(),
                                Action::JumpToAmsAndDecrease => app.jump_to_ams_and_decrease(),
                                Action::JumpToNoteAndIncrease => app.jump_to_note_and_increase(),
                                Action::JumpToNoteAndDecrease => app.jump_to_note_and_decrease(),
                                Action::SaveToGmVariations => {
                                    let _ = app.save_to_gm_variations();
                                }
                                Action::OpenVariationSelector => {
                                    handle_open_variation_selector(terminal, app)?;
                                }
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
                if mouse.kind == MouseEventKind::Moved {
                    // ペンタトニック鍵盤ホバー座標を更新
                    let term_size = terminal.size().unwrap_or(ratatui::prelude::Size {
                        width: 80,
                        height: 24,
                    });
                    // ui.rsのレイアウト計算を再現
                    let inner_x = 1u16; // Block border
                    let inner_y = 1u16;
                    let inner = ratatui::layout::Rect {
                        x: inner_x,
                        y: inner_y,
                        width: term_size.width - 2,
                        height: term_size.height - 2,
                    };
                    let label_offset = 1u16;
                    let ch_row_y = inner.y + label_offset + 5;
                    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
                    let diagram = crate::ui::get_algorithm_diagram(alg_value);
                    let diagram_start_y = ch_row_y + 2;
                    let penta_keyboard_y = diagram_start_y + diagram.len() as u16 + 1;
                    // Only update hover if keyboard is within terminal bounds
                    if penta_keyboard_y < term_size.height - 1 {
                        app.update_hovered_penta_x(
                            mouse.column,
                            mouse.row,
                            inner,
                            penta_keyboard_y,
                        );
                    } else {
                        app.hovered_penta_x = None;
                    }
                    // 旧モード: パラメータ値も更新
                    if app.value_by_mouse_move {
                        app.update_value_from_mouse_x(mouse.column, term_size.width);
                    }
                } else {
                    // Default mode: Handle mouse wheel events at mouse pointer position
                    match mouse.kind {
                        MouseEventKind::ScrollUp => {
                            app.move_cursor_to_mouse_position(mouse.column, mouse.row);
                            app.increase_value();
                        }
                        MouseEventKind::ScrollDown => {
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
