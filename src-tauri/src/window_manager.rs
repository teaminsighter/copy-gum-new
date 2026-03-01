// Window Manager - Handles global shortcuts and window visibility (Windows-only)
use tauri::{AppHandle, Manager, PhysicalPosition};

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowPos, SetForegroundWindow,
    HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
    SystemParametersInfoW, SPI_GETWORKAREA, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};
use windows::Win32::Foundation::RECT;

/// Debug logging macro - compiles to nothing in release builds
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[tauri::command]
pub fn toggle_window(app: AppHandle) -> Result<(), String> {
    debug_log!("[CopyGum] toggle_window called");

    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                debug_log!("[CopyGum] Window is visible, hiding...");
                window.hide().map_err(|e| e.to_string())?;
            }
            Ok(false) => {
                debug_log!("[CopyGum] Window is hidden, showing...");

                // Position window at bottom of screen
                position_window_right(&window)?;

                // Show window first using Tauri
                window.show().map_err(|e| e.to_string())?;

                // Set always on top via Tauri API
                window.set_always_on_top(true).map_err(|e| e.to_string())?;

                if let Ok(hwnd) = window.hwnd() {
                    unsafe {
                        let hwnd = HWND(hwnd.0);

                        // Set window as topmost
                        let _ = SetWindowPos(
                            hwnd,
                            HWND_TOPMOST,
                            0, 0, 0, 0,
                            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
                        );

                        // Bring to foreground
                        let _ = SetForegroundWindow(hwnd);
                    }
                }

                // Focus the window
                let _ = window.set_focus();
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
pub fn hide_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

/// Show window and fully activate it (for setup wizard, first-run experience)
/// Unlike toggle_window, this activates the app to ensure user can interact
#[tauri::command]
pub fn show_window_activated(app: AppHandle) -> Result<(), String> {
    debug_log!("[CopyGum] show_window_activated called (first run)");

    if let Some(window) = app.get_webview_window("main") {
        // Position window at bottom of screen
        position_window_right(&window)?;

        // Show window
        window.show().map_err(|e| e.to_string())?;
        window.set_always_on_top(true).map_err(|e| e.to_string())?;

        if let Ok(hwnd) = window.hwnd() {
            unsafe {
                let hwnd = HWND(hwnd.0);
                let _ = SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    0, 0, 0, 0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
                );
                let _ = SetForegroundWindow(hwnd);
            }
        }
        let _ = window.set_focus();

        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

fn position_window_right(window: &tauri::WebviewWindow) -> Result<(), String> {
    // Get primary monitor
    if let Some(_monitor) = window.current_monitor().map_err(|e| e.to_string())? {
        let mut work_area = RECT::default();
        unsafe {
            let _ = SystemParametersInfoW(
                SPI_GETWORKAREA,
                0,
                Some(&mut work_area as *mut _ as *mut std::ffi::c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            );
        }

        // Work area is in physical pixels (already DPI-scaled by Windows)
        let work_width = (work_area.right - work_area.left) as u32;
        let work_height = (work_area.bottom - work_area.top) as u32;
        let scale_factor = window.scale_factor().unwrap_or(1.0);

        // Scale min/max constraints by DPI factor so the window looks
        // the same logical size regardless of display scaling
        let min_height = (280.0 * scale_factor) as u32;
        let max_height = (550.0 * scale_factor) as u32;

        // Dynamic height: 45% of work area height, clamped to scaled bounds
        let base_height = (work_height as f64 * 0.45) as u32;
        let window_height = base_height.clamp(min_height, max_height);

        // Use full work area width (SystemParametersInfo already accounts for taskbar)
        let window_width = work_width;

        debug_log!("[CopyGum] work_area={}x{}, scale={:.2}, height={}(min={},max={}), width={}",
            work_width, work_height, scale_factor, window_height, min_height, max_height, window_width);

        // PhysicalSize is correct here because work_area values from Win32 are already
        // in physical pixels — Tauri's set_size(PhysicalSize) expects physical pixels
        window.set_size(tauri::PhysicalSize::new(window_width, window_height))
            .map_err(|e| e.to_string())?;

        // Position at bottom of work area (above taskbar)
        let x = work_area.left;
        let y = work_area.bottom - window_height as i32;

        window
            .set_position(PhysicalPosition::new(x, y))
            .map_err(|e| e.to_string())?;

        debug_log!("[CopyGum] Window positioned at ({}, {}) size {}x{}", x, y, window_width, window_height);
    }

    Ok(())
}

pub fn setup_global_shortcut(app: &AppHandle) -> Result<(), String> {
    // Read the user's saved shortcut from settings, fall back to default
    let shortcut_str = match crate::settings::AppSettings::load(app) {
        Ok(settings) => settings.toggle_window_shortcut,
        Err(_) => "Ctrl+Shift+V".to_string(),
    };

    register_shortcut(app, &shortcut_str)
}

/// Register a specific shortcut string for the toggle-window action.
/// Unregisters all existing shortcuts first to avoid conflicts.
fn register_shortcut(app: &AppHandle, shortcut_str: &str) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

    debug_log!("[CopyGum] Registering global shortcut: {}", shortcut_str);

    let shortcut_parsed = shortcut_str.parse::<Shortcut>().map_err(|e| {
        eprintln!("[CopyGum] Failed to parse shortcut '{}': {}", shortcut_str, e);
        format!("Invalid shortcut '{}': {}", shortcut_str, e)
    })?;

    let app_handle = app.clone();
    app.global_shortcut()
        .on_shortcut(shortcut_parsed, move |_app, _shortcut, event| {
            debug_log!("[CopyGum] Shortcut event received: {:?}", event.state);
            if event.state == ShortcutState::Released {
                debug_log!("[CopyGum] Shortcut released, toggling window...");
                let _ = toggle_window(app_handle.clone());
            }
        })
        .map_err(|e| {
            eprintln!("[CopyGum] Failed to register shortcut: {}", e);
            e.to_string()
        })?;

    debug_log!("[CopyGum] Global shortcut registered successfully!");
    Ok(())
}

/// Tauri command: update the global toggle shortcut at runtime.
/// Unregisters the old shortcut and registers the new one.
#[tauri::command]
pub fn update_global_shortcut(app: AppHandle, old_shortcut: String, new_shortcut: String) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

    debug_log!("[CopyGum] Updating shortcut: '{}' -> '{}'", old_shortcut, new_shortcut);

    // Validate the new shortcut can be parsed before unregistering the old one
    let new_parsed = new_shortcut.parse::<Shortcut>().map_err(|e| {
        format!("Invalid shortcut '{}': {}", new_shortcut, e)
    })?;

    // Check the new shortcut isn't already registered by another app (best-effort)
    let gs = app.global_shortcut();
    if gs.is_registered(new_parsed) {
        // It might be our own old shortcut if old == new, that's fine
        if old_shortcut != new_shortcut {
            return Err(format!("Shortcut '{}' is already in use", new_shortcut));
        }
    }

    // Unregister the old shortcut
    if let Ok(old_parsed) = old_shortcut.parse::<Shortcut>() {
        let _ = gs.unregister(old_parsed);
    }

    // Register the new one
    register_shortcut(&app, &new_shortcut)
}
