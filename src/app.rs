use crate::models::*;
use crate::{file_ops, register};

pub struct App {
    pub values: ToneData,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub terminal_height: u16,
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
        values[4] = [4, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0];
        
        let mut app = App {
            values,
            cursor_x: 0,
            cursor_y: 0,
            terminal_height: 24, // Default, will be updated in main loop
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

    /// Update the parameter value based on mouse Y position
    /// Maps mouse Y position to parameter value range (0 to PARAM_MAX)
    pub fn update_value_from_mouse_y(&mut self, mouse_y: u16) {
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

    /// Save tone data to JSON file
    pub fn save_to_json(&self) -> std::io::Result<()> {
        file_ops::save_to_json(&self.values)
    }

    /// Call ym2151-log-play-server client with current tone data as JSON string
    /// This function sends JSON content directly via named pipe to the server
    /// Windows-only functionality
    #[cfg(windows)]
    fn call_cat_play_mml(&self) {
        // Get JSON string of current tone data
        let json_string = match register::to_json_string(&self.values) {
            Ok(json) => json,
            Err(_) => return, // Silently fail if JSON conversion fails
        };

        // Send JSON content string directly to server via named pipe
        // Using the ym2151-log-play-server client library with send_json_direct
        // This improves response time and allows sound during key repeat
        let _ = ym2151_log_play_server::client::send_json_direct(&json_string);
        
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
}