// Window Manager - Handles global shortcuts and window visibility (macOS)
use tauri::{AppHandle, Manager};

// macOS-specific imports
#[allow(deprecated)]
use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
#[allow(deprecated)]
use cocoa::base::{id, NO};

// Window level constants for macOS
// NSStatusWindowLevel (25) - above normal windows and floating panels, below alerts
// This is the level used by menu bar extras and overlay utilities
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

                // macOS-specific window configuration
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

        // macOS-specific window configuration
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

        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

fn position_window_right(window: &tauri::WebviewWindow) -> Result<(), String> {
    // Get primary monitor
    if let Some(monitor) = window.current_monitor().map_err(|e| e.to_string())? {
        let screen_size = monitor.size();

        // macOS: Smart positioning using visibleFrame (excludes dock and menu bar)
        // Use LOGICAL coordinates (points) - Tauri handles pixel conversion
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
            let visible_origin_y = visible_frame.origin.y;

            // RESPONSIVE HEIGHT SCALING based on screen size
            // Small screens (laptops): 35% height, min 280pt, max 380pt
            // Medium screens (iMac 24"): 30% height, min 320pt, max 450pt
            // Large screens (27"+): 28% height, min 350pt, max 520pt
            let (height_percent, min_height, max_height) = if screen_width >= 2400.0 {
                // Large screens: 27" iMac, external 4K, ultrawide
                (0.28, 350.0, 520.0)
            } else if screen_width >= 1800.0 {
                // Medium screens: 24" iMac, large external
                (0.30, 320.0, 450.0)
            } else {
                // Small screens: MacBook Air/Pro 13-16"
                (0.35, 280.0, 380.0)
            };

            let base_height = visible_height * height_percent;
            let window_height = base_height.clamp(min_height, max_height);

            // Position at bottom of visible frame
            // macOS: Y=0 at bottom, increases upward
            // Tauri: Y=0 at top, increases downward
            let y = screen_height - visible_origin_y - window_height;

            println!("[CopyGum] macOS: screen={}x{}, visible={}x{} at ({},{})",
                screen_width, screen_height,
                visible_frame.size.width, visible_height,
                visible_frame.origin.x, visible_origin_y);
            println!("[CopyGum] macOS: Window height={}pt ({}% of visible, range {}-{})",
                window_height, (height_percent * 100.0) as i32, min_height, max_height);

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

    Ok(())
}

/// Get screen size category for responsive UI scaling
/// Returns: "small" (laptops), "medium" (24" iMac), "large" (27"+)
#[tauri::command]
pub fn get_screen_size_category() -> String {
    use objc::msg_send;
    use objc::sel;
    use objc::sel_impl;
    use objc::runtime::Object;

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
        let screen_width = screen_frame.size.width;

        if screen_width >= 2400.0 {
            "large".to_string()  // 27" iMac, 4K external, ultrawide
        } else if screen_width >= 1800.0 {
            "medium".to_string() // 24" iMac, large external
        } else {
            "small".to_string()  // MacBook Air/Pro 13-16"
        }
    } else {
        "small".to_string() // Default fallback
    }
}

/// Get detailed screen info for frontend
#[tauri::command]
pub fn get_screen_info() -> serde_json::Value {
    use objc::msg_send;
    use objc::sel;
    use objc::sel_impl;
    use objc::runtime::Object;

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
        let backing_scale: f64 = unsafe { msg_send![ns_screen, backingScaleFactor] };

        let screen_width = screen_frame.size.width;
        let category = if screen_width >= 2400.0 {
            "large"
        } else if screen_width >= 1800.0 {
            "medium"
        } else {
            "small"
        };

        serde_json::json!({
            "width": screen_width,
            "height": screen_frame.size.height,
            "scale": backing_scale,
            "category": category
        })
    } else {
        serde_json::json!({
            "width": 1440,
            "height": 900,
            "scale": 2.0,
            "category": "small"
        })
    }
}

pub fn setup_global_shortcut(app: &AppHandle) -> Result<(), String> {
    use crate::settings::AppSettings;

    // Load shortcut from settings
    let settings = AppSettings::load(app)?;
    let shortcut_str = &settings.toggle_window_shortcut;

    register_shortcut_internal(app, shortcut_str)
}

/// Internal function to register a shortcut
fn register_shortcut_internal(app: &AppHandle, shortcut_str: &str) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

    println!("[CopyGum] Registering global shortcut: {}", shortcut_str);

    let shortcut_parsed = shortcut_str.parse::<Shortcut>().map_err(|e| {
        println!("[CopyGum] Failed to parse shortcut: {}", e);
        format!("Invalid shortcut format: {}", e)
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

/// Update the global shortcut (called when settings change)
#[tauri::command]
pub fn update_global_shortcut(app: AppHandle, old_shortcut: String, new_shortcut: String) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

    println!("[CopyGum] Updating shortcut from '{}' to '{}'", old_shortcut, new_shortcut);

    // Unregister old shortcut
    if let Ok(old_parsed) = old_shortcut.parse::<Shortcut>() {
        if let Err(e) = app.global_shortcut().unregister(old_parsed) {
            println!("[CopyGum] Warning: Failed to unregister old shortcut: {}", e);
            // Continue anyway - old shortcut might not exist
        }
    }

    // Register new shortcut
    register_shortcut_internal(&app, &new_shortcut)?;

    println!("[CopyGum] Shortcut updated successfully!");
    Ok(())
}
