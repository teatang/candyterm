# Candyterm 🍬

<div align="center">

![Candyterm Logo](src-tauri/icons/128x128.png)

**基于 Rust Tauri 的高性能、极简美学跨平台 SSH / SFTP 客户端桌面应用**

*High-performance, minimalist, cross-platform SSH & SFTP desktop client built with Rust and Tauri.*

[![CI / Release](https://github.com/candyterm/candyterm/actions/workflows/release.yml/badge.svg)](https://github.com/candyterm/candyterm/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tauri 2.x](https://img.shields.io/badge/Tauri-2.x-orange.svg)](https://tauri.app/)
[![Vue 3](https://img.shields.io/badge/Vue-3.x-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.77+-black.svg)](https://www.rust-lang.org/)

[简体中文](#简体中文) | [English](#english)

</div>

---

<a name="简体中文"></a>
## 简体中文

### 🌟 核心特性

- ⚡ **极速与低资源占用**：
  - 基于 **Rust Tauri 2.x** 与 **Tokio** 纯异步高并发网络运行时。
  - 使用纯 Rust 实现的 `russh` SSH 协议栈，免去 C 动态库依赖（如 libssh2 / OpenSSL），保证极佳的跨平台稳定性与毫秒级连接建立。
  - 终端基于 **xterm.js + WebGL 硬件加速渲染**，支持自动降级至 Canvas/DOM，60fps 高刷滚动不卡顿。
  - 终端流式传输采用 Tauri 2.x 零拷贝二进制通道（`Channel<Vec<u8>>`），彻底摒弃高延迟的 JSON 序列化。

- 🖥️ **现代多窗格分屏工作区 (Multi-Pane Split)**：
  - 灵感汲取自优秀的开源终端工具（如 Electerm），提供灵活的多窗格二叉树分屏（水平 / 垂直切分）。
  - 支持窗格拖拽调节比例（0.1~0.9 智能边界保护）与防抖节流（Throttle Resize）。
  - 支持**多窗格键盘广播输入**（Broadcast Input），一次敲击同时操作多台集群服务器。

- 📁 **创新混合式 SFTP 管理 (Hybrid SFTP)**：
  - **随动快速抽屉**：终端窗格右侧一键轻量呼出 SFTP 抽屉，支持将操作系统桌面的文件直接拖拽至窗格一键极速上传。
  - **独立全屏双栏管理器**：支持本地路径与远程路径双栏对比、目录面包屑导航、文件大小与修改时间实时排序。
  - **后台传输队列**：任务分块传输、多任务并发控制、进度条与传输状态统计。

- 🔒 **工业级安全保险箱 (Argon2id + AES-256-GCM)**：
  - 支持通过单一**主密码 (Master Password)** 对所有服务器连接凭据与私钥进行高强度端到端加密保护。
  - 使用 **Argon2id** 算法进行内存硬化密钥派生（支持包含 Emoji、Unicode 特殊符号的超长主密码）。
  - 密文通过 **AES-256-GCM** 进行附带认证的对称加密，配合文件原子性重命名写入（Atomic Rename），杜绝断电或崩溃损坏。
  - 锁定保险箱时内存敏感数据即刻擦除（Zeroize）。

- 🔑 **丰富的认证方式支持**：
  - 密码认证 (Password)。
  - 私钥认证 (Private Key，支持带口令 Passphrase 解密的 OpenSSH / PEM 格式私钥)。
  - SSH 代理认证 (SSH Agent，本地 ssh-agent / 1Password / GPG 代理透传)。
  - 前置跳板机链式代理 (ProxyJump / Jump Host)，支持多跳链路智能拓扑解析与循环成环引用自动检测拦截。

- 🎨 **极简美学设计与国际化**：
  - 基于 Naive UI 与 TailwindCSS 构建，视觉风格精炼现代化。
  - 完善的色彩模式切换：支持明亮模式 (Light)、暗黑模式 (Dark) 以及跟随系统 (System Follow)。
  - 内置多语言国际化 (i18n)：支持简体中文 (`zh-CN`) 与 English (`en-US`)，默认自动适配操作系统语言。

- 🛡️ **工程规范与全中文注释**：
  - 遵循严格的测试驱动开发（TDD）规范，所有核心模块均覆盖单元测试。
  - 后端 Rust 与前端 TypeScript/Vue 源码均配有详尽的全中文规范注释。

---

### 🏛️ 系统架构简图

```
+-------------------------------------------------------------------------+
|                              Candyterm UI                               |
|        (Vue 3 + Pinia + TypeScript + TailwindCSS + Naive UI)            |
|                                                                         |
|  +--------------------+  +----------------------+  +-----------------+  |
|  |   TerminalPane     |  |   SplitWorkspace     |  |   Hybrid SFTP   |  |
|  |  (xterm.js + WebGL)|  | (Binary Tree Splits) |  | (Drawer & Full) |  |
|  +--------------------+  +----------------------+  +-----------------+  |
|           ^                         ^                       ^           |
+-----------|-------------------------|-----------------------|-----------+
            | Tauri IPC Commands      | Binary Channel Stream |
+-----------v-------------------------v-----------------------v-----------+
|                          Tauri 2.x Rust Core                            |
|                                                                         |
|  +--------------------+  +----------------------+  +-----------------+  |
|  |    Vault Storage   |  |   SSH Session Pool   |  |   SFTP Engine   |  |
|  | Argon2id + AES-GCM |  | russh + ProxyJump    |  | Tokio Chunked   |  |
|  +--------------------+  +----------------------+  +-----------------+  |
|                                                                         |
|                         Tokio Async Runtime                             |
+-------------------------------------------------------------------------+
```

---

### 🚀 快速开始

#### 开发环境要求
- **Node.js**: >= 20.0.0
- **pnpm**: >= 9.0.0
- **Rust**: >= 1.77.0 (Cargo 与 Rustup)
- **Linux 附加依赖**（如在 Ubuntu 上编译）：
  ```bash
  sudo apt-get update && sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libsoup-3.0-dev libssl-dev
  ```

#### 安装依赖与本地开发
```bash
# 1. 克隆代码仓库
git clone https://github.com/candyterm/candyterm.git
cd candyterm

# 2. 安装前端依赖
pnpm install

# 3. 启动桌面端开发调试模式 (自动唤起 Tauri 窗口与前端热重载)
pnpm tauri dev
```

#### 运行单元测试
```bash
# 运行前端 Vitest 单元测试 (含 Stores, SFTP, I18n, Theme, Modals 等)
pnpm test

# 运行后端 Rust 单元测试 (含 Argon2id, AES-GCM, Vault, ProxyJump, SFTP 等)
cargo test --manifest-path src-tauri/Cargo.toml
```

#### 构建发行安装包
```bash
# 构建生产环境桌面安装程序 (.dmg / .app, .deb / .AppImage, .msi / .exe)
pnpm tauri build
```

---

### 📦 自动构建与发布流水线 (CI/CD)

项目配置了完整的 GitHub Actions 自动化工作流（`.github/workflows/release.yml`）：
- 触发条件：推送带有版本号的 Git Tag（如 `git tag -a v1.0.0 -m "Release v1.0.0" && git push origin v1.0.0`）。
- 跨平台构建矩阵：`macos-latest` (包含 Apple Silicon `aarch64` 与 Intel `x86_64`)、`ubuntu-22.04`、`windows-latest`。
- 质量门禁：每次自动构建前均严格执行 `cargo test` 与 `pnpm test run`，测试通过后方执行打包并自动生成 GitHub Release 草稿。

---

<a name="english"></a>
## English

### 🌟 Key Features

- ⚡ **Blazing Fast & Lightweight**:
  - Built on **Rust Tauri 2.x** and **Tokio** asynchronous non-blocking runtime.
  - Pure Rust SSH stack via `russh`, eliminating heavy C dynamic library dependencies.
  - High-performance terminal emulation powered by **xterm.js + WebGL hardware acceleration** (60fps smooth scrolling).
  - Low-latency binary data streaming between Rust and the frontend via Tauri 2.x `Channel<Vec<u8>>`.

- 🖥️ **Flexible Multi-Pane Workspace**:
  - Binary tree split layout supporting arbitrary horizontal and vertical splits.
  - Drag-to-resize panes with ratio boundary protection (0.1–0.9) and resize throttling.
  - Unified **Keyboard Broadcast Input** to command multiple remote servers simultaneously.

- 📁 **Hybrid SFTP File Manager**:
  - **Pane-Attached Slide-Out Drawer**: Instant file transfers right next to your active shell with desktop drag-and-drop upload.
  - **Dual-Pane Full View**: Side-by-side local and remote navigation with breadcrumbs and sorting.
  - **Background Transfer Queue**: Chunked transfers, progress tracking, and concurrency controls.

- 🔒 **Enterprise-Grade Security Vault**:
  - Master Password protection for all saved servers and credentials.
  - **Argon2id** memory-hard key derivation supporting complex Unicode/Emoji passphrases.
  - **AES-256-GCM** authenticated encryption with atomic file rename updates (`vault.enc`).
  - In-memory data zeroization on vault lock.

- 🔑 **Comprehensive Authentication Methods**:
  - Password, Encrypted Private Key (with passphrase), SSH Agent, and Multi-hop ProxyJump with circular dependency detection.

- 🎨 **Minimalist Design & Internationalization**:
  - Crafted with Naive UI and TailwindCSS.
  - Seamless Dark, Light, and System-follow themes.
  - Built-in Chinese (`zh-CN`) and English (`en-US`) support, defaulting to system locale.

- 🧪 **Test-Driven & Full Bilingual Documentation**:
  - 100% test pass rate across Rust (`cargo test`) and Vue (`vitest`).
  - Full Chinese comment coverage across all source files.

---

### 🛠️ Development & Building

```bash
# Install dependencies
pnpm install

# Start local Tauri development environment
pnpm tauri dev

# Run full unit tests
pnpm test
cargo test --manifest-path src-tauri/Cargo.toml

# Build production binaries
pnpm tauri build
```

---

### 📄 License

Candyterm is open-sourced under the [MIT License](LICENSE).
