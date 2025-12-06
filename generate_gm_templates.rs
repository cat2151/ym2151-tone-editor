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
//! # Compile with serde dependencies
//! rustc --edition 2021 generate_gm_templates.rs \
//!   --extern serde=/path/to/libserde.rlib \
//!   --extern serde_json=/path/to/libserde_json.rlib
//!
//! # Or use a simple script without serde:
//! rustc --edition 2021 -C opt-level=2 generate_gm_templates.rs && ./generate_gm_templates
//! ```
//!
//! Dependencies when using rust-script:
//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```

// Try to use serde if available (for rust-script), otherwise use manual parsing
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::fs;

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct ToneFile {
    description: String,
    variations: Vec<Variation>,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct Variation {
    description: String,
    note_number: u8,
    registers: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read tone_names.json
    let input_path = "tones/general_midi/tone_names.json";
    let tone_names_content = fs::read_to_string(input_path)?;

    // Parse tone names
    let tone_names = parse_tone_names(&tone_names_content)?;

    println!("Loaded {} tone names from {}", tone_names.len(), input_path);

    // Default register values (from 000_AcousticGrand.json)
    let default_registers =
        "40016014801FA00AC005E0574801681E8819A808C804E866500270009014B006D003F075580178009816B807D804F86620C4283E30000878";

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

        // Create JSON content with specific formatting
        #[cfg(feature = "serde")]
        let json_content = {
            let tone_file = ToneFile {
                description: description.clone(),
                variations: vec![Variation {
                    description: "Edited Tone".to_string(),
                    note_number: 60,
                    registers: default_registers.to_string(),
                }],
            };
            // Use custom formatting to match the original file format
            format!(
                r#"{{
  "description": "{}",
  "variations": [
    {{"description":"Edited Tone","note_number":60,"registers":"{}"}}
  ]
}}"#,
                description.replace('\\', "\\\\").replace('"', "\\\""),
                default_registers
            )
        };

        #[cfg(not(feature = "serde"))]
        let json_content = format!(
            r#"{{
  "description": "{}",
  "variations": [
    {{"description":"Edited Tone","note_number":60,"registers":"{}"}}
  ]
}}"#,
            escape_json_string(&description),
            default_registers
        );

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

/// Parse tone_names.json
/// Expects format: [["name1", "mml1"], ["name2", "mml2"], ...]
fn parse_tone_names(json_str: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    #[cfg(feature = "serde")]
    {
        let parsed: Vec<(String, String)> = serde_json::from_str(json_str)?;
        Ok(parsed)
    }

    #[cfg(not(feature = "serde"))]
    {
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
}

/// Escape special characters in JSON strings (used when serde is not available)
#[cfg(not(feature = "serde"))]
fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Create a filename from index and description
/// e.g., (0, "Acoustic Grand Piano") -> "000_AcousticGrand.json"
fn create_filename(index: usize, description: &str) -> String {
    // Take first few words from description and create CamelCase
    let words: Vec<&str> = description
        .split_whitespace()
        .filter(|word| {
            // Filter out common filler words and parenthetical content
            !word.starts_with('(') && !matches!(word.to_lowercase().as_str(), "the" | "a" | "an")
        })
        .collect();

    let mut camel_case = String::new();

    // Take up to 2 meaningful words for the filename
    for word in words.iter().take(2) {
        // Capitalize first letter and append the rest
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            camel_case.push_str(&first_char.to_uppercase().to_string());
            // Remove parentheses and special characters, keep only alphanumeric
            let rest: String = chars.filter(|c| c.is_alphanumeric() || *c == '-').collect();
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
    #[cfg(not(feature = "serde"))]
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
}
