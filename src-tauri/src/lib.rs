mod config;
mod shortcuts;
pub mod tasks;
mod tray;
pub mod window;

use std::sync::Mutex;
use tauri::Manager;
use tasks::{load_tasks, TaskState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize task persistence
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");
            let tasks_path = data_dir.join("tasks.json");
            let tasks_file = load_tasks(&tasks_path);

            app.manage(TaskState {
                tasks: Mutex::new(tasks_file),
                data_path: tasks_path,
            });

            // Register global shortcut
            shortcuts::register_shortcut(app.handle());

            // Handle window blur (click outside) — hide peek
            let app_handle = app.handle().clone();
            if let Some(win) = app.get_webview_window("main") {
                win.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        if !window::is_persistent() {
                            window::hide_peek(&app_handle);
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tasks::get_tasks,
            tasks::create_task,
            window::hide_peek_command,
            window::set_persistent,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
