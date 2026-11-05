//! 主机连接配置与认证信息数据模型

use serde::{Deserialize, Serialize};

/// 认证方式枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthType {
    /// 密码直接认证
    Password,
    /// 私钥文件/文本认证
    PrivateKey,
    /// 系统 SSH Agent 转发认证
    SshAgent,
}

/// 主机连接配置结构体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostConfig {
    /// 唯一标识 ID
    pub id: String,
    /// 会话显示名称（如：生产服务器-01）
    pub label: String,
    /// 目标服务器 IP 或域名
    pub host: String,
    /// SSH 服务端口（默认 22）
    pub port: u16,
    /// 登录用户名
    pub username: String,
    /// 认证类型
    pub auth_type: AuthType,
    /// 登录密码（密码认证时使用）
    pub password: Option<String>,
    /// 私钥文件路径或私钥字符串（私钥认证时使用）
    pub private_key: Option<String>,
    /// 私钥密码口令 Passphrase（可选）
    pub passphrase: Option<String>,
    /// 前置跳板机主机 ID（可选，ProxyJump）
    pub jump_host_id: Option<String>,
}

impl HostConfig {
    /// 校验主机配置基本合法性
    pub fn validate(&self) -> Result<(), String> {
        if self.host.trim().is_empty() {
            return Err("主机地址不得为空".to_string());
        }
        if self.username.trim().is_empty() {
            return Err("用户名不得为空".to_string());
        }
        if self.port == 0 {
            return Err("端口号必须在 1-65535 之间".to_string());
        }
        match self.auth_type {
            AuthType::Password => {
                if self.password.as_deref().unwrap_or("").is_empty() {
                    return Err("密码认证方式必须提供非空密码".to_string());
                }
            }
            AuthType::PrivateKey => {
                if self.private_key.as_deref().unwrap_or("").trim().is_empty() {
                    return Err("密钥认证方式必须提供非空私钥路径或内容".to_string());
                }
            }
            AuthType::SshAgent => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 验证主机配置校验逻辑
    #[test]
    fn test_host_config_validation() {
        let valid_host = HostConfig {
            id: "h1".to_string(),
            label: "Prod-01".to_string(),
            host: "192.168.1.100".to_string(),
            port: 22,
            username: "root".to_string(),
            auth_type: AuthType::Password,
            password: Some("secret123".to_string()),
            private_key: None,
            passphrase: None,
            jump_host_id: None,
        };
        assert!(valid_host.validate().is_ok());

        let invalid_empty_host = HostConfig {
            host: "".to_string(),
            ..valid_host.clone()
        };
        assert!(invalid_empty_host.validate().is_err());

        let invalid_empty_password = HostConfig {
            password: Some("".to_string()),
            ..valid_host.clone()
        };
        assert!(invalid_empty_password.validate().is_err());
    }
}
