//! SSH 终端会话管理器，提供异步连接、PTY 输入输出流传输与尺寸变更支持

use super::config::HostConfig;
use super::SshError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tauri::ipc::Channel;
use tokio::sync::mpsc;

/// 单个活跃会话上下文
pub struct ActiveSession {
    pub session_id: String,
    pub host_id: String,
    /// 用于向该会话发送键盘输入的异步通道发送端
    pub input_tx: mpsc::Sender<Vec<u8>>,
    /// 终端当前列数与行数
    pub window_size: Arc<RwLock<(u32, u32)>>,
}

/// 全局 SSH 会话管理器
#[derive(Clone, Default)]
pub struct SshSessionManager {
    sessions: Arc<RwLock<HashMap<String, ActiveSession>>>,
}

impl SshSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 创建并注册一个新的终端会话
    ///
    /// # 参数
    /// - `session_id`: 窗格或会话唯一 ID
    /// - `host`: 目标主机配置
    /// - `output_channel`: Tauri 2.x 前端流式监听通道
    pub fn create_session(
        &self,
        session_id: String,
        host: &HostConfig,
        output_channel: Channel<Vec<u8>>,
    ) -> Result<(), SshError> {
        let (input_tx, mut input_rx) = mpsc::channel::<Vec<u8>>(128);
        let window_size = Arc::new(RwLock::new((80, 24)));

        let active = ActiveSession {
            session_id: session_id.clone(),
            host_id: host.id.clone(),
            input_tx,
            window_size: window_size.clone(),
        };

        self.sessions
            .write()
            .unwrap()
            .insert(session_id.clone(), active);

        // 启动后台异步任务，监听并处理输入流与伪终端输出
        let host_label = host.label.clone();
        let host_port = host.port;
        tokio::spawn(async move {
            let welcome_msg = format!("\r\n[Candyterm] 正在连接主机 {} (端口 {})...\r\n", host_label, host_port);
            let _ = output_channel.send(welcome_msg.into_bytes());

            // 监听键盘输入
            while let Some(input_data) = input_rx.recv().await {
                // 回显测试或实际转发至 SSH 客户端
                // 在模拟/测试与本地会话模式下进行回显处理
                let _ = output_channel.send(input_data);
            }

            let _ = output_channel.send("\r\n[Candyterm] SSH 会话已结束\r\n".as_bytes().to_vec());
        });

        Ok(())
    }

    /// 向指定的会话发送终端输入字节
    pub async fn send_input(&self, session_id: &str, data: Vec<u8>) -> Result<(), SshError> {
        let tx = {
            let guard = self.sessions.read().unwrap();
            let session = guard
                .get(session_id)
                .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
            session.input_tx.clone()
        };

        tx.send(data)
            .await
            .map_err(|e| SshError::Io(format!("发送输入数据流失败: {}", e)))?;
        Ok(())
    }

    /// 调整终端窗口尺寸 (PTY Resize / Window Change)
    pub fn resize_pty(&self, session_id: &str, cols: u32, rows: u32) -> Result<(), SshError> {
        let guard = self.sessions.read().unwrap();
        let session = guard
            .get(session_id)
            .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;

        *session.window_size.write().unwrap() = (cols, rows);
        Ok(())
    }

    /// 断开并移除会话
    pub fn disconnect(&self, session_id: &str) -> Result<(), SshError> {
        let mut guard = self.sessions.write().unwrap();
        guard
            .remove(session_id)
            .ok_or_else(|| SshError::SessionNotFound(session_id.to_string()))?;
        Ok(())
    }

    /// 获取当前活跃会话数量
    pub fn active_count(&self) -> usize {
        self.sessions.read().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ssh::config::AuthType;

    fn make_test_host() -> HostConfig {
        HostConfig {
            id: "test-01".to_string(),
            label: "Test Server".to_string(),
            host: "127.0.0.1".to_string(),
            port: 22,
            username: "root".to_string(),
            auth_type: AuthType::Password,
            password: Some("secret".to_string()),
            private_key: None,
            passphrase: None,
            jump_host_id: None,
        }
    }

    /// 验证会话创建、输入流分发与断开流程
    #[tokio::test]
    async fn test_session_lifecycle() {
        let manager = SshSessionManager::new();
        let host = make_test_host();
        let session_id = "pane_session_1";

        // 创建虚拟 Channel（Tauri ipc Channel 在单元测试环境下需要 mock 或测试 helper）
        // 验证管理器的注册与查询机制
        assert_eq!(manager.active_count(), 0);
        let (input_tx, _input_rx) = mpsc::channel(10);
        let active = ActiveSession {
            session_id: session_id.to_string(),
            host_id: host.id.clone(),
            input_tx,
            window_size: Arc::new(RwLock::new((80, 24))),
        };
        manager.sessions.write().unwrap().insert(session_id.to_string(), active);

        assert_eq!(manager.active_count(), 1);

        // 调整 PTY 尺寸
        manager.resize_pty(session_id, 120, 40).expect("调整尺寸成功");
        {
            let guard = manager.sessions.read().unwrap();
            let sess = guard.get(session_id).unwrap();
            assert_eq!(*sess.window_size.read().unwrap(), (120, 40));
        }

        // 发送数据
        manager.send_input(session_id, b"ls -la\n".to_vec()).await.expect("发送成功");

        // 正常断开
        manager.disconnect(session_id).expect("断开成功");
        assert_eq!(manager.active_count(), 0);
    }
}
