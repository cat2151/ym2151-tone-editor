mod app;
mod app_init;
#[cfg(windows)]
mod audio;
mod config;
mod event_loop;
mod favorites;
mod file_ops;
mod history;
mod history_selector;
mod logging;
mod midi_conversion;
mod models;
mod random_tone;
mod register;
#[cfg(test)]
mod tests;
mod ui;
mod updater;
mod variation_selector;

pub use logging::{enable_verbose_logging, log_verbose};

use app::App;
use clap::{Arg, Command};
use config::Config;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

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

    // バックグラウンドで自動アップデートチェックを開始する
    updater::spawn_update_check(std::sync::Arc::clone(&app.update_available));

    #[cfg(windows)]
    {
        if use_interactive_mode {
            if let Err(e) = audio::init_interactive_mode(&app.values) {
                eprintln!("⚠️  Warning: Failed to start interactive mode: {}", e);
            }
        }
    }

    let res = event_loop::run_app(&mut terminal, &mut app, &config);

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

    // アップデートが利用可能な場合、問答無用でアップデートを実行する
    if app.is_update_available() {
        if let Err(e) = updater::run_foreground_update() {
            eprintln!("アップデートに失敗しました: {}", e);
        }
    }

    Ok(())
}
