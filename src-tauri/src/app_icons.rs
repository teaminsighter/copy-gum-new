// App Icons Module - Maps application bundle IDs to emoji icons
// Provides visual identification for the source application of clipboard content

use std::collections::HashMap;
use std::sync::LazyLock;

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
}
