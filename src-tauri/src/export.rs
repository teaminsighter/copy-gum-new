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
