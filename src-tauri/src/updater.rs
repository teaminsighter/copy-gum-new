// Auto-update functionality using tauri-plugin-updater
use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;
use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateInfo {
    pub available: bool,
    pub version: Option<String>,
    pub notes: Option<String>,
    pub date: Option<String>,
}

/// Check if an update is available
#[tauri::command]
pub async fn check_for_update(app: AppHandle) -> Result<UpdateInfo, String> {
    println!("[CopyGum] Checking for updates...");

    let updater = app.updater().map_err(|e| {
        println!("[CopyGum] Failed to get updater: {}", e);
        format!("Failed to initialize updater: {}", e)
    })?;

    match updater.check().await {
        Ok(Some(update)) => {
            println!("[CopyGum] Update available: v{}", update.version);
            Ok(UpdateInfo {
                available: true,
                version: Some(update.version.clone()),
                notes: update.body.clone(),
                date: update.date.map(|d| d.to_string()),
            })
        }
        Ok(None) => {
            println!("[CopyGum] Already up to date");
            Ok(UpdateInfo {
                available: false,
                version: None,
                notes: None,
                date: None,
            })
        }
        Err(e) => {
            println!("[CopyGum] Update check failed: {}", e);
            Err(format!("Failed to check for updates: {}", e))
        }
    }
}

/// Download and install the update
#[tauri::command]
pub async fn install_update(app: AppHandle) -> Result<(), String> {
    println!("[CopyGum] Starting update installation...");

    let updater = app.updater().map_err(|e| {
        format!("Failed to initialize updater: {}", e)
    })?;

    let update = updater.check().await
        .map_err(|e| format!("Failed to check for updates: {}", e))?
        .ok_or_else(|| "No update available".to_string())?;

    println!("[CopyGum] Downloading update v{}...", update.version);

    // Download and install
    let mut downloaded = 0;
    let bytes = update.download(
        |chunk_length, content_length| {
            downloaded += chunk_length;
            if let Some(total) = content_length {
                let percent = (downloaded as f64 / total as f64 * 100.0) as u32;
                if percent % 10 == 0 {
                    println!("[CopyGum] Download progress: {}%", percent);
                }
            }
        },
        || {
            println!("[CopyGum] Download complete, ready to install");
        },
    ).await.map_err(|e| {
        println!("[CopyGum] Download failed: {}", e);
        format!("Failed to download update: {}", e)
    })?;

    println!("[CopyGum] Installing update...");

    update.install(bytes).map_err(|e| {
        println!("[CopyGum] Installation failed: {}", e);
        format!("Failed to install update: {}", e)
    })?;

    println!("[CopyGum] Update installed successfully. Restart required.");

    Ok(())
}

/// Get the current app version
#[tauri::command]
pub fn get_current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
