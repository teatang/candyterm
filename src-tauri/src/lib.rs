//! Candyterm 核心库模块
//! 提供 Tauri 应用生命周期管理与 IPC 路由分发。

/// 启动 Tauri 应用核心
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("运行 Candyterm Tauri 应用失败");
}

#[cfg(test)]
mod tests {
    /// 验证 Rust 后端基础测试环境运行正常
    #[test]
    fn test_backend_baseline() {
        assert_eq!(1 + 1, 2);
    }
}
