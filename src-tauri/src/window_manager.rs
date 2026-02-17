// Window Manager - Handles global shortcuts and window visibility
use tauri::{AppHandle, Manager, PhysicalPosition};

// Allow deprecated cocoa APIs until migration to objc2
#[allow(deprecated)]
#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
#[allow(deprecated)]
#[cfg(target_os = "macos")]
use cocoa::base::{id, NO};

// Window level constants for macOS
// NSStatusWindowLevel (25) - above normal windows and floating panels, below alerts
// This is the level used by menu bar extras and overlay utilities
#[cfg(target_os = "macos")]
const OVERLAY_WINDOW_LEVEL: i64 = 25;

#[allow(deprecated)]
#[tauri::command]
pub fn toggle_window(app: AppHandle) -> Result<(), String> {
    println!("[CopyGum] toggle_window called");

    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                println!("[CopyGum] Window is visible, hiding...");
                window.hide().map_err(|e| e.to_string())?;
            }
            Ok(false) => {
                println!("[CopyGum] Window is hidden, showing...");

                // Position window at bottom of screen
                position_window_right(&window)?;

                // Show window first using Tauri
                window.show().map_err(|e| e.to_string())?;

                // Set always on top via Tauri API
                window.set_always_on_top(true).map_err(|e| e.to_string())?;

                #[cfg(target_os = "macos")]
                {
                    use objc::msg_send;
                    use objc::sel;
                    use objc::sel_impl;

                    if let Ok(ns_win_ptr) = window.ns_window() {
                        unsafe {
                            let ns_win = ns_win_ptr as id;

                            // Set overlay window level (NSStatusWindowLevel = 25)
                            // High enough to float over all normal windows but below system alerts
                            let _: () = msg_send![ns_win, setLevel: OVERLAY_WINDOW_LEVEL];

                            // Consistent collection behavior - stationary overlay across all spaces
                            let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle;
                            ns_win.setCollectionBehavior_(behavior);

                            // Don't hide when app deactivates
                            let _: () = msg_send![ns_win, setHidesOnDeactivate: NO];

                            // Bring to front WITHOUT activating the app (non-focus-stealing)
                            // orderFrontRegardless shows the window above others without
                            // making CopyGum the active application
                            let _: () = msg_send![ns_win, orderFrontRegardless];

                            // Make key so it can receive keyboard input
                            let _: () = msg_send![ns_win, makeKeyWindow];

                            println!("[CopyGum] Window shown with overlay level {}", OVERLAY_WINDOW_LEVEL);
                        }
                    }
                }

                // For Windows, use Windows API for overlay behavior
                #[cfg(target_os = "windows")]
                {
                    use windows::Win32::Foundation::HWND;
                    use windows::Win32::UI::WindowsAndMessaging::{
                        SetWindowPos, SetForegroundWindow,
                        HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
                    };

                    if let Ok(hwnd) = window.hwnd() {
                        unsafe {
                            let hwnd = HWND(hwnd.0 as *mut std::ffi::c_void);

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

                // For other platforms (Linux, etc.), just show the window
                #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                {
                    window.show().map_err(|e| e.to_string())?;
                }
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
#[allow(deprecated)]
#[tauri::command]
pub fn show_window_activated(app: AppHandle) -> Result<(), String> {
    println!("[CopyGum] show_window_activated called (first run)");

    if let Some(window) = app.get_webview_window("main") {
        // Position window at bottom of screen
        position_window_right(&window)?;

        // Show window
        window.show().map_err(|e| e.to_string())?;
        window.set_always_on_top(true).map_err(|e| e.to_string())?;

        #[cfg(target_os = "macos")]
        {
            use objc::msg_send;
            use objc::sel;
            use objc::sel_impl;

            if let Ok(ns_win_ptr) = window.ns_window() {
                unsafe {
                    let ns_win = ns_win_ptr as id;

                    // Set overlay window level
                    let _: () = msg_send![ns_win, setLevel: OVERLAY_WINDOW_LEVEL];

                    // Collection behavior
                    let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle;
                    ns_win.setCollectionBehavior_(behavior);

                    let _: () = msg_send![ns_win, setHidesOnDeactivate: NO];

                    // For first run: ACTIVATE the app and make window key+main
                    // This ensures user can click on setup wizard
                    let _: () = msg_send![ns_win, makeKeyAndOrderFront: cocoa::base::nil];

                    println!("[CopyGum] Window shown and activated for first run");
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                SetWindowPos, SetForegroundWindow,
                HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
            };

            if let Ok(hwnd) = window.hwnd() {
                unsafe {
                    let hwnd = HWND(hwnd.0 as *mut std::ffi::c_void);
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
        }

        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

fn position_window_right(window: &tauri::WebviewWindow) -> Result<(), String> {
    // Get primary monitor
    if let Some(monitor) = window.current_monitor().map_err(|e| e.to_string())? {
        let screen_size = monitor.size();
        let monitor_position = monitor.position();

        // Fixed height of 400px for consistent design
        // Original responsive heights caused layout issues with settings panel
        let window_height = 400_u32;

        // On Windows, we need to account for the taskbar
        // Use the full screen width but position above the taskbar
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::UI::WindowsAndMessaging::{
                SystemParametersInfoW, SPI_GETWORKAREA, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
            };
            use windows::Win32::Foundation::RECT;

            let mut work_area = RECT::default();
            unsafe {
                let _ = SystemParametersInfoW(
                    SPI_GETWORKAREA,
                    0,
                    Some(&mut work_area as *mut _ as *mut std::ffi::c_void),
                    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
                );
            }

            let work_width = (work_area.right - work_area.left) as u32;
            let work_height = (work_area.bottom - work_area.top) as u32;

            // Resize window to match work area width
            window.set_size(tauri::PhysicalSize::new(work_width, window_height))
                .map_err(|e| e.to_string())?;

            // Position at bottom of work area (above taskbar)
            let x = work_area.left;
            let y = work_area.bottom - window_height as i32;

            window
                .set_position(PhysicalPosition::new(x, y))
                .map_err(|e| e.to_string())?;

            println!("[CopyGum] Window positioned at ({}, {}) with size {}x{}", x, y, work_width, window_height);
        }

        // On macOS, smart positioning based on Dock settings
        #[cfg(target_os = "macos")]
        {
            use objc::msg_send;
            use objc::sel;
            use objc::sel_impl;
            use objc::runtime::Object;
            use std::process::Command;

            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            struct NSPoint { x: f64, y: f64 }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            struct NSSize { width: f64, height: f64 }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            struct NSRect { origin: NSPoint, size: NSSize }

            // Check if Dock auto-hide is enabled
            let dock_autohide = Command::new("defaults")
                .args(["read", "com.apple.dock", "autohide"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "1")
                .unwrap_or(false);

            // Check Dock position (bottom, left, right)
            let dock_position = Command::new("defaults")
                .args(["read", "com.apple.dock", "orientation"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "bottom".to_string());

            let ns_screen: *mut Object = unsafe { msg_send![objc::class!(NSScreen), mainScreen] };

            if !ns_screen.is_null() {
                let screen_frame: NSRect = unsafe { msg_send![ns_screen, frame] };
                let visible_frame: NSRect = unsafe { msg_send![ns_screen, visibleFrame] };
                let scale_factor = window.scale_factor().unwrap_or(1.0);

                let screen_width = (screen_frame.size.width * scale_factor) as u32;
                let screen_height = (screen_frame.size.height * scale_factor) as i32;

                // Dock at bottom: visible_frame.origin.y > 0
                // Dock at left: visible_frame.origin.x > 0
                // Dock at right: visible_frame width < screen width
                let dock_at_bottom = visible_frame.origin.y > 0.0;
                let dock_height = if dock_at_bottom {
                    (visible_frame.origin.y * scale_factor) as i32
                } else {
                    0
                };

                // Smart positioning:
                // - If Dock auto-hides OR Dock is not at bottom: position at absolute bottom
                // - If Dock is visible at bottom: position above it
                let y = if dock_autohide || !dock_at_bottom {
                    // Position at absolute bottom - window floats over hidden Dock
                    screen_height - window_height as i32
                } else {
                    // Position above visible Dock
                    screen_height - dock_height - window_height as i32
                };

                // Ensure y doesn't go above menu bar
                let menu_bar_height = 25; // Standard macOS menu bar
                let y = y.max(menu_bar_height);

                window.set_size(tauri::PhysicalSize::new(screen_width, window_height))
                    .map_err(|e| e.to_string())?;

                window
                    .set_position(PhysicalPosition::new(0, y))
                    .map_err(|e| e.to_string())?;

                let position_type = if dock_autohide || !dock_at_bottom {
                    "at bottom (Dock auto-hides or not at bottom)"
                } else {
                    "above Dock"
                };

                println!("[CopyGum] macOS: screen={}x{}, dock_autohide={}, dock_position={}",
                    screen_frame.size.width, screen_frame.size.height,
                    dock_autohide, dock_position);
                println!("[CopyGum] macOS: Window at (0, {}) size {}x{} - {}",
                    y, screen_width, window_height, position_type);
            } else {
                // Fallback positioning
                window.set_size(tauri::PhysicalSize::new(screen_size.width, window_height))
                    .map_err(|e| e.to_string())?;

                let x = monitor_position.x;
                let y = monitor_position.y + screen_size.height as i32 - window_height as i32;

                window
                    .set_position(PhysicalPosition::new(x, y))
                    .map_err(|e| e.to_string())?;

                println!("[CopyGum] macOS: Fallback positioning at ({}, {})", x, y);
            }
        }

        // On other platforms (Linux, etc.), use full screen
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            // Resize window to match screen width
            window.set_size(tauri::PhysicalSize::new(screen_size.width, window_height))
                .map_err(|e| e.to_string())?;

            // Position at bottom of screen, full width
            let x = monitor_position.x;
            let y = monitor_position.y + screen_size.height as i32 - window_height as i32;

            window
                .set_position(PhysicalPosition::new(x, y))
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

pub fn setup_global_shortcut(app: &AppHandle) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

    // Register Cmd+Shift+V (or Ctrl+Shift+V on other platforms)
    let shortcut = "CommandOrControl+Shift+V";
    println!("[CopyGum] Registering global shortcut: {}", shortcut);

    let shortcut_parsed = shortcut.parse::<Shortcut>().map_err(|e| {
        println!("[CopyGum] Failed to parse shortcut: {}", e);
        e.to_string()
    })?;

    let app_handle = app.clone();
    app.global_shortcut()
        .on_shortcut(shortcut_parsed, move |_app, _shortcut, event| {
            println!("[CopyGum] Shortcut event received: {:?}", event.state);
            // Only toggle on key press (Released state), not on key down
            if event.state == ShortcutState::Released {
                println!("[CopyGum] Shortcut released, toggling window...");
                let _ = toggle_window(app_handle.clone());
            }
        })
        .map_err(|e| {
            println!("[CopyGum] Failed to register shortcut: {}", e);
            e.to_string()
        })?;

    println!("[CopyGum] Global shortcut registered successfully!");
    Ok(())
}
