use std::{fs, io, path::Path};

/// Load a capped, unique, newest-first register list from the given path.
/// Returns an empty Vec if the file does not exist.
/// Returns an error if the file is corrupted (not valid JSON).
pub fn load_register_list_at_path(path: &Path) -> io::Result<Vec<String>> {
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

/// Prepend `registers` to the list stored at `path`, keeping the list unique and capped at `max`.
/// Any existing occurrence of the same registers is removed first so the list stays unique.
/// Returns an error if the file exists but cannot be parsed (corrupted).
pub fn save_register_list_at_path(path: &Path, registers: &str, max: usize) -> io::Result<()> {
    let mut list: Vec<String> = if path.exists() {
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

    // Remove any existing occurrence so the list stays unique
    list.retain(|s| s != registers);

    list.insert(0, registers.to_owned());
    list.truncate(max);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json_string = serde_json::to_string(&list).map_err(io::Error::other)?;
    fs::write(path, json_string)?;

    Ok(())
}
