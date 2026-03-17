use crate::models::ToneData;
use crate::register;
use crate::register_list;
use std::{
    io,
    path::{Path, PathBuf},
};

/// Load favorites register strings from the given path.
/// Returns an empty Vec if the file does not exist.
pub fn load_favorites_at_path(path: &Path) -> io::Result<Vec<String>> {
    register_list::load_register_list_at_path(path)
}

/// Load favorites register strings from the local config directory.
/// Returns an empty Vec if the file does not exist.
/// Returns an error if the config directory cannot be determined or the file is corrupted.
#[allow(dead_code)]
pub fn load_favorites() -> io::Result<Vec<String>> {
    let path = favorites_file_path().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
    })?;
    load_favorites_at_path(&path)
}

const FAVORITES_MAX: usize = 20;

/// Get the path to the favorites file in the local config directory.
/// Returns None if the config directory cannot be determined.
pub fn favorites_file_path() -> Option<PathBuf> {
    dirs::config_local_dir().map(|dir| dir.join("ym2151-tone-editor").join("favorites.json"))
}

/// Save tone data to favorites at the given path.
/// Adds the current tone as the newest entry (index 0), keeping up to FAVORITES_MAX entries.
/// Any existing occurrence of the same registers is removed first so each entry is unique.
/// Returns an error if the file exists but cannot be parsed (corrupted), rather than silently
/// overwriting favorites.
pub fn save_to_favorites_at_path(path: &Path, values: &ToneData) -> io::Result<()> {
    let registers = register::editor_rows_to_registers(values);
    register_list::save_register_list_at_path(path, &registers, FAVORITES_MAX)
}

/// Save tone data to favorites in the local config directory.
/// Path: `{config_local_dir}/ym2151-tone-editor/favorites.json`
/// Errors are intentionally ignored by the caller to avoid disrupting the main flow.
pub fn save_to_favorites(values: &ToneData) -> io::Result<()> {
    let path = favorites_file_path().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
    })?;
    save_to_favorites_at_path(&path, values)
}
