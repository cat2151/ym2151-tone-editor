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
mod register_list;
#[cfg(test)]
mod tests;
mod ui;
mod updater;
mod variation_selector;

pub use logging::{enable_verbose_logging, log_verbose};

use app::App;
use clap::{Parser, Subcommand};
use config::Config;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[derive(Parser, Debug, PartialEq, Eq)]
#[command(name = "ym2151-tone-editor")]
#[command(version = "0.1.0")]
#[command(about = "YM2151 FM音色エディタ")]
#[command(
    after_help = "例:\n  ym2151-tone-editor --verbose\n  ym2151-tone-editor check\n  ym2151-tone-editor update"
)]
/// YM2151 tone editor のコマンドラインインターフェース定義。
struct Cli {
    #[arg(
        long = "legacy-play-mode",
        global = true,
        help = "Windows限定: ym2151-log-play-serverを使わないレガシープレイモードで起動"
    )]
    legacy_play_mode: bool,
    #[arg(
        long = "value-by-mouse-move",
        global = true,
        help = "マウス移動で値変更するレガシーモードを有効化"
    )]
    value_by_mouse_move: bool,
    #[arg(long = "verbose", global = true, help = "詳細なログ出力を有効化")]
    verbose: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
/// ym2151-tone-editor が提供する CLI サブコマンド。
enum Commands {
    /// アプリを自己更新する
    Update,
    /// ビルド時コミットとリモート main の差分を確認する
    Check,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        enable_verbose_logging();
        log_verbose("Verbose logging enabled");
    }

    if let Some(command) = cli.command.as_ref() {
        return run_command(command);
    }

    run_tui(&cli)?;
    Ok(())
}

fn run_command(command: &Commands) -> anyhow::Result<()> {
    match command {
        Commands::Update => updater::run_foreground_update(),
        Commands::Check => {
            println!("{}", updater::check_for_update_output()?);
            Ok(())
        }
    }
}

fn run_tui(cli: &Cli) -> Result<(), io::Error> {
    #[cfg(windows)]
    ym2151_log_play_server::client::init_client(cli.verbose);

    let config = Config::load_or_default();
    let use_interactive_mode = !cli.legacy_play_mode;

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
        cli.value_by_mouse_move,
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
