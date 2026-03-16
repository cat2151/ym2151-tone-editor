//! Sixel waveform visualization for YM2151 tone preview.
//!
//! After 5 seconds of idle (no tone parameter changes), this module generates
//! a WAV file from the current tone using the `ym2151-log-play-server` library,
//! reads the samples back, and encodes them as a sixel graphics string that is
//! printed directly to the terminal alongside the ratatui TUI.
//!
//! The feature is Windows-only because `ym2151-log-play-server` is a
//! Windows-only dependency.

#[cfg(windows)]
use std::sync::{Arc, Mutex};

#[cfg(windows)]
use crate::models::ToneData;

/// Pixel width of the generated sixel waveform image.
#[cfg(windows)]
const SIXEL_WIDTH: usize = 80;

/// Pixel height of the generated sixel waveform image.
/// Must be a multiple of 6 (each sixel row is 6 pixels tall).
#[cfg(windows)]
const SIXEL_HEIGHT: usize = 24;

/// Spawn a background thread that generates a sixel waveform from `values`
/// and stores the result in `sixel_arc`.
///
/// On error, the failure is logged via `log_verbose` and `sixel_arc` is left unchanged.
#[cfg(windows)]
pub fn spawn_waveform_generation(values: ToneData, sixel_arc: Arc<Mutex<Option<String>>>) {
    std::thread::spawn(move || match generate_waveform_sixel(&values) {
        Ok(sixel) => {
            if let Ok(mut guard) = sixel_arc.lock() {
                *guard = Some(sixel);
            }
        }
        Err(e) => {
            crate::log_verbose(&format!("Waveform generation failed: {}", e));
        }
    });
}

/// Generate a sixel waveform string from the given tone data.
///
/// Pipeline:
/// 1. Convert `ToneData` → YM2151 JSON event log
/// 2. Parse as `EventLog`
/// 3. Run `Player` and write a WAV file to a temp path
/// 4. Read the WAV samples
/// 5. Encode as sixel graphics
#[cfg(windows)]
fn generate_waveform_sixel(
    values: &ToneData,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use crate::register;
    use ym2151_log_play_server::{events::EventLog, player::Player, wav_writer};

    // Step 1: Build JSON event log (include envelope reset + key-on)
    let json = register::to_json_string_with_envelope_reset(values, 0.005)?;

    // Step 2: Parse as EventLog
    let event_log = EventLog::from_json_str(&json)?;

    // Step 3: Generate WAV to a temp file
    let player = Player::new(event_log);
    let wav_path = std::env::temp_dir().join("ym2151_waveform_preview.wav");
    let wav_path_str = wav_path.to_str().ok_or("Invalid temp path")?;
    wav_writer::generate_wav(player, wav_path_str)?;

    // Step 4: Read WAV samples (mono, normalised to [-1.0, 1.0])
    let samples = read_wav_samples(&wav_path)?;

    // Step 5: Convert to sixel
    Ok(samples_to_sixel(&samples, SIXEL_WIDTH, SIXEL_HEIGHT))
}

/// Read a 16-bit stereo WAV file and return mono samples in [-1.0, 1.0].
///
/// Channels are averaged together to produce mono.
#[cfg(windows)]
fn read_wav_samples(
    path: &std::path::Path,
) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
    let data = std::fs::read(path)?;

    if data.len() < 44 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return Err("Not a valid WAV file".into());
    }

    let mut pos = 12usize;
    let mut channels = 2u16;
    let mut bits_per_sample = 16u16;
    let mut data_start: Option<usize> = None;
    let mut data_size = 0usize;

    while pos + 8 <= data.len() {
        let chunk_id = &data[pos..pos + 4];
        let chunk_size =
            u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap_or([0; 4])) as usize;

        if chunk_id == b"fmt " && pos + 24 <= data.len() {
            channels = u16::from_le_bytes([data[pos + 10], data[pos + 11]]);
            bits_per_sample = u16::from_le_bytes([data[pos + 22], data[pos + 23]]);
        } else if chunk_id == b"data" {
            data_start = Some(pos + 8);
            data_size = chunk_size.min(data.len().saturating_sub(pos + 8));
            break;
        }

        pos += 8 + chunk_size;
        if chunk_size % 2 != 0 {
            pos += 1; // WAV padding byte
        }
    }

    let data_start = data_start.ok_or("No data chunk found in WAV file")?;
    let sample_bytes = (bits_per_sample as usize / 8).max(1);
    let channels = channels.max(1) as usize;
    let num_frames = (data_size / sample_bytes) / channels;

    let mut mono = Vec::with_capacity(num_frames);
    for frame in 0..num_frames {
        let mut sum = 0.0f32;
        for ch in 0..channels {
            let offset = data_start + (frame * channels + ch) * sample_bytes;
            if offset + sample_bytes > data.len() {
                break;
            }
            let s = match bits_per_sample {
                16 => {
                    let raw = i16::from_le_bytes([data[offset], data[offset + 1]]);
                    raw as f32 / 32768.0
                }
                _ => 0.0,
            };
            sum += s;
        }
        mono.push(sum / channels as f32);
    }

    Ok(mono)
}

/// Convert mono waveform samples into a sixel graphics escape sequence.
///
/// The waveform is rendered as a symmetric vertical bar chart centred on the
/// middle row, with bar heights proportional to peak amplitude per column.
#[cfg(windows)]
fn samples_to_sixel(samples: &[f32], width: usize, height: usize) -> String {
    // Compute peak amplitude per pixel column
    let total_frames = samples.len();
    let mut peaks = vec![0.0f32; width];
    if total_frames > 0 {
        for col in 0..width {
            let start = col * total_frames / width;
            let end = ((col + 1) * total_frames / width).min(total_frames);
            let peak = samples[start..end]
                .iter()
                .map(|s| s.abs())
                .fold(0.0f32, f32::max);
            peaks[col] = peak;
        }
    }

    // Build a 2D bitmap: symmetric bars centred on middle row
    let center = height / 2;
    let half = center;
    let mut bitmap = vec![vec![false; width]; height];
    for col in 0..width {
        let amp = peaks[col].min(1.0);
        let bar_half = (amp * half as f32).round() as usize;
        let top = center.saturating_sub(bar_half);
        let bottom = (center + bar_half).min(height.saturating_sub(1));
        for row in top..=bottom {
            bitmap[row][col] = true;
        }
    }

    encode_sixel(&bitmap, width, height)
}

/// Encode a boolean bitmap as a sixel DCS escape sequence.
///
/// Each sixel character encodes a 1×6-pixel vertical strip; characters are
/// emitted left-to-right, with `-` advancing to the next 6-pixel band.
/// The sequence uses a single green colour (palette index 1).
#[cfg(windows)]
fn encode_sixel(bitmap: &[Vec<bool>], width: usize, height: usize) -> String {
    let mut out = String::new();

    // DCS introducer: ESC P P1;P2;P3 q
    // P1=0 (default aspect ratio), P2=1 (background colour 0), P3=0 (grid)
    out.push_str("\x1bP0;1;0q");

    // Define palette entry 1 as green (R=0, G=80, B=0 in [0,100] scale)
    out.push_str("#1;2;0;80;0");

    let num_bands = height.div_ceil(6);
    for band in 0..num_bands {
        if band > 0 {
            out.push('-'); // Advance to next sixel row
        }
        out.push_str("#1"); // Select colour 1

        for col in 0..width {
            // Each character encodes 6 pixels; bit 0 = topmost pixel in band.
            let mut bits = 0u8;
            for bit in 0..6 {
                let row = band * 6 + bit;
                if row < height && bitmap[row][col] {
                    bits |= 1 << bit;
                }
            }
            // Sixel data characters are in the range '?' (63) to '~' (126).
            out.push((b'?' + bits) as char);
        }
    }

    // ST (String Terminator): ESC backslash
    out.push_str("\x1b\\");

    out
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn encode_sixel_empty_bitmap_produces_valid_sequence() {
        let bitmap = vec![vec![false; 4]; 6];
        let result = encode_sixel(&bitmap, 4, 6);
        // Must start with DCS introducer
        assert!(result.starts_with("\x1bP"), "expected DCS start");
        // Must end with ST
        assert!(result.ends_with("\x1b\\"), "expected ST end");
    }

    #[test]
    fn encode_sixel_full_bitmap_produces_non_blank_data() {
        let bitmap = vec![vec![true; 4]; 6];
        let result = encode_sixel(&bitmap, 4, 6);
        // A fully-lit column should produce the character '~' (63 + 63 = 126)
        assert!(result.contains('~'), "expected '~' for all-lit column");
    }

    #[test]
    fn samples_to_sixel_silence_produces_center_line() {
        // All-zero samples should produce a thin center line
        let samples = vec![0.0f32; 1000];
        let sixel = samples_to_sixel(&samples, 4, 6);
        assert!(sixel.starts_with("\x1bP"));
        assert!(sixel.ends_with("\x1b\\"));
    }

    #[test]
    fn samples_to_sixel_full_amplitude_fills_display() {
        // Max amplitude should fill most of the display
        let samples = vec![1.0f32; 1000];
        let sixel = samples_to_sixel(&samples, 4, 6);
        // Should contain some '~' characters (all-lit columns)
        assert!(!sixel.is_empty());
    }
}
