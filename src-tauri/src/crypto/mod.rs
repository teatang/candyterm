//! 加密核心模块，封装 Argon2id 密钥派生与 AES-256-GCM 认证加密

pub mod cipher;
pub mod kdf;

use thiserror::Error;

/// 加密与解密过程中的错误类型
#[derive(Error, Debug, PartialEq, Eq)]
pub enum CryptoError {
    #[error("Argon2id 密钥派生失败: {0}")]
    KdfFailed(String),
    #[error("AES-256-GCM 数据加密失败")]
    EncryptionFailed,
    #[error("AES-256-GCM 数据解密失败或认证标签无效")]
    DecryptionFailed,
    #[error("密文长度不足，缺少有效 Nonce 或 Tag")]
    InvalidCiphertextLength,
}
