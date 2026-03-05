pub mod config;
pub mod shortcuts;
pub mod tasks;
mod tray;
pub mod window;

use std::sync::Mutex;
use tauri::Manager;
use config::{load_config, ConfigState};
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

            // Initialize data directory
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Initialize task persistence
            let tasks_path = data_dir.join("tasks.json");
            let tasks_file = load_tasks(&tasks_path);
            app.manage(TaskState {
                tasks: Mutex::new(tasks_file),
                data_path: tasks_path,
            });

            // Initialize config persistence
            let config_path = data_dir.join("config.json");
            let config = load_config(&config_path);
            app.manage(ConfigState {
                config: Mutex::new(config),
                config_path,
            });

            // Register global shortcut
            shortcuts::register_shortcut(app.handle());

            // Set up system tray
            if let Err(e) = tray::setup_tray(app.handle()) {
                log::error!("Failed to setup tray: {}", e);
            }

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
            tasks::get_history,
            tasks::create_task,
            tasks::toggle_task,
            tasks::update_task,
            tasks::reorder_tasks,
            config::get_config,
            config::save_config_command,
            shortcuts::update_shortcut,
            window::hide_peek_command,
            window::set_persistent,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
