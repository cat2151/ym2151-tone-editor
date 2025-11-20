use crate::models::*;
use crate::file_ops;
#[cfg(windows)]
use crate::audio;

pub struct App {
    pub values: ToneData,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub value_by_mouse_move: bool,
    #[cfg(windows)]
    pub use_interactive_mode: bool,
}

impl App {
    pub fn new(#[allow(unused_variables)] use_interactive_mode: bool, value_by_mouse_move: bool) -> App {
        // Initialize with a basic FM piano-like tone
        // Based on typical YM2151 patch settings
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // New parameter order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
        // Operator 1 (M1): SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
        values[0] = [1, 20, 1, 31, 10, 5, 5, 7, 0, 0, 0, 0];
        
        // Operator 2 (M2): softer attack
        values[1] = [1, 30, 1, 25, 8, 6, 4, 6, 0, 0, 0, 0];
        
        // Operator 3 (C1): even softer
        values[2] = [1, 40, 2, 20, 6, 7, 3, 5, 0, 0, 0, 0];
        
        // Operator 4 (C2): gentle
        values[3] = [1, 35, 1, 22, 7, 6, 4, 6, 0, 0, 0, 0];
        
        // Channel settings: ALG (algorithm), FB (feedback), and MIDI Note Number
        // Default to ALG=4 (simple FM) and FB=0 (no feedback)
        // MIDI Note Number: 60 (middle C)
        values[4] = [4, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        
        let mut app = App {
            values,
            cursor_x: 0,
            cursor_y: 0,
            value_by_mouse_move,
            #[cfg(windows)]
            use_interactive_mode,
        };

        // Try to load from GM file format first, then fall back to legacy format
        const GM_FILE_PATH: &str = "tones/general_midi/000_AcousticGrand.json";
        
        if let Ok(loaded_values) = file_ops::load_from_gm_file(GM_FILE_PATH) {
            app.values = loaded_values;
        } else if let Ok(loaded_values) = file_ops::load_newest_json() {
            // Fall back to loading from legacy format
            app.values = loaded_values;
        }

        // Initialize interactive mode if enabled (Windows only)
        #[cfg(windows)]
        if use_interactive_mode {
            if let Err(e) = audio::init_interactive_mode(&app.values) {
                eprintln!("⚠️  Warning: Failed to start interactive mode: {}", e);
            }
        }

        app
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let max_x = if self.cursor_y == ROW_CH {
            CH_PARAM_COUNT - 1
        } else {
            GRID_WIDTH - 1
        };
        
        if self.cursor_x < max_x {
            self.cursor_x += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
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

    pub fn move_cursor_down(&mut self) {
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

    /// Get the data row index from the current cursor position (display row)
    /// For operator rows (0-3), maps display row to data row
    /// For CH row (4), returns ROW_CH
    fn get_data_row(&self) -> usize {
        if self.cursor_y < 4 {
            DISPLAY_ROW_TO_DATA_ROW[self.cursor_y]
        } else {
            self.cursor_y
        }
    }

    pub fn increase_value(&mut self) {
        let data_row = self.get_data_row();
        let current = self.values[data_row][self.cursor_x];
        let max = if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        };
        if current < max {
            self.values[data_row][self.cursor_x] = current + 1;
            #[cfg(windows)]
            audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
        }
    }

    pub fn decrease_value(&mut self) {
        let data_row = self.get_data_row();
        let current = self.values[data_row][self.cursor_x];
        if current > 0 {
            self.values[data_row][self.cursor_x] = current - 1;
            #[cfg(windows)]
            audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
        }
    }

    /// Increase the current parameter value by a specified amount
    /// Used for number key shortcuts (1-9 for +1 to +9, 0 for +10)
    pub fn increase_value_by(&mut self, amount: u8) {
        let data_row = self.get_data_row();
        let current = self.values[data_row][self.cursor_x];
        let max = if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        };
        
        // Calculate new value, clamping to max
        let new_value = current.saturating_add(amount).min(max);
        
        if new_value != current {
            self.values[data_row][self.cursor_x] = new_value;
            #[cfg(windows)]
            audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
        }
    }

    /// Decrease the current parameter value by a specified amount
    /// Used for SHIFT + number key shortcuts (SHIFT+1-9 for -1 to -9, SHIFT+0 for -10)
    pub fn decrease_value_by(&mut self, amount: u8) {
        let data_row = self.get_data_row();
        let current = self.values[data_row][self.cursor_x];
        
        // Calculate new value, clamping to 0
        let new_value = current.saturating_sub(amount);
        
        if new_value != current {
            self.values[data_row][self.cursor_x] = new_value;
            #[cfg(windows)]
            audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
        }
    }

    pub fn set_value_to_max(&mut self) {
        let data_row = self.get_data_row();
        let max = if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        };
        self.values[data_row][self.cursor_x] = max;
        #[cfg(windows)]
        audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
    }

    pub fn set_value_to_min(&mut self) {
        let data_row = self.get_data_row();
        self.values[data_row][self.cursor_x] = 0;
        #[cfg(windows)]
        audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
    }

    pub fn set_value_to_random(&mut self) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};
        
        let data_row = self.get_data_row();
        let max = if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        };
        
        // Use RandomState to generate a random value
        // This is a simple approach that doesn't require adding new dependencies
        let random_state = RandomState::new();
        let mut hasher = random_state.build_hasher();
        
        // Hash current time and position to get variation
        std::time::SystemTime::now().hash(&mut hasher);
        self.cursor_x.hash(&mut hasher);
        self.cursor_y.hash(&mut hasher);
        data_row.hash(&mut hasher);
        
        let hash = hasher.finish();
        let random_value = (hash % (max as u64 + 1)) as u8;
        
        self.values[data_row][self.cursor_x] = random_value;
        #[cfg(windows)]
        audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
    }

    /// Move cursor to a specific mouse position
    /// Maps mouse x,y coordinates to cursor position in the grid
    /// Based on the UI layout from ui.rs
    pub fn move_cursor_to_mouse_position(&mut self, mouse_x: u16, mouse_y: u16) {
        // UI layout constants (from ui.rs)
        const ROW_LABEL_WIDTH: u16 = 4;
        const CELL_WIDTH: u16 = 4;
        const LABEL_OFFSET: u16 = 1;
        const INNER_X: u16 = 1; // Border takes 1 character
        const INNER_Y: u16 = 1; // Border takes 1 character
        
        // Check if mouse is within the grid area (after row labels)
        if mouse_x < INNER_X + ROW_LABEL_WIDTH {
            return; // Mouse is in row label area
        }
        
        // Calculate column from mouse X position
        let relative_x = mouse_x - INNER_X - ROW_LABEL_WIDTH;
        let col = (relative_x / CELL_WIDTH) as usize;
        
        // Calculate row from mouse Y position
        // Operator rows: y = INNER_Y + LABEL_OFFSET + row (1-4)
        // CH row header: y = INNER_Y + LABEL_OFFSET + 4 (5)
        // CH row values: y = INNER_Y + LABEL_OFFSET + 5 (6)
        if mouse_y < INNER_Y + LABEL_OFFSET {
            return; // Mouse is in header area
        }
        
        let relative_y = mouse_y - INNER_Y - LABEL_OFFSET;
        
        // Determine which row the mouse is on
        let new_cursor_y = match relative_y {
            0..=3 => relative_y as usize, // Operator rows (M1, C1, M2, C2)
            5 => ROW_CH,                  // CH row (skip row 4 which is CH header)
            _ => return,                  // Outside valid rows
        };
        
        // Validate column bounds
        let max_x = if new_cursor_y == ROW_CH {
            CH_PARAM_COUNT - 1
        } else {
            GRID_WIDTH - 1
        };
        
        if col > max_x {
            return; // Column out of bounds
        }
        
        // Update cursor position
        self.cursor_x = col;
        self.cursor_y = new_cursor_y;
    }

    /// Update the parameter value based on mouse X position
    /// Maps mouse X position to parameter value range (0 to PARAM_MAX)
    /// Uses the middle third of the terminal width for full range
    /// Left of middle third sets to min (0), right of middle third sets to max
    pub fn update_value_from_mouse_x(&mut self, mouse_x: u16, terminal_width: u16) {
        if terminal_width == 0 {
            return; // Avoid division by zero
        }

        // Calculate middle third boundaries
        let third_width = terminal_width / 3;
        let left_boundary = third_width;
        let right_boundary = third_width * 2;
        
        let max_value = if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        };
        
        let new_value = if mouse_x < left_boundary {
            // Mouse is left of middle third -> set to minimum (0)
            0
        } else if mouse_x > right_boundary {
            // Mouse is right of middle third -> set to maximum
            max_value
        } else {
            // Mouse is within middle third -> map proportionally
            // left_boundary -> 0, right_boundary -> max value
            let middle_width = right_boundary - left_boundary;
            let relative_x = mouse_x - left_boundary;
            let normalized = if middle_width == 0 {
                0.0
            } else {
                relative_x as f32 / middle_width as f32
            };
            
            let value = (normalized * max_value as f32).round() as u8;
            value.min(max_value)
        };
        
        // Only update and play sound if the value actually changed
        let data_row = self.get_data_row();
        if self.values[data_row][self.cursor_x] != new_value {
            self.values[data_row][self.cursor_x] = new_value;
            #[cfg(windows)]
            audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
        }
    }

    /// Save tone data to JSON file
    pub fn save_to_json(&self) -> std::io::Result<()> {
        const GM_FILE_PATH: &str = "tones/general_midi/000_AcousticGrand.json";
        
        // Save to GM format
        file_ops::save_to_gm_file(GM_FILE_PATH, &self.values, "Edited Tone")?;
        
        // Also save to legacy format for backward compatibility
        file_ops::save_to_json(&self.values)?;
        
        Ok(())
    }

    /// Play the current tone without modifying any parameters
    /// This is triggered by 'P' or 'SPACE' key
    pub fn play_current_tone(&self) {
        #[cfg(windows)]
        audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
    }

    /// Move cursor to FB parameter and increase its value
    /// This is triggered by 'F' key
    pub fn increase_fb(&mut self) {
        // Move cursor to FB position (row 4, column 1)
        self.cursor_y = ROW_CH;
        self.cursor_x = CH_PARAM_FB;
        
        // Increase FB value
        let current = self.values[ROW_CH][CH_PARAM_FB];
        let max = CH_PARAM_MAX[CH_PARAM_FB];
        if current < max {
            self.values[ROW_CH][CH_PARAM_FB] = current + 1;
            #[cfg(windows)]
            audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
        }
    }

    /// Move cursor to FB parameter and decrease its value
    /// This is triggered by 'Shift+F' key
    pub fn decrease_fb(&mut self) {
        // Move cursor to FB position (row 4, column 1)
        self.cursor_y = ROW_CH;
        self.cursor_x = CH_PARAM_FB;
        
        // Decrease FB value
        let current = self.values[ROW_CH][CH_PARAM_FB];
        if current > 0 {
            self.values[ROW_CH][CH_PARAM_FB] = current - 1;
            #[cfg(windows)]
            audio::play_tone(&self.values, self.use_interactive_mode, self.cursor_x, self.cursor_y);
        }
    }

    /// Jump to operator row and increase value at current column
    /// operator_row: 0=M1, 1=C1, 2=M2, 3=C2
    pub fn jump_to_operator_and_increase(&mut self, operator_row: usize) {
        if operator_row >= 4 {
            return; // Invalid operator row
        }
        
        // Move cursor to operator row, preserving column
        self.cursor_y = operator_row;
        
        // Clamp cursor_x to valid range for operator rows
        let max_x = GRID_WIDTH - 1;
        if self.cursor_x > max_x {
            self.cursor_x = max_x;
        }
        
        // Increase value at current position
        self.increase_value();
    }

    /// Jump to operator row and decrease value at current column
    /// operator_row: 0=M1, 1=C1, 2=M2, 3=C2
    pub fn jump_to_operator_and_decrease(&mut self, operator_row: usize) {
        if operator_row >= 4 {
            return; // Invalid operator row
        }
        
        // Move cursor to operator row, preserving column
        self.cursor_y = operator_row;
        
        // Clamp cursor_x to valid range for operator rows
        let max_x = GRID_WIDTH - 1;
        if self.cursor_x > max_x {
            self.cursor_x = max_x;
        }
        
        // Decrease value at current position
        self.decrease_value();
    }

    /// Cleanup - stop interactive mode if active
    #[cfg(windows)]
    pub fn cleanup(&self) {
        if self.use_interactive_mode {
            audio::cleanup_interactive_mode();
        }
    }
}

