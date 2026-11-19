use crate::core::paths::data_dir;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Microsoft,
    Offline,
    Guest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub kind: Kind,
    pub name: String,
    pub uuid: String,
    pub skin_url: Option<String>,
}

/// UUID offline chuẩn Minecraft: UUIDv3 (MD5) của "OfflinePlayer:<name>"
fn offline_uuid(name: &str) -> String {
    let mut h = md5::compute(format!("OfflinePlayer:{name}").as_bytes()).0;
    h[6] = (h[6] & 0x0f) | 0x30; // version 3
    h[8] = (h[8] & 0x3f) | 0x80; // variant RFC4122
    Uuid::from_bytes(h).to_string()
}

fn file(app: &AppHandle) -> std::path::PathBuf {
    data_dir(app).join("accounts.json")
}

fn load(app: &AppHandle) -> Vec<Account> {
    std::fs::read_to_string(file(app))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save(app: &AppHandle, list: &[Account]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(list).map_err(|e| e.to_string())?;
    std::fs::write(file(app), json).map_err(|e| e.to_string())
}

/// Lần mở đầu tiên: tự tạo hồ sơ Khách "Player_XXXX" để bấm PLAY ngay
#[tauri::command]
pub fn list_accounts(app: AppHandle) -> Result<Vec<Account>, String> {
    let mut list = load(&app);
    if list.is_empty() {
        let name = format!("Player_{:04}", rand::thread_rng().gen_range(0..10000));
        list.push(Account {
            id: Uuid::new_v4().to_string(),
            kind: Kind::Guest,
            uuid: offline_uuid(&name),
            name,
            skin_url: None,
        });
        save(&app, &list)?;
    }
    Ok(list)
}

#[tauri::command]
pub fn add_offline_account(app: AppHandle, name: String) -> Result<Account, String> {
    let name = name.trim().to_string();
    if name.is_empty() || name.len() > 16 || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err("Tên chỉ gồm chữ, số, dấu _ (tối đa 16 ký tự)".into());
    }
    let mut list = load(&app);
    let acc = Account {
        id: Uuid::new_v4().to_string(),
        kind: Kind::Offline,
        uuid: offline_uuid(&name),
        name,
        skin_url: None,
    };
    list.push(acc.clone());
    save(&app, &list)?;
    Ok(acc)
}

/// Dùng cho engine khi cần thông tin tài khoản
pub fn get_account(app: &AppHandle, id: &str) -> Option<Account> {
    load(app).into_iter().find(|a| a.id == id)
}
