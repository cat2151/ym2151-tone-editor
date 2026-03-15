use crate::models::ToneData;
use crate::register;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Load favorites register strings from the given path.
/// Returns an empty Vec if the file does not exist.
pub fn load_favorites_at_path(path: &Path) -> io::Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)?;
    serde_json::from_str(&content).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} is corrupted: {}", path.display(), e),
        )
    })
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
    dirs::config_dir().map(|dir| dir.join("ym2151-tone-editor").join("favorites.json"))
}

/// Save tone data to favorites at the given path.
/// Adds the current tone as the newest entry (index 0), keeping up to FAVORITES_MAX entries.
/// Any existing occurrence of the same registers is removed first so each entry is unique.
/// Returns an error if the file exists but cannot be parsed (corrupted), rather than silently
/// overwriting favorites.
pub fn save_to_favorites_at_path(path: &Path, values: &ToneData) -> io::Result<()> {
    let registers = register::editor_rows_to_registers(values);

    let mut favorites: Vec<String> = if path.exists() {
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{} is corrupted: {}", path.display(), e),
            )
        })?
    } else {
        Vec::new()
    };

    // Remove any existing occurrence of the same registers so favorites stays unique
    favorites.retain(|s| s != &registers);

    favorites.insert(0, registers);
    favorites.truncate(FAVORITES_MAX);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json_string = serde_json::to_string(&favorites).map_err(io::Error::other)?;
    fs::write(path, json_string)?;

    Ok(())
}

/// Save tone data to favorites in the local config directory.
/// Path: `{config_dir}/ym2151-tone-editor/favorites.json`
/// Errors are intentionally ignored by the caller to avoid disrupting the main flow.
pub fn save_to_favorites(values: &ToneData) -> io::Result<()> {
    let path = favorites_file_path().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
    })?;
    save_to_favorites_at_path(&path, values)
}
