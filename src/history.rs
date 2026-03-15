use crate::models::ToneData;
use crate::register;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const HISTORY_MAX: usize = 20;

/// Get the path to the history file in the local config directory.
/// Returns None if the config directory cannot be determined.
pub fn history_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("ym2151-tone-editor").join("history_tone.json"))
}

/// Save tone data to history at the given path.
/// Adds the current tone as the newest entry (index 0), keeping up to HISTORY_MAX entries.
/// Any existing occurrence of the same registers is removed first so each entry is unique.
/// Returns an error if the file exists but cannot be parsed (corrupted), rather than silently
/// overwriting history.
pub fn save_to_history_at_path(path: &Path, values: &ToneData) -> io::Result<()> {
    let registers = register::editor_rows_to_registers(values);

    let mut history: Vec<String> = if path.exists() {
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("history_tone.json is corrupted: {}", e),
            )
        })?
    } else {
        Vec::new()
    };

    // Remove any existing occurrence of the same registers so history stays unique
    history.retain(|s| s != &registers);

    history.insert(0, registers);
    history.truncate(HISTORY_MAX);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json_string = serde_json::to_string(&history).map_err(io::Error::other)?;
    fs::write(path, json_string)?;

    Ok(())
}

/// Save tone data to history in the local config directory.
/// Path: `{config_dir}/ym2151-tone-editor/history_tone.json`
/// Errors are intentionally ignored by the caller to avoid disrupting the main flow.
pub fn save_to_history(values: &ToneData) -> io::Result<()> {
    let path = history_file_path().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
    })?;
    save_to_history_at_path(&path, values)
}
