use crate::app::App;
#[cfg(windows)]
use crate::audio;
use crate::file_ops;
use crate::models::*;

pub fn init_app(
    #[allow(unused_variables)] use_interactive_mode: bool,
    value_by_mouse_move: bool,
) -> App {
    let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
    values[0] = [1, 20, 1, 31, 10, 5, 5, 7, 0, 0, 0, 0];
    values[1] = [1, 30, 1, 25, 8, 6, 4, 6, 0, 0, 0, 0];
    values[2] = [1, 0, 2, 20, 6, 7, 3, 5, 0, 0, 0, 0];
    values[3] = [1, 0, 1, 22, 7, 6, 4, 6, 0, 0, 0, 0];
    values[4] = [4, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut app = App {
        values,
        cursor_x: 0,
        cursor_y: 0,
        value_by_mouse_move,
        #[cfg(windows)]
        use_interactive_mode,
    };
    const GM_FILE_PATH: &str = "tones/general_midi/000_AcousticGrand.json";
    if let Ok(loaded_values) = file_ops::load_from_gm_file(GM_FILE_PATH) {
        app.values = loaded_values;
    } else if let Ok(loaded_values) = file_ops::load_newest_json() {
        app.values = loaded_values;
    }
    #[cfg(windows)]
    if use_interactive_mode {
        if let Err(e) = audio::init_interactive_mode(&app.values) {
            eprintln!("⚠️  Warning: Failed to start interactive mode: {}", e);
        }
    }
    app
}
