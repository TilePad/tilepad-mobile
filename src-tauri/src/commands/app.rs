use super::CmdResult;
use tauri::{AppHandle, Manager};

/// Get the third party licenses file
#[tauri::command]
pub async fn app_get_licenses(app: AppHandle) -> CmdResult<String> {
    let file = app.path().resource_dir()?.join("THIRD_PARTY_LICENSES.md");
    let contents = tokio::fs::read_to_string(file).await?;

    Ok(contents)
}
