use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

/// Global verbose logging flag
pub static VERBOSE_LOGGING: Mutex<bool> = Mutex::new(false);

pub static LOG_FILENAME: &str = "ym2151-tone-editor.log";

/// Log a message to ym2151-tone-editor.log if verbose logging is enabled
pub fn log_verbose(message: &str) {
    if let Ok(enabled) = VERBOSE_LOGGING.lock() {
        if *enabled {
            drop(enabled); // Release lock before file I/O
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(LOG_FILENAME)
            {
                use chrono::Local;
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
                let _ = writeln!(file, "[{}] {}", timestamp, message);
            }
        }
    }
}

/// Enable verbose logging
pub fn enable_verbose_logging() {
    if let Ok(mut enabled) = VERBOSE_LOGGING.lock() {
        *enabled = true;
    }
}
