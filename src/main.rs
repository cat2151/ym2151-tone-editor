use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, MouseEventKind, EnableMouseCapture, DisableMouseCapture},
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
#[cfg(windows)]
use std::process::{Command, Stdio};

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 5;

// Parameter names for each column
const PARAM_NAMES: [&str; GRID_WIDTH] = [
    "DT", "MUL", "TL", "KS", "AR", "D1R", "D1L", "D2R", "RR", "DT2"
];

// CH row has only 2 parameters
const CH_PARAM_COUNT: usize = 2;
const CH_PARAM_NAMES: [&str; CH_PARAM_COUNT] = ["ALG", "FB"];

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
    3    // DT2: 2 bits (0-3)
];

// Maximum values for CH row parameters
const CH_PARAM_MAX: [u8; CH_PARAM_COUNT] = [
    7,  // ALG: 3 bits (0-7) - Algorithm
    7   // FB: 3 bits (0-7) - Feedback
];

// Row names for operators
const ROW_NAMES: [&str; GRID_HEIGHT] = [
    "OP1", "OP2", "OP3", "OP4", "CH "
];

// Parameter column indices for operator rows (matching PARAM_NAMES order)
const PARAM_DT: usize = 0;
const PARAM_MUL: usize = 1;
const PARAM_TL: usize = 2;
const PARAM_KS: usize = 3;
const PARAM_AR: usize = 4;
const PARAM_D1R: usize = 5;
const PARAM_D1L: usize = 6;
const PARAM_D2R: usize = 7;
const PARAM_RR: usize = 8;
const PARAM_DT2: usize = 9;

// Parameter column indices for CH row (matching CH_PARAM_NAMES order)
const CH_PARAM_ALG: usize = 0;
const CH_PARAM_FB: usize = 1;

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
    terminal_height: u16,
}

impl App {
    fn new() -> App {
        // Initialize with a basic FM piano-like tone
        // Based on typical YM2151 patch settings
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Operator 1 (Carrier): DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2
        values[0] = [0, 1, 20, 0, 31, 10, 5, 5, 7, 0];
        
        // Operator 2 (Modulator): softer attack
        values[1] = [0, 1, 30, 0, 25, 8, 6, 4, 6, 0];
        
        // Operator 3 (Modulator): even softer
        values[2] = [0, 2, 40, 0, 20, 6, 7, 3, 5, 0];
        
        // Operator 4 (Modulator): gentle
        values[3] = [0, 1, 35, 0, 22, 7, 6, 4, 6, 0];
        
        // Channel settings: ALG (algorithm) and FB (feedback) in first 2 positions
        // Default to ALG=4 (simple FM) and FB=0 (no feedback)
        values[4] = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        
        let mut app = App {
            values,
            cursor_x: 0,
            cursor_y: 0,
            terminal_height: 24, // Default, will be updated in main loop
        };

        // Try to load the newest JSON file from current directory
        if let Ok(loaded_values) = App::load_newest_json() {
            app.values = loaded_values;
        }

        app
    }

    /// Find the newest JSON file in the current directory matching the pattern ym2151_tone_*.json
    fn find_newest_json_file() -> io::Result<String> {
        let entries = fs::read_dir(".")?;
        
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

        if json_files.is_empty() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "No JSON files found"));
        }

        // Sort by modification time (newest first)
        json_files.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .ok()
        });
        json_files.reverse();

        json_files
            .first()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not get filename"))
    }

    /// Load tone data from a JSON file
    fn load_from_json(filename: &str) -> io::Result<[[u8; GRID_WIDTH]; GRID_HEIGHT]> {
        let json_string = fs::read_to_string(filename)?;
        let log: Ym2151Log = serde_json::from_str(&json_string)
            .map_err(io::Error::other)?;

        App::events_to_tone_data(&log.events)
    }

    /// Load the newest JSON file and convert to tone data
    fn load_newest_json() -> io::Result<[[u8; GRID_WIDTH]; GRID_HEIGHT]> {
        let filename = App::find_newest_json_file()?;
        App::load_from_json(&filename)
    }

    /// Convert YM2151 events back to tone data
    fn events_to_tone_data(events: &[Ym2151Event]) -> io::Result<[[u8; GRID_WIDTH]; GRID_HEIGHT]> {
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];

        for event in events {
            // Parse address and data
            let addr = u8::from_str_radix(event.addr.trim_start_matches("0x"), 16)
                .map_err(io::Error::other)?;
            let data = u8::from_str_radix(event.data.trim_start_matches("0x"), 16)
                .map_err(io::Error::other)?;

            // Decode based on register address range
            match addr {
                // DT1/MUL registers (0x40-0x5F)
                0x40..=0x5F => {
                    let op = ((addr - 0x40) / 8) as usize;
                    if op < 4 {
                        values[op][PARAM_DT] = (data >> 4) & 0x07;
                        values[op][PARAM_MUL] = data & 0x0F;
                    }
                }
                // TL registers (0x60-0x7F)
                0x60..=0x7F => {
                    let op = ((addr - 0x60) / 8) as usize;
                    if op < 4 {
                        values[op][PARAM_TL] = data & 0x7F;
                    }
                }
                // KS/AR registers (0x80-0x9F)
                0x80..=0x9F => {
                    let op = ((addr - 0x80) / 8) as usize;
                    if op < 4 {
                        values[op][PARAM_KS] = (data >> 6) & 0x03;
                        values[op][PARAM_AR] = data & 0x1F;
                    }
                }
                // AMS-EN/D1R registers (0xA0-0xBF)
                0xA0..=0xBF => {
                    let op = ((addr - 0xA0) / 8) as usize;
                    if op < 4 {
                        values[op][PARAM_D1R] = data & 0x1F;
                    }
                }
                // DT2/D2R registers (0xC0-0xDF)
                0xC0..=0xDF => {
                    let op = ((addr - 0xC0) / 8) as usize;
                    if op < 4 {
                        values[op][PARAM_DT2] = (data >> 6) & 0x03;
                        values[op][PARAM_D2R] = data & 0x0F;
                    }
                }
                // D1L/RR registers (0xE0-0xFF)
                0xE0..=0xFF => {
                    let op = ((addr - 0xE0) / 8) as usize;
                    if op < 4 {
                        values[op][PARAM_D1L] = (data >> 4) & 0x0F;
                        values[op][PARAM_RR] = data & 0x0F;
                    }
                }
                // RL/FB/CON register (0x20-0x27)
                0x20..=0x27 => {
                    // This register contains RL (bit 7-6), FB (bit 5-3), and CON/ALG (bit 2-0)
                    // Extract ALG and FB to CH row
                    values[ROW_CH][CH_PARAM_ALG] = data & 0x07; // ALG is bits 0-2
                    values[ROW_CH][CH_PARAM_FB] = (data >> 3) & 0x07; // FB is bits 3-5
                }
                _ => {}
            }
        }

        Ok(values)
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        let max_x = if self.cursor_y == ROW_CH {
            CH_PARAM_COUNT - 1
        } else {
            GRID_WIDTH - 1
        };
        
        if self.cursor_x < max_x {
            self.cursor_x += 1;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            
            // Clamp cursor_x if moving from CH row to operator row or vice versa
            let max_x = if self.cursor_y == ROW_CH {
                CH_PARAM_COUNT - 1
            } else {
                GRID_WIDTH - 1
            };
            
            if self.cursor_x > max_x {
                self.cursor_x = max_x;
            }
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_y < GRID_HEIGHT - 1 {
            self.cursor_y += 1;
            
            // Clamp cursor_x if moving from operator row to CH row or vice versa
            let max_x = if self.cursor_y == ROW_CH {
                CH_PARAM_COUNT - 1
            } else {
                GRID_WIDTH - 1
            };
            
            if self.cursor_x > max_x {
                self.cursor_x = max_x;
            }
        }
    }

    fn increase_value(&mut self) {
        let current = self.values[self.cursor_y][self.cursor_x];
        let max = if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        };
        if current < max {
            self.values[self.cursor_y][self.cursor_x] = current + 1;
            #[cfg(windows)]
            self.call_cat_play_mml();
        }
    }

    fn decrease_value(&mut self) {
        let current = self.values[self.cursor_y][self.cursor_x];
        if current > 0 {
            self.values[self.cursor_y][self.cursor_x] = current - 1;
            #[cfg(windows)]
            self.call_cat_play_mml();
        }
    }

    /// Update the parameter value based on mouse Y position
    /// Maps mouse Y position to parameter value range (0 to PARAM_MAX)
    fn update_value_from_mouse_y(&mut self, mouse_y: u16) {
        if self.terminal_height == 0 {
            return; // Avoid division by zero
        }

        // Map mouse Y position to parameter value
        // Y=0 (top) -> max value, Y=terminal_height (bottom) -> 0
        let max_value = if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        };
        
        // Calculate the parameter value based on mouse position
        // Invert Y so that top = max value, bottom = 0
        let normalized = if mouse_y >= self.terminal_height {
            0.0
        } else {
            1.0 - (mouse_y as f32 / self.terminal_height as f32)
        };
        
        let new_value = (normalized * max_value as f32).round() as u8;
        let clamped_value = new_value.min(max_value);
        
        // Only update and play sound if the value actually changed
        if self.values[self.cursor_y][self.cursor_x] != clamped_value {
            self.values[self.cursor_y][self.cursor_x] = clamped_value;
            #[cfg(windows)]
            self.call_cat_play_mml();
        }
    }

    /// Convert tone data to YM2151 register events
    /// This generates register writes for the YM2151 chip based on the current tone parameters
    fn to_ym2151_events(&self) -> Vec<Ym2151Event> {
        let mut events = Vec::new();

        // YM2151 Register Map:
        // $08: Key On/Off - Note on/off control
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

            // DT2 (bits 7-6) and D2R (bits 3-0) - Register $C0-$DF
            let dt2 = self.values[op][PARAM_DT2];
            let d2r = self.values[op][PARAM_D2R];
            let dt2_d2r = ((dt2 & 0x03) << 6) | (d2r & 0x0F);
            events.push(Ym2151Event {
                time: 0,
                addr: format!("0x{:02X}", 0xC0 + op_offset),
                data: format!("0x{:02X}", dt2_d2r),
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
        // Use ALG and FB from CH row
        let alg = self.values[ROW_CH][CH_PARAM_ALG];
        let fb = self.values[ROW_CH][CH_PARAM_FB];
        let rl = 0xC0; // Both L and R enabled
        let rl_fb_con = rl | ((fb & 0x07) << 3) | (alg & 0x07);
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x20 + channel),
            data: format!("0x{:02X}", rl_fb_con),
        });
        
        // Key Code (KC) - Register $28-$2F - Set note to middle C (around KC=0x4C)
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x28 + channel),
            data: "0x4C".to_string(),
        });
        
        // Key Fraction (KF) - Register $30-$37 - Fine frequency adjust
        events.push(Ym2151Event {
            time: 0,
            addr: format!("0x{:02X}", 0x30 + channel),
            data: "0x00".to_string(),
        });
        
        // Note On - Register $08 - Key On with all operators enabled
        // Bits 0-2: Channel (0-7)
        // Bits 3-6: Operator enable (M1=bit3, M2=bit4, C1=bit5, C2=bit6)
        // For YM2151, enabling all 4 operators = 0x78 (bits 3-6 set)
        let key_on_data = 0x78 | channel; // All operators on for channel
        events.push(Ym2151Event {
            time: 0,
            addr: "0x08".to_string(),
            data: format!("0x{:02X}", key_on_data),
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
    /// Windows-only functionality
    #[cfg(windows)]
    fn call_cat_play_mml(&self) {
        // Get JSON string of current tone data
        let json_string = match self.to_json_string() {
            Ok(json) => json,
            Err(_) => return, // Silently fail if JSON conversion fails
        };

        // Create a temporary JSON file in the current directory
        // Fixed filename since it will be recreated multiple times during a session
        let temp_filename = "ym2151_temp.json";

        // Write JSON to temporary file
        if fs::write(temp_filename, json_string).is_err() {
            return; // Silently fail if unable to write file
        }

        // Spawn cat-play-mml process with the JSON filename as argument
        // Using spawn() to make it non-blocking
        let _child = Command::new("cat-play-mml")
            .arg(temp_filename)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        // Don't wait for the child process to complete (non-blocking)
        // Note: The temporary file will remain in current directory
    }
}

/// Get ASCII art diagram for YM2151 algorithm (0-7)
/// Returns a vector of strings, one per line of the diagram
fn get_algorithm_diagram(alg: u8) -> Vec<&'static str> {
    match alg {
        0 => vec![
            "ALG 0: 4->3->2->1->OUT",
            "       (Pure FM cascade)",
        ],
        1 => vec![
            "ALG 1: 4->3-+",
            "       2----+->1->OUT",
            "       (Parallel mod)",
        ],
        2 => vec![
            "ALG 2: 4-+",
            "       3-+->2->1->OUT",
            "       (Fork cascade)",
        ],
        3 => vec![
            "ALG 3: 4->3->1->OUT",
            "       2-------->OUT",
            "       (Cascade+carrier)",
        ],
        4 => vec![
            "ALG 4: 4->3->OUT",
            "       2->1->OUT",
            "       (Two FM pairs)",
        ],
        5 => vec![
            "ALG 5: 4->3->OUT",
            "       4->2->OUT",
            "       4->1->OUT",
            "       (Fan out)",
        ],
        6 => vec![
            "ALG 6: 4->3->OUT",
            "       2------>OUT",
            "       1------>OUT",
            "       (Cascade+carriers)",
        ],
        7 => vec![
            "ALG 7: 4->OUT",
            "       3->OUT",
            "       2->OUT",
            "       1->OUT",
            "       (Additive)",
        ],
        _ => vec!["Invalid ALG"],
    }
}

fn main() -> Result<(), io::Error> {
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
            ui(f, app);
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

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.area();

    let block = Block::default()
        .title("YM2151 Tone Editor (hjkl/wasd:move, q/e:dec/inc, mouse:move to change value, ESC:quit)")
        .borders(Borders::ALL);
    let inner = block.inner(size);
    f.render_widget(block, size);

    // Calculate cell dimensions
    let cell_width = 4; // 2 digits + spacing
    let cell_height = 1;
    let label_offset = 1; // Space for parameter name labels
    let row_label_width = 4; // Width for row labels (e.g., "OP1 ")

    // Draw parameter names (column headers) for operator rows
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

    // Draw grid values with row labels for operators (rows 0-3)
    for row in 0..4 {
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

    // Draw CH row header (parameter names for CH row)
    let ch_header_y = inner.y + label_offset + 4;
    for col in 0..CH_PARAM_COUNT {
        let x = inner.x + row_label_width + (col as u16 * cell_width);

        let area = Rect {
            x,
            y: ch_header_y,
            width: cell_width,
            height: cell_height,
        };

        let param_name = CH_PARAM_NAMES[col];
        let paragraph = Paragraph::new(Span::styled(
            param_name,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    // Draw CH row (row 4) with only ALG and FB
    let ch_row_y = inner.y + label_offset + 5;
    
    // Draw row label (CH)
    let row_label_area = Rect {
        x: inner.x,
        y: ch_row_y,
        width: row_label_width,
        height: cell_height,
    };
    let row_label = Paragraph::new(Span::styled(
        ROW_NAMES[ROW_CH],
        Style::default().fg(Color::Yellow),
    ));
    f.render_widget(row_label, row_label_area);

    // Draw only ALG and FB values
    for col in 0..CH_PARAM_COUNT {
        let value = app.values[ROW_CH][col];
        let x = inner.x + row_label_width + (col as u16 * cell_width);

        let area = Rect {
            x,
            y: ch_row_y,
            width: cell_width,
            height: cell_height,
        };

        let style = if app.cursor_x == col && app.cursor_y == ROW_CH {
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

    // Draw algorithm diagram below the CH row
    let alg_value = app.values[ROW_CH][CH_PARAM_ALG];
    let diagram = get_algorithm_diagram(alg_value);
    let diagram_start_y = ch_row_y + 2; // Leave one line of space
    
    for (i, line) in diagram.iter().enumerate() {
        let y = diagram_start_y + i as u16;
        if y < size.height - 1 { // Make sure we don't draw outside the terminal
            let area = Rect {
                x: inner.x,
                y,
                width: inner.width,
                height: 1,
            };
            let paragraph = Paragraph::new(Span::styled(
                *line,
                Style::default().fg(Color::Green),
            ));
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
        
        // Should have events for:
        // - 4 operators × 6 registers = 24 events
        // - 1 channel register (RL/FB/CON)
        // - 1 Key Code register
        // - 1 Key Fraction register  
        // - 1 Note On register
        // Total = 28 events
        assert_eq!(events.len(), 28);
        
        // Check that events have correct format
        for event in &events {
            assert_eq!(event.time, 0);
            assert!(event.addr.starts_with("0x"));
            assert!(event.data.starts_with("0x"));
        }
        
        // Verify note on event is present
        let note_on_event = events.iter().find(|e| e.addr == "0x08");
        assert!(note_on_event.is_some(), "Note on event should be present");
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
        assert_eq!(parsed["event_count"].as_u64().unwrap(), 28);
        
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
        assert_eq!(parsed["event_count"].as_u64().unwrap(), 28);
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

    #[test]
    fn test_events_to_tone_data() {
        // Create sample events
        let events = vec![
            Ym2151Event {
                time: 0,
                addr: "0x40".to_string(),
                data: "0x12".to_string(), // DT=1, MUL=2
            },
            Ym2151Event {
                time: 0,
                addr: "0x60".to_string(),
                data: "0x1F".to_string(), // TL=31
            },
            Ym2151Event {
                time: 0,
                addr: "0x80".to_string(),
                data: "0x8A".to_string(), // KS=2, AR=10
            },
            Ym2151Event {
                time: 0,
                addr: "0xA0".to_string(),
                data: "0x0C".to_string(), // D1R=12
            },
            Ym2151Event {
                time: 0,
                addr: "0xC0".to_string(),
                data: "0x85".to_string(), // DT2=2, D2R=5
            },
            Ym2151Event {
                time: 0,
                addr: "0xE0".to_string(),
                data: "0x78".to_string(), // D1L=7, RR=8
            },
        ];

        let result = App::events_to_tone_data(&events);
        assert!(result.is_ok());

        let values = result.unwrap();
        
        // Check operator 1 values
        assert_eq!(values[0][PARAM_DT], 1);
        assert_eq!(values[0][PARAM_MUL], 2);
        assert_eq!(values[0][PARAM_TL], 31);
        assert_eq!(values[0][PARAM_KS], 2);
        assert_eq!(values[0][PARAM_AR], 10);
        assert_eq!(values[0][PARAM_D1R], 12);
        assert_eq!(values[0][PARAM_D1L], 7);
        assert_eq!(values[0][PARAM_D2R], 5);
        assert_eq!(values[0][PARAM_RR], 8);
        assert_eq!(values[0][PARAM_DT2], 2);
    }

    #[test]
    fn test_load_from_json() {
        // Create a test JSON file
        let app = App::new();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_filename = format!("ym2151_tone_test_{}.json", timestamp);
        
        // Save current tone data
        let json_string = app.to_json_string().unwrap();
        std::fs::write(&test_filename, json_string).unwrap();
        
        // Load it back
        let result = App::load_from_json(&test_filename);
        assert!(result.is_ok());
        
        let loaded_values = result.unwrap();
        
        // Verify loaded values match original (at least some key values)
        assert_eq!(loaded_values[0][PARAM_MUL], app.values[0][PARAM_MUL]);
        assert_eq!(loaded_values[0][PARAM_TL], app.values[0][PARAM_TL]);
        assert_eq!(loaded_values[0][PARAM_DT2], app.values[0][PARAM_DT2]);
        
        // Clean up
        std::fs::remove_file(&test_filename).ok();
    }

    #[test]
    fn test_find_newest_json_file() {
        // Create multiple test JSON files with different timestamps
        let base_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let file1 = format!("ym2151_tone_{}.json", base_time);
        let file2 = format!("ym2151_tone_{}.json", base_time + 1);
        let file3 = format!("ym2151_tone_{}.json", base_time + 2);
        
        std::fs::write(&file1, "{}").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        std::fs::write(&file2, "{}").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        std::fs::write(&file3, "{}").unwrap();
        
        // Find newest file
        let result = App::find_newest_json_file();
        assert!(result.is_ok());
        
        let newest = result.unwrap();
        assert_eq!(newest, file3);
        
        // Clean up
        std::fs::remove_file(&file1).ok();
        std::fs::remove_file(&file2).ok();
        std::fs::remove_file(&file3).ok();
    }

    #[test]
    fn test_cursor_movement() {
        let mut app = App::new();
        
        // Test initial position
        assert_eq!(app.cursor_x, 0);
        assert_eq!(app.cursor_y, 0);
        
        // Test move right
        app.move_cursor_right();
        assert_eq!(app.cursor_x, 1);
        assert_eq!(app.cursor_y, 0);
        
        // Test move down
        app.move_cursor_down();
        assert_eq!(app.cursor_x, 1);
        assert_eq!(app.cursor_y, 1);
        
        // Test move left
        app.move_cursor_left();
        assert_eq!(app.cursor_x, 0);
        assert_eq!(app.cursor_y, 1);
        
        // Test move up
        app.move_cursor_up();
        assert_eq!(app.cursor_x, 0);
        assert_eq!(app.cursor_y, 0);
        
        // Test boundary: can't move left from 0,0
        app.move_cursor_left();
        assert_eq!(app.cursor_x, 0);
        app.move_cursor_up();
        assert_eq!(app.cursor_y, 0);
        
        // Test boundary: move to max position
        for _ in 0..GRID_WIDTH {
            app.move_cursor_right();
        }
        assert_eq!(app.cursor_x, GRID_WIDTH - 1);
        
        for _ in 0..GRID_HEIGHT {
            app.move_cursor_down();
        }
        assert_eq!(app.cursor_y, GRID_HEIGHT - 1);
        // After moving down to CH row, cursor_x should be clamped to CH_PARAM_COUNT - 1
        assert_eq!(app.cursor_x, CH_PARAM_COUNT - 1);
        
        // Test can't exceed boundaries on CH row
        app.move_cursor_right();
        assert_eq!(app.cursor_x, CH_PARAM_COUNT - 1);
        app.move_cursor_down();
        assert_eq!(app.cursor_y, GRID_HEIGHT - 1);
    }

    #[test]
    fn test_wasd_cursor_movement() {
        // This test verifies WASD key mapping works the same as HJKL
        // The actual key handling is in run_app, but we test the movement functions
        // to ensure they work correctly when called by either key set
        let mut app = App::new();
        
        // Reset to known position
        app.cursor_x = 2;
        app.cursor_y = 2;
        
        // Test that movement functions work (which are called by both hjkl and wasd)
        app.move_cursor_left();  // Called by 'h' or 'a'
        assert_eq!(app.cursor_x, 1);
        
        app.move_cursor_right(); // Called by 'l' or 'd'
        assert_eq!(app.cursor_x, 2);
        
        app.move_cursor_up();    // Called by 'k' or 'w'
        assert_eq!(app.cursor_y, 1);
        
        app.move_cursor_down();  // Called by 'j' or 's'
        assert_eq!(app.cursor_y, 2);
    }

    #[test]
    fn test_get_algorithm_diagram() {
        // Test that each algorithm returns a diagram
        for alg in 0..=7 {
            let diagram = get_algorithm_diagram(alg);
            assert!(!diagram.is_empty(), "Algorithm {} should have a diagram", alg);
            assert!(diagram[0].starts_with("ALG "), "First line should start with 'ALG '");
        }
        
        // Test specific algorithms
        let alg0 = get_algorithm_diagram(0);
        assert!(alg0[0].contains("4->3->2->1->OUT"), "ALG 0 should show cascade");
        
        let alg7 = get_algorithm_diagram(7);
        assert!(alg7.len() >= 5, "ALG 7 should have at least 5 lines");
        assert!(alg7[0].contains("4->OUT"), "ALG 7 should show operator 4 to output");
        
        // Test invalid algorithm
        let invalid = get_algorithm_diagram(8);
        assert_eq!(invalid.len(), 1);
        assert_eq!(invalid[0], "Invalid ALG");
    }

    #[test]
    fn test_carrier_tl_is_editable() {
        let mut app = App::new();
        
        // Set all operator TL values to non-zero
        for op in 0..4 {
            app.values[op][PARAM_TL] = 50;
        }
        
        // Test that TL values are preserved in the output
        let events = app.to_ym2151_events();
        
        // Check TL registers for each operator
        for op in 0..4 {
            let tl_addr = format!("0x{:02X}", 0x60 + op * 8);
            let tl_event = events.iter().find(|e| e.addr == tl_addr);
            assert!(tl_event.is_some(), "TL event for operator {} should exist", op);
            
            let tl_value = u8::from_str_radix(
                tl_event.unwrap().data.trim_start_matches("0x"),
                16
            ).unwrap();
            
            // All operators should preserve their TL value (carrier TL is now editable)
            assert_eq!(tl_value, 50, "Operator {} should preserve TL value", op);
        }
    }

    #[test]
    fn test_update_value_from_mouse_y() {
        let mut app = App::new();
        app.terminal_height = 100;
        app.cursor_x = PARAM_DT; // DT has max value of 7
        app.cursor_y = 0;
        
        // Test mouse at top (y=0) should give max value
        app.update_value_from_mouse_y(0);
        assert_eq!(app.values[0][PARAM_DT], 7, "Top of screen should give max value");
        
        // Test mouse at bottom (y=100) should give min value (0)
        app.update_value_from_mouse_y(100);
        assert_eq!(app.values[0][PARAM_DT], 0, "Bottom of screen should give min value");
        
        // Test mouse at middle (y=50) should give approximately half of max
        app.update_value_from_mouse_y(50);
        let middle_value = app.values[0][PARAM_DT];
        assert!(middle_value >= 3 && middle_value <= 4, "Middle of screen should give ~half of max value, got {}", middle_value);
        
        // Test with different parameter (MUL has max value of 15)
        app.cursor_x = PARAM_MUL;
        app.update_value_from_mouse_y(0);
        assert_eq!(app.values[0][PARAM_MUL], 15, "Top of screen should give max value for MUL");
        
        app.update_value_from_mouse_y(100);
        assert_eq!(app.values[0][PARAM_MUL], 0, "Bottom of screen should give min value for MUL");
        
        // Test edge case: mouse beyond terminal height
        app.cursor_x = PARAM_DT;
        app.update_value_from_mouse_y(150);
        assert_eq!(app.values[0][PARAM_DT], 0, "Mouse beyond terminal should give min value");
    }

    #[test]
    fn test_update_value_from_mouse_y_zero_height() {
        let mut app = App::new();
        app.terminal_height = 0;
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        
        let initial_value = app.values[0][PARAM_DT];
        
        // Should not crash or change value when terminal_height is 0
        app.update_value_from_mouse_y(50);
        assert_eq!(app.values[0][PARAM_DT], initial_value, "Value should not change when terminal_height is 0");
    }

    #[test]
    fn test_dt2_parameter() {
        let mut app = App::new();
        
        // Set DT2 values for all operators
        app.values[0][PARAM_DT2] = 1;
        app.values[1][PARAM_DT2] = 2;
        app.values[2][PARAM_DT2] = 3;
        app.values[3][PARAM_DT2] = 0;
        
        // Set D2R values to distinguish from DT2
        app.values[0][PARAM_D2R] = 5;
        app.values[1][PARAM_D2R] = 7;
        app.values[2][PARAM_D2R] = 9;
        app.values[3][PARAM_D2R] = 11;
        
        // Generate YM2151 events
        let events = app.to_ym2151_events();
        
        // Verify DT2/D2R register values
        for op in 0..4 {
            let dt2_d2r_addr = format!("0x{:02X}", 0xC0 + op * 8);
            let event = events.iter().find(|e| e.addr == dt2_d2r_addr);
            assert!(event.is_some(), "DT2/D2R event for operator {} should exist", op);
            
            let value = u8::from_str_radix(
                event.unwrap().data.trim_start_matches("0x"),
                16
            ).unwrap();
            
            let dt2 = (value >> 6) & 0x03;
            let d2r = value & 0x0F;
            
            assert_eq!(dt2, app.values[op][PARAM_DT2], "Operator {} DT2 should match", op);
            assert_eq!(d2r, app.values[op][PARAM_D2R], "Operator {} D2R should match", op);
        }
        
        // Test round-trip: convert back to tone data
        let loaded_values = App::events_to_tone_data(&events).unwrap();
        
        for op in 0..4 {
            assert_eq!(loaded_values[op][PARAM_DT2], app.values[op][PARAM_DT2], 
                "Operator {} DT2 should round-trip correctly", op);
            assert_eq!(loaded_values[op][PARAM_D2R], app.values[op][PARAM_D2R], 
                "Operator {} D2R should round-trip correctly", op);
        }
    }

    #[test]
    fn test_ch_row_alg_fb_parameters() {
        let mut app = App::new();
        
        // Set ALG and FB values
        app.values[ROW_CH][CH_PARAM_ALG] = 7; // Max ALG value
        app.values[ROW_CH][CH_PARAM_FB] = 5;
        
        // Generate YM2151 events
        let events = app.to_ym2151_events();
        
        // Find the channel register event (0x20)
        let ch_event = events.iter().find(|e| e.addr == "0x20");
        assert!(ch_event.is_some(), "Channel register event should exist");
        
        let value = u8::from_str_radix(
            ch_event.unwrap().data.trim_start_matches("0x"),
            16
        ).unwrap();
        
        let alg = value & 0x07;
        let fb = (value >> 3) & 0x07;
        
        assert_eq!(alg, 7, "ALG should be 7");
        assert_eq!(fb, 5, "FB should be 5");
        
        // Test round-trip: convert back to tone data
        let loaded_values = App::events_to_tone_data(&events).unwrap();
        
        assert_eq!(loaded_values[ROW_CH][CH_PARAM_ALG], 7, "ALG should round-trip correctly");
        assert_eq!(loaded_values[ROW_CH][CH_PARAM_FB], 5, "FB should round-trip correctly");
    }

    #[test]
    fn test_ch_row_cursor_restriction() {
        let mut app = App::new();
        
        // Start on an operator row, move to the right edge
        app.cursor_y = 0;
        app.cursor_x = 9; // Last column (DT2)
        
        // Move down to CH row - cursor should be clamped
        app.move_cursor_down();
        app.move_cursor_down();
        app.move_cursor_down();
        app.move_cursor_down();
        
        assert_eq!(app.cursor_y, ROW_CH);
        assert_eq!(app.cursor_x, CH_PARAM_COUNT - 1, "Cursor should be clamped to last CH column");
        
        // Try to move right - should not move
        app.move_cursor_right();
        assert_eq!(app.cursor_x, CH_PARAM_COUNT - 1, "Cursor should not move beyond CH columns");
        
        // Move to column 0
        app.cursor_x = 0;
        
        // Move up to operator row - cursor should stay at 0
        app.move_cursor_up();
        assert_eq!(app.cursor_y, 3);
        assert_eq!(app.cursor_x, 0, "Cursor x should remain valid");
    }
}
