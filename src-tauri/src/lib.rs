//! Candyterm 核心库模块
//! 提供 Tauri 应用生命周期管理与 IPC 路由分发。

pub mod commands;
pub mod crypto;
pub mod sftp;
pub mod ssh;
pub mod vault;

use commands::*;
use ssh::session::SshSessionManager;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;
use vault::storage::VaultManager;

/// 启动 Tauri 应用核心
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| PathBuf::from("./data"));
            let vault_path = app_data_dir.join("vault.enc");

            let vault_manager = VaultManager::new(vault_path);
            let ssh_sessions = SshSessionManager::new();

            app.manage(AppState {
                vault: Mutex::new(vault_manager),
                ssh_sessions: Mutex::new(ssh_sessions),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            is_vault_initialized,
            is_vault_unlocked,
            init_vault,
            unlock_vault,
            lock_vault,
            save_host,
            get_hosts,
            delete_host,
            connect_ssh,
            send_terminal_input,
            resize_pty,
            disconnect_ssh,
            sftp_list_dir,
            sftp_upload_file
        ])
        .run(tauri::generate_context!())
        .expect("运行 Candyterm Tauri 应用失败");
}

#[cfg(test)]
mod tests {
    /// 验证 Rust 后端基础测试环境运行正常
    #[test]
    fn test_backend_baseline() {
        assert_eq!(1 + 1, 2);
    }
}
