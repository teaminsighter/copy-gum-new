// Clipboard Monitoring Module
// Uses Windows native clipboard listener (WM_CLIPBOARDUPDATE) for zero-CPU-when-idle monitoring

use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Manager, Emitter};
use tokio::sync::Mutex;

use crate::app_detector::get_frontmost_app;
use crate::app_icons::get_app_icon;
use crate::settings::AppSettings;

/// Debug logging macro - compiles to nothing in release builds
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[derive(Debug, Clone)]
pub struct ClipboardMonitor {
    is_running: Arc<Mutex<bool>>,
    last_content: Arc<Mutex<String>>,
    last_image_hash: Arc<Mutex<String>>,
    last_timestamp: Arc<Mutex<i64>>,
    debounce_ms: i64,
    /// Handle to the hidden message window for clipboard notifications
    listener_hwnd: Arc<Mutex<Option<isize>>>,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            last_content: Arc::new(Mutex::new(String::new())),
            last_image_hash: Arc::new(Mutex::new(String::new())),
            last_timestamp: Arc::new(Mutex::new(0)),
            debounce_ms: 1000, // 1 second debounce window (prevents accidental double-copy)
            listener_hwnd: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&self, app: AppHandle) {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            return;
        }

        *is_running = true;
        drop(is_running); // Release lock before spawning task

        let monitor = self.clone();

        // Create a channel for clipboard change notifications
        let (tx, rx) = tokio::sync::mpsc::channel::<()>(32);

        // Start the Win32 message loop on a dedicated OS thread
        let listener_hwnd = self.listener_hwnd.clone();
        let is_running_flag = self.is_running.clone();
        std::thread::spawn(move || {
            run_clipboard_listener(tx, listener_hwnd, is_running_flag);
        });

        // Spawn the async monitor loop that reacts to clipboard changes
        tokio::spawn(async move {
            monitor.monitor_loop(app, rx).await;
        });
    }

    pub async fn stop(&self) {
        let mut is_running = self.is_running.lock().await;
        *is_running = false;

        // Post WM_QUIT to the listener thread to exit its message loop
        if let Some(hwnd) = self.listener_hwnd.lock().await.take() {
            use windows::Win32::Foundation::{HWND, WPARAM, LPARAM};
            use windows::Win32::System::DataExchange::RemoveClipboardFormatListener;
            use windows::Win32::UI::WindowsAndMessaging::{PostMessageW, WM_QUIT, DestroyWindow};

            unsafe {
                let h = HWND(hwnd as *mut std::ffi::c_void);
                let _ = RemoveClipboardFormatListener(h);
                let _ = PostMessageW(h, WM_QUIT, WPARAM(0), LPARAM(0));
                let _ = DestroyWindow(h);
            }
        }
    }

    pub async fn is_running(&self) -> bool {
        *self.is_running.lock().await
    }

    async fn monitor_loop(&self, app: AppHandle, mut rx: tokio::sync::mpsc::Receiver<()>) {
        // Debounce: wait at least 200ms between processing clipboard changes
        let debounce_duration = Duration::from_millis(200);

        while *self.is_running.lock().await {
            // Wait for a clipboard change notification (or check if stopped)
            let received = tokio::select! {
                msg = rx.recv() => msg.is_some(),
                _ = tokio::time::sleep(Duration::from_secs(2)) => {
                    // Periodic check to see if we should stop
                    continue;
                }
            };

            if !received || !*self.is_running.lock().await {
                break;
            }

            // Debounce: drain any rapid successive notifications
            tokio::time::sleep(debounce_duration).await;
            while rx.try_recv().is_ok() {
                // Drain extra notifications
            }

            // Check settings for save_images preference
            let save_images = match AppSettings::load(&app) {
                Ok(settings) => settings.save_images,
                Err(_) => true,
            };

            // Check clipboard format BEFORE reading to avoid unnecessary work
            let has_image = is_clipboard_format_available_image();
            let has_text = is_clipboard_format_available_text();

            // Check for images first (higher priority) - only if save_images is enabled AND image format is available
            if save_images && has_image {
                if let Some(image_data) = self.read_clipboard_image().await {
                    self.handle_clipboard_image(&app, image_data).await;
                    continue;
                }
            }

            // Fall back to text reading - only if text format is available
            if has_text {
                match self.read_clipboard(&app).await {
                    Ok(Some(content)) => {
                        let current_time = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as i64;

                        if self.should_save(&content, current_time).await {
                            let _ = self.save_to_database(&app, &content).await;

                            let mut last_content = self.last_content.lock().await;
                            let mut last_timestamp = self.last_timestamp.lock().await;
                            *last_content = content;
                            *last_timestamp = current_time;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => {}
                }
            }
        }
    }

    async fn read_clipboard(&self, app: &AppHandle) -> Result<Option<String>, String> {
        use tauri_plugin_clipboard_manager::ClipboardExt;

        match app.clipboard().read_text() {
            Ok(text) => {
                if text.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(text))
                }
            }
            Err(e) => Err(format!("Failed to read clipboard: {}", e)),
        }
    }

    async fn read_clipboard_image(&self) -> Option<Vec<u8>> {
        use arboard::Clipboard;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Try to get clipboard image
        let mut clipboard = Clipboard::new().ok()?;
        let image_data = clipboard.get_image().ok()?;

        // Hash raw RGBA bytes for dedup BEFORE encoding to PNG (optimization)
        let mut hasher = DefaultHasher::new();
        image_data.bytes.hash(&mut hasher);
        let image_hash = format!("{:x}", hasher.finish());

        // Check if this is the same image as last time (early exit before PNG encoding)
        {
            let last_hash = self.last_image_hash.lock().await;
            if *last_hash == image_hash {
                return None;
            }
        }

        // Only now convert to PNG (expensive operation)
        let img = image::RgbaImage::from_raw(
            image_data.width as u32,
            image_data.height as u32,
            image_data.bytes.to_vec(),
        )?;

        let mut png_bytes = Vec::new();
        image::DynamicImage::ImageRgba8(img)
            .write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .ok()?;

        Some(png_bytes)
    }

    async fn should_save(&self, new_content: &str, current_time: i64) -> bool {
        // Filter out unwanted content patterns
        if self.should_ignore_content(new_content) {
            return false;
        }

        let last_content = self.last_content.lock().await;
        let last_timestamp = self.last_timestamp.lock().await;

        // If content is different, always save
        if new_content != *last_content {
            return true;
        }

        // Same content - check if enough time has passed (debounce)
        let time_diff = current_time - *last_timestamp;
        time_diff > self.debounce_ms
    }

    fn should_ignore_content(&self, content: &str) -> bool {
        // Ignore very short content (less than 2 characters)
        if content.trim().len() < 2 {
            return true;
        }

        // Ignore content that looks like console logs or debug output
        let lower = content.to_lowercase();
        let ignore_patterns = [
            "[log]",
            "[error]",
            "[warn]",
            "[info]",
            "[debug]",
            "console.",
            "clipboard changed event",
            "saved clipboard item",
            ".ts:",
            ".js:",
            "(clipboardstore",
            "(database.ts",
        ];

        for pattern in &ignore_patterns {
            if lower.contains(pattern) {
                return true;
            }
        }

        false
    }

    async fn save_to_database(&self, app: &AppHandle, content: &str) -> Result<i64, String> {
        use crate::content_detector::detect_content_type;
        use serde_json::json;

        let content_type = detect_content_type(content);
        let content_type_str = content_type.as_str().to_string();
        let category = self.map_content_type_to_category(&content_type);

        let app_info = get_frontmost_app();
        let source_app_name = app_info.name.clone();
        let source_bundle_id = app_info.bundle_id.clone();
        let source_exe_path = app_info.exe_path.clone();
        let source_app_icon = get_app_icon(
            app_info.bundle_id.as_deref(),
            &app_info.name
        ).to_string();

        let _ = app.emit("clipboard-changed", json!({
            "content": content,
            "contentType": content_type_str,
            "category": category,
            "isImage": false,
            "sourceAppName": source_app_name,
            "sourceAppIcon": source_app_icon,
            "sourceBundleId": source_bundle_id,
            "sourceExePath": source_exe_path
        }));

        Ok(0)
    }

    fn map_content_type_to_category(&self, content_type: &crate::content_detector::ContentType) -> &'static str {
        use crate::content_detector::ContentType;

        match content_type {
            ContentType::Password => "password",
            ContentType::ApiKey => "apikey",
            ContentType::Color => "color",
            ContentType::Url => "links",
            ContentType::Email => "email",
            ContentType::Phone => "phone",
            ContentType::Number => "number",
            ContentType::Code => "code",
            ContentType::Text => "text",
        }
    }

    async fn handle_clipboard_image(&self, app: &AppHandle, image_data: Vec<u8>) {
        use crate::image_handler::save_clipboard_image;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Create hash of image data for deduplication
        let mut hasher = DefaultHasher::new();
        image_data.hash(&mut hasher);
        let image_hash = format!("{:x}", hasher.finish());

        // Check if this is the same image as last time
        let mut last_hash = self.last_image_hash.lock().await;
        if *last_hash == image_hash {
            return;
        }

        // Check image size against max_image_size_mb setting
        let max_size_bytes = match AppSettings::load(app) {
            Ok(settings) => (settings.max_image_size_mb as u64) * 1024 * 1024,
            Err(_) => 10 * 1024 * 1024,
        };

        let image_size = image_data.len() as u64;
        if image_size > max_size_bytes {
            return;
        }

        let app_data_dir = match app.path().app_data_dir() {
            Ok(dir) => dir,
            Err(_) => return,
        };

        if let Ok(metadata) = save_clipboard_image(&app_data_dir, &image_data).await {
            // Brief delay to ensure file is fully flushed and accessible
            tokio::time::sleep(Duration::from_millis(200)).await;

            if self.save_image_to_database(app, metadata).await.is_ok() {
                *last_hash = image_hash;
            }
        }
    }

    async fn save_image_to_database(
        &self,
        app: &AppHandle,
        metadata: crate::image_handler::ImageMetadata,
    ) -> Result<i64, String> {
        use serde_json::json;

        let app_info = get_frontmost_app();
        let source_app_name = app_info.name.clone();
        let source_bundle_id = app_info.bundle_id.clone();
        let source_exe_path = app_info.exe_path.clone();
        let source_app_icon = get_app_icon(
            app_info.bundle_id.as_deref(),
            &app_info.name
        ).to_string();

        let _ = app.emit("clipboard-changed", json!({
            "content": "",
            "contentType": "image",
            "category": "image",
            "isImage": true,
            "imagePath": metadata.image_path,
            "thumbnailPath": metadata.thumbnail_path,
            "imageWidth": metadata.width,
            "imageHeight": metadata.height,
            "imageSize": metadata.file_size,
            "dominantColor": metadata.dominant_color,
            "sourceAppName": source_app_name,
            "sourceAppIcon": source_app_icon,
            "sourceBundleId": source_bundle_id,
            "sourceExePath": source_exe_path
        }));

        Ok(0)
    }
}

// ============================================
// WIN32 NATIVE CLIPBOARD LISTENER
// ============================================

// Standard clipboard format constants
const CF_BITMAP: u32 = 2;
const CF_DIB: u32 = 8;
const CF_UNICODETEXT: u32 = 13;

/// Check if clipboard has image format available (avoids expensive image read when only text changed)
fn is_clipboard_format_available_image() -> bool {
    use windows::Win32::System::DataExchange::IsClipboardFormatAvailable;

    unsafe {
        IsClipboardFormatAvailable(CF_BITMAP).is_ok()
            || IsClipboardFormatAvailable(CF_DIB).is_ok()
    }
}

/// Check if clipboard has text format available
fn is_clipboard_format_available_text() -> bool {
    use windows::Win32::System::DataExchange::IsClipboardFormatAvailable;

    unsafe {
        IsClipboardFormatAvailable(CF_UNICODETEXT).is_ok()
    }
}

/// Run the Win32 message loop for clipboard notifications on a dedicated OS thread.
/// This creates a hidden message-only window, registers for clipboard updates,
/// and forwards WM_CLIPBOARDUPDATE messages to the async monitor via an mpsc channel.
fn run_clipboard_listener(
    tx: tokio::sync::mpsc::Sender<()>,
    listener_hwnd: Arc<Mutex<Option<isize>>>,
    is_running: Arc<Mutex<bool>>,
) {
    use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows::Win32::System::DataExchange::AddClipboardFormatListener;
    use windows::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, GetMessageW, TranslateMessage, DispatchMessageW,
        RegisterClassW, WNDCLASSW, HWND_MESSAGE, MSG,
        WS_EX_NOACTIVATE, WM_CLIPBOARDUPDATE,
    };
    use windows::core::PCWSTR;

    // Window procedure
    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    unsafe {
        // Register window class
        let class_name_str: Vec<u16> = "CopyGumClipboardListener\0".encode_utf16().collect();

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            lpszClassName: PCWSTR(class_name_str.as_ptr()),
            ..Default::default()
        };

        RegisterClassW(&wc);

        // Create a hidden message-only window
        let hwnd = CreateWindowExW(
            WS_EX_NOACTIVATE,
            PCWSTR(class_name_str.as_ptr()),
            PCWSTR::null(),
            Default::default(),
            0, 0, 0, 0,
            HWND_MESSAGE,
            None,
            None,
            None,
        );

        if let Ok(hwnd) = hwnd {
            // Register for clipboard change notifications
            if AddClipboardFormatListener(hwnd).is_ok() {
                debug_log!("[CopyGum] Clipboard listener registered successfully");

                // Store hwnd so we can clean up later
                {
                    let rt = tokio::runtime::Handle::current();
                    rt.block_on(async {
                        *listener_hwnd.lock().await = Some(hwnd.0 as isize);
                    });
                }

                // Run Win32 message loop
                let mut msg = MSG::default();
                loop {
                    let result = GetMessageW(&mut msg, hwnd, 0, 0);
                    if !result.as_bool() {
                        // WM_QUIT received or error
                        break;
                    }

                    if msg.message == WM_CLIPBOARDUPDATE {
                        // Notify the async monitor
                        let _ = tx.try_send(());
                    }

                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);

                    // Check if we should stop
                    let should_stop = {
                        let rt = tokio::runtime::Handle::current();
                        rt.block_on(async {
                            !*is_running.lock().await
                        })
                    };
                    if should_stop {
                        break;
                    }
                }

                debug_log!("[CopyGum] Clipboard listener message loop exited");
            } else {
                eprintln!("[CopyGum] Failed to register clipboard format listener");
            }
        } else {
            eprintln!("[CopyGum] Failed to create clipboard listener window");
        }
    }
}

// ============================================
// TAURI COMMANDS
// ============================================

#[tauri::command]
pub async fn start_clipboard_monitoring(
    app: AppHandle,
) -> Result<(), String> {
    let monitor = app.state::<ClipboardMonitor>();
    monitor.start(app.clone()).await;
    Ok(())
}

#[tauri::command]
pub async fn stop_clipboard_monitoring(
    app: AppHandle,
) -> Result<(), String> {
    let monitor = app.state::<ClipboardMonitor>();
    monitor.stop().await;
    Ok(())
}

#[tauri::command]
pub async fn is_clipboard_monitoring(
    app: AppHandle,
) -> Result<bool, String> {
    let monitor = app.state::<ClipboardMonitor>();
    Ok(monitor.is_running().await)
}

#[tauri::command]
pub async fn pause_clipboard_monitoring(
    app: AppHandle,
) -> Result<(), String> {
    let monitor = app.state::<ClipboardMonitor>();
    monitor.stop().await;
    Ok(())
}

#[tauri::command]
pub async fn resume_clipboard_monitoring(
    app: AppHandle,
) -> Result<(), String> {
    let monitor = app.state::<ClipboardMonitor>();
    monitor.start(app.clone()).await;
    Ok(())
}

/// Copy an image file to the system clipboard
/// This allows users to paste the image into other applications
#[tauri::command]
pub async fn copy_image_to_clipboard(image_path: String) -> Result<(), String> {
    use arboard::Clipboard;
    use image::ImageReader;
    use std::path::Path;

    debug_log!("[CopyGum] copy_image_to_clipboard called with path: {}", image_path);

    let path = Path::new(&image_path);
    if !path.exists() {
        let err = format!("Image file not found: {}", image_path);
        eprintln!("[CopyGum] Error: {}", err);
        return Err(err);
    }

    let img = ImageReader::open(path)
        .map_err(|e| {
            let err = format!("Failed to open image file: {}", e);
            eprintln!("[CopyGum] Error: {}", err);
            err
        })?
        .decode()
        .map_err(|e| {
            let err = format!("Failed to decode image: {}", e);
            eprintln!("[CopyGum] Error: {}", err);
            err
        })?;

    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    let pixels = rgba_img.into_raw();

    debug_log!("[CopyGum] Creating clipboard image data ({}x{}, {} bytes)", width, height, pixels.len());

    let image_data = arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::Owned(pixels),
    };

    let mut clipboard = Clipboard::new()
        .map_err(|e| {
            let err = format!("Failed to access clipboard: {}", e);
            eprintln!("[CopyGum] Error: {}", err);
            err
        })?;

    clipboard.set_image(image_data)
        .map_err(|e| {
            let err = format!("Failed to set clipboard image: {}", e);
            eprintln!("[CopyGum] Error: {}", err);
            err
        })?;

    debug_log!("[CopyGum] Image copied to clipboard successfully!");
    Ok(())
}
