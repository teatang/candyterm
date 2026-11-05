//! SSH 核心网络模块，提供主机配置、跳板机链式解析与异步终端会话管理

pub mod config;
pub mod jump;
pub mod session;

use thiserror::Error;

/// SSH 相关操作错误枚举
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SshError {
    #[error("目标主机未找到: {0}")]
    HostNotFound(String),
    #[error("跳板机存在循环引用依赖: {0}")]
    CircularJumpHost(String),
    #[error("跳板机代理链路超出最大跳数限制 (最大支持 5 跳)")]
    MaxHopsExceeded,
    #[error("认证配置无效: {0}")]
    InvalidAuth(String),
    #[error("SSH 网络连接失败: {0}")]
    ConnectionFailed(String),
    #[error("SSH 会话已断开或不存在: {0}")]
    SessionNotFound(String),
    #[error("IO 错误: {0}")]
    Io(String),
}
