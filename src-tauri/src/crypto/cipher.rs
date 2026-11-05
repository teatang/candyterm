//! 基于 AES-256-GCM 的认证加密与解密实现

use crate::crypto::CryptoError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{rngs::OsRng, RngCore};

/// 标准 Nonce 长度（12 字节 / 96 位）
pub const NONCE_SIZE: usize = 12;
/// GCM 认证标签长度（16 字节 / 128 位）
pub const TAG_SIZE: usize = 16;

/// 使用 256 位密钥对明文进行 AES-256-GCM 加密
///
/// # 返回密文格式
/// `[12 字节随机 Nonce] + [密文正文 + 16 字节认证 Tag]`
pub fn encrypt_data(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::EncryptionFailed)?;

    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CryptoError::EncryptionFailed)?;

    // 拼接 Nonce 与密文
    let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// 使用 256 位密钥解密 AES-256-GCM 封装的密文
pub fn decrypt_data(key: &[u8; 32], payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if payload.len() < NONCE_SIZE + TAG_SIZE {
        return Err(CryptoError::InvalidCiphertextLength);
    }

    let (nonce_bytes, ciphertext) = payload.split_at(NONCE_SIZE);
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::DecryptionFailed)?;
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 验证加密与解密回环正确性
    #[test]
    fn test_aes_gcm_encrypt_decrypt_roundtrip() {
        let key = [9u8; 32];
        let original_data = b"Hello, Candyterm SSH Client!";
        let encrypted = encrypt_data(&key, original_data).expect("加密成功");
        assert_ne!(encrypted, original_data);

        let decrypted = decrypt_data(&key, &encrypted).expect("解密成功");
        assert_eq!(decrypted, original_data);
    }

    /// 验证密钥错误时解密必须失败且不崩溃
    #[test]
    fn test_aes_gcm_wrong_key_fails() {
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];
        let original_data = b"Secret data";
        let encrypted = encrypt_data(&key1, original_data).unwrap();

        let err = decrypt_data(&key2, &encrypted).unwrap_err();
        assert_eq!(err, CryptoError::DecryptionFailed);
    }

    /// 验证密文损坏或篡改时认证标签校验失败
    #[test]
    fn test_aes_gcm_tampered_ciphertext_fails() {
        let key = [3u8; 32];
        let mut encrypted = encrypt_data(&key, b"Critical host password").unwrap();

        // 篡改密文正文的一个字节
        let last_idx = encrypted.len() - 1;
        encrypted[last_idx] ^= 0xFF;

        let err = decrypt_data(&key, &encrypted).unwrap_err();
        assert_eq!(err, CryptoError::DecryptionFailed);
    }

    /// 验证超短数据无法通过长度校验
    #[test]
    fn test_aes_gcm_invalid_length() {
        let key = [4u8; 32];
        let short_data = vec![0u8; 5];
        let err = decrypt_data(&key, &short_data).unwrap_err();
        assert_eq!(err, CryptoError::InvalidCiphertextLength);
    }
}
