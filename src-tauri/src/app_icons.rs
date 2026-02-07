// App Icons Module - Fetches app icons from macOS system and provides emoji fallbacks
// Provides visual identification for the source application of clipboard content

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Cache for system app icons (bundle_id -> base64 PNG data URL)
static ICON_CACHE: LazyLock<Mutex<HashMap<String, Option<String>>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

/// Map of bundle IDs to emoji icons for top 20 common apps
static APP_ICONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Browsers
    map.insert("com.google.Chrome", "🌐");
    map.insert("com.apple.Safari", "🧭");
    map.insert("org.mozilla.firefox", "🦊");
    map.insert("com.microsoft.edgemac", "🔷");
    map.insert("com.brave.Browser", "🦁");
    map.insert("com.operasoftware.Opera", "🔴");
    map.insert("com.vivaldi.Vivaldi", "🎼");

    // IDEs & Code Editors
    map.insert("com.microsoft.VSCode", "💻");
    map.insert("com.apple.dt.Xcode", "🔨");
    map.insert("com.jetbrains.intellij", "🧠");
    map.insert("com.sublimetext.4", "📝");
    map.insert("com.sublimetext.3", "📝");
    map.insert("io.zed.Zed", "⚡");

    // Productivity
    map.insert("com.apple.Notes", "📒");
    map.insert("notion.id", "📓");
    map.insert("com.apple.finder", "📁");
    map.insert("com.apple.TextEdit", "📄");
    map.insert("com.apple.Preview", "🖼️");

    // Communication
    map.insert("com.tinyspeck.slackmacgap", "💬");
    map.insert("com.apple.MobileSMS", "💭");
    map.insert("us.zoom.xos", "📹");
    map.insert("com.microsoft.teams", "👥");
    map.insert("com.hnc.Discord", "🎮");

    // Terminal
    map.insert("com.apple.Terminal", "🖥️");
    map.insert("com.googlecode.iterm2", "⌨️");
    map.insert("dev.warp.Warp-Stable", "🚀");

    // Design
    map.insert("com.figma.Desktop", "🎨");
    map.insert("com.bohemiancoding.sketch3", "✏️");

    // Other common apps
    map.insert("com.spotify.client", "🎵");
    map.insert("com.apple.mail", "📧");

    map
});

/// Map of app names to emoji icons (fallback when bundle ID is not available)
static APP_NAME_ICONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Browsers
    map.insert("Google Chrome", "🌐");
    map.insert("Safari", "🧭");
    map.insert("Firefox", "🦊");
    map.insert("Microsoft Edge", "🔷");
    map.insert("Brave Browser", "🦁");
    map.insert("Opera", "🔴");
    map.insert("Vivaldi", "🎼");

    // IDEs & Code Editors
    map.insert("Visual Studio Code", "💻");
    map.insert("Code", "💻");
    map.insert("Xcode", "🔨");
    map.insert("IntelliJ IDEA", "🧠");
    map.insert("Sublime Text", "📝");
    map.insert("Zed", "⚡");

    // Productivity
    map.insert("Notes", "📒");
    map.insert("Notion", "📓");
    map.insert("Finder", "📁");
    map.insert("TextEdit", "📄");
    map.insert("Preview", "🖼️");

    // Communication
    map.insert("Slack", "💬");
    map.insert("Messages", "💭");
    map.insert("zoom.us", "📹");
    map.insert("Microsoft Teams", "👥");
    map.insert("Discord", "🎮");

    // Terminal
    map.insert("Terminal", "🖥️");
    map.insert("iTerm2", "⌨️");
    map.insert("iTerm", "⌨️");
    map.insert("Warp", "🚀");

    // Design
    map.insert("Figma", "🎨");
    map.insert("Sketch", "✏️");

    // Other common apps
    map.insert("Spotify", "🎵");
    map.insert("Mail", "📧");

    map
});

/// Fetch app icon from macOS system as base64 PNG
#[cfg(target_os = "macos")]
#[allow(deprecated)]
fn fetch_system_icon(bundle_id: &str) -> Option<String> {
    use cocoa::base::{id, nil};
    use cocoa::foundation::NSString;
    use objc::{class, msg_send, sel, sel_impl};

    unsafe {
        // Get NSWorkspace
        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        if workspace == nil {
            return None;
        }

        // Create NSString for bundle ID
        let bundle_id_nsstring = NSString::alloc(nil).init_str(bundle_id);
        if bundle_id_nsstring == nil {
            return None;
        }

        // Get app URL from bundle ID
        let app_url: id = msg_send![workspace, URLForApplicationWithBundleIdentifier: bundle_id_nsstring];
        if app_url == nil {
            return None;
        }

        // Get path from URL
        let path: id = msg_send![app_url, path];
        if path == nil {
            return None;
        }

        // Get icon for the app
        let icon: id = msg_send![workspace, iconForFile: path];
        if icon == nil {
            return None;
        }

        // Set icon size to 32x32
        let size = cocoa::foundation::NSSize::new(32.0, 32.0);
        let _: () = msg_send![icon, setSize: size];

        // Convert NSImage to PNG data
        // First, get TIFF representation
        let tiff_data: id = msg_send![icon, TIFFRepresentation];
        if tiff_data == nil {
            return None;
        }

        // Create NSBitmapImageRep from TIFF data
        let bitmap_rep: id = msg_send![class!(NSBitmapImageRep), imageRepWithData: tiff_data];
        if bitmap_rep == nil {
            return None;
        }

        // Convert to PNG (NSBitmapImageFileTypePNG = 4)
        let png_data: id = msg_send![bitmap_rep, representationUsingType: 4_u64 properties: nil];
        if png_data == nil {
            return None;
        }

        // Get bytes from NSData
        let length: usize = msg_send![png_data, length];
        let bytes: *const u8 = msg_send![png_data, bytes];

        if bytes.is_null() || length == 0 {
            return None;
        }

        // Copy bytes to Vec
        let slice = std::slice::from_raw_parts(bytes, length);
        let png_bytes = slice.to_vec();

        // Encode as base64 data URL
        let base64_data = BASE64.encode(&png_bytes);
        Some(format!("data:image/png;base64,{}", base64_data))
    }
}

#[cfg(not(target_os = "macos"))]
fn fetch_system_icon(_bundle_id: &str) -> Option<String> {
    None
}

/// Get system icon for an app by bundle ID (with caching)
pub fn get_system_icon(bundle_id: &str) -> Option<String> {
    // Check cache first
    if let Ok(cache) = ICON_CACHE.lock() {
        if let Some(cached) = cache.get(bundle_id) {
            return cached.clone();
        }
    }

    // Fetch from system
    let icon = fetch_system_icon(bundle_id);

    // Cache the result (even if None, to avoid repeated lookups)
    if let Ok(mut cache) = ICON_CACHE.lock() {
        cache.insert(bundle_id.to_string(), icon.clone());
    }

    icon
}

/// Get emoji icon for an app by bundle ID
pub fn get_icon_by_bundle_id(bundle_id: &str) -> Option<&'static str> {
    APP_ICONS.get(bundle_id).copied()
}

/// Get emoji icon for an app by name
pub fn get_icon_by_name(app_name: &str) -> Option<&'static str> {
    APP_NAME_ICONS.get(app_name).copied()
}

/// Get emoji icon for an app, trying bundle ID first, then name
pub fn get_app_icon(bundle_id: Option<&str>, app_name: &str) -> &'static str {
    // Try bundle ID first (more reliable)
    if let Some(bid) = bundle_id {
        if let Some(icon) = get_icon_by_bundle_id(bid) {
            return icon;
        }
    }

    // Fall back to app name
    if let Some(icon) = get_icon_by_name(app_name) {
        return icon;
    }

    // Default icon for unknown apps
    "📋"
}

/// Tauri command to get app icon (system icon or emoji fallback)
/// Returns either a data:image/png;base64,... URL or an emoji string
#[tauri::command]
pub fn get_app_icon_data(bundle_id: Option<String>, app_name: String) -> String {
    // Try to get system icon first (macOS only)
    if let Some(ref bid) = bundle_id {
        if let Some(icon_data) = get_system_icon(bid) {
            return icon_data;
        }
    }

    // Fallback to emoji
    get_app_icon(bundle_id.as_deref(), &app_name).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_icon_by_bundle_id() {
        assert_eq!(get_icon_by_bundle_id("com.google.Chrome"), Some("🌐"));
        assert_eq!(get_icon_by_bundle_id("com.apple.Safari"), Some("🧭"));
        assert_eq!(get_icon_by_bundle_id("unknown.app"), None);
    }

    #[test]
    fn test_get_icon_by_name() {
        assert_eq!(get_icon_by_name("Google Chrome"), Some("🌐"));
        assert_eq!(get_icon_by_name("Safari"), Some("🧭"));
        assert_eq!(get_icon_by_name("Unknown App"), None);
    }

    #[test]
    fn test_get_app_icon() {
        // Bundle ID takes priority
        assert_eq!(get_app_icon(Some("com.google.Chrome"), "Chrome"), "🌐");

        // Falls back to name
        assert_eq!(get_app_icon(None, "Google Chrome"), "🌐");

        // Returns default for unknown
        assert_eq!(get_app_icon(None, "Unknown App"), "📋");
    }

    #[test]
    fn test_get_app_icon_data_fallback() {
        // Should return emoji for unknown app
        let result = get_app_icon_data(None, "Unknown App".to_string());
        assert_eq!(result, "📋");

        // Should return emoji for known app by name
        let result = get_app_icon_data(None, "Safari".to_string());
        assert_eq!(result, "🧭");
    }
}
