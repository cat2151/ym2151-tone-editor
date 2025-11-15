use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::process::{Command, Stdio};

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 5;

// Parameter names for each column
const PARAM_NAMES: [&str; GRID_WIDTH] = [
    "DT", "MUL", "TL", "KS", "AR", "D1R", "D1L", "D2R", "RR", "ALG"
];

// Maximum values for each parameter (respecting YM2151 bit ranges)
const PARAM_MAX: [u8; GRID_WIDTH] = [
    7,   // DT: 3 bits (0-7)
    15,  // MUL: 4 bits (0-15)
    99,  // TL: 7 bits (0-127, limited to 99 for display)
    3,   // KS: 2 bits (0-3)
    31,  // AR: 5 bits (0-31)
    31,  // D1R: 5 bits (0-31)
    15,  // D1L: 4 bits (0-15)
    15,  // D2R: 4 bits (0-15)
    15,  // RR: 4 bits (0-15)
    7    // ALG: 3 bits (0-7)
];

// Row names for operators
const ROW_NAMES: [&str; GRID_HEIGHT] = [
    "OP1", "OP2", "OP3", "OP4", "CH "
];

// Parameter column indices (matching PARAM_NAMES order)
const PARAM_DT: usize = 0;
const PARAM_MUL: usize = 1;
const PARAM_TL: usize = 2;
const PARAM_KS: usize = 3;
const PARAM_AR: usize = 4;
const PARAM_D1R: usize = 5;
const PARAM_D1L: usize = 6;
const PARAM_D2R: usize = 7;
const PARAM_RR: usize = 8;
const PARAM_ALG: usize = 9;

// Row index for channel settings
const ROW_CH: usize = 4;

/// JSON event structure for ym2151-log-play-server
#[derive(Serialize, Deserialize, Debug)]
struct Ym2151Event {
    time: u32,
    addr: String,
    data: String,
}

/// JSON log structure for ym2151-log-play-server
#[derive(Serialize, Deserialize, Debug)]
struct Ym2151Log {
    event_count: usize,
    events: Vec<Ym2151Event>,
}

struct App {
    values: [[u8; GRID_WIDTH]; GRID_HEIGHT],
    cursor_x: usize,
    cursor_y: usize,
}

impl App {
    fn new() -> App {
        // Initialize with a basic FM piano-like tone
        // Based on typical YM2151 patch settings
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Operator 1 (Carrier): DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, ALG
        values[0] = [0, 1, 20, 0, 31, 10, 5, 5, 7, 4];
        
        // Operator 2 (Modulator): softer attack
        values[1] = [0, 1, 30, 0, 25, 8, 6, 4, 6, 0];
        
        // Operator 3 (Modulator): even softer
        values[2] = [0, 2, 40, 0, 20, 6, 7, 3, 5, 0];
        
        // Operator 4 (Modulator): gentle
        values[3] = [0, 1, 35, 0, 22, 7, 6, 4, 6, 0];
        
        // Channel settings: can be used for feedback, LFO, etc.
        values[4] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 4];
        
        App {
            values,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_x < GRID_WIDTH - 1 {
            self.cursor_x += 1;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_y < GRID_HEIGHT - 1 {
            self.cursor_y += 1;
        }
    }

    fn increase_value(&mut self) {
        let current = self.values[self.cursor_y][self.cursor_x];
        let max = PARAM_MAX[self.cursor_x];
        if current < max {
            self.values[self.cursor_y][self.cursor_x] = current + 1;
            self.call_cat_play_mml();
        }
    }

    fn decrease_value(&mut self) {
        let current = self.values[self.cursor_y][self.cursor_x];
        if current > 0 {
            self.values[self.cursor_y][self.cursor_x] = current - 1;
            self.call_cat_play_mml();
        }
    }

    /// Convert tone data to YM2151 register events
    /// This generates register writes for the YM2151 chip based on the current tone parameters
    fn to_ym2151_events(&self) -> Vec<Ym2151Event> {
        let mut events = Vec::new();

        // YM2151 Register Map:
        // $20-$27: RL, FB, CON (channel 0-7) - Algorithm/Feedback
        // $28-$2F: KC (Key Code) - Note frequency
        // $30-$37: KF (Key Fraction) - Fine frequency
        // $38-$3F: PMS, AMS (Phase/Amplitude Modulation Sensitivity)
        // $40-$5F: DT1, MUL (Detune/Multiply) - 4 operators x 8 channels
        // $60-$7F: TL (Total Level) - 4 operators x 8 channels
        // $80-$9F: KS, AR (Key Scale/Attack Rate) - 4 operators x 8 channels
        // $A0-$BF: AMS-EN, D1R (Decay 1 Rate) - 4 operators x 8 channels
        // $C0-$DF: DT2, D2R (Decay 2 Rate) - 4 operators x 8 channels
        // $E0-$FF: D1L, RR (Decay 1 Level/Release Rate) - 4 operators x 8 channels

        // We'll use channel 0 for this example
        let channel = 0;

        // For each of 4 operators (M1, M2, C1, C2 in YM2151 terminology)
        // We map our OP1-OP4 to operators
        for op in 0..4 {
            let op_offset = op * 8 + channel; // Operator offset in register map
            
            // DT1 (bits 6-4) and MUL (bits 3-0) - Register $40-$5F
            let dt = self.values[op][PARAM_DT];
            let mul = self.values[op][PARAM_MUL];
            let dt_mul = ((dt & 0x07) << 4) | (mul & 0x0F);
            events.push(Ym2151Event {
                time: 0,
                addr: format!("0x{:02X}", 0x40 + op_offset),
                data: format!("0x{:02X}", dt_mul),
            });

            // TL (Total Level) - Register $60-$7F (7 bits)
            let tl = self.values[op][PARAM_TL];
            events.push(Ym2151Event {
                time: 0,
                addr: format!("0x{:02X}", 0x60 + op_offset),
                data: format!("0x{:02X}", tl & 0x7F),
            });

            // KS (bits 7-6) and AR (bits 4-0) - Register $80-$9F
            let ks = self.values[op][PARAM_KS];
            let ar = self.values[op][PARAM_AR];
            let ks_ar = ((ks & 0x03) << 6) | (ar & 0x1F);
            events.push(Ym2151Event {
                time: 0,
                addr: format!("0x{:02X}", 0x80 + op_offset),
                data: format!("0x{:02X}", ks_ar),
            });

            // AMS-EN (bit 7, set to 0) and D1R (bits 4-0) - Register $A0-$BF
            let d1r = self.values[op][PARAM_D1R];
            events.push(Ym2151Event {
                time: 0,
                addr: format!("0x{:02X}", 0xA0 + op_offset),
                data: format!("0x{:02X}", d1r & 0x1F),
            });

            // DT2 (bits 7-6, set to 0) and D2R (bits 4-0) - Register $C0-$DF
            let d2r = self.values[op][PARAM_D2R];
            events.push(Ym2151Event {
                time: 0,
                addr: format!("0x{:02X}", 0xC0 + op_offset),
                data: format!("0x{:02X}", d2r & 0x0F),
            });

            // D1L (bits 7-4) and RR (bits 3-0) - Register $E0-$FF
            let d1l = self.values[op][PARAM_D1L];
            let rr = self.values[op][PARAM_RR];
            let d1l_rr = ((d1l & 0x0F) << 4) | (rr & 0x0F);
            events.push(Ym2151Event {
                time: 0,
                addr: format!("0x{:02X}", 0xE0 + op_offset),
                data: format!("0x{:02X}", d1l_rr),
            });
        }

        // Channel settings: RL, FB, CON (Algorithm) - Register $20-$27
        let alg = self.values[ROW_CH][PARAM_ALG];
        let fb = 0; // Feedback, default to 0
        let rl = 0xC0; // Both L and R enabled
        let rl_fb_con = rl | ((fb & 0x07) << 3) | (alg & 0x07);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x20 + channel),
            data: format!("0x{:02X}", rl_fb_con),
        });

        events
    }

    /// Convert tone data to JSON string in ym2151-log-play-server format
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        let events = self.to_ym2151_events();
        let log = Ym2151Log {
            event_count: events.len(),
            events,
        };
        serde_json::to_string_pretty(&log)
    }

    /// Save tone data to JSON file in ym2151-log-play-server format
    fn save_to_json(&self) -> io::Result<()> {
        let json_string = self.to_json_string()
            .map_err(io::Error::other)?;

        // Generate filename with timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("ym2151_tone_{}.json", timestamp);

        fs::write(&filename, json_string)?;
        Ok(())
    }

    /// Call cat-play-mml with current tone data as JSON file
    /// This function spawns a child process and passes JSON filename as argument
    fn call_cat_play_mml(&self) {
        // Get JSON string of current tone data
        let json_string = match self.to_json_string() {
            Ok(json) => json,
            Err(_) => return, // Silently fail if JSON conversion fails
        };

        // Create a temporary JSON file
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros(); // Use microseconds for uniqueness
        let temp_filename = format!("/tmp/ym2151_temp_{}.json", timestamp);

        // Write JSON to temporary file
        if fs::write(&temp_filename, json_string).is_err() {
            return; // Silently fail if unable to write file
        }

        // Spawn cat-play-mml process with the JSON filename as argument
        // Using spawn() to make it non-blocking
        let _child = Command::new("cat-play-mml")
            .arg(&temp_filename)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        // Don't wait for the child process to complete (non-blocking)
        // Note: The temporary file will remain in /tmp and can be cleaned up later
        // or will be cleaned up by the OS on reboot
    }
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Main loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
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
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => app.decrease_value(),
                KeyCode::Char('e') => app.increase_value(),
                KeyCode::Char('h') => app.move_cursor_left(),
                KeyCode::Char('j') => app.move_cursor_down(),
                KeyCode::Char('k') => app.move_cursor_up(),
                KeyCode::Char('l') => app.move_cursor_right(),
                KeyCode::Esc => {
                    // Save tone data to JSON before exiting
                    app.save_to_json()?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.area();

    let block = Block::default()
        .title("YM2151 Tone Editor (hjkl:move, q/e:dec/inc, ESC:quit)")
        .borders(Borders::ALL);
    let inner = block.inner(size);
    f.render_widget(block, size);

    // Calculate cell dimensions
    let cell_width = 4; // 2 digits + spacing
    let cell_height = 1;
    let label_offset = 1; // Space for parameter name labels
    let row_label_width = 4; // Width for row labels (e.g., "OP1 ")

    // Draw parameter names (column headers)
    for col in 0..GRID_WIDTH {
        let x = inner.x + row_label_width + (col as u16 * cell_width);
        let y = inner.y;

        let area = Rect {
            x,
            y,
            width: cell_width,
            height: cell_height,
        };

        let param_name = PARAM_NAMES[col];
        let paragraph = Paragraph::new(Span::styled(
            param_name,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    // Draw grid values with row labels
    for row in 0..GRID_HEIGHT {
        // Draw row label (operator name)
        let row_label_area = Rect {
            x: inner.x,
            y: inner.y + label_offset + row as u16,
            width: row_label_width,
            height: cell_height,
        };
        let row_name = ROW_NAMES[row];
        let row_label = Paragraph::new(Span::styled(
            row_name,
            Style::default().fg(Color::Yellow),
        ));
        f.render_widget(row_label, row_label_area);

        // Draw values
        for col in 0..GRID_WIDTH {
            let value = app.values[row][col];
            let x = inner.x + row_label_width + (col as u16 * cell_width);
            let y = inner.y + label_offset + row as u16;

            let area = Rect {
                x,
                y,
                width: cell_width,
                height: cell_height,
            };

            let style = if app.cursor_x == col && app.cursor_y == row {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let text = format!("{:2}", value);
            let paragraph = Paragraph::new(Span::styled(text, style));
            f.render_widget(paragraph, area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ym2151_events() {
        let app = App::new();
        let events = app.to_ym2151_events();
        
        // Should have events for 4 operators (6 registers each) + 1 channel register
        assert_eq!(events.len(), 25);
        
        // Check that events have correct format
        for event in &events {
            assert_eq!(event.time, 0);
            assert!(event.addr.starts_with("0x"));
            assert!(event.data.starts_with("0x"));
        }
    }

    #[test]
    fn test_json_serialization() {
        let app = App::new();
        let events = app.to_ym2151_events();
        let log = Ym2151Log {
            event_count: events.len(),
            events,
        };

        // Test that JSON serialization works
        let json_result = serde_json::to_string_pretty(&log);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        assert!(json_string.contains("event_count"));
        assert!(json_string.contains("events"));
    }

    #[test]
    fn test_save_to_json_creates_valid_file() {
        let app = App::new();
        
        // Save to JSON
        let result = app.save_to_json();
        assert!(result.is_ok());
        
        // Find the generated JSON file
        let entries = std::fs::read_dir(".").unwrap();
        let mut json_files: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.starts_with("ym2151_tone_") && s.ends_with(".json"))
                    .unwrap_or(false)
            })
            .collect();
        
        assert!(!json_files.is_empty(), "No JSON file was created");
        
        // Get the most recent file
        json_files.sort_by_key(|e| e.metadata().unwrap().modified().unwrap());
        let json_file = json_files.last().unwrap();
        
        // Read and parse the JSON
        let content = std::fs::read_to_string(json_file.path()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        // Verify structure
        assert!(parsed.get("event_count").is_some());
        assert!(parsed.get("events").is_some());
        assert!(parsed["events"].is_array());
        assert_eq!(parsed["event_count"].as_u64().unwrap(), 25);
        
        // Clean up
        std::fs::remove_file(json_file.path()).ok();
    }

    #[test]
    fn test_to_json_string() {
        let app = App::new();
        
        // Test that to_json_string works
        let json_result = app.to_json_string();
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        assert!(json_string.contains("event_count"));
        assert!(json_string.contains("events"));
        
        // Parse the JSON to verify structure
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        assert!(parsed.get("event_count").is_some());
        assert!(parsed.get("events").is_some());
        assert!(parsed["events"].is_array());
        assert_eq!(parsed["event_count"].as_u64().unwrap(), 25);
    }

    #[test]
    fn test_increase_decrease_value() {
        let mut app = App::new();
        
        // Store initial value
        let initial_value = app.values[0][0];
        
        // Increase value
        app.increase_value();
        assert_eq!(app.values[0][0], initial_value + 1);
        
        // Decrease value
        app.decrease_value();
        assert_eq!(app.values[0][0], initial_value);
        
        // Test boundary: decrease at 0 should not go negative
        app.values[0][0] = 0;
        app.decrease_value();
        assert_eq!(app.values[0][0], 0);
        
        // Test boundary: increase at max should not exceed
        app.cursor_x = 0; // DT parameter
        app.values[0][0] = PARAM_MAX[0]; // Set to max (7)
        app.increase_value();
        assert_eq!(app.values[0][0], PARAM_MAX[0]);
    }
}
