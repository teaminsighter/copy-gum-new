// Window Manager - Handles global shortcuts and window visibility
use tauri::{AppHandle, Manager, PhysicalPosition};

// Allow deprecated cocoa APIs until migration to objc2
#[allow(deprecated)]
#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
#[allow(deprecated)]
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil, YES};

// CGWindowLevel constants from CoreGraphics
#[cfg(target_os = "macos")]
const K_CG_MAXIMUM_WINDOW_LEVEL_KEY: i32 = 14;

#[cfg(target_os = "macos")]
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGWindowLevelForKey(key: i32) -> i32;
}

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

                // On macOS, set window level and activate BEFORE showing
                #[cfg(target_os = "macos")]
                {
                    use objc::msg_send;
                    use objc::sel;
                    use objc::sel_impl;
                    use objc::class;

                    // Get the maximum window level from CoreGraphics
                    let max_level = unsafe { CGWindowLevelForKey(K_CG_MAXIMUM_WINDOW_LEVEL_KEY) };

                    // STEP 1: Activate the app FIRST (before showing window)
                    unsafe {
                        let ns_app: id = msg_send![class!(NSApplication), sharedApplication];

                        // NSApplicationActivationPolicyRegular = 0 (standard app behavior)
                        let _: () = msg_send![ns_app, setActivationPolicy: 0_i64];

                        // Force activate ignoring other apps
                        let _: () = msg_send![ns_app, activateIgnoringOtherApps: YES];
                    }

                    // STEP 2: Configure window level BEFORE showing
                    if let Ok(ns_win_ptr) = window.ns_window() {
                        unsafe {
                            let ns_win = ns_win_ptr as id;

                            // Set collection behavior
                            let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
                                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle;
                            ns_win.setCollectionBehavior_(behavior);

                            // Don't hide when app loses focus
                            let _: () = msg_send![ns_win, setHidesOnDeactivate: false];

                            // Use maximum window level BEFORE showing (critical fix!)
                            let _: () = msg_send![ns_win, setLevel: max_level as i64];
                        }
                    }

                    // STEP 3: Now show the window (after level is set)
                    window.show().map_err(|e| e.to_string())?;

                    // STEP 4: Order front after showing
                    if let Ok(ns_win_ptr) = window.ns_window() {
                        unsafe {
                            let ns_win = ns_win_ptr as id;

                            // Make it the key window and bring to front
                            let _: () = msg_send![ns_win, makeKeyAndOrderFront: nil];

                            // Nuclear option - order front regardless of everything
                            let _: () = msg_send![ns_win, orderFrontRegardless];
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

                    // Show the window first
                    window.show().map_err(|e| e.to_string())?;

                    // Get HWND and set topmost
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

fn position_window_right(window: &tauri::WebviewWindow) -> Result<(), String> {
    // Get primary monitor
    if let Some(monitor) = window.current_monitor().map_err(|e| e.to_string())? {
        let screen_size = monitor.size();
        let window_size = window.outer_size().map_err(|e| e.to_string())?;

        // Position at bottom of screen, full width
        let x = 0;
        let y = screen_size.height as i32 - window_size.height as i32;

        window
            .set_position(PhysicalPosition::new(x, y))
            .map_err(|e| e.to_string())?;
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
