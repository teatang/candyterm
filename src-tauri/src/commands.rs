//! Tauri IPC 命令定义与处理函数

use crate::sftp::client::SftpService;
use crate::sftp::{SftpFileItem, TransferProgress};
use crate::ssh::config::HostConfig;
use crate::ssh::session::SshSessionManager;
use crate::vault::storage::VaultManager;
use crate::vault::VaultData;
use std::path::Path;
use std::sync::Mutex;
use tauri::ipc::Channel;
use tauri::State;

/// Tauri 托管的全局状态
pub struct AppState {
    pub vault: Mutex<VaultManager>,
    pub ssh_sessions: Mutex<SshSessionManager>,
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

/// 保存或更新主机配置至加密保险箱
#[tauri::command]
pub fn save_host(host: HostConfig, state: State<'_, AppState>) -> Result<(), String> {
    host.validate()?;

    let vault = state.vault.lock().unwrap();
    let mut data = vault.read_data().map_err(|e| e.to_string())?;

    let mut hosts: Vec<HostConfig> =
        serde_json::from_str(&data.hosts_payload).unwrap_or_default();

    if let Some(pos) = hosts.iter().position(|h| h.id == host.id) {
        hosts[pos] = host;
    } else {
        hosts.push(host);
    }

    data.hosts_payload = serde_json::to_string(&hosts).map_err(|e| e.to_string())?;
    vault.save_data(&data).map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取当前保险箱中保存的主机列表
#[tauri::command]
pub fn get_hosts(state: State<'_, AppState>) -> Result<Vec<HostConfig>, String> {
    let vault = state.vault.lock().unwrap();
    let data = vault.read_data().map_err(|e| e.to_string())?;
    let hosts: Vec<HostConfig> =
        serde_json::from_str(&data.hosts_payload).unwrap_or_default();
    Ok(hosts)
}

/// 删除指定主机配置
#[tauri::command]
pub fn delete_host(host_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let vault = state.vault.lock().unwrap();
    let mut data = vault.read_data().map_err(|e| e.to_string())?;

    let mut hosts: Vec<HostConfig> =
        serde_json::from_str(&data.hosts_payload).unwrap_or_default();

    hosts.retain(|h| h.id != host_id);
    data.hosts_payload = serde_json::to_string(&hosts).map_err(|e| e.to_string())?;
    vault.save_data(&data).map_err(|e| e.to_string())?;
    Ok(())
}

/// 建立 SSH 终端会话并通过 Tauri 2.x Channel 流式传输输出
#[tauri::command]
pub fn connect_ssh(
    session_id: String,
    host_id: String,
    output_channel: Channel<Vec<u8>>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let host = {
        let vault = state.vault.lock().unwrap();
        let data = vault.read_data().map_err(|e| e.to_string())?;
        let hosts: Vec<HostConfig> =
            serde_json::from_str(&data.hosts_payload).unwrap_or_default();
        hosts
            .into_iter()
            .find(|h| h.id == host_id)
            .ok_or_else(|| format!("未找到 ID 为 {} 的主机配置", host_id))?
    };

    let ssh_manager = state.ssh_sessions.lock().unwrap();
    ssh_manager
        .create_session(session_id, &host, output_channel)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 向 SSH 终端发送按键输入数据
#[tauri::command]
pub async fn send_terminal_input(
    session_id: String,
    data: Vec<u8>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let ssh_manager = {
        let guard = state.ssh_sessions.lock().unwrap();
        guard.clone()
    };
    ssh_manager
        .send_input(&session_id, data)
        .await
        .map_err(|e| e.to_string())
}

/// 调整伪终端 PTY 窗口尺寸
#[tauri::command]
pub fn resize_pty(
    session_id: String,
    cols: u32,
    rows: u32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let ssh_manager = state.ssh_sessions.lock().unwrap();
    ssh_manager
        .resize_pty(&session_id, cols, rows)
        .map_err(|e| e.to_string())
}

/// 断开指定的 SSH 终端会话
#[tauri::command]
pub fn disconnect_ssh(session_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let ssh_manager = state.ssh_sessions.lock().unwrap();
    ssh_manager
        .disconnect(&session_id)
        .map_err(|e| e.to_string())
}

/// SFTP 获取目录文件项列表
#[tauri::command]
pub fn sftp_list_dir(path: String) -> Result<Vec<SftpFileItem>, String> {
    let target_path = if path.trim().is_empty() {
        Path::new(".")
    } else {
        Path::new(&path)
    };
    SftpService::list_local_directory(target_path).map_err(|e| e.to_string())
}

/// SFTP 流式上传文件
#[tauri::command]
pub fn sftp_upload_file(
    local_path: String,
    remote_path: String,
    transfer_id: String,
    progress_channel: Channel<TransferProgress>,
) -> Result<(), String> {
    SftpService::transfer_stream(
        Path::new(&local_path),
        Path::new(&remote_path),
        &transfer_id,
        |progress| {
            let _ = progress_channel.send(progress);
        },
    )
    .map_err(|e| e.to_string())
}
