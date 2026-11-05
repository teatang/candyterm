//! SFTP 文件传输与远程目录管理模块

pub mod client;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// SFTP 操作错误枚举
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SftpError {
    #[error("SFTP 路径不存在: {0}")]
    PathNotFound(String),
    #[error("SFTP 权限不足: {0}")]
    PermissionDenied(String),
    #[error("SFTP 连接断开或会话不存在: {0}")]
    SessionDisconnected(String),
    #[error("文件 I/O 错误: {0}")]
    Io(String),
    #[error("文件已被占用或传输中断")]
    TransferInterrupted,
}

/// 远程文件元数据信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SftpFileItem {
    /// 文件或目录名
    pub name: String,
    /// 完整绝对路径
    pub path: String,
    /// 是否为目录
    pub is_dir: bool,
    /// 文件大小（字节）
    pub size: u64,
    /// Unix 文件权限掩码（如 0o755）
    pub permissions: u32,
    /// 最后修改时间戳
    pub modified_at: u64,
}

/// 文件传输进度更新数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferProgress {
    /// 传输任务唯一 ID
    pub transfer_id: String,
    /// 当前已传输字节数
    pub transferred_bytes: u64,
    /// 总字节数
    pub total_bytes: u64,
    /// 是否已完成
    pub is_finished: bool,
    /// 是否发生错误
    pub error: Option<String>,
}
