use crate::models::ToneData;
use crate::register;
use crate::register_list;
use std::{
    fs,
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
    migrate_history_from_roaming(&path);
    load_history_at_path(&path)
}

const HISTORY_MAX: usize = 26;

/// Get the path to the history file in the local config directory.
/// Returns None if the config directory cannot be determined.
pub fn history_file_path() -> Option<PathBuf> {
    dirs::config_local_dir().map(|dir| dir.join("ym2151-tone-editor").join("history_tone.json"))
}

/// Get the legacy (Roaming) path for the history file used before the AppData\Local migration.
/// Returns None if the config directory cannot be determined.
fn history_file_path_legacy() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("ym2151-tone-editor").join("history_tone.json"))
}

/// If the new local path does not exist but the legacy Roaming path does, copy the file to the
/// new location so existing history is preserved across the migration.
pub fn migrate_history_from_roaming(new_path: &Path) {
    if let Some(legacy) = history_file_path_legacy() {
        migrate_history_from_roaming_at_paths(&legacy, new_path);
    }
}

/// Inner migration helper that accepts explicit paths (also used by tests).
pub fn migrate_history_from_roaming_at_paths(legacy_path: &Path, new_path: &Path) {
    if new_path.exists() {
        return;
    }
    if legacy_path.exists() {
        if let Some(parent) = new_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::copy(legacy_path, new_path);
    }
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
