// Export/Import Functionality
// Handles exporting clipboard history to JSON/CSV and importing from JSON

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub version: String,
    pub export_date: String,
    pub item_count: usize,
}

/// Save export data to a file
/// The actual data is prepared by the frontend and passed as a JSON string
#[tauri::command]
pub async fn save_export_file(
    file_path: String,
    data: String,
) -> Result<String, String> {
    fs::write(&file_path, data)
        .map_err(|e| format!("Failed to save export file: {}", e))?;

    Ok(file_path)
}

/// Read import file contents
/// Returns the file contents as a string for frontend to parse
#[tauri::command]
pub async fn read_import_file(
    file_path: String,
) -> Result<String, String> {
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read import file: {}", e))?;

    Ok(contents)
}

/// Get export statistics (called before export to show preview)
#[tauri::command]
pub async fn get_export_stats() -> Result<ExportData, String> {
    // This will be called by frontend with actual item counts
    Ok(ExportData {
        version: "1.0".to_string(),
        export_date: chrono::Local::now().to_rfc3339(),
        item_count: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_export_data_serialization() {
        let export_data = ExportData {
            version: "1.0".to_string(),
            export_date: "2024-01-15T10:30:00Z".to_string(),
            item_count: 42,
        };

        let json = serde_json::to_string(&export_data).expect("Failed to serialize");
        assert!(json.contains("\"version\":\"1.0\""));
        assert!(json.contains("\"item_count\":42"));
    }

    #[test]
    fn test_export_data_deserialization() {
        let json = r#"{
            "version": "1.0",
            "export_date": "2024-01-15T10:30:00Z",
            "item_count": 100
        }"#;

        let data: ExportData = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(data.version, "1.0");
        assert_eq!(data.item_count, 100);
    }

    #[tokio::test]
    async fn test_save_and_read_export_file() {
        // Create temp file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let file_path = temp_file.path().to_string_lossy().to_string();

        // Test data
        let test_data = r#"{"items": [{"id": 1, "content": "test"}]}"#.to_string();

        // Save
        let result = save_export_file(file_path.clone(), test_data.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), file_path);

        // Read back
        let read_result = read_import_file(file_path).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_read_nonexistent_file() {
        let result = read_import_file("/nonexistent/path/file.json".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to read import file"));
    }

    #[tokio::test]
    async fn test_get_export_stats() {
        let stats = get_export_stats().await.expect("Failed to get export stats");

        assert_eq!(stats.version, "1.0");
        assert_eq!(stats.item_count, 0);
        // Date should be a valid RFC3339 string
        assert!(stats.export_date.contains("T"));
    }
}
