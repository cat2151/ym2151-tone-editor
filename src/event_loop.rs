use crate::app::App;
#[cfg(windows)]
use crate::audio;
use crate::config::{Action, Config};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;

use crate::models::{CH_PARAM_ALG, ROW_CH};

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
            // Handle space key
            else if c == ' ' {
                Some("Space".to_string())
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
        KeyCode::F(n) => Some(format!("F{}", n)),
        _ => None,
    }
}

/// Handle variation selector action by suspending TUI, running selector, and restoring state
/// Returns Ok(()) if successful, Err if terminal operations fail
fn handle_open_variation_selector<B: Backend>(
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

/// Handle history selector action by suspending TUI, running selector, and restoring state
/// Returns Ok(()) if successful, Err if terminal operations fail
fn handle_open_history_selector<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    // Suspend terminal UI to allow history selector to take over
    let mut stdout = io::stdout();
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    // Run history selector
    #[cfg(windows)]
    let use_interactive_mode = app.use_interactive_mode;
    #[cfg(not(windows))]
    let use_interactive_mode = false;
    let selection_result = crate::history_selector::open_history_selector(use_interactive_mode);

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
            // User pressed ESC without selecting, do nothing
        }
        Err(e) => {
            eprintln!("Error loading history entry: {}", e);
        }
    }

    Ok(())
}

pub(crate) fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    config: &Config,
) -> io::Result<()> {
    // 初回描画
    terminal.draw(|f| {
        crate::ui::ui(f, app);
    })?;
    #[cfg(windows)]
    print_sixel_waveform(app)?;

    loop {
        // アップデートが利用可能になったら保存・後始末してループを抜ける
        if app.is_update_available() {
            app.save_to_json()?;
            #[cfg(windows)]
            app.cleanup();
            return Ok(());
        }

        // アイドル検出: 5秒間音色変更がなければsixel波形を生成する
        #[cfg(windows)]
        {
            if app.use_interactive_mode
                && !app.waveform_generating
                && app
                    .sixel_waveform
                    .lock()
                    .ok()
                    .map(|g| g.is_none())
                    .unwrap_or(false)
                && app.last_tone_change.elapsed() >= std::time::Duration::from_secs(5)
            {
                app.waveform_generating = true;
                let sixel_arc = std::sync::Arc::clone(&app.sixel_waveform);
                let expected_gen = app
                    .waveform_generation
                    .load(std::sync::atomic::Ordering::SeqCst);
                let generation_arc = std::sync::Arc::clone(&app.waveform_generation);
                crate::waveform::spawn_waveform_generation(
                    app.values,
                    sixel_arc,
                    expected_gen,
                    generation_arc,
                );
            }
        }

        // イベントをポーリング（タイムアウト付き）。イベントがなければ再描画せずに次ループへ
        if !event::poll(std::time::Duration::from_millis(50))? {
            // sixel生成が完了していたら再描画して表示を更新する。
            // waveform_generating フラグは使わない: 生成カウンタが世代ミスマッチを防ぐため
            // sixel_ready が true なら常に有効な波形が格納されている。
            #[cfg(windows)]
            {
                let sixel_ready = app
                    .sixel_waveform
                    .lock()
                    .ok()
                    .map(|g| g.is_some())
                    .unwrap_or(false);
                if sixel_ready {
                    terminal.draw(|f| {
                        crate::ui::ui(f, app);
                    })?;
                    print_sixel_waveform(app)?;
                }
            }
            continue;
        }

        // イベント処理前の音色データを記録（変更検出用）
        #[cfg(windows)]
        let values_before = app.values;

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
                                Action::OpenHistorySelector => {
                                    handle_open_history_selector(terminal, app)?;
                                }
                                Action::RandomizeTone => app.randomize_tone(),
                                Action::ToggleHelp => app.toggle_help(),
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

        // 音色データが変更されたらアイドルタイマーをリセットする
        #[cfg(windows)]
        if app.values != values_before {
            app.on_tone_changed();
        }

        // イベント処理後に再描画
        terminal.draw(|f| {
            crate::ui::ui(f, app);
        })?;
        // sixel波形が生成済みなら再描画後に端末へ書き出す
        #[cfg(windows)]
        print_sixel_waveform(app)?;
    }
}

/// Print the sixel waveform to stdout at the envelope display area position.
///
/// Called after each ratatui draw so that the sixel waveform replaces the
/// braille envelope canvas when a waveform has been generated.
///
/// If `app.sixel_waveform` is `None` (generation not yet complete) or the
/// mutex is poisoned the function returns early without printing anything.
///
/// # Terminal compatibility
/// Terminals that do not support sixel will display the raw DCS escape
/// sequence, which is an accepted limitation for this experimental feature.
#[cfg(windows)]
fn print_sixel_waveform(app: &App) -> io::Result<()> {
    use std::io::Write;

    let sixel = {
        match app.sixel_waveform.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => return Ok(()),
        }
    };

    let Some(sixel_str) = sixel else {
        return Ok(());
    };

    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let envelope_y = crate::ui::compute_envelope_area_y(alg_value);

    let mut stdout = io::stdout();
    // カーソルをエンベロープ表示エリアの先頭に移動してsixelを書き出す
    execute!(stdout, crossterm::cursor::MoveTo(0, envelope_y))?;
    stdout.write_all(sixel_str.as_bytes())?;
    stdout.flush()?;

    Ok(())
}

#[cfg(test)]
mod key_to_string_tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_space_maps_to_space_string() {
        let result = key_to_string(KeyCode::Char(' '), KeyModifiers::NONE);
        assert_eq!(result, Some("Space".to_string()));
    }

    #[test]
    fn test_shift_space_maps_to_space_string() {
        let result = key_to_string(KeyCode::Char(' '), KeyModifiers::SHIFT);
        assert_eq!(result, Some("Space".to_string()));
    }

    #[test]
    fn test_regular_char_maps_to_itself() {
        let result = key_to_string(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(result, Some("a".to_string()));
    }

    #[test]
    fn test_function_key_f5_maps_to_f5_string() {
        let result = key_to_string(KeyCode::F(5), KeyModifiers::NONE);
        assert_eq!(result, Some("F5".to_string()));
    }

    #[test]
    fn test_function_key_maps_generically() {
        let result = key_to_string(KeyCode::F(1), KeyModifiers::NONE);
        assert_eq!(result, Some("F1".to_string()));
        let result = key_to_string(KeyCode::F(12), KeyModifiers::NONE);
        assert_eq!(result, Some("F12".to_string()));
    }

    #[test]
    fn test_question_mark_shift_slash_maps_to_question_mark() {
        // On most keyboard layouts, '?' is Shift+/ and crossterm delivers it as Char('?') with SHIFT
        let result = key_to_string(KeyCode::Char('?'), KeyModifiers::SHIFT);
        assert_eq!(result, Some("?".to_string()));
    }

    #[test]
    fn test_question_mark_no_modifier_maps_to_question_mark() {
        let result = key_to_string(KeyCode::Char('?'), KeyModifiers::NONE);
        assert_eq!(result, Some("?".to_string()));
    }
}
