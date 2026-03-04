use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter, Manager};

/// Whether the window is in persistent mode (input mode active — don't auto-hide)
static PERSISTENT: AtomicBool = AtomicBool::new(false);

/// Show the peek window and focus it
pub fn show_peek(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = app.emit("show-peek", ());
    }
}

/// Hide the peek window (only if currently visible and not persistent)
pub fn hide_peek(app: &AppHandle) {
    if is_visible(app) {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
            let _ = app.emit("hide-peek", ());
            // Reset persistent mode when hiding
            PERSISTENT.store(false, Ordering::Relaxed);
        }
    }
}

/// Check if the peek window is currently visible
pub fn is_visible(app: &AppHandle) -> bool {
    app.get_webview_window("main")
        .and_then(|w| w.is_visible().ok())
        .unwrap_or(false)
}

/// Check if the window is in persistent mode
pub fn is_persistent() -> bool {
    PERSISTENT.load(Ordering::Relaxed)
}

/// Toggle the peek window visibility
pub fn toggle_peek(app: &AppHandle) {
    if is_visible(app) {
        hide_peek(app);
    } else {
        show_peek(app);
    }
}

/// IPC command: hide the peek window (called from frontend on Escape/blur)
#[tauri::command]
pub fn hide_peek_command(app: AppHandle) {
    hide_peek(&app);
}

/// IPC command: set persistent mode (prevents auto-hide on blur)
#[tauri::command]
pub fn set_persistent(persistent: bool) {
    PERSISTENT.store(persistent, Ordering::Relaxed);
}
