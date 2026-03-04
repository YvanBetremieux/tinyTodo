use serde::{Deserialize, Serialize};

/// Shortcut activation mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShortcutMode {
    Toggle,
    Hold,
}

impl Default for ShortcutMode {
    fn default() -> Self {
        Self::Toggle
    }
}

/// Application configuration — persisted in config.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub shortcut: String,
    pub shortcut_mode: ShortcutMode,
    pub autostart: bool,
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shortcut: "CmdOrCtrl+Shift+Space".to_string(),
            shortcut_mode: ShortcutMode::default(),
            autostart: false,
            theme: "ultra-minimal".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.shortcut, "CmdOrCtrl+Shift+Space");
        assert_eq!(config.shortcut_mode, ShortcutMode::Toggle);
        assert!(!config.autostart);
        assert_eq!(config.theme, "ultra-minimal");
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.shortcut, config.shortcut);
        assert_eq!(deserialized.shortcut_mode, config.shortcut_mode);
        assert_eq!(deserialized.autostart, config.autostart);
        assert_eq!(deserialized.theme, config.theme);
    }

    #[test]
    fn test_shortcut_mode_serialization() {
        let toggle = ShortcutMode::Toggle;
        let json = serde_json::to_string(&toggle).unwrap();
        assert_eq!(json, "\"toggle\"");

        let hold = ShortcutMode::Hold;
        let json = serde_json::to_string(&hold).unwrap();
        assert_eq!(json, "\"hold\"");
    }
}
