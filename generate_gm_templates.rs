//! Generate GM tone template JSON files from tone_names.json
//!
//! This script reads tones/general_midi/tone_names.json and generates
//! 128 JSON template files (000_AcousticGrand.json to 127_Gunshot.json)
//! in the tones/general_midi/ directory.
//!
//! ## Usage with rust-script (recommended):
//!
//! ```bash
//! cargo install rust-script  # if not already installed
//! rust-script generate_gm_templates.rs
//! ```
//!
//! ## Usage with manual compilation:
//!
//! ```bash
//! # Simple compilation (no external dependencies required)
//! rustc --edition 2021 generate_gm_templates.rs && ./generate_gm_templates
//! ```

use std::fs;

/// Default YM2151 register values for a basic tone template
/// (copied from tones/general_midi/000_AcousticGrand.json)
const DEFAULT_REGISTERS: &str = "40016014801FA00AC005E0574801681E8819A808C804E866500270009014B006D003F075580178009816B807D804F86620C4283E30000878";

/// Default MIDI note number for tone templates
const DEFAULT_NOTE_NUMBER: u8 = 60;

/// Default variation description
const DEFAULT_VARIATION_DESCRIPTION: &str = "Edited Tone";

/// Common filler words to exclude from filenames
const FILLER_WORDS: &[&str] = &["the", "a", "an"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read tone_names.json
    let input_path = "tones/general_midi/tone_names.json";
    let tone_names_content = fs::read_to_string(input_path)?;

    // Parse tone names
    let tone_names = parse_tone_names(&tone_names_content)?;

    println!("Loaded {} tone names from {}", tone_names.len(), input_path);

    // Generate JSON files for each tone
    for (index, (name, _mml)) in tone_names.iter().enumerate() {
        // Extract description from name (remove "GMxxx " prefix)
        let description = if let Some(pos) = name.find(' ') {
            name[pos + 1..].to_string()
        } else {
            name.clone()
        };

        // Generate filename (e.g., "000_AcousticGrand.json")
        let filename = create_filename(index, &description);
        let output_path = format!("tones/general_midi/{}", filename);

        // Create JSON content with proper formatting to match original file
        let json_content = create_tone_json(&description);

        // Write to file
        fs::write(&output_path, json_content)?;
        println!("Generated: {}", output_path);
    }

    println!(
        "\nSuccessfully generated {} tone template files!",
        tone_names.len()
    );

    Ok(())
}

/// Create JSON content for a tone file with proper formatting
fn create_tone_json(description: &str) -> String {
    format!(
        r#"{{
  "description": "{}",
  "variations": [
    {{"description":"{}","note_number":{},"registers":"{}"}}
  ]
}}"#,
        escape_json_string(description),
        DEFAULT_VARIATION_DESCRIPTION,
        DEFAULT_NOTE_NUMBER,
        DEFAULT_REGISTERS
    )
}

/// Parse tone_names.json
/// Expects format: [["name1", "mml1"], ["name2", "mml2"], ...]
fn parse_tone_names(json_str: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    // Simple manual parser for the specific format
    let mut result = Vec::new();
    let trimmed = json_str.trim();

    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err("Invalid JSON array format".into());
    }

    let mut chars = trimmed[1..trimmed.len() - 1].chars().peekable();
    let mut current_name = String::new();
    let mut current_mml = String::new();
    let mut in_string = false;
    let mut in_first = true;
    let mut escape_next = false;
    let mut in_pair = false;

    while let Some(ch) = chars.next() {
        if escape_next {
            let escaped_char = match ch {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                _ => ch,
            };
            if in_first {
                current_name.push(escaped_char);
            } else {
                current_mml.push(escaped_char);
            }
            escape_next = false;
            continue;
        }

        match ch {
            '[' if !in_string => {
                in_pair = true;
                in_first = true;
                current_name.clear();
                current_mml.clear();
            }
            ']' if !in_string && in_pair => {
                if !current_name.is_empty() || !current_mml.is_empty() {
                    result.push((current_name.clone(), current_mml.clone()));
                }
                in_pair = false;
            }
            '"' => {
                in_string = !in_string;
            }
            '\\' if in_string => {
                escape_next = true;
            }
            ',' if !in_string && in_first && in_pair => {
                in_first = false;
            }
            _ if in_string => {
                if in_first {
                    current_name.push(ch);
                } else {
                    current_mml.push(ch);
                }
            }
            _ => {}
        }
    }

    Ok(result)
}

/// Escape special characters in JSON strings
fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Check if a character is valid for use in a filename
/// Allows alphanumeric characters and hyphens
fn is_valid_filename_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-'
}

/// Create a filename from index and description
/// e.g., (0, "Acoustic Grand Piano") -> "000_AcousticGrand.json"
fn create_filename(index: usize, description: &str) -> String {
    // Take first few words from description and create CamelCase
    // Filter out common filler words and parenthetical content
    let words: Vec<&str> = description
        .split_whitespace()
        .filter(|word| {
            // Filter out parenthetical content first (cheaper check)
            if word.starts_with('(') {
                return false;
            }
            // Then check for filler words (requires string conversion)
            let word_lower = word.to_lowercase();
            !FILLER_WORDS.contains(&word_lower.as_str())
        })
        .collect();

    let mut camel_case = String::new();

    // Take up to 2 meaningful words for the filename
    for word in words.iter().take(2) {
        // Capitalize first letter and append the rest
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            // Extend is more efficient than creating intermediate string
            camel_case.extend(first_char.to_uppercase());
            // Keep only valid filename characters (alphanumeric and hyphen)
            let rest: String = chars.filter(|&c| is_valid_filename_char(c)).collect();
            camel_case.push_str(&rest);
        }
    }

    // If camel_case is empty or too short, use a generic name
    if camel_case.is_empty() {
        camel_case = "Tone".to_string();
    }

    format!("{:03}_{}.json", index, camel_case)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_filename() {
        assert_eq!(
            create_filename(0, "Acoustic Grand Piano"),
            "000_AcousticGrand.json"
        );
        assert_eq!(
            create_filename(1, "Bright Acoustic Piano"),
            "001_BrightAcoustic.json"
        );
        assert_eq!(create_filename(127, "Gunshot"), "127_Gunshot.json");
    }

    #[test]
    fn test_escape_json_string() {
        assert_eq!(escape_json_string("hello"), "hello");
        assert_eq!(escape_json_string("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(escape_json_string("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_parse_simple_pair() {
        let json = r#"[["GM000 Test", "@000 test"]]"#;
        let result = parse_tone_names(json).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "GM000 Test");
        assert_eq!(result[0].1, "@000 test");
    }

    #[test]
    fn test_is_valid_filename_char() {
        assert!(is_valid_filename_char('a'));
        assert!(is_valid_filename_char('Z'));
        assert!(is_valid_filename_char('0'));
        assert!(is_valid_filename_char('-'));
        assert!(!is_valid_filename_char('('));
        assert!(!is_valid_filename_char(' '));
        assert!(!is_valid_filename_char('/'));
    }

    #[test]
    fn test_create_tone_json() {
        let json = create_tone_json("Test Tone");
        assert!(json.contains("\"description\": \"Test Tone\""));
        assert!(json.contains("\"Edited Tone\""));
        assert!(json.contains("60"));
        assert!(json.contains(DEFAULT_REGISTERS));
    }
}
