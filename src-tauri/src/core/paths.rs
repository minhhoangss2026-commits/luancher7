use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// Thư mục dữ liệu RIÊNG của launcher (không đụng ~/.minecraft)
pub fn data_dir(app: &AppHandle) -> PathBuf {
    let dir = app
        .path()
        .app_data_dir()
        .expect("không xác định được app_data_dir")
        .join("CosmicLauncher");
    std::fs::create_dir_all(&dir).ok();
    dir
}

/// libraries / assets / versions dùng chung giữa các profile
pub fn game_root(app: &AppHandle) -> PathBuf {
    data_dir(app).join("minecraft")
}

/// Java Runtime tự tải về
pub fn runtime_dir(app: &AppHandle) -> PathBuf {
    data_dir(app).join("runtime")
}

/// Thư mục game riêng của từng phiên bản (saves, mods, config...)
pub fn instance_dir(app: &AppHandle, name: &str) -> PathBuf {
    data_dir(app).join("instances").join(name)
}
