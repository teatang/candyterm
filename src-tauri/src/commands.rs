//! Tauri IPC 命令定义与处理函数

use crate::vault::storage::VaultManager;
use crate::vault::VaultData;
use std::sync::Mutex;
use tauri::State;

/// Tauri 托管的全局状态
pub struct AppState {
    pub vault: Mutex<VaultManager>,
}

/// 检查本地是否已经创建初始化过保险箱
#[tauri::command]
pub fn is_vault_initialized(state: State<'_, AppState>) -> bool {
    let vault = state.vault.lock().unwrap();
    vault.exists()
}

/// 检查保险箱当前是否已在内存中解锁
#[tauri::command]
pub fn is_vault_unlocked(state: State<'_, AppState>) -> bool {
    let vault = state.vault.lock().unwrap();
    vault.is_unlocked()
}

/// 首次初始化保险箱
#[tauri::command]
pub fn init_vault(password: String, state: State<'_, AppState>) -> Result<VaultData, String> {
    let vault = state.vault.lock().unwrap();
    vault.init(&password).map_err(|e| e.to_string())
}

/// 输入主密码解锁已有保险箱
#[tauri::command]
pub fn unlock_vault(password: String, state: State<'_, AppState>) -> Result<VaultData, String> {
    let vault = state.vault.lock().unwrap();
    vault.unlock(&password).map_err(|e| e.to_string())
}

/// 锁定保险箱
#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>) {
    let vault = state.vault.lock().unwrap();
    vault.lock();
}
