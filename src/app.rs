use crate::models::*;
use crate::file_ops;
#[cfg(windows)]
use crate::register;

pub struct App {
    pub values: ToneData,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl App {
    pub fn new() -> App {
        // Initialize with a basic FM piano-like tone
        // Based on typical YM2151 patch settings
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Operator 1 (Carrier): DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, DT2, AMS
        values[0] = [0, 1, 20, 0, 31, 10, 5, 5, 7, 0, 0];
        
        // Operator 2 (Modulator): softer attack
        values[1] = [0, 1, 30, 0, 25, 8, 6, 4, 6, 0, 0];
        
        // Operator 3 (Modulator): even softer
        values[2] = [0, 2, 40, 0, 20, 6, 7, 3, 5, 0, 0];
        
        // Operator 4 (Modulator): gentle
        values[3] = [0, 1, 35, 0, 22, 7, 6, 4, 6, 0, 0];
        
        // Channel settings: ALG (algorithm) and FB (feedback) in first 2 positions
        // Default to ALG=4 (simple FM) and FB=0 (no feedback)
        // Slot masks: OP1, OP2, OP3, OP4 all enabled (1)
        // MIDI Note Number: 60 (middle C)
        values[4] = [4, 0, 1, 1, 1, 1, 60, 0, 0, 0, 0];
        
        let mut app = App {
            values,
            cursor_x: 0,
            cursor_y: 0,
        };

        // Try to load the newest JSON file from current directory
        if let Ok(loaded_values) = file_ops::load_newest_json() {
            app.values = loaded_values;
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

    pub fn increase_value(&mut self) {
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

    pub fn decrease_value(&mut self) {
        let current = self.values[self.cursor_y][self.cursor_x];
        if current > 0 {
            self.values[self.cursor_y][self.cursor_x] = current - 1;
            #[cfg(windows)]
            self.call_cat_play_mml();
        }
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
        if self.values[self.cursor_y][self.cursor_x] != new_value {
            self.values[self.cursor_y][self.cursor_x] = new_value;
            #[cfg(windows)]
            self.call_cat_play_mml();
        }
    }

    /// Save tone data to JSON file
    pub fn save_to_json(&self) -> std::io::Result<()> {
        file_ops::save_to_json(&self.values)
    }

    /// Call ym2151-log-play-server client with current tone data as JSON string
    /// This function sends JSON content via named pipe to the server
    /// Windows-only functionality
    #[cfg(windows)]
    fn call_cat_play_mml(&self) {
        // Get JSON string of current tone data
        let json_string = match register::to_json_string(&self.values) {
            Ok(json) => json,
            Err(_) => return, // Silently fail if JSON conversion fails
        };

        // Send JSON content to server via named pipe
        // Using the ym2151-log-play-server client library with send_json
        // Automatically chooses optimal method (direct or file-based) based on size
        let _ = ym2151_log_play_server::client::send_json(&json_string);
        
        // Silently ignore errors - server should be auto-started at app launch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_update_value_from_mouse_x() {
        let mut app = App::new();
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
        let mut app = App::new();
        let terminal_width = 0;
        app.cursor_x = PARAM_DT;
        app.cursor_y = 0;
        
        let initial_value = app.values[0][PARAM_DT];
        
        // Should not crash or change value when terminal_width is 0
        app.update_value_from_mouse_x(50, terminal_width);
        assert_eq!(app.values[0][PARAM_DT], initial_value, "Value should not change when terminal_width is 0");
    }

    #[test]
    fn test_update_value_from_mouse_x_left_right_edges() {
        let mut app = App::new();
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
}