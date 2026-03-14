mod shortcuts;

#[cfg(windows)]
use crate::audio;
use crate::file_ops;
use crate::models::*;

pub struct App {
    pub values: ToneData,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub value_by_mouse_move: bool,
    #[cfg(windows)]
    pub use_interactive_mode: bool,
    /// ペンタトニック鍵盤のマウスホバー座標(Noneなら未ホバー)
    pub hovered_penta_x: Option<usize>,
    /// Envelope delay in seconds before tone parameters are set (default: 0.005)
    #[allow(dead_code)] // Used on Windows builds for audio playback
    pub envelope_delay_seconds: f64,
    /// Last operator row (0-3) the cursor was on before moving to CH row
    /// Used for displaying operation guides when cursor is on CH row
    pub last_operator_row: usize,
}

impl App {
    /// 仮想ペンタトニック鍵盤上のマウス座標からホバーx座標を更新
    /// ALG図直下の描画位置に合わせて判定
    pub fn update_hovered_penta_x(
        &mut self,
        mouse_x: u16,
        mouse_y: u16,
        inner: ratatui::layout::Rect,
        penta_keyboard_y: u16,
    ) {
        if mouse_y != penta_keyboard_y {
            self.hovered_penta_x = None;
            return;
        }
        if mouse_x >= inner.x && mouse_x < inner.x + inner.width {
            let rel_x = mouse_x - inner.x;
            self.hovered_penta_x = Some(rel_x as usize);
        } else {
            self.hovered_penta_x = None;
        }
    }
    pub fn new(
        #[allow(unused_variables)] use_interactive_mode: bool,
        value_by_mouse_move: bool,
        envelope_delay_seconds: f64,
    ) -> App {
        let mut app = crate::app_init::init_app(
            use_interactive_mode,
            value_by_mouse_move,
            envelope_delay_seconds,
        );
        app.hovered_penta_x = None;
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

            // Track the new position if it's an operator row
            if self.cursor_y < ROW_CH {
                self.last_operator_row = self.cursor_y;
            }

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
            // Track current position if it's an operator row (before moving)
            if self.cursor_y < ROW_CH {
                self.last_operator_row = self.cursor_y;
            }

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

    /// Play audio feedback for the current tone (Windows only)
    fn play_audio(&self) {
        #[cfg(windows)]
        audio::play_tone(
            &self.values,
            self.use_interactive_mode,
            self.cursor_x,
            self.cursor_y,
            self.envelope_delay_seconds,
        );
    }

    /// Get the maximum allowed value for the current cursor position
    fn get_current_max(&self) -> u8 {
        if self.cursor_y == ROW_CH && self.cursor_x < CH_PARAM_COUNT {
            CH_PARAM_MAX[self.cursor_x]
        } else {
            PARAM_MAX[self.cursor_x]
        }
    }

    /// Set cursor_x to the given parameter column and increase or decrease its value.
    /// Only applies when the cursor is on an operator row (not CH row).
    fn jump_to_op_param(&mut self, param_x: usize, increase: bool) {
        self.cursor_x = param_x;
        if self.cursor_y < ROW_CH {
            if increase {
                self.increase_value();
            } else {
                self.decrease_value();
            }
        }
    }

    /// Jump to a CH row parameter and increase or decrease its value
    fn jump_to_ch_param(&mut self, ch_param: usize, increase: bool) {
        self.cursor_y = ROW_CH;
        self.cursor_x = ch_param;
        if increase {
            self.increase_value();
        } else {
            self.decrease_value();
        }
    }

    pub fn increase_value(&mut self) {
        let data_row = self.cursor_y;
        let current = self.values[data_row][self.cursor_x];
        let max = self.get_current_max();
        if current < max {
            self.values[data_row][self.cursor_x] = current + 1;
            self.play_audio();
        }
    }

    pub fn decrease_value(&mut self) {
        let data_row = self.cursor_y;
        let current = self.values[data_row][self.cursor_x];
        if current > 0 {
            self.values[data_row][self.cursor_x] = current - 1;
            self.play_audio();
        }
    }

    /// Increase the current parameter value by a specified amount
    /// Used for number key shortcuts (1-9 for +1 to +9, 0 for +10)
    pub fn increase_value_by(&mut self, amount: u8) {
        let data_row = self.cursor_y;
        let current = self.values[data_row][self.cursor_x];
        let max = self.get_current_max();
        let new_value = current.saturating_add(amount).min(max);
        if new_value != current {
            self.values[data_row][self.cursor_x] = new_value;
            self.play_audio();
        }
    }

    /// Decrease the current parameter value by a specified amount
    /// Used for SHIFT + number key shortcuts (SHIFT+1-9 for -1 to -9, SHIFT+0 for -10)
    pub fn decrease_value_by(&mut self, amount: u8) {
        let data_row = self.cursor_y;
        let current = self.values[data_row][self.cursor_x];
        let new_value = current.saturating_sub(amount);
        if new_value != current {
            self.values[data_row][self.cursor_x] = new_value;
            self.play_audio();
        }
    }

    pub fn set_value_to_max(&mut self) {
        let max = self.get_current_max();
        self.values[self.cursor_y][self.cursor_x] = max;
        self.play_audio();
    }

    pub fn set_value_to_min(&mut self) {
        self.values[self.cursor_y][self.cursor_x] = 0;
        self.play_audio();
    }

    pub fn set_value_to_random(&mut self) {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let max = self.get_current_max();
        let random_state = RandomState::new();
        let mut hasher = random_state.build_hasher();
        std::time::SystemTime::now().hash(&mut hasher);
        self.cursor_x.hash(&mut hasher);
        self.cursor_y.hash(&mut hasher);
        let hash = hasher.finish();
        let random_value = (hash % (max as u64 + 1)) as u8;
        self.values[self.cursor_y][self.cursor_x] = random_value;
        self.play_audio();
    }

    /// Randomize all tone parameters using web-ym2151 random-tone logic.
    /// Triggered by F5 key.
    pub fn randomize_tone(&mut self) {
        use crate::random_tone::{generate_random_tone, RandomToneConfig};
        let current_note = self.values[ROW_CH][CH_PARAM_NOTE];
        self.values = generate_random_tone(&RandomToneConfig::default(), current_note);
        self.play_audio();
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
            0..=3 => relative_y as usize, // Operator rows
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

        let max_value = self.get_current_max();

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
            (normalized * max_value as f32).round() as u8
        };

        // Only update and play sound if the value actually changed
        let data_row = self.cursor_y;
        if self.values[data_row][self.cursor_x] != new_value {
            self.values[data_row][self.cursor_x] = new_value;
            self.play_audio();
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

    /// Append current tone data as a new variation to GM file
    /// This is triggered by CTRL+S
    pub fn save_to_gm_variations(&self) -> std::io::Result<()> {
        const GM_FILE_PATH: &str = "tones/general_midi/000_AcousticGrand.json";

        // Append to GM format variations array
        file_ops::append_to_gm_file(GM_FILE_PATH, &self.values, "Edited Tone")?;

        Ok(())
    }

    /// Play the current tone without modifying any parameters
    /// This is triggered by 'P' or 'SPACE' key
    pub fn play_current_tone(&self) {
        self.play_audio();
    }

    /// Move cursor to FB parameter and increase its value
    /// This is triggered by 'F' key
    pub fn increase_fb(&mut self) {
        self.jump_to_ch_param(CH_PARAM_FB, true);
    }

    /// Move cursor to FB parameter and decrease its value
    /// This is triggered by 'Shift+F' key
    pub fn decrease_fb(&mut self) {
        self.jump_to_ch_param(CH_PARAM_FB, false);
    }

    /// Move cursor to ALG parameter and increase its value
    /// This is triggered by 'g' key
    pub fn increase_alg(&mut self) {
        self.jump_to_ch_param(CH_PARAM_ALG, true);
    }

    /// Move cursor to ALG parameter and decrease its value
    /// This is triggered by 'G' key (Shift+g)
    pub fn decrease_alg(&mut self) {
        self.jump_to_ch_param(CH_PARAM_ALG, false);
    }

    /// Jump to operator row and increase value at current column
    pub fn jump_to_operator_and_increase(&mut self, operator_row: usize) {
        if operator_row >= 4 {
            return; // Invalid operator row
        }
        self.cursor_y = operator_row;
        self.last_operator_row = operator_row;
        if self.cursor_x > GRID_WIDTH - 1 {
            self.cursor_x = GRID_WIDTH - 1;
        }
        self.increase_value();
    }

    /// Jump to operator row and decrease value at current column
    pub fn jump_to_operator_and_decrease(&mut self, operator_row: usize) {
        if operator_row >= 4 {
            return; // Invalid operator row
        }
        self.cursor_y = operator_row;
        self.last_operator_row = operator_row;
        if self.cursor_x > GRID_WIDTH - 1 {
            self.cursor_x = GRID_WIDTH - 1;
        }
        self.decrease_value();
    }

    /// Jump to Note Number parameter and increase its value
    /// This is triggered by 'j' key
    pub fn jump_to_note_and_increase(&mut self) {
        self.jump_to_ch_param(CH_PARAM_NOTE, true);
    }

    /// Jump to Note Number parameter and decrease its value
    /// This is triggered by 'J' key (Shift+j)
    pub fn jump_to_note_and_decrease(&mut self) {
        self.jump_to_ch_param(CH_PARAM_NOTE, false);
    }

    /// Cleanup - stop interactive mode if active
    #[cfg(windows)]
    pub fn cleanup(&self) {
        if self.use_interactive_mode {
            audio::cleanup_interactive_mode();
        }
    }
}
