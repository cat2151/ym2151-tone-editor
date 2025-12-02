use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;

/// Action that can be performed by a keybind
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    DecreaseValue,
    IncreaseValue,
    SetValueToMax,
    SetValueToMin,
    SetValueToRandom,
    IncreaseValueBy1,
    IncreaseValueBy2,
    IncreaseValueBy3,
    IncreaseValueBy4,
    IncreaseValueBy5,
    IncreaseValueBy6,
    IncreaseValueBy7,
    IncreaseValueBy8,
    IncreaseValueBy9,
    IncreaseValueBy10,
    DecreaseValueBy1,
    DecreaseValueBy2,
    DecreaseValueBy3,
    DecreaseValueBy4,
    DecreaseValueBy5,
    DecreaseValueBy6,
    DecreaseValueBy7,
    DecreaseValueBy8,
    DecreaseValueBy9,
    DecreaseValueBy10,
    PlayCurrentTone,
    IncreaseFb,
    DecreaseFb,
    IncreaseAlg,
    DecreaseAlg,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    JumpToOp1AndIncrease,
    JumpToOp2AndIncrease,
    JumpToOp3AndIncrease,
    JumpToOp4AndIncrease,
    JumpToOp1AndDecrease,
    JumpToOp2AndDecrease,
    JumpToOp3AndDecrease,
    JumpToOp4AndDecrease,
    JumpToArAndIncrease,
    JumpToD1rAndIncrease,
    JumpToD2rAndIncrease,
    JumpToRrAndIncrease,
    JumpToArAndDecrease,
    JumpToD1rAndDecrease,
    JumpToD2rAndDecrease,
    JumpToRrAndDecrease,
    JumpToMulAndIncrease,
    JumpToMulAndDecrease,
    JumpToSmAndIncrease,
    JumpToSmAndDecrease,
    JumpToTlAndIncrease,
    JumpToTlAndDecrease,
    JumpToD1lAndIncrease,
    JumpToD1lAndDecrease,
    JumpToDtAndIncrease,
    JumpToDtAndDecrease,
    JumpToDt2AndIncrease,
    JumpToDt2AndDecrease,
    JumpToKsAndIncrease,
    JumpToKsAndDecrease,
    JumpToAmsAndIncrease,
    JumpToAmsAndDecrease,
    JumpToNoteAndIncrease,
    JumpToNoteAndDecrease,
    Exit,
}

/// Configuration for keybinds
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeybindsConfig {
    #[serde(default)]
    pub keybinds: HashMap<String, Action>,
}

impl Default for KeybindsConfig {
    fn default() -> Self {
        let mut keybinds = HashMap::new();

        // Value modification keys
        keybinds.insert("q".to_string(), Action::DecreaseValue);
        keybinds.insert("PageDown".to_string(), Action::DecreaseValue);
        keybinds.insert("e".to_string(), Action::IncreaseValue);
        keybinds.insert("PageUp".to_string(), Action::IncreaseValue);
        keybinds.insert("Home".to_string(), Action::SetValueToMax);
        keybinds.insert("End".to_string(), Action::SetValueToMin);
        keybinds.insert("r".to_string(), Action::SetValueToRandom);
        keybinds.insert("R".to_string(), Action::SetValueToRandom);

        // "+" to increase by 1
        keybinds.insert("+".to_string(), Action::IncreaseValueBy1);

        // "." and ">" for increase
        keybinds.insert(".".to_string(), Action::IncreaseValueBy1);
        keybinds.insert(">".to_string(), Action::IncreaseValueBy10);

        // "-" and "=" for decrease
        keybinds.insert("-".to_string(), Action::DecreaseValueBy1);
        keybinds.insert("=".to_string(), Action::DecreaseValueBy1); // +とセットで押す場合はどちらもSHIFTを押しながらの操作になると判断し、増減は10でなく1とした

        // "," and "<" for decrease
        keybinds.insert(",".to_string(), Action::DecreaseValueBy1);
        keybinds.insert("<".to_string(), Action::DecreaseValueBy10);

        // Number keys for quick value adjustment
        keybinds.insert("5".to_string(), Action::IncreaseValueBy5);
        keybinds.insert("6".to_string(), Action::IncreaseValueBy6);
        keybinds.insert("7".to_string(), Action::IncreaseValueBy7);
        keybinds.insert("8".to_string(), Action::IncreaseValueBy8);
        keybinds.insert("9".to_string(), Action::IncreaseValueBy9);
        // keybinds.insert("0".to_string(), Action::IncreaseValueBy10); // SHIFTを押しながらでも0なので混乱した。後回し

        // SHIFT + number keys for decrease
        keybinds.insert("%".to_string(), Action::DecreaseValueBy5); // Shift+5
        keybinds.insert("&".to_string(), Action::DecreaseValueBy6); // Shift+6
        keybinds.insert("\'".to_string(), Action::DecreaseValueBy7); // Shift+7
        keybinds.insert("(".to_string(), Action::DecreaseValueBy8); // Shift+8
        keybinds.insert(")".to_string(), Action::DecreaseValueBy9); // Shift+9

        // keybinds.insert("0".to_string(), Action::DecreaseValueBy10); // Shift+0 → Shift+0も0なので意味がない。後回し

        // Play current tone
        keybinds.insert("p".to_string(), Action::PlayCurrentTone);
        keybinds.insert("P".to_string(), Action::PlayCurrentTone);
        keybinds.insert("Space".to_string(), Action::PlayCurrentTone);

        // FB shortcuts
        keybinds.insert("f".to_string(), Action::IncreaseFb);
        keybinds.insert("F".to_string(), Action::DecreaseFb);

        // ALG shortcuts
        keybinds.insert("g".to_string(), Action::IncreaseAlg);
        keybinds.insert("G".to_string(), Action::DecreaseAlg);

        // Cursor movement
        keybinds.insert("h".to_string(), Action::MoveCursorLeft);
        keybinds.insert("Left".to_string(), Action::MoveCursorLeft);
        keybinds.insert("Down".to_string(), Action::MoveCursorDown);

        // Note Number shortcuts (jump to CH row's Note parameter)
        keybinds.insert("j".to_string(), Action::JumpToNoteAndIncrease);
        keybinds.insert("J".to_string(), Action::JumpToNoteAndDecrease);
        keybinds.insert("Up".to_string(), Action::MoveCursorUp);
        keybinds.insert("Right".to_string(), Action::MoveCursorRight);

        // Jump to operator row and increase value
        keybinds.insert("1".to_string(), Action::JumpToOp1AndIncrease);
        keybinds.insert("2".to_string(), Action::JumpToOp2AndIncrease);
        keybinds.insert("3".to_string(), Action::JumpToOp3AndIncrease);
        keybinds.insert("4".to_string(), Action::JumpToOp4AndIncrease);

        // Jump to operator row and decrease value
        keybinds.insert("!".to_string(), Action::JumpToOp1AndDecrease);
        keybinds.insert("\"".to_string(), Action::JumpToOp2AndDecrease);
        keybinds.insert("#".to_string(), Action::JumpToOp3AndDecrease);
        keybinds.insert("$".to_string(), Action::JumpToOp4AndDecrease);

        // ADSR envelope shortcuts (jump to current row's AR, D1R, D2R, RR parameters)
        keybinds.insert("a".to_string(), Action::JumpToArAndIncrease);
        keybinds.insert("d".to_string(), Action::JumpToD1rAndIncrease);
        keybinds.insert("s".to_string(), Action::JumpToD2rAndIncrease);
        keybinds.insert("r".to_string(), Action::JumpToRrAndIncrease);
        keybinds.insert("A".to_string(), Action::JumpToArAndDecrease);
        keybinds.insert("D".to_string(), Action::JumpToD1rAndDecrease);
        keybinds.insert("S".to_string(), Action::JumpToD2rAndDecrease);
        keybinds.insert("R".to_string(), Action::JumpToRrAndDecrease);

        // MUL shortcuts (jump to current row's MUL parameter)
        keybinds.insert("m".to_string(), Action::JumpToMulAndIncrease);
        keybinds.insert("M".to_string(), Action::JumpToMulAndDecrease);

        // SM (Slot Mask) shortcuts (jump to current row's SM parameter)
        keybinds.insert("o".to_string(), Action::JumpToSmAndIncrease);
        keybinds.insert("O".to_string(), Action::JumpToSmAndDecrease);

        // TL (Total Level) shortcuts (jump to current row's TL parameter)
        keybinds.insert("t".to_string(), Action::JumpToTlAndIncrease);
        keybinds.insert("T".to_string(), Action::JumpToTlAndDecrease);

        // D1L (Decay 1 Level) shortcuts (jump to current row's D1L parameter)
        keybinds.insert("l".to_string(), Action::JumpToD1lAndIncrease);
        keybinds.insert("L".to_string(), Action::JumpToD1lAndDecrease);

        // DT (Detune 1) shortcuts (jump to current row's DT parameter)
        keybinds.insert("u".to_string(), Action::JumpToDtAndIncrease);
        keybinds.insert("U".to_string(), Action::JumpToDtAndDecrease);

        // DT2 (Detune 2) shortcuts (jump to current row's DT2 parameter)
        keybinds.insert("n".to_string(), Action::JumpToDt2AndIncrease);
        keybinds.insert("N".to_string(), Action::JumpToDt2AndDecrease);

        // KS (Key Scaling) shortcuts (jump to current row's KS parameter)
        keybinds.insert("k".to_string(), Action::JumpToKsAndIncrease);
        keybinds.insert("K".to_string(), Action::JumpToKsAndDecrease);

        // AMS (Amplitude Modulation Sensitivity) shortcuts (jump to current row's AMS parameter)
        keybinds.insert("i".to_string(), Action::JumpToAmsAndIncrease);
        keybinds.insert("I".to_string(), Action::JumpToAmsAndDecrease);

        // Exit
        keybinds.insert("Esc".to_string(), Action::Exit);

        KeybindsConfig { keybinds }
    }
}

impl KeybindsConfig {
    /// Load keybinds configuration from a TOML file
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let toml_string = fs::read_to_string(filename)?;
        let config: KeybindsConfig = toml::from_str(&toml_string)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    /// Load keybinds from current directory's ym2151-tone-editor.toml if it exists,
    /// otherwise use default keybinds
    pub fn load_or_default() -> Self {
        const CONFIG_FILE: &str = "ym2151-tone-editor.toml";

        match Self::load_from_file(CONFIG_FILE) {
            Ok(config) => {
                crate::log_verbose(&format!("Loaded keybinds from {}", CONFIG_FILE));
                config
            }
            Err(_) => {
                crate::log_verbose("Using default keybinds");
                Self::default()
            }
        }
    }

    /// Get the action for a given key string
    pub fn get_action(&self, key: &str) -> Option<&Action> {
        self.keybinds.get(key)
    }

    /// Save the current keybinds configuration to a TOML file
    #[cfg(test)]
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(filename, toml_string)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_keybinds_has_expected_actions() {
        let config = KeybindsConfig::default();

        // Test a few key bindings
        assert_eq!(config.get_action("q"), Some(&Action::DecreaseValue));
        assert_eq!(config.get_action("e"), Some(&Action::IncreaseValue));
        assert_eq!(config.get_action("Esc"), Some(&Action::Exit));
        assert_eq!(config.get_action("1"), Some(&Action::IncreaseValueBy1));
        assert_eq!(config.get_action("h"), Some(&Action::MoveCursorLeft));
    }

    #[test]
    fn test_load_from_toml_string() {
        let toml_str = r#"
[keybinds]
"q" = "decrease_value"
"w" = "increase_value"
"Esc" = "exit"
"#;

        let config: KeybindsConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.get_action("q"), Some(&Action::DecreaseValue));
        assert_eq!(config.get_action("w"), Some(&Action::IncreaseValue));
        assert_eq!(config.get_action("Esc"), Some(&Action::Exit));
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let mut config = KeybindsConfig::default();

        // Modify a keybind
        config.keybinds.insert("x".to_string(), Action::Exit);

        // Save to temp file
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_keybinds.toml");
        let temp_file = temp_file.to_str().unwrap();
        config.save_to_file(temp_file).unwrap();

        // Load back
        let loaded_config = KeybindsConfig::load_from_file(temp_file).unwrap();
        assert_eq!(loaded_config.get_action("x"), Some(&Action::Exit));

        // Clean up
        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn test_custom_keybinds_file() {
        // Create a custom keybinds file
        let custom_toml = r#"
[keybinds]
"u" = "decrease_value"
"i" = "increase_value"
"x" = "set_value_to_random"
"Esc" = "exit"
"#;

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_custom_keybinds.toml");
        let temp_file = temp_file.to_str().unwrap();
        std::fs::write(temp_file, custom_toml).unwrap();

        // Load the custom config
        let config = KeybindsConfig::load_from_file(temp_file).unwrap();

        // Verify custom keybinds
        assert_eq!(config.get_action("u"), Some(&Action::DecreaseValue));
        assert_eq!(config.get_action("i"), Some(&Action::IncreaseValue));
        assert_eq!(config.get_action("x"), Some(&Action::SetValueToRandom));
        assert_eq!(config.get_action("Esc"), Some(&Action::Exit));

        // Verify that default keybinds are NOT present (since we only defined a few)
        assert_eq!(config.get_action("q"), None);
        assert_eq!(config.get_action("e"), None);

        // Clean up
        let _ = std::fs::remove_file(temp_file);
    }
}
