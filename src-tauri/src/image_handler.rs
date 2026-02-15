// Image Handler Module
// Handles image clipboard capture, storage, and thumbnail generation

use image::{DynamicImage, ImageFormat, imageops::FilterType, GenericImageView};
use std::path::Path;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// Image metadata extracted from captured clipboard image
#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub image_path: String,
    pub thumbnail_path: String,
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub dominant_color: Option<String>,
}

/// Save image from clipboard to filesystem
///
/// Steps:
/// 1. Create image storage directory if it doesn't exist
/// 2. Generate unique filename based on timestamp
/// 3. Save original image
/// 4. Generate and save thumbnail
/// 5. Extract metadata (dimensions, file size)
/// 6. Return ImageMetadata
pub async fn save_clipboard_image(
    app_data_dir: &Path,
    image_data: &[u8],
) -> Result<ImageMetadata, String> {
    // Load image from bytes
    let img = image::load_from_memory(image_data)
        .map_err(|e| format!("Failed to load image from clipboard: {}", e))?;

    // Create images directory
    let images_dir = app_data_dir.join("CopyGum").join("images");
    fs::create_dir_all(&images_dir)
        .map_err(|e| format!("Failed to create images directory: {}", e))?;

    // Generate unique filename
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let random_suffix = fastrand::u32(1000..9999);
    let filename = format!("{}_{}.png", timestamp, random_suffix);

    // Save original image
    let image_path = images_dir.join(&filename);
    img.save_with_format(&image_path, ImageFormat::Png)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    // Generate thumbnail
    let thumbnail_filename = format!("{}_{}_thumb.png", timestamp, random_suffix);
    let thumbnail_path = images_dir.join(&thumbnail_filename);
    let thumbnail = generate_thumbnail(&img);
    thumbnail.save_with_format(&thumbnail_path, ImageFormat::Png)
        .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

    // Get file size
    let file_size = fs::metadata(&image_path)
        .map(|m| m.len())
        .unwrap_or(0);

    // Extract dimensions
    let (width, height) = img.dimensions();

    // Extract dominant color (optional - basic implementation)
    let dominant_color = extract_dominant_color(&img);

    Ok(ImageMetadata {
        image_path: image_path.to_string_lossy().to_string(),
        thumbnail_path: thumbnail_path.to_string_lossy().to_string(),
        width,
        height,
        file_size,
        dominant_color,
    })
}

/// Generate thumbnail for image
///
/// Resizes image to fit within 400x400 while maintaining aspect ratio
fn generate_thumbnail(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let max_dimension = 400;

    // If image is already smaller than max dimension, return as-is
    if width <= max_dimension && height <= max_dimension {
        return img.clone();
    }

    // Calculate new dimensions maintaining aspect ratio
    let (new_width, new_height) = if width > height {
        let ratio = max_dimension as f32 / width as f32;
        (max_dimension, (height as f32 * ratio) as u32)
    } else {
        let ratio = max_dimension as f32 / height as f32;
        ((width as f32 * ratio) as u32, max_dimension)
    };

    // Resize using Lanczos3 filter (high quality)
    img.resize(new_width, new_height, FilterType::Lanczos3)
}

/// Extract dominant color from image
///
/// Simple implementation: samples pixels and calculates average color
fn extract_dominant_color(img: &DynamicImage) -> Option<String> {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    // Sample every 10th pixel for performance
    let sample_rate = 10;
    let mut r_sum: u64 = 0;
    let mut g_sum: u64 = 0;
    let mut b_sum: u64 = 0;
    let mut count: u64 = 0;

    for y in (0..height).step_by(sample_rate) {
        for x in (0..width).step_by(sample_rate) {
            let pixel = rgb_img.get_pixel(x, y);
            r_sum += pixel[0] as u64;
            g_sum += pixel[1] as u64;
            b_sum += pixel[2] as u64;
            count += 1;
        }
    }

    if count == 0 {
        return None;
    }

    let r_avg = (r_sum / count) as u8;
    let g_avg = (g_sum / count) as u8;
    let b_avg = (b_sum / count) as u8;

    Some(format!("#{:02X}{:02X}{:02X}", r_avg, g_avg, b_avg))
}

/// Read image file and return as base64 data URL
/// This bypasses asset protocol issues on Windows
#[tauri::command]
pub fn get_image_base64(image_path: String) -> Result<String, String> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    let path = Path::new(&image_path);

    if !path.exists() {
        return Err(format!("Image file not found: {}", image_path));
    }

    let data = fs::read(path)
        .map_err(|e| format!("Failed to read image: {}", e))?;

    // Determine MIME type from extension
    let mime_type = match path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "image/png",
    };

    let base64_data = STANDARD.encode(&data);
    Ok(format!("data:{};base64,{}", mime_type, base64_data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    #[test]
    fn test_generate_thumbnail_large_image() {
        // Create a 1000x800 test image
        let img = DynamicImage::ImageRgb8(RgbImage::new(1000, 800));

        let thumbnail = generate_thumbnail(&img);
        let (thumb_width, thumb_height) = thumbnail.dimensions();

        // Should be resized to fit within 400x400
        assert!(thumb_width <= 400);
        assert!(thumb_height <= 400);

        // Aspect ratio should be preserved (1000:800 = 1.25:1)
        let original_ratio = 1000.0 / 800.0;
        let thumb_ratio = thumb_width as f32 / thumb_height as f32;
        assert!((original_ratio - thumb_ratio).abs() < 0.01);
    }

    #[test]
    fn test_generate_thumbnail_small_image() {
        // Create a 200x150 test image
        let img = DynamicImage::ImageRgb8(RgbImage::new(200, 150));

        let thumbnail = generate_thumbnail(&img);
        let (thumb_width, thumb_height) = thumbnail.dimensions();

        // Should remain unchanged (already smaller than 400x400)
        assert_eq!(thumb_width, 200);
        assert_eq!(thumb_height, 150);
    }

    #[test]
    fn test_extract_dominant_color() {
        // Create a solid red image
        let mut img_data = RgbImage::new(100, 100);
        for pixel in img_data.pixels_mut() {
            *pixel = Rgb([255, 0, 0]); // Red
        }
        let img = DynamicImage::ImageRgb8(img_data);

        let color = extract_dominant_color(&img);
        assert_eq!(color, Some("#FF0000".to_string()));
    }

    #[test]
    fn test_extract_dominant_color_mixed() {
        // Create an image with mixed colors
        let mut img_data = RgbImage::new(100, 100);
        for (x, _y, pixel) in img_data.enumerate_pixels_mut() {
            if x < 50 {
                *pixel = Rgb([255, 0, 0]); // Red
            } else {
                *pixel = Rgb([0, 0, 255]); // Blue
            }
        }
        let img = DynamicImage::ImageRgb8(img_data);

        let color = extract_dominant_color(&img);
        // Should be somewhere between red and blue (purple-ish)
        assert!(color.is_some());

        let hex = color.unwrap();
        assert!(hex.starts_with("#"));
        assert_eq!(hex.len(), 7);
    }
}
