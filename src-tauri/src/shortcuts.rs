use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::window;

/// Register the default global shortcut for toggling the peek window
pub fn register_shortcut(app: &AppHandle) {
    let shortcut: Shortcut = "CmdOrCtrl+Shift+Space"
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
