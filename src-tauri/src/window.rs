use tauri::{AppHandle, Emitter, Manager};

/// Show the peek window and focus it
pub fn show_peek(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = app.emit("show-peek", ());
    }
}

/// Hide the peek window
pub fn hide_peek(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
        let _ = app.emit("hide-peek", ());
    }
}

/// Check if the peek window is currently visible
pub fn is_visible(app: &AppHandle) -> bool {
    app.get_webview_window("main")
        .and_then(|w| w.is_visible().ok())
        .unwrap_or(false)
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
