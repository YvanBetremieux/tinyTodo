use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter,
};

use crate::window;

/// Set up the system tray with context menu
pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "Afficher", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "Paramètres", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_item, &settings_item, &quit_item])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().cloned().unwrap())
        .menu(&menu)
        .tooltip("tinyTodo")
        .on_menu_event(move |app_handle, event| match event.id.as_ref() {
            "show" => {
                window::show_peek(app_handle);
            }
            "settings" => {
                // Emit event to frontend to show settings view
                let _ = app_handle.emit("show-settings", ());
            }
            "quit" => {
                app_handle.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
