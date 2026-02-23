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
        let _monitor_position = monitor.position();

        // On Windows, we need to account for the taskbar and DPI scaling
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
            let scale_factor = window.scale_factor().unwrap_or(1.0);

            // Dynamic height: 45% of work area height, min 420px, max 550px
            let base_height = (work_height as f64 * 0.45) as u32;
            let window_height = base_height.clamp(420, 550);

            println!("[CopyGum] Windows: work_area={}x{}, scale={}, base_height={}, final_height={}",
                work_width, work_height, scale_factor, base_height, window_height);

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

        // On macOS, smart positioning using visibleFrame (excludes dock and menu bar)
        // Use LOGICAL coordinates (points) - Tauri handles pixel conversion
        #[cfg(target_os = "macos")]
        {
            use objc::msg_send;
            use objc::sel;
            use objc::sel_impl;
            use objc::runtime::Object;
            use tauri::LogicalPosition;
            use tauri::LogicalSize;

            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            struct NSPoint { x: f64, y: f64 }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            struct NSSize { width: f64, height: f64 }
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            struct NSRect { origin: NSPoint, size: NSSize }

            let ns_screen: *mut Object = unsafe { msg_send![objc::class!(NSScreen), mainScreen] };

            if !ns_screen.is_null() {
                let screen_frame: NSRect = unsafe { msg_send![ns_screen, frame] };
                let visible_frame: NSRect = unsafe { msg_send![ns_screen, visibleFrame] };

                // All values in POINTS (logical coordinates) - don't multiply by scale_factor
                let screen_width = screen_frame.size.width;
                let screen_height = screen_frame.size.height;
                let visible_height = visible_frame.size.height;
                let visible_origin_y = visible_frame.origin.y; // Distance from screen bottom to visible area bottom

                // Dynamic height: 35% of visible height, min 280pt, max 380pt
                // These are POINT values for logical sizing
                let base_height = visible_height * 0.35;
                let window_height = base_height.clamp(280.0, 380.0);

                // Position at bottom of visible frame
                // macOS: Y=0 at bottom, increases upward
                // Tauri: Y=0 at top, increases downward
                // visible_origin_y = dock height (if dock at bottom)
                // Window bottom should be at visible_origin_y (just above dock)
                // In Tauri coords: y = screen_height - visible_origin_y - window_height
                let y = screen_height - visible_origin_y - window_height;

                println!("[CopyGum] macOS: screen={}x{}, visible={}x{} at ({},{})",
                    screen_width, screen_height,
                    visible_frame.size.width, visible_height,
                    visible_frame.origin.x, visible_origin_y);
                println!("[CopyGum] macOS: Window at y={} (logical), size {}x{} (logical)",
                    y, screen_width, window_height);

                window.set_size(LogicalSize::new(screen_width, window_height))
                    .map_err(|e| e.to_string())?;

                window
                    .set_position(LogicalPosition::new(0.0, y))
                    .map_err(|e| e.to_string())?;
            } else {
                // Fallback positioning using Tauri's monitor info
                let scale = window.scale_factor().unwrap_or(1.0);
                let logical_height = screen_size.height as f64 / scale;
                let logical_width = screen_size.width as f64 / scale;

                let base_height = logical_height * 0.35;
                let window_height = base_height.clamp(280.0, 380.0);
                let y = logical_height - window_height;

                window.set_size(tauri::LogicalSize::new(logical_width, window_height))
                    .map_err(|e| e.to_string())?;

                window
                    .set_position(tauri::LogicalPosition::new(0.0, y))
                    .map_err(|e| e.to_string())?;

                println!("[CopyGum] macOS: Fallback at y={} size {}x{}", y, logical_width, window_height);
            }
        }

        // On other platforms (Linux, etc.), use dynamic height
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            // Dynamic height: 45% of screen height, min 420px, max 550px
            let base_height = (screen_size.height as f64 * 0.45) as u32;
            let window_height = base_height.clamp(420, 550);

            // Resize window to match screen width
            window.set_size(tauri::PhysicalSize::new(screen_size.width, window_height))
                .map_err(|e| e.to_string())?;

            // Position at bottom of screen, full width
            let x = monitor_position.x;
            let y = monitor_position.y + screen_size.height as i32 - window_height as i32;

            window
                .set_position(PhysicalPosition::new(x, y))
                .map_err(|e| e.to_string())?;

            println!("[CopyGum] Linux: Window at ({}, {}) size {}x{}", x, y, screen_size.width, window_height);
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
