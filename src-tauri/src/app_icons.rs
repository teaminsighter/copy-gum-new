// App Icons Module - Fetches app icons from system and provides emoji fallbacks
// Provides visual identification for the source application of clipboard content

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Cache for system app icons (key -> base64 PNG data URL)
static ICON_CACHE: LazyLock<Mutex<HashMap<String, Option<String>>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

/// Map of bundle IDs to emoji icons for common apps
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
/// Includes both macOS display names and Windows process names
static APP_NAME_ICONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Browsers (macOS names + Windows process names)
    map.insert("Google Chrome", "🌐");
    map.insert("chrome", "🌐");
    map.insert("Safari", "🧭");
    map.insert("Firefox", "🦊");
    map.insert("firefox", "🦊");
    map.insert("Microsoft Edge", "🔷");
    map.insert("msedge", "🔷");
    map.insert("Brave Browser", "🦁");
    map.insert("brave", "🦁");
    map.insert("Opera", "🔴");
    map.insert("opera", "🔴");
    map.insert("Vivaldi", "🎼");
    map.insert("vivaldi", "🎼");
    map.insert("Arc", "🌐");
    map.insert("arc", "🌐");

    // IDEs & Code Editors
    map.insert("Visual Studio Code", "💻");
    map.insert("Code", "💻");
    map.insert("Cursor", "💻");
    map.insert("cursor", "💻");
    map.insert("Xcode", "🔨");
    map.insert("IntelliJ IDEA", "🧠");
    map.insert("idea64", "🧠");
    map.insert("idea", "🧠");
    map.insert("Sublime Text", "📝");
    map.insert("sublime_text", "📝");
    map.insert("Zed", "⚡");
    map.insert("zed", "⚡");
    map.insert("Notepad++", "📝");
    map.insert("notepad++", "📝");
    map.insert("devenv", "💻"); // Visual Studio
    map.insert("WindowsTerminal", "🖥️");
    map.insert("Windsurf", "💻");
    map.insert("windsurf", "💻");

    // Productivity
    map.insert("Notes", "📒");
    map.insert("Notion", "📓");
    map.insert("Notion Calendar", "📓");
    map.insert("Finder", "📁");
    map.insert("explorer", "📁"); // Windows Explorer
    map.insert("TextEdit", "📄");
    map.insert("notepad", "📄"); // Windows Notepad
    map.insert("Preview", "🖼️");
    map.insert("WINWORD", "📄"); // Microsoft Word
    map.insert("EXCEL", "📊"); // Microsoft Excel
    map.insert("POWERPNT", "📊"); // Microsoft PowerPoint
    map.insert("OUTLOOK", "📧"); // Microsoft Outlook
    map.insert("Obsidian", "📓");

    // Communication
    map.insert("Slack", "💬");
    map.insert("slack", "💬");
    map.insert("Messages", "💭");
    map.insert("zoom.us", "📹");
    map.insert("Zoom", "📹");
    map.insert("Microsoft Teams", "👥");
    map.insert("ms-teams", "👥");
    map.insert("Teams", "👥");
    map.insert("Discord", "🎮");
    map.insert("discord", "🎮");
    map.insert("Discord Canary", "🎮");
    map.insert("WhatsApp", "💬");
    map.insert("Telegram", "💬");
    map.insert("Signal", "💬");
    map.insert("Skype", "💬");

    // Terminal
    map.insert("Terminal", "🖥️");
    map.insert("iTerm2", "⌨️");
    map.insert("iTerm", "⌨️");
    map.insert("Warp", "🚀");
    map.insert("Alacritty", "🖥️");
    map.insert("alacritty", "🖥️");
    map.insert("cmd", "🖥️"); // Windows Command Prompt
    map.insert("powershell", "🖥️");
    map.insert("pwsh", "🖥️");
    map.insert("wt", "🖥️"); // Windows Terminal
    map.insert("Hyper", "🖥️");
    map.insert("Tabby", "🖥️");

    // Design
    map.insert("Figma", "🎨");
    map.insert("figma", "🎨");
    map.insert("Sketch", "✏️");
    map.insert("Photoshop", "🎨");
    map.insert("Illustrator", "🎨");

    // Other common apps
    map.insert("Spotify", "🎵");
    map.insert("spotify", "🎵");
    map.insert("Mail", "📧");
    map.insert("Thunderbird", "📧");
    map.insert("thunderbird", "📧");
    map.insert("CopyGum", "📋");
    map.insert("copygum-app", "📋");

    map
});

/// Fetch app icon from a Windows .exe file as base64 PNG
pub fn fetch_icon_from_exe(exe_path: &str) -> Option<String> {
    use std::ptr;
    use windows::Win32::UI::Shell::ExtractIconExW;
    use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};
    use windows::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, SelectObject,
        BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    };
    use windows::core::PCWSTR;

    unsafe {
        // Convert path to wide string
        let wide_path: Vec<u16> = exe_path.encode_utf16().chain(std::iter::once(0)).collect();

        // Extract the first icon from the exe
        let mut large_icon: HICON = HICON(ptr::null_mut());
        let count = ExtractIconExW(
            PCWSTR(wide_path.as_ptr()),
            0,
            Some(&mut large_icon),
            None,
            1,
        );

        if count == 0 || large_icon.0.is_null() {
            return None;
        }

        // Get icon info to access the bitmap
        let mut icon_info = ICONINFO::default();
        if GetIconInfo(large_icon, &mut icon_info).is_err() {
            let _ = DestroyIcon(large_icon);
            return None;
        }

        // Get bitmap dimensions
        let hdc = CreateCompatibleDC(None);
        if hdc.is_invalid() {
            if !icon_info.hbmColor.is_invalid() { let _ = DeleteObject(icon_info.hbmColor); }
            if !icon_info.hbmMask.is_invalid() { let _ = DeleteObject(icon_info.hbmMask); }
            let _ = DestroyIcon(large_icon);
            return None;
        }

        let old_bmp = SelectObject(hdc, icon_info.hbmColor);

        // Setup bitmap info header
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: 32,
                biHeight: -32, // Top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [Default::default()],
        };

        // Allocate buffer for pixel data (32x32 RGBA)
        let mut pixels: Vec<u8> = vec![0; 32 * 32 * 4];

        let result = GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            32,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        // Cleanup GDI
        SelectObject(hdc, old_bmp);
        let _ = DeleteDC(hdc);
        if !icon_info.hbmColor.is_invalid() { let _ = DeleteObject(icon_info.hbmColor); }
        if !icon_info.hbmMask.is_invalid() { let _ = DeleteObject(icon_info.hbmMask); }
        let _ = DestroyIcon(large_icon);

        if result == 0 {
            return None;
        }

        // Convert BGRA to RGBA
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2); // Swap B and R
        }

        // Create PNG using the image crate
        let img = image::RgbaImage::from_raw(32, 32, pixels)?;
        let mut png_bytes: Vec<u8> = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        image::ImageEncoder::write_image(
            encoder,
            img.as_raw(),
            32,
            32,
            image::ExtendedColorType::Rgba8,
        ).ok()?;

        let base64_data = BASE64.encode(&png_bytes);
        Some(format!("data:image/png;base64,{}", base64_data))
    }
}

/// Get system icon for an app (with caching)
/// Uses bundle_id for emoji lookup, exe_path for Windows icon extraction
pub fn get_system_icon(bundle_id: &str) -> Option<String> {
    // Check cache first
    if let Ok(cache) = ICON_CACHE.lock() {
        if let Some(cached) = cache.get(bundle_id) {
            return cached.clone();
        }
    }

    // On Windows, bundle_id lookup always returns None for system icons
    // (system icons come from exe path instead)
    let icon: Option<String> = None;

    // Cache the result (even if None, to avoid repeated lookups)
    if let Ok(mut cache) = ICON_CACHE.lock() {
        cache.insert(bundle_id.to_string(), icon.clone());
    }

    icon
}

/// Get system icon from exe path (Windows) with caching
pub fn get_system_icon_from_exe(exe_path: &str) -> Option<String> {
    // Check cache first
    if let Ok(cache) = ICON_CACHE.lock() {
        if let Some(cached) = cache.get(exe_path) {
            return cached.clone();
        }
    }

    // Fetch from exe
    let icon = fetch_icon_from_exe(exe_path);

    // Cache the result
    if let Ok(mut cache) = ICON_CACHE.lock() {
        cache.insert(exe_path.to_string(), icon.clone());
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
pub fn get_app_icon_data(bundle_id: Option<String>, app_name: String, exe_path: Option<String>) -> String {
    // Try to get system icon via bundle ID
    if let Some(ref bid) = bundle_id {
        if let Some(icon_data) = get_system_icon(bid) {
            return icon_data;
        }
    }

    // Try to get system icon via exe path (Windows)
    if let Some(ref path) = exe_path {
        if let Some(icon_data) = get_system_icon_from_exe(path) {
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
        let result = get_app_icon_data(None, "Unknown App".to_string(), None);
        assert_eq!(result, "📋");

        // Should return emoji for known app by name
        let result = get_app_icon_data(None, "Safari".to_string(), None);
        assert_eq!(result, "🧭");
    }
}
