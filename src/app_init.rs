use crate::app::App;
use crate::file_ops;
use crate::models::*;

pub fn init_app(
    #[allow(unused_variables)] use_interactive_mode: bool,
    value_by_mouse_move: bool,
) -> App {
    let mut editor_rows = [[0; GRID_WIDTH]; GRID_HEIGHT];
    // order: SM, TL, MUL, AR, D1R, D1L, D2R, RR, DT, DT2, KS, AMS
    editor_rows[0] = [1, 20, 1, 31, 10, 5, 5, 7, 0, 0, 0, 0];
    editor_rows[1] = [1, 0, 0, 20, 6, 7, 3, 5, 0, 0, 0, 0];
    editor_rows[2] = [1, 30, 1, 25, 8, 6, 4, 6, 0, 0, 0, 0];
    editor_rows[3] = [1, 0, 1, 22, 7, 6, 4, 6, 0, 0, 0, 0];
    editor_rows[4] = [4, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut app = App {
        values: editor_rows,
        cursor_x: 0,
        cursor_y: 0,
        value_by_mouse_move,
        #[cfg(windows)]
        use_interactive_mode,
        hovered_penta_x: None,
    };
    const GM_FILE_PATH: &str = "tones/general_midi/000_AcousticGrand.json";
    #[cfg(windows)]
    use crate::audio::log_verbose;

    if let Ok(loaded_values) = file_ops::load_from_gm_file(GM_FILE_PATH) {
        app.values = loaded_values;
        #[cfg(windows)]
        log_verbose("init_app: loaded from GM_FILE_PATH");
    } else if let Ok(loaded_values) = file_ops::load_newest_json() {
        app.values = loaded_values;
        #[cfg(windows)]
        log_verbose("init_app: loaded from newest JSON");
    } else {
        #[cfg(windows)]
        log_verbose("init_app: using hardcoded default values");
    }
    app
}
