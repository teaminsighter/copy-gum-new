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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();

        // General defaults
        assert!(settings.auto_start_monitoring);
        assert!(!settings.show_on_startup);
        assert!(settings.minimize_to_tray);

        // Storage defaults
        assert_eq!(settings.history_limit, 500);
        assert_eq!(settings.auto_delete_days, 0);
        assert!(settings.save_images);
        assert_eq!(settings.max_image_size_mb, 10);

        // Appearance defaults
        assert_eq!(settings.theme, "auto");
        assert_eq!(settings.card_size, "medium");
        assert_eq!(settings.font_size, 14);
        assert!(settings.show_thumbnails);
        assert_eq!(settings.density, "comfortable");
        assert!(settings.accent_color.is_none());
        assert_eq!(settings.window_opacity, 100);
        assert!(!settings.enable_blur);

        // Shortcuts defaults
        assert_eq!(settings.toggle_window_shortcut, "CommandOrControl+Shift+V");
        assert_eq!(settings.search_shortcut, "CommandOrControl+F");

        // Privacy defaults
        assert!(settings.exclude_apps.is_empty());
        assert_eq!(settings.sensitive_keywords.len(), 2);
        assert!(!settings.enable_analytics);

        // First-run defaults
        assert!(!settings.has_shown_overlay_info);
    }

    #[test]
    fn test_settings_serialization() {
        let settings = AppSettings::default();

        // Serialize to JSON
        let json = serde_json::to_string(&settings).expect("Failed to serialize settings");

        // Deserialize back
        let deserialized: AppSettings =
            serde_json::from_str(&json).expect("Failed to deserialize settings");

        // Verify key fields match
        assert_eq!(settings.theme, deserialized.theme);
        assert_eq!(settings.font_size, deserialized.font_size);
        assert_eq!(settings.history_limit, deserialized.history_limit);
        assert_eq!(settings.auto_start_monitoring, deserialized.auto_start_monitoring);
    }

    #[test]
    fn test_settings_deserialization_with_defaults() {
        // Minimal JSON (missing optional fields)
        let minimal_json = r#"{
            "auto_start_monitoring": true,
            "show_on_startup": false,
            "minimize_to_tray": true,
            "history_limit": 1000,
            "auto_delete_days": 30,
            "save_images": true,
            "max_image_size_mb": 5,
            "theme": "dark",
            "card_size": "large",
            "font_size": 16,
            "show_thumbnails": true,
            "toggle_window_shortcut": "Ctrl+Shift+V",
            "search_shortcut": "Ctrl+F",
            "exclude_apps": [],
            "sensitive_keywords": ["password"],
            "enable_analytics": false
        }"#;

        let settings: AppSettings =
            serde_json::from_str(minimal_json).expect("Failed to deserialize minimal JSON");

        // Fields from JSON
        assert_eq!(settings.history_limit, 1000);
        assert_eq!(settings.theme, "dark");
        assert_eq!(settings.font_size, 16);

        // Default values for missing fields
        assert_eq!(settings.density, "comfortable");
        assert_eq!(settings.window_opacity, 100);
        assert!(!settings.enable_blur);
        assert!(!settings.has_shown_overlay_info);
    }

    #[test]
    fn test_settings_with_custom_values() {
        let mut settings = AppSettings::default();

        // Modify some settings
        settings.theme = "dracula".to_string();
        settings.font_size = 18;
        settings.history_limit = 1000;
        settings.accent_color = Some("#FF5733".to_string());
        settings.exclude_apps = vec!["Terminal".to_string(), "iTerm".to_string()];

        // Serialize and deserialize
        let json = serde_json::to_string(&settings).expect("Failed to serialize");
        let restored: AppSettings = serde_json::from_str(&json).expect("Failed to deserialize");

        // Verify custom values preserved
        assert_eq!(restored.theme, "dracula");
        assert_eq!(restored.font_size, 18);
        assert_eq!(restored.history_limit, 1000);
        assert_eq!(restored.accent_color, Some("#FF5733".to_string()));
        assert_eq!(restored.exclude_apps.len(), 2);
        assert!(restored.exclude_apps.contains(&"Terminal".to_string()));
    }

    #[test]
    fn test_sensitive_keywords_default() {
        let settings = AppSettings::default();

        // Default sensitive keywords
        assert!(settings.sensitive_keywords.contains(&"password".to_string()));
        assert!(settings.sensitive_keywords.contains(&"secret".to_string()));
    }
}
