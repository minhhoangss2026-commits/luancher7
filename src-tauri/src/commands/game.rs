use crate::commands::accounts::get_account;
use crate::core::{game, mods, paths};
use tauri::AppHandle;

#[tauri::command]
pub async fn list_versions() -> Result<Vec<String>, String> {
    game::list_release_versions().await.map_err(|e| format!("{e:#}"))
}

#[tauri::command]
pub async fn launch_game(
    app: AppHandle,
    account_id: String,
    version: String,
    loader: String,
    ram_mb: u32,
    boost: bool,
) -> Result<(), String> {
    if !matches!(loader.as_str(), "Vanilla" | "Fabric" | "Quilt") {
        return Err(format!("{loader} chưa được hỗ trợ ở bản này (chọn Vanilla, Fabric hoặc Quilt)"));
    }
    let acc = get_account(&app, &account_id).ok_or("Không tìm thấy tài khoản")?;

    // Lần đầu dùng Fabric/Quilt + bật FPS Boost: tự cài bộ mod tăng FPS
    if boost && loader != "Vanilla" && !mods::manifest_path(&app, &loader, &version).exists() {
        mods::install_boost(&app, &version, &loader)
            .await
            .map_err(|e| format!("{e:#}"))?;
    }
    game::launch(&app, &acc, &version, &loader, ram_mb, boost)
        .await
        .map_err(|e| format!("{e:#}"))
}

#[tauri::command]
pub fn get_data_dir(app: AppHandle) -> String {
    paths::data_dir(&app).to_string_lossy().to_string()
}
