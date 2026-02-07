// Clipboard Monitoring Module
// Monitors clipboard changes and captures content

use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Manager, Emitter};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::app_detector::get_frontmost_app;
use crate::app_icons::get_app_icon;
use crate::settings::AppSettings;

#[derive(Debug, Clone)]
pub struct ClipboardMonitor {
    is_running: Arc<Mutex<bool>>,
    last_content: Arc<Mutex<String>>,
    last_image_hash: Arc<Mutex<String>>,
    last_timestamp: Arc<Mutex<i64>>,
    debounce_ms: i64,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            last_content: Arc::new(Mutex::new(String::new())),
            last_image_hash: Arc::new(Mutex::new(String::new())),
            last_timestamp: Arc::new(Mutex::new(0)),
            debounce_ms: 1000, // 1 second debounce window (prevents accidental double-copy)
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

        tokio::spawn(async move {
            monitor.monitor_loop(app).await;
        });
    }

    pub async fn stop(&self) {
        let mut is_running = self.is_running.lock().await;
        *is_running = false;
    }

    pub async fn is_running(&self) -> bool {
        *self.is_running.lock().await
    }

    async fn monitor_loop(&self, app: AppHandle) {
        while *self.is_running.lock().await {
            // Check settings for save_images preference
            let save_images = match AppSettings::load(&app) {
                Ok(settings) => settings.save_images,
                Err(_) => true, // Default to true if settings can't be loaded
            };

            // Check for images first (higher priority) - only if save_images is enabled
            if save_images {
                if let Some(image_data) = self.read_clipboard_image().await {
                    self.handle_clipboard_image(&app, image_data).await;
                    sleep(Duration::from_millis(500)).await;
                    continue;
                }
            }

            // Fall back to text reading
            match self.read_clipboard(&app).await {
                Ok(Some(content)) => {
                    // Get current timestamp
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as i64;

                    // Check if we should save (debounce logic)
                    if self.should_save(&content, current_time).await {
                        // Save to database
                        let _ = self.save_to_database(&app, &content).await;

                        // Update last content and timestamp
                        let mut last_content = self.last_content.lock().await;
                        let mut last_timestamp = self.last_timestamp.lock().await;
                        *last_content = content;
                        *last_timestamp = current_time;
                    }
                }
                Ok(None) => {
                    // Clipboard is empty or has no text
                }
                Err(_) => {
                    // Error reading clipboard - silent
                }
            }

            // Poll every 500ms
            sleep(Duration::from_millis(500)).await;
        }
    }

    async fn read_clipboard(&self, app: &AppHandle) -> Result<Option<String>, String> {
        // Use tauri-plugin-clipboard-manager to read clipboard
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
        use image::{DynamicImage, ImageFormat};
        use std::io::Cursor;

        // Try to get clipboard image
        let mut clipboard = Clipboard::new().ok()?;
        let image_data = clipboard.get_image().ok()?;

        // Convert arboard::ImageData to PNG bytes
        let img = image::RgbaImage::from_raw(
            image_data.width as u32,
            image_data.height as u32,
            image_data.bytes.to_vec(),
        )?;

        let mut png_bytes = Vec::new();
        DynamicImage::ImageRgba8(img)
            .write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)
            .ok()?;

        Some(png_bytes)
    }

    #[allow(dead_code)]
    fn truncate_for_display(&self, text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len])
        }
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

        // Detect content type automatically
        let content_type = detect_content_type(content);
        let content_type_str = content_type.as_str().to_string();

        // Auto-assign category based on content type
        let category = self.map_content_type_to_category(&content_type);

        // Detect source application
        let app_info = get_frontmost_app();
        let source_app_name = app_info.name.clone();
        let source_bundle_id = app_info.bundle_id.clone();
        let source_app_icon = get_app_icon(
            app_info.bundle_id.as_deref(),
            &app_info.name
        ).to_string();

        // Emit event to frontend with clipboard data
        // Frontend will save to database using @tauri-apps/plugin-sql
        let _ = app.emit("clipboard-changed", json!({
            "content": content,
            "contentType": content_type_str,
            "category": category,
            "isImage": false,
            "sourceAppName": source_app_name,
            "sourceAppIcon": source_app_icon,
            "sourceBundleId": source_bundle_id
        }));

        // Return dummy ID - actual ID will be generated by frontend
        Ok(0)
    }

    /// Map content type to appropriate category
    /// This ensures content is automatically organized
    fn map_content_type_to_category(&self, content_type: &crate::content_detector::ContentType) -> &'static str {
        use crate::content_detector::ContentType;

        match content_type {
            ContentType::Color => "color",      // Hex colors → color category
            ContentType::Url => "links",        // URLs → links category
            ContentType::Email => "email",      // Emails → email category
            ContentType::Phone => "phone",      // Phone numbers → phone category
            ContentType::Number => "number",    // Numbers → number category
            ContentType::Code => "code",        // Code → code category
            ContentType::Text => "text",        // Default → text category
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
            // Same image, skip
            return;
        }

        // Check image size against max_image_size_mb setting
        let max_size_bytes = match AppSettings::load(app) {
            Ok(settings) => (settings.max_image_size_mb as u64) * 1024 * 1024,
            Err(_) => 10 * 1024 * 1024, // Default to 10MB if settings can't be loaded
        };

        let image_size = image_data.len() as u64;
        if image_size > max_size_bytes {
            return;
        }

        // Get app data directory
        let app_data_dir = match app.path().app_data_dir() {
            Ok(dir) => dir,
            Err(_) => {
                return;
            }
        };

        // Save image and generate thumbnail
        match save_clipboard_image(&app_data_dir, &image_data).await {
            Ok(metadata) => {
                // Save to database
                if self.save_image_to_database(app, metadata).await.is_ok() {
                    // Update last hash only after successful save
                    *last_hash = image_hash;
                }
            }
            Err(_) => {
                // Failed to save image - silent
            }
        }
    }

    async fn save_image_to_database(
        &self,
        app: &AppHandle,
        metadata: crate::image_handler::ImageMetadata,
    ) -> Result<i64, String> {
        use serde_json::json;

        // Detect source application
        let app_info = get_frontmost_app();
        let source_app_name = app_info.name.clone();
        let source_bundle_id = app_info.bundle_id.clone();
        let source_app_icon = get_app_icon(
            app_info.bundle_id.as_deref(),
            &app_info.name
        ).to_string();

        // Emit event to frontend with image metadata
        // Frontend will save to database using @tauri-apps/plugin-sql
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
            "sourceBundleId": source_bundle_id
        }));

        // Return dummy ID - actual ID will be generated by frontend
        Ok(0)
    }
}

// ============================================
// TAURI COMMANDS
// ============================================

#[tauri::command]
pub async fn start_clipboard_monitoring(
    app: AppHandle,
) -> Result<(), String> {
    // Get or create monitor instance from app state
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

    // Verify the file exists
    let path = Path::new(&image_path);
    if !path.exists() {
        return Err(format!("Image file not found: {}", image_path));
    }

    // Load the image
    let img = ImageReader::open(path)
        .map_err(|e| format!("Failed to open image file: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Convert to RGBA8
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    let pixels = rgba_img.into_raw();

    // Create arboard ImageData
    let image_data = arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::Owned(pixels),
    };

    // Write to clipboard
    let mut clipboard = Clipboard::new()
        .map_err(|e| format!("Failed to access clipboard: {}", e))?;

    clipboard.set_image(image_data)
        .map_err(|e| format!("Failed to copy image to clipboard: {}", e))?;

    Ok(())
}
