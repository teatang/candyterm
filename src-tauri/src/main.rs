// 阻止在 Windows Release 模式下生成额外的控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Candyterm 二进制入口主程序

fn main() {
    candyterm_lib::run();
}
