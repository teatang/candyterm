//! 跳板机 (Jump Host / ProxyJump) 链式依赖解析与循环引用检测

use super::config::HostConfig;
use super::SshError;
use std::collections::HashSet;

/// 最大允许的跳板机链路层数
pub const MAX_JUMP_HOPS: usize = 5;

/// 解析目标主机的跳板机代理链路
///
/// 返回从第一个起始跳板机到最终目标主机的有序数组，例如：
/// `[Jump1, Jump2, TargetHost]`
///
/// 若检测到循环依赖（如 A -> B -> A），立即返回 `SshError::CircularJumpHost`
pub fn resolve_jump_chain<'a>(
    hosts: &'a [HostConfig],
    target_id: &str,
) -> Result<Vec<&'a HostConfig>, SshError> {
    let mut chain = Vec::new();
    let mut visited = HashSet::new();

    let mut current_id = target_id.to_string();

    while !current_id.is_empty() {
        if visited.contains(&current_id) {
            return Err(SshError::CircularJumpHost(format!(
                "检测到跳板机存在循环代理引用成环: {}",
                current_id
            )));
        }
        visited.insert(current_id.clone());

        let host = hosts
            .iter()
            .find(|h| h.id == current_id)
            .ok_or_else(|| SshError::HostNotFound(current_id.clone()))?;

        chain.push(host);

        if chain.len() > MAX_JUMP_HOPS {
            return Err(SshError::MaxHopsExceeded);
        }

        match &host.jump_host_id {
            Some(parent_id) if !parent_id.trim().is_empty() => {
                current_id = parent_id.clone();
            }
            _ => break,
        }
    }

    // 从最前端跳板机到最终目标的顺序逆转
    chain.reverse();
    Ok(chain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ssh::config::AuthType;

    fn make_host(id: &str, jump_id: Option<&str>) -> HostConfig {
        HostConfig {
            id: id.to_string(),
            label: format!("Host-{}", id),
            host: format!("10.0.0.{}", id),
            port: 22,
            username: "admin".to_string(),
            auth_type: AuthType::Password,
            password: Some("pwd".to_string()),
            private_key: None,
            passphrase: None,
            jump_host_id: jump_id.map(|s| s.to_string()),
        }
    }

    /// 验证无跳板机直接连接的单个解析
    #[test]
    fn test_direct_host_no_jump() {
        let hosts = vec![make_host("h1", None)];
        let chain = resolve_jump_chain(&hosts, "h1").expect("直接主机解析成功");
        assert_eq!(chain.len(), 1);
        assert_eq!(chain[0].id, "h1");
    }

    /// 验证双跳代理链解析顺序正确 (Jump -> Target)
    #[test]
    fn test_valid_two_hop_chain() {
        let hosts = vec![
            make_host("bastion", None),
            make_host("target", Some("bastion")),
        ];
        let chain = resolve_jump_chain(&hosts, "target").expect("双跳解析成功");
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].id, "bastion");
        assert_eq!(chain[1].id, "target");
    }

    /// 验证跳板机循环依赖成环检测与防死循环 (Review Focus 2)
    #[test]
    fn test_jump_host_circular_dependency_detected() {
        // A -> B 且 B -> A
        let hosts = vec![
            make_host("A", Some("B")),
            make_host("B", Some("A")),
        ];
        let err = resolve_jump_chain(&hosts, "A").unwrap_err();
        match err {
            SshError::CircularJumpHost(msg) => {
                assert!(msg.contains("A") || msg.contains("B"));
            }
            other => panic!("预期 CircularJumpHost，实际得到: {:?}", other),
        }
    }

    /// 验证超出最大跳数限制
    #[test]
    fn test_exceeding_max_hops() {
        let hosts = vec![
            make_host("h1", None),
            make_host("h2", Some("h1")),
            make_host("h3", Some("h2")),
            make_host("h4", Some("h3")),
            make_host("h5", Some("h4")),
            make_host("h6", Some("h5")),
            make_host("h7", Some("h6")),
        ];
        let err = resolve_jump_chain(&hosts, "h7").unwrap_err();
        assert_eq!(err, SshError::MaxHopsExceeded);
    }
}
