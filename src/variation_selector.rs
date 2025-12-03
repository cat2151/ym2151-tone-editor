use crate::models::{ToneData, ToneFile};
use crate::register;
use skim::prelude::*;
use std::fs;
use std::io;

/// Open variation selector using skim and load selected variation
/// Reads GM000 file (000_AcousticGrand.json) and displays variations for selection
pub fn open_variation_selector() -> io::Result<Option<ToneData>> {
    // Load GM000 tone file
    let filename = "tones/general_midi/000_AcousticGrand.json";
    let json_string = fs::read_to_string(filename)?;
    let tone_file: ToneFile = serde_json::from_str(&json_string).map_err(io::Error::other)?;

    if tone_file.variations.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No variations found in tone file",
        ));
    }

    // Create skim items from variations
    let items: Vec<String> = tone_file
        .variations
        .iter()
        .enumerate()
        .map(|(i, v)| format!("{}: {}", i + 1, v.description))
        .collect();

    // Configure skim options
    let options = SkimOptionsBuilder::default()
        .height("50%".to_string())
        .multi(false)
        .build()
        .map_err(io::Error::other)?;

    // Create item reader
    let item_reader = SkimItemReader::default();
    let items_str = items.join("\n");
    let items_rx = item_reader.of_bufread(io::Cursor::new(items_str));

    // Run skim selector
    let output = Skim::run_with(&options, Some(items_rx));

    // Handle selection
    match output {
        Some(out) if !out.is_abort => {
            if let Some(selected_item) = out.selected_items.first() {
                // Parse the index from the selected item text
                let text = selected_item.output().to_string();
                if let Some(idx_str) = text.split(':').next() {
                    if let Ok(idx) = idx_str.trim().parse::<usize>() {
                        if idx > 0 && idx <= tone_file.variations.len() {
                            let variation = &tone_file.variations[idx - 1];
                            let tone_data =
                                register::registers_to_editor_rows(&variation.registers)?;
                            
                            // Validate tone data dimensions
                            if tone_data.len() != crate::models::GRID_HEIGHT {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Invalid tone data: incorrect number of rows",
                                ));
                            }
                            for row in tone_data.iter() {
                                if row.len() != crate::models::GRID_WIDTH {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Invalid tone data: incorrect row width",
                                    ));
                                }
                            }
                            
                            return Ok(Some(tone_data));
                        }
                    }
                }
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}
