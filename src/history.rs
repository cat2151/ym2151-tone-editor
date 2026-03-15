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
/// Skips saving if the new entry is identical to the most recent entry.
pub fn save_to_history_at_path(path: &Path, values: &ToneData) -> io::Result<()> {
    let registers = register::editor_rows_to_registers(values);

    let mut history: Vec<String> = if path.exists() {
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Skip if identical to most recent entry
    if history.first().map(|s| s.as_str()) == Some(registers.as_str()) {
        return Ok(());
    }

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
