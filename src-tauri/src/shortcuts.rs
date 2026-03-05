use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::window;

/// Register a global shortcut for toggling the peek window
pub fn register_shortcut(app: &AppHandle) {
    register_shortcut_str(app, "CmdOrCtrl+Shift+Space");
}

/// Register a specific shortcut string
pub fn register_shortcut_str(app: &AppHandle, shortcut_str: &str) {
    let shortcut: Shortcut = shortcut_str
        .parse()
        .expect("Failed to parse shortcut");

    app.global_shortcut()
        .on_shortcut(shortcut, move |app_handle, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                window::toggle_peek(app_handle);
            }
        })
        .expect("Failed to register global shortcut");
}

/// IPC command: change the global shortcut
#[tauri::command]
pub fn update_shortcut(shortcut: String, app: AppHandle) -> Result<(), String> {
    // Unregister all existing shortcuts
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| format!("Failed to unregister shortcuts: {}", e))?;

    // Parse and register the new shortcut
    let parsed: Shortcut = shortcut
        .parse()
        .map_err(|e| format!("Invalid shortcut: {}", e))?;

    app.global_shortcut()
        .on_shortcut(parsed, move |app_handle, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                window::toggle_peek(app_handle);
            }
        })
        .map_err(|e| format!("Failed to register shortcut: {}", e))?;

    Ok(())
}
