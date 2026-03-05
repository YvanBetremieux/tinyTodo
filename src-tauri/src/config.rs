use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

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

/// Managed state for config persistence
pub struct ConfigState {
    pub config: Mutex<Config>,
    pub config_path: PathBuf,
}

/// Load config from JSON file, creating default if absent
pub fn load_config(path: &Path) -> Config {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }

    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => {
            let default = Config::default();
            let _ = save_config(path, &default);
            default
        }
    }
}

/// Save config to JSON file using atomic write (temp + rename)
pub fn save_config(path: &Path, config: &Config) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Serialization error: {}", e))?;

    let parent = path.parent().ok_or("Invalid path")?;
    if !parent.exists() {
        fs::create_dir_all(parent).map_err(|e| format!("Cannot create directory: {}", e))?;
    }

    let temp_path = path.with_extension("json.tmp");
    let mut file =
        fs::File::create(&temp_path).map_err(|e| format!("Cannot create temp file: {}", e))?;
    file.write_all(json.as_bytes())
        .map_err(|e| format!("Write error: {}", e))?;
    file.sync_all()
        .map_err(|e| format!("Sync error: {}", e))?;

    fs::rename(&temp_path, path).map_err(|e| format!("Rename error: {}", e))?;

    Ok(())
}

/// IPC command: get current config
#[tauri::command]
pub fn get_config(state: tauri::State<'_, ConfigState>) -> Config {
    state.config.lock().unwrap().clone()
}

/// IPC command: save config
#[tauri::command]
pub fn save_config_command(
    config: Config,
    state: tauri::State<'_, ConfigState>,
) -> Result<(), String> {
    let mut current = state.config.lock().unwrap();
    *current = config;
    save_config(&state.config_path, &current)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_path(dir: &TempDir) -> PathBuf {
        dir.path().join("config.json")
    }

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

    #[test]
    fn test_load_config_creates_default_if_absent() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);

        assert!(!path.exists());
        let config = load_config(&path);
        assert!(path.exists());
        assert_eq!(config.shortcut, "CmdOrCtrl+Shift+Space");
    }

    #[test]
    fn test_save_load_config_roundtrip() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);

        let config = Config {
            shortcut: "Alt+Space".to_string(),
            shortcut_mode: ShortcutMode::Hold,
            autostart: true,
            theme: "glass".to_string(),
        };

        save_config(&path, &config).unwrap();
        let loaded = load_config(&path);

        assert_eq!(loaded.shortcut, "Alt+Space");
        assert_eq!(loaded.shortcut_mode, ShortcutMode::Hold);
        assert!(loaded.autostart);
        assert_eq!(loaded.theme, "glass");
    }

    #[test]
    fn test_save_config_atomic_no_temp_file_left() {
        let dir = TempDir::new().unwrap();
        let path = test_path(&dir);

        save_config(&path, &Config::default()).unwrap();

        let temp_path = path.with_extension("json.tmp");
        assert!(!temp_path.exists());
        assert!(path.exists());
    }
}
