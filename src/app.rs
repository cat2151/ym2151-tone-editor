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
            self.call_cat_play_mml();
        }
    }

    pub fn decrease_value(&mut self) {
        let data_row = self.get_data_row();
        let current = self.values[data_row][self.cursor_x];
        if current > 0 {
            self.values[data_row][self.cursor_x] = current - 1;
            #[cfg(windows)]
            self.call_cat_play_mml();
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
        self.call_cat_play_mml();
    }

    pub fn set_value_to_min(&mut self) {
        let data_row = self.get_data_row();
        self.values[data_row][self.cursor_x] = 0;
        #[cfg(windows)]
        self.call_cat_play_mml();
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
        self.call_cat_play_mml();
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

    /// Cleanup - stop interactive mode if active
    #[cfg(windows)]
    pub fn cleanup(&self) {
        if self.use_interactive_mode {
            audio::cleanup_interactive_mode();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_movement() {
        let mut app = App::new(false, false);
        
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
    fn test_increase_decrease_value() {
        let mut app = App::new(false, false);
        
        // Move cursor to a parameter with a wider range (e.g., TL at index 1)
        app.cursor_x = PARAM_TL;
        
        // Store initial value
        let initial_value = app.values[0][PARAM_TL];
        
        // Increase value
        app.increase_value();
        assert_eq!(app.values[0][PARAM_TL], initial_value + 1);
        
        // Decrease value
        app.decrease_value();
        assert_eq!(app.values[0][PARAM_TL], initial_value);
        
        // Test boundary: decrease at 0 should not go negative
        app.values[0][PARAM_TL] = 0;
        app.decrease_value();
        assert_eq!(app.values[0][PARAM_TL], 0);
        
        // Test boundary: increase at max should not exceed
        app.cursor_x = PARAM_SM; // SM parameter
        app.values[0][PARAM_SM] = PARAM_MAX[PARAM_SM]; // Set to max (1)
        app.increase_value();
        assert_eq!(app.values[0][PARAM_SM], PARAM_MAX[PARAM_SM]);
    }

    #[test]
    fn test_ch_row_cursor_restriction() {
        let mut app = App::new(false, false);
        
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

    #[test]
    fn test_update_value_from_mouse_x() {
        let mut app = App::new(false, false);
        let terminal_width = 120;
        app.cursor_x = PARAM_DT; // DT has max value of 7
        app.cursor_y = 0;
        
        // Test mouse at left boundary of middle third (x=40) should give min value (0)
        app.update_value_from_mouse_x(40, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 0, "Left boundary of middle third should give min value");
        
        // Test mouse at right boundary of middle third (x=80) should give max value
        app.update_value_from_mouse_x(80, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 7, "Right boundary of middle third should give max value");
        
        // Test mouse at center of middle third (x=60) should give approximately half of max
        app.update_value_from_mouse_x(60, terminal_width);
        let middle_value = app.values[0][PARAM_DT];
        assert!(middle_value >= 3 && middle_value <= 4, "Center of middle third should give ~half of max value, got {}", middle_value);
        
        // Test with different parameter (MUL has max value of 15)
        app.cursor_x = PARAM_MUL;
        app.update_value_from_mouse_x(40, terminal_width);
        assert_eq!(app.values[0][PARAM_MUL], 0, "Left boundary of middle third should give min value for MUL");
        
        app.update_value_from_mouse_x(80, terminal_width);
        assert_eq!(app.values[0][PARAM_MUL], 15, "Right boundary of middle third should give max value for MUL");
        
        // Test edge case: mouse left of middle third (left side) should set to min (0)
        app.cursor_x = PARAM_DT;
        app.update_value_from_mouse_x(20, terminal_width); // Left third
        assert_eq!(app.values[0][PARAM_DT], 0, "Mouse left of middle third should set to min value (0)");
        
        // Test edge case: mouse right of middle third (right side) should set to max
        app.update_value_from_mouse_x(100, terminal_width); // Right third
        assert_eq!(app.values[0][PARAM_DT], 7, "Mouse right of middle third should set to max value (7)");
    }

    #[test]
    fn test_update_value_from_mouse_x_zero_width() {
        let mut app = App::new(false, false);
        let terminal_width = 0;
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        
        let initial_value = app.values[0][PARAM_DT];
        
        // Should not crash or change value when terminal_width is 0
        app.update_value_from_mouse_x(50, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], initial_value, "Value should not change when terminal_width is 0");
    }

    #[test]
    fn test_set_value_to_max() {
        let mut app = App::new(false, false);
        
        // Test with operator row parameter (DT, max = 7)
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        app.values[0][PARAM_DT] = 3;
        
        app.set_value_to_max();
        assert_eq!(app.values[0][PARAM_DT], 7, "DT should be set to max value (7)");
        
        // Test with different parameter (MUL, max = 15)
        app.cursor_x = PARAM_MUL;
        app.values[0][PARAM_MUL] = 5;
        
        app.set_value_to_max();
        assert_eq!(app.values[0][PARAM_MUL], 15, "MUL should be set to max value (15)");
        
        // Test with CH row parameter (ALG, max = 7)
        app.cursor_y = ROW_CH;
        app.cursor_x = CH_PARAM_ALG;
        app.values[ROW_CH][CH_PARAM_ALG] = 2;
        
        app.set_value_to_max();
        assert_eq!(app.values[ROW_CH][CH_PARAM_ALG], 7, "ALG should be set to max value (7)");
        
        // Test with CH row FB (max = 7)
        app.cursor_x = CH_PARAM_FB;
        app.values[ROW_CH][CH_PARAM_FB] = 1;
        
        app.set_value_to_max();
        assert_eq!(app.values[ROW_CH][CH_PARAM_FB], 7, "FB should be set to max value (7)");
    }

    #[test]
    fn test_set_value_to_min() {
        let mut app = App::new(false, false);
        
        // Test with operator row parameter
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        app.values[0][PARAM_DT] = 5;
        
        app.set_value_to_min();
        assert_eq!(app.values[0][PARAM_DT], 0, "DT should be set to min value (0)");
        
        // Test with different parameter
        app.cursor_x = PARAM_MUL;
        app.values[0][PARAM_MUL] = 10;
        
        app.set_value_to_min();
        assert_eq!(app.values[0][PARAM_MUL], 0, "MUL should be set to min value (0)");
        
        // Test with CH row parameter
        app.cursor_y = ROW_CH;
        app.cursor_x = CH_PARAM_ALG;
        app.values[ROW_CH][CH_PARAM_ALG] = 5;
        
        app.set_value_to_min();
        assert_eq!(app.values[ROW_CH][CH_PARAM_ALG], 0, "ALG should be set to min value (0)");
    }

    #[test]
    fn test_set_value_to_random() {
        let mut app = App::new(false, false);
        
        // Test with operator row parameter (DT, max = 7)
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        
        app.set_value_to_random();
        let random_value = app.values[0][PARAM_DT];
        assert!(random_value <= 7, "Random DT value should be <= 7, got {}", random_value);
        
        // Test with different parameter (TL, max = 99)
        app.cursor_x = PARAM_TL;
        
        app.set_value_to_random();
        let random_value = app.values[0][PARAM_TL];
        assert!(random_value <= 99, "Random TL value should be <= 99, got {}", random_value);
        
        // Test with CH row parameter (ALG, max = 7)
        app.cursor_y = ROW_CH;
        app.cursor_x = CH_PARAM_ALG;
        
        app.set_value_to_random();
        let random_value = app.values[ROW_CH][CH_PARAM_ALG];
        assert!(random_value <= 7, "Random ALG value should be <= 7, got {}", random_value);
        
        // Test that calling multiple times produces values in valid range
        for _ in 0..10 {
            app.cursor_x = PARAM_MUL;
            app.cursor_y = 0;
            app.set_value_to_random();
            let random_value = app.values[0][PARAM_MUL];
            assert!(random_value <= 15, "Random MUL value should be <= 15, got {}", random_value);
        }
    }

    #[test]
    fn test_update_value_from_mouse_x_left_right_edges() {
        let mut app = App::new(false, false);
        let terminal_width = 120;
        
        // Test with DT parameter (max value = 7)
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        
        // Far left (x=0) should set to min (0)
        app.update_value_from_mouse_x(0, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 0, "Far left should set to min value (0)");
        
        // Far right (x=119) should set to max (7)
        app.update_value_from_mouse_x(119, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 7, "Far right should set to max value (7)");
        
        // Just before left boundary (x=39) should set to min (0)
        app.update_value_from_mouse_x(39, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 0, "Just before left boundary should set to min value (0)");
        
        // Just after right boundary (x=81) should set to max (7)
        app.update_value_from_mouse_x(81, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], 7, "Just after right boundary should set to max value (7)");
        
        // Test with TL parameter (max value = 99)
        app.cursor_x = PARAM_TL;
        
        // Far left should set to min (0)
        app.update_value_from_mouse_x(0, terminal_width);
        assert_eq!(app.values[0][PARAM_TL], 0, "Far left should set to min value (0) for TL");
        
        // Far right should set to max (99)
        app.update_value_from_mouse_x(119, terminal_width);
        assert_eq!(app.values[0][PARAM_TL], 99, "Far right should set to max value (99) for TL");
        
        // Test with CH row parameters
        app.cursor_y = ROW_CH;
        app.cursor_x = CH_PARAM_ALG; // ALG has max value of 7
        
        // Far left should set to min (0)
        app.update_value_from_mouse_x(0, terminal_width);
        assert_eq!(app.values[ROW_CH][CH_PARAM_ALG], 0, "Far left should set to min value (0) for CH ALG");
        
        // Far right should set to max (7)
        app.update_value_from_mouse_x(119, terminal_width);
        assert_eq!(app.values[ROW_CH][CH_PARAM_ALG], 7, "Far right should set to max value (7) for CH ALG");
    }

    #[test]
    fn test_move_cursor_to_mouse_position() {
        let mut app = App::new(false, false);
        
        // Test moving cursor to operator row (M1, row 0)
        // UI layout: row_label_width=4, cell_width=4, inner_x=1, inner_y=1, label_offset=1
        // First cell starts at x=5 (1+4), operator rows start at y=2 (1+1+0)
        app.move_cursor_to_mouse_position(5, 2); // M1, column 0
        assert_eq!(app.cursor_x, 0);
        assert_eq!(app.cursor_y, 0);
        
        // Test moving cursor to different column
        app.move_cursor_to_mouse_position(9, 2); // M1, column 1
        assert_eq!(app.cursor_x, 1);
        assert_eq!(app.cursor_y, 0);
        
        // Test moving cursor to C1 row (display row 1, y=3)
        app.move_cursor_to_mouse_position(5, 3); // C1, column 0
        assert_eq!(app.cursor_x, 0);
        assert_eq!(app.cursor_y, 1);
        
        // Test moving cursor to CH row (y=7: 1+1+5)
        app.move_cursor_to_mouse_position(5, 7); // CH row, column 0
        assert_eq!(app.cursor_x, 0);
        assert_eq!(app.cursor_y, ROW_CH);
        
        // Test moving cursor to CH row, column 1
        app.move_cursor_to_mouse_position(9, 7); // CH row, column 1
        assert_eq!(app.cursor_x, 1);
        assert_eq!(app.cursor_y, ROW_CH);
        
        // Test clicking outside valid columns for CH row (should be ignored)
        let prev_x = app.cursor_x;
        let prev_y = app.cursor_y;
        app.move_cursor_to_mouse_position(25, 7); // CH row, column 5 (out of bounds, CH has only 3 columns)
        assert_eq!(app.cursor_x, prev_x, "Cursor X should not change when clicking outside valid CH columns");
        assert_eq!(app.cursor_y, prev_y, "Cursor Y should not change when clicking outside valid CH columns");
        
        // Test clicking in row label area (should be ignored)
        let prev_x = app.cursor_x;
        let prev_y = app.cursor_y;
        app.move_cursor_to_mouse_position(3, 2); // In row label area
        assert_eq!(app.cursor_x, prev_x, "Cursor should not move when clicking in row label area");
        assert_eq!(app.cursor_y, prev_y, "Cursor should not move when clicking in row label area");
        
        // Test clicking in header area (should be ignored)
        let prev_x = app.cursor_x;
        let prev_y = app.cursor_y;
        app.move_cursor_to_mouse_position(5, 1); // In header area
        assert_eq!(app.cursor_x, prev_x, "Cursor should not move when clicking in header area");
        assert_eq!(app.cursor_y, prev_y, "Cursor should not move when clicking in header area");
        
        // Test clicking on CH header row (y=6, should be ignored)
        let prev_x = app.cursor_x;
        let prev_y = app.cursor_y;
        app.move_cursor_to_mouse_position(5, 6); // CH header row
        assert_eq!(app.cursor_x, prev_x, "Cursor should not move when clicking on CH header row");
        assert_eq!(app.cursor_y, prev_y, "Cursor should not move when clicking on CH header row");
    }
}