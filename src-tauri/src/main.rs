// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod window_manager;
mod clipboard_monitor;
mod content_detector;
mod image_handler;
mod settings;
mod export;
mod app_detector;
mod app_icons;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:copygum.db", db::init_database())
                .build()
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // Database operations now handled in frontend via @tauri-apps/plugin-sql
            // Keeping only clipboard monitoring, window management, and settings commands
            clipboard_monitor::start_clipboard_monitoring,
            clipboard_monitor::stop_clipboard_monitoring,
            clipboard_monitor::is_clipboard_monitoring,
            clipboard_monitor::pause_clipboard_monitoring,
            clipboard_monitor::resume_clipboard_monitoring,
            clipboard_monitor::copy_image_to_clipboard,
            window_manager::toggle_window,
            window_manager::hide_window,
            settings::get_settings,
            settings::save_settings,
            settings::reset_settings,
            export::save_export_file,
            export::read_import_file,
            export::get_export_stats,
        ])
        .manage(clipboard_monitor::ClipboardMonitor::new())
        .setup(|app| {
            // Setup global shortcut for Cmd+Shift+V
            if let Err(e) = window_manager::setup_global_shortcut(&app.handle()) {
                eprintln!("Failed to setup global shortcut: {}", e);
            }

            // Configure window for overlay behavior on macOS
            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
                    use cocoa::base::id;
                    use objc::{msg_send, sel, sel_impl};

                    if let Ok(ns_win_ptr) = window.ns_window() {
                        unsafe {
                            let ns_win = ns_win_ptr as id;

                            // Set collection behavior for multi-space and fullscreen support
                            let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle;
                            ns_win.setCollectionBehavior_(behavior);

                            // Set window level to screensaver level (highest)
                            let _: () = msg_send![ns_win, setLevel: 1000_i64];

                            // Don't hide when app loses focus
                            let _: () = msg_send![ns_win, setHidesOnDeactivate: false];
                        }
                    }
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
