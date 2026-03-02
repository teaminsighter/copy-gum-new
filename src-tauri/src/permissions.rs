// Permissions Helper Module
// Helps check and request macOS permissions

use std::fs;
use tauri::{AppHandle, Manager};

/// Open System Settings to Accessibility permissions
#[tauri::command]
pub fn open_accessibility_settings() -> Result<(), String> {
    std::process::Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn()
        .map_err(|e| format!("Failed to open settings: {}", e))?;
    Ok(())
}

/// Open System Settings to Screen Recording permissions
#[tauri::command]
pub fn open_screen_recording_settings() -> Result<(), String> {
    std::process::Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture")
        .spawn()
        .map_err(|e| format!("Failed to open settings: {}", e))?;
    Ok(())
}

/// Check if app has been launched before (for first-run detection)
#[tauri::command]
pub fn check_first_run(app: AppHandle) -> Result<bool, String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    let marker_file = app_data_dir.join("CopyGum").join(".setup_complete");
    
    Ok(!marker_file.exists())
}

/// Mark setup as complete
#[tauri::command]
pub fn mark_setup_complete(app: AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    let copygum_dir = app_data_dir.join("CopyGum");
    fs::create_dir_all(&copygum_dir)
        .map_err(|e| format!("Failed to create dir: {}", e))?;
    
    let marker_file = copygum_dir.join(".setup_complete");
    fs::write(&marker_file, "1")
        .map_err(|e| format!("Failed to write marker: {}", e))?;
    
    Ok(())
}
