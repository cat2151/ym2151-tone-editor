use crate::models::ToneData;
use crate::register;
use crate::register_list;
use std::{
    io,
    path::{Path, PathBuf},
};

/// Load history register strings from the given path.
/// Returns an empty Vec if the file does not exist.
pub fn load_history_at_path(path: &Path) -> io::Result<Vec<String>> {
    register_list::load_register_list_at_path(path)
}

/// Load history register strings from the local config directory.
/// Returns an empty Vec if the file does not exist.
/// Returns an error if the config directory cannot be determined or the file is corrupted.
pub fn load_history() -> io::Result<Vec<String>> {
    let path = history_file_path().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
    })?;
    load_history_at_path(&path)
}

const HISTORY_MAX: usize = 26;

/// Get the path to the history file in the local config directory.
/// Returns None if the config directory cannot be determined.
pub fn history_file_path() -> Option<PathBuf> {
    dirs::config_local_dir().map(|dir| dir.join("ym2151-tone-editor").join("history_tone.json"))
}

/// Save tone data to history at the given path.
/// Adds the current tone as the newest entry (index 0), keeping up to HISTORY_MAX entries.
/// Any existing occurrence of the same registers is removed first so each entry is unique.
/// Returns an error if the file exists but cannot be parsed (corrupted), rather than silently
/// overwriting history.
pub fn save_to_history_at_path(path: &Path, values: &ToneData) -> io::Result<()> {
    let registers = register::editor_rows_to_registers(values);
    register_list::save_register_list_at_path(path, &registers, HISTORY_MAX)
}

/// Save tone data to history in the local config directory.
/// Path: `{config_local_dir}/ym2151-tone-editor/history_tone.json`
/// Errors are intentionally ignored by the caller to avoid disrupting the main flow.
pub fn save_to_history(values: &ToneData) -> io::Result<()> {
    let path = history_file_path().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
    })?;
    save_to_history_at_path(&path, values)
}
