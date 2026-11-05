//! 保险箱模块，负责本地敏感凭据的安全持久化存储与会话解锁

pub mod storage;

use crate::crypto::CryptoError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 保险箱操作错误枚举
#[derive(Error, Debug)]
pub enum VaultError {
    #[error("加密核心异常: {0}")]
    Crypto(#[from] CryptoError),
    #[error("文件 I/O 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("数据序列化/反序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("主密码错误，解密失败")]
    InvalidPassword,
    #[error("保险箱尚未解锁")]
    NotUnlocked,
    #[error("保险箱已被锁定")]
    AlreadyLocked,
    #[error("保险箱数据文件损坏或校验无效")]
    CorruptedFile,
}

/// 保险箱内部持久化保存的数据模型
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct VaultData {
    /// 格式版本号
    pub version: u32,
    /// 创建时间戳
    pub created_at: u64,
    /// 更新时间戳
    pub updated_at: u64,
    /// 保存的服务器主机列表（序列化为 JSON 字符串或直接结构体）
    pub hosts_payload: String,
}
