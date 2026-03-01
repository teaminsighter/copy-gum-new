// App Detector Module - Detects the foreground application on Windows
// This module provides functionality to identify which app the user copied from

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId};
use windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;

/// Information about the detected source application
#[derive(Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub bundle_id: Option<String>,
    pub exe_path: Option<String>,
}

impl Default for AppInfo {
    fn default() -> Self {
        AppInfo {
            name: "Unknown".to_string(),
            bundle_id: None,
            exe_path: None,
        }
    }
}

/// Get the foreground (active) application on Windows
pub fn get_frontmost_app() -> AppInfo {
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

        let mut process_name = String::new();
        let mut exe_path: Option<String> = None;

        if process_id != 0 {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
                // Get process base name
                let mut name_buf = [0u16; 256];
                let len = GetModuleBaseNameW(handle, None, &mut name_buf);
                if len > 0 {
                    let name = String::from_utf16_lossy(&name_buf[..len as usize]);
                    // Remove .exe extension
                    process_name = name.trim_end_matches(".exe").to_string();
                }

                // Get full exe path for icon extraction
                let mut path_buf = [0u16; 1024];
                let mut path_len = path_buf.len() as u32;
                if QueryFullProcessImageNameW(
                    handle,
                    PROCESS_NAME_FORMAT(0),
                    windows::core::PWSTR(path_buf.as_mut_ptr()),
                    &mut path_len,
                ).is_ok() {
                    let path = String::from_utf16_lossy(&path_buf[..path_len as usize]);
                    if !path.is_empty() {
                        exe_path = Some(path);
                    }
                }
            }
        }

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
            exe_path,
        }
    }
}

/// Get the exe path from AppInfo (convenience helper)
#[allow(dead_code)]
pub fn get_exe_path(app_info: &AppInfo) -> Option<&str> {
    app_info.exe_path.as_deref()
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
