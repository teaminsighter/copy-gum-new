// App Detector Module - Detects the frontmost application on macOS
// This module provides functionality to identify which app the user copied from

// Allow deprecated cocoa APIs until migration to objc2
#[allow(deprecated)]
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

/// Information about the detected source application
#[derive(Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub bundle_id: Option<String>,
}

impl Default for AppInfo {
    fn default() -> Self {
        AppInfo {
            name: "Unknown".to_string(),
            bundle_id: None,
        }
    }
}

/// Get the frontmost (active) application on macOS
#[allow(deprecated)]
#[cfg(target_os = "macos")]
pub fn get_frontmost_app() -> AppInfo {
    unsafe {
        // Get NSWorkspace shared instance
        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        if workspace == nil {
            return AppInfo::default();
        }

        // Get frontmost application
        let frontmost_app: id = msg_send![workspace, frontmostApplication];
        if frontmost_app == nil {
            return AppInfo::default();
        }

        // Get localized name
        let name: id = msg_send![frontmost_app, localizedName];
        let app_name = if name != nil {
            nsstring_to_string(name)
        } else {
            "Unknown".to_string()
        };

        // Get bundle identifier
        let bundle_id: id = msg_send![frontmost_app, bundleIdentifier];
        let bundle_identifier = if bundle_id != nil {
            Some(nsstring_to_string(bundle_id))
        } else {
            None
        };

        AppInfo {
            name: app_name,
            bundle_id: bundle_identifier,
        }
    }
}

/// Get the frontmost (active) application on Windows
#[cfg(target_os = "windows")]
pub fn get_frontmost_app() -> AppInfo {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId};
    use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
    use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;

    unsafe {
        // Get the foreground window
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0.is_null() {
            return AppInfo::default();
        }

        // Get window title
        let mut title_buf = [0u16; 256];
        let len = GetWindowTextW(hwnd, &mut title_buf);
        let window_title = if len > 0 {
            String::from_utf16_lossy(&title_buf[..len as usize])
        } else {
            String::new()
        };

        // Get process ID
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        // Get process name
        let process_name = if process_id != 0 {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
                let mut name_buf = [0u16; 256];
                let len = GetModuleBaseNameW(handle, None, &mut name_buf);
                if len > 0 {
                    let name = String::from_utf16_lossy(&name_buf[..len as usize]);
                    // Remove .exe extension
                    name.trim_end_matches(".exe").to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Use process name if available, otherwise window title
        let app_name = if !process_name.is_empty() {
            process_name
        } else if !window_title.is_empty() {
            window_title
        } else {
            "Unknown".to_string()
        };

        AppInfo {
            name: app_name,
            bundle_id: None, // Windows doesn't have bundle IDs
        }
    }
}

/// Fallback for other platforms (Linux, etc.)
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn get_frontmost_app() -> AppInfo {
    AppInfo::default()
}

/// Convert NSString to Rust String
#[allow(deprecated)]
#[cfg(target_os = "macos")]
unsafe fn nsstring_to_string(nsstring: id) -> String {
    let utf8_ptr: *const i8 = msg_send![nsstring, UTF8String];
    if utf8_ptr.is_null() {
        return String::new();
    }
    std::ffi::CStr::from_ptr(utf8_ptr)
        .to_string_lossy()
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_frontmost_app() {
        let app_info = get_frontmost_app();
        println!("Frontmost app: {:?}", app_info);
        // Should return some app info (at least "Unknown" if detection fails)
        assert!(!app_info.name.is_empty());
    }
}
