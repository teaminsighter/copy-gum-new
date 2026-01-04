// Settings Module
// Handles application settings storage and retrieval

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

// Default functions for new settings fields
fn default_density() -> String {
    "comfortable".to_string()
}

fn default_opacity() -> i32 {
    100
}

fn default_blur() -> bool {
    false
}

fn default_has_shown_overlay_info() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    // General
    pub auto_start_monitoring: bool,
    pub show_on_startup: bool,
    pub minimize_to_tray: bool,

    // Storage
    pub history_limit: i32,  // 100, 500, 1000, -1 (unlimited)
    pub auto_delete_days: i32,  // 0 (never), 7, 30, 90
    pub save_images: bool,
    pub max_image_size_mb: i32,

    // Appearance
    pub theme: String,  // "light", "dark", "auto", "high-contrast", "nord", "dracula", "solarized"
    pub card_size: String,  // "small", "medium", "large"
    pub font_size: i32,  // 12, 14, 16, 18
    pub show_thumbnails: bool,
    #[serde(default = "default_density")]
    pub density: String,  // "compact", "comfortable", "spacious"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,  // Custom accent color (optional)
    #[serde(default = "default_opacity")]
    pub window_opacity: i32,  // 0-100, default 95
    #[serde(default = "default_blur")]
    pub enable_blur: bool,  // Enable blur effect

    // Shortcuts
    pub toggle_window_shortcut: String,
    pub search_shortcut: String,

    // Privacy
    pub exclude_apps: Vec<String>,
    pub sensitive_keywords: Vec<String>,
    pub enable_analytics: bool,

    // First-run
    #[serde(default = "default_has_shown_overlay_info", rename = "hasShownOverlayInfo")]
    pub has_shown_overlay_info: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            // General defaults
            auto_start_monitoring: true,
            show_on_startup: false,
            minimize_to_tray: true,

            // Storage defaults
            history_limit: 500,
            auto_delete_days: 0,  // Never auto-delete by default
            save_images: true,
            max_image_size_mb: 10,

            // Appearance defaults
            theme: "auto".to_string(),
            card_size: "medium".to_string(),
            font_size: 14,
            show_thumbnails: true,
            density: "comfortable".to_string(),
            accent_color: None,
            window_opacity: 100,
            enable_blur: false,

            // Shortcuts defaults
            toggle_window_shortcut: "CommandOrControl+Shift+V".to_string(),
            search_shortcut: "CommandOrControl+F".to_string(),

            // Privacy defaults
            exclude_apps: vec![],
            sensitive_keywords: vec!["password".to_string(), "secret".to_string()],
            enable_analytics: false,

            // First-run defaults
            has_shown_overlay_info: false,
        }
    }
}

impl AppSettings {
    /// Load settings from file
    pub fn load(app: &AppHandle) -> Result<Self, String> {
        let settings_path = Self::get_settings_path(app)?;

        if settings_path.exists() {
            let contents = fs::read_to_string(&settings_path)
                .map_err(|e| format!("Failed to read settings: {}", e))?;

            let settings: AppSettings = serde_json::from_str(&contents)
                .map_err(|e| format!("Failed to parse settings: {}", e))?;

            Ok(settings)
        } else {
            Ok(AppSettings::default())
        }
    }

    /// Save settings to file
    pub fn save(&self, app: &AppHandle) -> Result<(), String> {
        let settings_path = Self::get_settings_path(app)?;

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(&settings_path, json)
            .map_err(|e| format!("Failed to write settings: {}", e))?;

        Ok(())
    }

    /// Get the path to the settings file
    fn get_settings_path(app: &AppHandle) -> Result<PathBuf, String> {
        let app_data = app.path().app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        // Create directory if it doesn't exist
        fs::create_dir_all(&app_data)
            .map_err(|e| format!("Failed to create app data dir: {}", e))?;

        Ok(app_data.join("settings.json"))
    }
}

// ============================================
// TAURI COMMANDS
// ============================================

/// Get current settings
#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    AppSettings::load(&app)
}

/// Save settings
#[tauri::command]
pub async fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    settings.save(&app)?;

    // Emit event so other parts of app can react to settings changes
    let _ = app.emit("settings-changed", &settings);

    Ok(())
}

/// Reset settings to defaults
#[tauri::command]
pub async fn reset_settings(app: AppHandle) -> Result<AppSettings, String> {
    let default = AppSettings::default();
    default.save(&app)?;

    // Emit event
    let _ = app.emit("settings-changed", &default);

    Ok(default)
}
