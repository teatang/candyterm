//! 基于 Argon2id 的主密码密钥派生 (KDF) 实现

use crate::crypto::CryptoError;
use argon2::Argon2;

/// 使用 Argon2id 从主密码和 16 字节盐值派生出 256 位 (32 字节) 对称加密密钥
///
/// # 参数
/// - `password`: 用户输入的主密码（支持任意 UTF-8 字符）
/// - `salt`: 16 字节随机盐
///
/// # 返回
/// 派生出的 32 字节对称密钥数组
pub fn derive_key(password: &str, salt: &[u8; 16]) -> Result<[u8; 32], CryptoError> {
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| CryptoError::KdfFailed(e.to_string()))?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 验证正常密码派生的一致性与确定性
    #[test]
    fn test_argon2_derive_key_deterministic() {
        let salt = [42u8; 16];
        let pwd = "MySecureMasterPassword123!";
        let key1 = derive_key(pwd, &salt).expect("派生密钥成功");
        let key2 = derive_key(pwd, &salt).expect("二次派生密钥成功");
        assert_eq!(key1, key2);
        assert_ne!(key1, [0u8; 32]);
    }

    /// 验证不同盐值产生不同密钥
    #[test]
    fn test_argon2_different_salts() {
        let salt1 = [1u8; 16];
        let salt2 = [2u8; 16];
        let pwd = "SamePassword";
        let key1 = derive_key(pwd, &salt1).unwrap();
        let key2 = derive_key(pwd, &salt2).unwrap();
        assert_ne!(key1, key2);
    }

    /// 验证复杂 Unicode、Emoji 及特殊符号密码派生的稳定性 (Review Focus 3)
    #[test]
    fn test_master_password_special_characters_argon2() {
        let salt = [77u8; 16];
        let unicode_pwd = "🔐Candyterm!密码#€£中英结合🚀~`@#$%^&*()_+=-{}[]:;\"'|<>,.?/";
        let key = derive_key(unicode_pwd, &salt).expect("特殊字符与Emoji密码派生成功");
        assert_ne!(key, [0u8; 32]);
        assert_eq!(key.len(), 32);
    }
}
