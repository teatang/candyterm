//! 保险箱存储持久化与原子写入实现

use super::{VaultData, VaultError};
use crate::crypto::{cipher, kdf};
use rand::{rngs::OsRng, RngCore};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// 盐值长度（16 字节）
pub const SALT_SIZE: usize = 16;

/// 线程安全的全局保险箱状态管理器
#[derive(Clone)]
pub struct VaultManager {
    /// 保险箱文件存储物理路径
    vault_path: PathBuf,
    /// 内存中保存的当前解密密钥（锁定状态下为 None）
    active_key: Arc<RwLock<Option<[u8; 32]>>>,
}

impl VaultManager {
    /// 创建保险箱管理器实例
    pub fn new(vault_path: PathBuf) -> Self {
        Self {
            vault_path,
            active_key: Arc::new(RwLock::new(None)),
        }
    }

    /// 当前保险箱是否已存在文件
    pub fn exists(&self) -> bool {
        self.vault_path.exists()
    }

    /// 判断保险箱当前是否已在内存中解锁
    pub fn is_unlocked(&self) -> bool {
        self.active_key.read().unwrap().is_some()
    }

    /// 锁定保险箱，立即清空内存中的密钥
    pub fn lock(&self) {
        let mut key_guard = self.active_key.write().unwrap();
        if let Some(mut key) = key_guard.take() {
            // 清理内存敏感数据
            key.fill(0);
        }
    }

    /// 首次初始化保险箱，生成随机盐值与默认数据
    pub fn init(&self, master_password: &str) -> Result<VaultData, VaultError> {
        let mut salt = [0u8; SALT_SIZE];
        OsRng.fill_bytes(&mut salt);

        let key = kdf::derive_key(master_password, &salt)?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let initial_data = VaultData {
            version: 1,
            created_at: now,
            updated_at: now,
            hosts_payload: "[]".to_string(),
        };

        // 写入磁盘
        self.write_vault_atomic(&salt, &key, &initial_data)?;

        // 在内存中标记为已解锁
        *self.active_key.write().unwrap() = Some(key);

        Ok(initial_data)
    }

    /// 使用主密码解锁已有保险箱
    pub fn unlock(&self, master_password: &str) -> Result<VaultData, VaultError> {
        if !self.vault_path.exists() {
            return Err(VaultError::CorruptedFile);
        }

        let mut file = File::open(&self.vault_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        if content.len() < SALT_SIZE + cipher::NONCE_SIZE + cipher::TAG_SIZE {
            return Err(VaultError::CorruptedFile);
        }

        let (salt_bytes, encrypted_payload) = content.split_at(SALT_SIZE);
        let mut salt = [0u8; SALT_SIZE];
        salt.copy_from_slice(salt_bytes);

        // 派生密钥并尝试解密
        let key = kdf::derive_key(master_password, &salt)?;
        let decrypted_bytes = cipher::decrypt_data(&key, encrypted_payload)
            .map_err(|_| VaultError::InvalidPassword)?;

        let data: VaultData = serde_json::from_slice(&decrypted_bytes)?;

        // 解锁成功，缓存密钥
        *self.active_key.write().unwrap() = Some(key);

        Ok(data)
    }

    /// 保存更新后的数据到保险箱（要求处于已解锁状态）
    pub fn save_data(&self, data: &VaultData) -> Result<(), VaultError> {
        let key_guard = self.active_key.read().unwrap();
        let key = key_guard.as_ref().ok_or(VaultError::NotUnlocked)?;

        // 读取当前文件现存的盐值（如果已存在），否则生成新盐
        let salt = if self.vault_path.exists() {
            let mut file = File::open(&self.vault_path)?;
            let mut salt_buf = [0u8; SALT_SIZE];
            file.read_exact(&mut salt_buf)
                .map_err(|_| VaultError::CorruptedFile)?;
            salt_buf
        } else {
            let mut s = [0u8; SALT_SIZE];
            OsRng.fill_bytes(&mut s);
            s
        };

        self.write_vault_atomic(&salt, key, data)?;
        Ok(())
    }

    /// 从当前已解锁的保险箱读取数据
    pub fn read_data(&self) -> Result<VaultData, VaultError> {
        let key_guard = self.active_key.read().unwrap();
        let key = key_guard.as_ref().ok_or(VaultError::NotUnlocked)?;

        let mut file = File::open(&self.vault_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        if content.len() < SALT_SIZE + cipher::NONCE_SIZE + cipher::TAG_SIZE {
            return Err(VaultError::CorruptedFile);
        }

        let (_, encrypted_payload) = content.split_at(SALT_SIZE);
        let decrypted_bytes = cipher::decrypt_data(key, encrypted_payload)
            .map_err(|_| VaultError::CorruptedFile)?;

        let data: VaultData = serde_json::from_slice(&decrypted_bytes)?;
        Ok(data)
    }

    /// 原子写入磁盘：先写入同目录下的临时文件，然后通过 rename 原子替换
    fn write_vault_atomic(
        &self,
        salt: &[u8; SALT_SIZE],
        key: &[u8; 32],
        data: &VaultData,
    ) -> Result<(), VaultError> {
        let serialized = serde_json::to_vec(data)?;
        let encrypted_payload = cipher::encrypt_data(key, &serialized)?;

        if let Some(parent) = self.vault_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let temp_path = self.vault_path.with_extension(format!("tmp.{}", OsRng.next_u64()));
        {
            let mut temp_file = File::create(&temp_path)?;
            temp_file.write_all(salt)?;
            temp_file.write_all(&encrypted_payload)?;
            temp_file.flush()?;
        }

        // 原子重命名替换
        fs::rename(&temp_path, &self.vault_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    /// 验证保险箱初始化、锁定与正常解锁流程
    #[test]
    fn test_vault_init_and_unlock_flow() {
        let dir = tempdir().unwrap();
        let vault_file = dir.path().join("vault.enc");
        let manager = VaultManager::new(vault_file);

        assert!(!manager.exists());
        assert!(!manager.is_unlocked());

        // 初始化
        let data = manager.init("Password123!").expect("初始化成功");
        assert_eq!(data.version, 1);
        assert!(manager.exists());
        assert!(manager.is_unlocked());

        // 锁定
        manager.lock();
        assert!(!manager.is_unlocked());

        // 解锁
        let unlocked_data = manager.unlock("Password123!").expect("解锁成功");
        assert_eq!(unlocked_data.version, 1);
        assert!(manager.is_unlocked());
    }

    /// 验证主密码错误时拒绝解锁并返回 InvalidPassword
    #[test]
    fn test_vault_wrong_password_returns_invalid_password() {
        let dir = tempdir().unwrap();
        let vault_file = dir.path().join("vault.enc");
        let manager = VaultManager::new(vault_file);

        manager.init("CorrectPassword").unwrap();
        manager.lock();

        let err = manager.unlock("WrongPassword").unwrap_err();
        match err {
            VaultError::InvalidPassword => (),
            other => panic!("预期 InvalidPassword，实际获得: {:?}", other),
        }
    }

    /// 验证损坏或截断的保险箱文件返回明确错误且绝不崩溃 (Review Focus 1)
    #[test]
    fn test_corrupted_vault_file_returns_error() {
        let dir = tempdir().unwrap();
        let vault_file = dir.path().join("vault.enc");
        let manager = VaultManager::new(vault_file.clone());

        // 写入一个损坏的短文件（仅 10 字节，远低于有效头部长度）
        fs::write(&vault_file, b"corrupted!").unwrap();

        let err = manager.unlock("AnyPassword").unwrap_err();
        match err {
            VaultError::CorruptedFile => (),
            other => panic!("预期 CorruptedFile，实际获得: {:?}", other),
        }

        // 写入长度足够但内容随机垃圾数据的文件
        let garbage = vec![0xEEu8; 100];
        fs::write(&vault_file, &garbage).unwrap();
        let err2 = manager.unlock("AnyPassword").unwrap_err();
        // 垃圾密文应当解密失败，转为 InvalidPassword 或 CorruptedFile
        assert!(matches!(err2, VaultError::InvalidPassword | VaultError::CorruptedFile));
    }

    /// 验证数据保存与读取持久化一致性
    #[test]
    fn test_vault_save_and_reload() {
        let dir = tempdir().unwrap();
        let vault_file = dir.path().join("vault.enc");
        let manager = VaultManager::new(vault_file);

        let mut data = manager.init("MasterPass").unwrap();
        data.hosts_payload = r#"[{"id":"h1","label":"Prod Server"}]"#.to_string();

        manager.save_data(&data).unwrap();

        // 重新读取
        let read_back = manager.read_data().unwrap();
        assert_eq!(read_back.hosts_payload, r#"[{"id":"h1","label":"Prod Server"}]"#);
    }
}
