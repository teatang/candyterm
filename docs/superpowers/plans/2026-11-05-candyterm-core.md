# Candyterm 核心功能实现计划 (Implementation Plan)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 基于 Tauri 2.x + Rust 异步底层与 Vue 3 + TypeScript + TailwindCSS + Naive UI 构建高性能、极简美观的桌面 SSH/SFTP 客户端，支持多窗格分屏、混合模式 SFTP、主密码加密保险箱与多认证方式。

**Architecture:** 后端采用 Rust Tokio 异步驱动 SSH/SFTP 与 Argon2id/AES-256-GCM 本地加密保险箱，利用 Tauri 2.x 二进制通道向前端流式传输 PTY 终端数据；前端采用 Vue 3 + Pinia + TailwindCSS + Naive UI + xterm.js (WebGL 硬件加速)，实现现代工作区多窗格分屏与混合 SFTP 交互。

**Tech Stack:** Tauri 2.x, Rust (edition 2021, Tokio, russh, argon2, aes-gcm), Vue 3, TypeScript, TailwindCSS, Naive UI, Pinia, pnpm, xterm.js + WebGL addon, vue-i18n, Vitest, cargo test.

**Spec:** `docs/superpowers/specs/2026-11-05-candyterm-design.md`

## Global Constraints

- 桌面架构：Tauri 2.x
- 后端语言：Rust 2021 edition，Tokio 异步运行时，纯 Rust 异步 SSH 引擎 (`russh`)，`argon2` + `aes-gcm`
- 前端技术：Vue 3 (Composition API `<script setup lang="ts">`)，TypeScript 严格模式，`pnpm` 包管理，Pinia 状态管理，TailwindCSS 样式，Naive UI 组件库，`xterm` + WebGL
- 代码注释：所有代码（Rust 与 TypeScript/Vue）均使用规范的完整中文注释
- 测试质量：核心功能全量单元测试覆盖，每次代码修改后必须执行 `cargo test` 与 `pnpm test` 并全部通过
- 国际化与主题：支持中英双语（默认系统语言），支持亮色/暗色/跟随系统
- 自动化：新功能同步自动更新 `README.md`，配置 GitHub Actions 跨平台构建与 Release 发布

## Review Focus

1. **凭据保险箱损坏/篡改时的容错处理**：若 `vault.enc` 文件被截断、损坏或认证标签不匹配，解密流程必须返回明确的错误类型，严禁 Panic 或导致程序崩溃。（归属于 Task 3，测试：`test_corrupted_vault_file_returns_error`）
2. **跳板机（Jump Host）循环依赖检测**：若主机 A 指定代理为主机 B，主机 B 又指向主机 A，解析连接链时必须主动检测成环并报错，防止无限递归或栈溢出。（归属于 Task 4，测试：`test_jump_host_circular_dependency_detected`）
3. **主密码特殊字符与多语言编码安全**：主密码包含特殊符号、Emoji 或超长 Unicode 时，Argon2id 派生需保持稳定的 UTF-8 字节处理，不得发生截断或编码转换异常。（归属于 Task 3，测试：`test_master_password_special_characters_argon2`）
4. **终端分屏高频拖拽 Resize 稳定性**：窗口或窗格在被高频拖拽调整大小时，`xterm.fit()` 与 PTY `window_change` 必须具备节流保护，防止零宽度或高频系统调用崩溃。（归属于 Task 5，测试：`test_split_pane_resize_throttling`）
5. **SFTP 零字节文件与断连边界处理**：上传/下载空文件（0-byte）以及网络意外中断时，传输队列必须正确释放资源并标记失败，不得悬挂任务。（归属于 Task 4 & Task 6，测试：`test_sftp_empty_file_and_error_handling`）

---

### Task 1: 项目工程初始化与双端测试脚手架构建 (Project Scaffolding & Toolchain Setup)

**Files:**
- Create: `package.json`
- Create: `pnpm-workspace.yaml`
- Create: `vite.config.ts`
- Create: `tsconfig.json`
- Create: `tailwind.config.js`
- Create: `postcss.config.js`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`
- Create: `vitest.config.ts`
- Create: `tests/frontend/setup.test.ts`

**Interfaces:**
- Consumes: 系统 Rust 工具链 (cargo)、Node.js (>=18)、pnpm
- Produces: 完整的 Tauri 2.x + Vue 3 + TailwindCSS + Naive UI 项目基架，以及 `pnpm test` 和 `cargo test` 运行环境

- [ ] **Step 1: 创建前端工程配置文件与安装依赖**
  编写 `package.json`（配置 Vue 3、Pinia、Naive UI、TailwindCSS、vue-i18n、xterm、vitest 等依赖），`tsconfig.json`，`tailwind.config.js`，`vite.config.ts`。
  运行：`pnpm install`

- [ ] **Step 2: 创建 Tauri 2.x Rust 后端脚手架配置**
  编写 `src-tauri/Cargo.toml`（包含 `tauri 2`、`tokio`、`serde`、`serde_json`、`argon2`、`aes-gcm`、`russh`、`anyhow` 等依赖），`src-tauri/tauri.conf.json`，`src-tauri/src/lib.rs`，`src-tauri/src/main.rs`。

- [ ] **Step 3: 编写基准验证单元测试**
  在 `tests/frontend/setup.test.ts` 编写验证前端测试环境就绪的测试用例：
  ```ts
  import { describe, it, expect } from 'vitest'
  describe('前端基础设施测试', () => {
    it('基础测试环境正常运行', () => {
      expect(true).toBe(true)
    })
  })
  ```
  在 `src-tauri/src/lib.rs` 编写 Rust 基准单元测试：
  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      fn test_backend_baseline() {
          assert_eq!(1 + 1, 2);
      }
  }
  ```

- [ ] **Step 4: 执行全量单元测试与编译检查**
  运行：`cargo test`
  预期：PASS (Rust 测试全部通过)
  运行：`pnpm test run`
  预期：PASS (Vitest 测试全部通过)

- [ ] **Step 5: 提交基架代码**
  ```bash
  git add package.json pnpm-lock.yaml vite.config.ts tsconfig.json tailwind.config.js postcss.config.js src-tauri/ tests/ vitest.config.ts
  git commit -m "chore: initialize tauri vue3 tailwindcss and test scaffolding"
  ```

---

### Task 2: 国际化 (i18n) 与深浅色主题系统 (i18n & Theme System)

**Files:**
- Create: `src/locales/zh-CN.json`
- Create: `src/locales/en-US.json`
- Create: `src/locales/index.ts`
- Create: `src/stores/theme.ts`
- Create: `src/stores/i18n.ts`
- Create: `tests/frontend/theme.test.ts`
- Create: `tests/frontend/i18n.test.ts`

**Interfaces:**
- Consumes: Task 1 的 Pinia 与 Vue 依赖
- Produces:
  - `useThemeStore()`: `{ mode: 'light' | 'dark' | 'auto', isDark: ComputedRef<boolean>, setThemeMode(mode: 'light' | 'dark' | 'auto') }`
  - `useI18nStore()`: `{ locale: 'zh-CN' | 'en-US', setLocale(lang: 'zh-CN' | 'en-US') }`
  - 全局中英文对照字典与系统语言自动嗅探

- [ ] **Step 1: 编写主题与国际化的失败测试**
  在 `tests/frontend/theme.test.ts` 编写主题切换测试（系统跟随、暗色、亮色），在 `tests/frontend/i18n.test.ts` 编写文案键值完整性校验（确保 zh-CN 与 en-US 拥有完全一致的 Key）。

- [ ] **Step 2: 运行测试验证失败**
  运行：`pnpm test run tests/frontend/theme.test.ts tests/frontend/i18n.test.ts`
  预期：FAIL（文件或模块未定义）

- [ ] **Step 3: 实现中英文词典与 Store 模块**
  实现 `zh-CN.json` 与 `en-US.json`，编写 `src/locales/index.ts` 自动读取 `navigator.language`，编写 `src/stores/theme.ts` 与 `src/stores/i18n.ts`，支持持久化至 `localStorage`。代码包含完整中文注释。

- [ ] **Step 4: 运行测试验证通过**
  运行：`pnpm test run`
  预期：PASS
  运行：`cargo test`
  预期：PASS

- [ ] **Step 5: 提交主题与国际化代码**
  ```bash
  git add src/locales/ src/stores/ tests/frontend/
  git commit -m "feat: implement i18n and dark light theme management with tests"
  ```

---

### Task 3: Rust 核心 - 主密码加密保险箱模块 (Master Password Vault & Crypto)

**Files:**
- Create: `src-tauri/src/crypto/mod.rs`
- Create: `src-tauri/src/crypto/kdf.rs`
- Create: `src-tauri/src/crypto/cipher.rs`
- Create: `src-tauri/src/vault/mod.rs`
- Create: `src-tauri/src/vault/storage.rs`
- Modify: `src-tauri/src/lib.rs`
- Test: `src-tauri/src/crypto/kdf.rs` (内联测试)
- Test: `src-tauri/src/crypto/cipher.rs` (内联测试)
- Test: `src-tauri/src/vault/storage.rs` (内联测试)

**Interfaces:**
- Consumes: Task 1 的 `argon2`、`aes-gcm`、`tokio`
- Produces:
  - `derive_key(password: &str, salt: &[u8; 16]) -> Result<[u8; 32], CryptoError>`
  - `encrypt_data(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError>`
  - `decrypt_data(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError>`
  - `VaultManager`: `{ init(master_pwd: &str), unlock(master_pwd: &str), lock(), is_unlocked() -> bool, save_vault(data: &VaultData), read_vault() -> Result<VaultData> }`
  - Tauri 命令：`init_vault`, `unlock_vault`, `lock_vault`, `is_vault_unlocked`

- [ ] **Step 1: 编写加密与保险箱失败单元测试**
  在 `kdf.rs` 编写 Argon2id 派生测试，包含特殊符号与 Unicode 密码（覆盖 Review Focus 3）；
  在 `cipher.rs` 编写 AES-256-GCM 加密解密回环测试；
  在 `storage.rs` 编写原子写入、错误密码失败测试以及损坏文件容错测试（覆盖 Review Focus 1：`test_corrupted_vault_file_returns_error`）。

- [ ] **Step 2: 运行 Rust 测试验证失败**
  运行：`cargo test --package candyterm-lib crypto vault`
  预期：FAIL

- [ ] **Step 3: 实现 KDF、加密算法与 VaultManager**
  - 使用 `argon2::Argon2` 生成 32 字节密钥。
  - 使用 `aes_gcm::Aes256Gcm` 与随机 12 字节 Nonce 进行认证加密，密文格式为 `[12 字节 Nonce] + [密文 + 16 字节 Tag]`。
  - 实现基于临时文件 + 原子重命名（Atomic Rename）的文件持久化。
  - 在 `src-tauri/src/lib.rs` 注册 Tauri IPC 命令并添加全中文注释。

- [ ] **Step 4: 运行全量测试验证通过**
  运行：`cargo test`
  预期：PASS
  运行：`pnpm test run`
  预期：PASS

- [ ] **Step 5: 提交加密保险箱代码**
  ```bash
  git add src-tauri/src/crypto/ src-tauri/src/vault/ src-tauri/src/lib.rs
  git commit -m "feat: implement argon2 and aes-256-gcm encrypted vault with full tests"
  ```

---

### Task 4: Rust 核心 - SSH 与 SFTP 异步网络通信引擎 (SSH & SFTP Engine)

**Files:**
- Create: `src-tauri/src/ssh/mod.rs`
- Create: `src-tauri/src/ssh/config.rs`
- Create: `src-tauri/src/ssh/session.rs`
- Create: `src-tauri/src/ssh/jump.rs`
- Create: `src-tauri/src/sftp/mod.rs`
- Create: `src-tauri/src/sftp/client.rs`
- Modify: `src-tauri/src/lib.rs`
- Test: `src-tauri/src/ssh/config.rs` (内联测试)
- Test: `src-tauri/src/ssh/jump.rs` (内联测试)
- Test: `src-tauri/src/sftp/client.rs` (内联测试)

**Interfaces:**
- Consumes: Task 3 的解密凭据接口、Task 1 的 Tokio 与 russh 依赖
- Produces:
  - `HostConfig`: 主机配置结构体（支持密码、密钥、SSH Agent、跳板机配置）
  - `resolve_jump_hosts(hosts: &[HostConfig], target_id: &str) -> Result<Vec<HostConfig>, SshError>`
  - `SshSessionManager`: 会话生命周期管理，集成 Tauri 2.x `Channel<Vec<u8>>` 双向流传输与 PTY 窗口尺寸更新（`resize_pty`）
  - `SftpClient`: `{ list_dir(path: &str) -> Result<Vec<FileInfo>>, upload_file(local: &str, remote: &str, progress_channel), download_file(remote: &str, local: &str) }`
  - Tauri 命令：`save_host`, `get_hosts`, `delete_host`, `connect_ssh`, `disconnect_ssh`, `resize_pty`, `send_terminal_input`, `sftp_list_dir`, `sftp_upload_file`, `sftp_download_file`

- [ ] **Step 1: 编写主机配置与跳板机环路检测的失败测试**
  在 `src-tauri/src/ssh/jump.rs` 编写跳板机循环依赖检测测试（覆盖 Review Focus 2：`test_jump_host_circular_dependency_detected`）；
  在 `src-tauri/src/sftp/client.rs` 编写 0 字节文件与断线处理测试（覆盖 Review Focus 5：`test_sftp_empty_file_and_error_handling`）。

- [ ] **Step 2: 运行测试验证失败**
  运行：`cargo test --package candyterm-lib ssh sftp`
  预期：FAIL

- [ ] **Step 3: 实现 SSH 配置解析、会话管理与 SFTP 逻辑**
  - 实现支持密码、私钥（带口令解密）、跳板机解析的数据结构。
  - 基于 `russh` 与 Tokio 异步通道构建会话管理机制，向前端实时推送数据流。
  - 实现 SFTP 文件遍历、流式分片上传与下载。
  - 为所有结构体、函数添加详尽的中文注释。

- [ ] **Step 4: 运行全量测试验证通过**
  运行：`cargo test`
  预期：PASS
  运行：`pnpm test run`
  预期：PASS

- [ ] **Step 5: 提交 SSH 与 SFTP 引擎代码**
  ```bash
  git add src-tauri/src/ssh/ src-tauri/src/sftp/ src-tauri/src/lib.rs
  git commit -m "feat: implement async ssh session and sftp transfer engine with tests"
  ```

---

### Task 5: 前端 - 现代工作区多分屏终端引擎 (Split Workspace & xterm.js Engine)

**Files:**
- Create: `src/types/terminal.ts`
- Create: `src/stores/workspace.ts`
- Create: `src/components/terminal/TerminalPane.vue`
- Create: `src/components/terminal/SplitWorkspace.vue`
- Create: `src/components/sidebar/IconSidebar.vue`
- Create: `src/views/MainWorkspace.vue`
- Create: `tests/frontend/workspace.test.ts`

**Interfaces:**
- Consumes: Task 2 的主题与国际化 Store、Task 4 的 Tauri SSH IPC 命令与 Channel
- Produces:
  - `useWorkspaceStore()`:
    - 树形多窗格分屏模型（Binary Split Tree）：`{ splitPane(targetId, direction: 'horizontal' | 'vertical', hostId), closePane(paneId), focusPane(paneId), setBroadcast(active: boolean) }`
    - 命令广播机制：当开启广播时，按键输入同步派发至当前标签页下的所有活跃分屏
  - 具备节流保护的 Terminal Resize 行为（覆盖 Review Focus 4：`test_split_pane_resize_throttling`）
  - xterm.js + WebGL 硬件加速渲染，高亮活动分屏与拖拽分割条

- [ ] **Step 1: 编写分屏树算法与广播机制失败单元测试**
  在 `tests/frontend/workspace.test.ts` 编写：
  - 单窗格横向切分、纵向切分算法测试
  - 窗格关闭与焦点自动重定向测试
  - 广播输入状态流转测试
  - 分割条拖拽尺寸节流防抖计算测试

- [ ] **Step 2: 运行测试验证失败**
  运行：`pnpm test run tests/frontend/workspace.test.ts`
  预期：FAIL

- [ ] **Step 3: 实现分屏状态管理与终端渲染组件**
  - 编写 `src/stores/workspace.ts` 实现分屏树增删与广播逻辑。
  - 编写 `TerminalPane.vue` 集成 `@xterm/xterm`，加载 `@xterm/addon-fit` 与 `@xterm/addon-webgl`（优雅降级处理）。
  - 编写 `SplitWorkspace.vue` 实现自适应分割条与拖拽交互。
  - 编写 `IconSidebar.vue` 实现会话、SFTP、设置及主题/语言切换按钮。
  - 所有代码配全中文注释。

- [ ] **Step 4: 运行全量测试验证通过**
  运行：`pnpm test run`
  预期：PASS
  运行：`cargo test`
  预期：PASS

- [ ] **Step 5: 提交终端分屏系统代码**
  ```bash
  git add src/types/ src/stores/workspace.ts src/components/ src/views/ tests/frontend/
  git commit -m "feat: implement split-pane terminal workspace and xterm rendering with tests"
  ```

---

### Task 6: 前端 - 混合协同 SFTP 交互系统 (Hybrid SFTP: Quick Drawer & Dedicated Tab)

**Files:**
- Create: `src/types/sftp.ts`
- Create: `src/stores/sftp.ts`
- Create: `src/components/sftp/SftpDrawer.vue`
- Create: `src/components/sftp/SftpFullView.vue`
- Create: `src/components/sftp/TransferQueue.vue`
- Modify: `src/components/terminal/TerminalPane.vue`
- Create: `tests/frontend/sftp.test.ts`

**Interfaces:**
- Consumes: Task 4 的 SFTP Tauri IPC 命令、Task 5 的 TerminalPane 窗格上下文
- Produces:
  - `useSftpStore()`: `{ currentPath, fileList, transferQueue, uploadFile(local, remote), downloadFile(remote, local), cancelTransfer(id) }`
  - 窗格随动 SFTP 抽屉：窗格右上角 📁 呼出，右侧滑出，支持直接从系统桌面拖入文件上传到当前目录
  - 独立全屏双栏 SFTP 标签页：本地/远程双栏对照、批量操作、传输队列面板

- [ ] **Step 1: 编写 SFTP 状态与传输队列失败测试**
  在 `tests/frontend/sftp.test.ts` 编写：
  - 路径切换与面包屑导航算法测试
  - 传输队列添加、进度更新与并发限制测试
  - 零字节文件与传输错误状态处理测试（覆盖 Review Focus 5）

- [ ] **Step 2: 运行测试验证失败**
  运行：`pnpm test run tests/frontend/sftp.test.ts`
  预期：FAIL

- [ ] **Step 3: 实现轻量抽屉与全尺寸 SFTP 视图组件**
  - 编写 `SftpDrawer.vue`：窗格右侧平滑滑出，展示简洁文件树，支持拖拽即传。
  - 编写 `SftpFullView.vue`：双栏布局，左侧本地文件浏览器，右侧远程文件浏览器。
  - 编写 `TransferQueue.vue`：底部可折叠传输队列，支持暂停、重试与速度展示。
  - 在 `TerminalPane.vue` 中集成抽屉呼出与拖放上传事件。
  - 全代码配完整中文注释。

- [ ] **Step 4: 运行全量测试验证通过**
  运行：`pnpm test run`
  预期：PASS
  运行：`cargo test`
  预期：PASS

- [ ] **Step 5: 提交 SFTP 协同交互代码**
  ```bash
  git add src/types/sftp.ts src/stores/sftp.ts src/components/sftp/ tests/frontend/sftp.test.ts src/components/terminal/TerminalPane.vue
  git commit -m "feat: implement hybrid sftp drawer and dedicated full tab with tests"
  ```

---

### Task 7: 主密码解锁弹窗、新建连接模态框与工程全量装配 (App Integration & Modals)

**Files:**
- Create: `src/components/auth/MasterPasswordModal.vue`
- Create: `src/components/connection/NewConnectionModal.vue`
- Create: `src/App.vue`
- Create: `src/main.ts`
- Modify: `tests/frontend/setup.test.ts`
- Create: `tests/frontend/integration.test.ts`

**Interfaces:**
- Consumes: Task 2, Task 3, Task 4, Task 5, Task 6 的全部 Stores 与 Components
- Produces: 完整可运行的应用装配入口，包含主密码解锁流程、新建/编辑连接模态框（支持密码、私钥、跳板机、SSH Agent 配置）

- [ ] **Step 1: 编写主密码解锁与主机配置装配的失败测试**
  在 `tests/frontend/integration.test.ts` 编写冷启动解锁流程与表单验证测试。

- [ ] **Step 2: 运行测试验证失败**
  运行：`pnpm test run tests/frontend/integration.test.ts`
  预期：FAIL

- [ ] **Step 3: 实现解锁窗口、新建主机表单与根组件**
  - 编写 `MasterPasswordModal.vue`（首次启动引导设置主密码，后续冷启动要求解锁）。
  - 编写 `NewConnectionModal.vue`（使用 Naive UI 表单组件，支持密码、密钥导入、SSH Agent、跳板机下拉选择）。
  - 编写 `App.vue` 装配 `NConfigProvider`、暗黑模式切换、全局消息与分屏工作区。
  - 所有代码配全中文注释。

- [ ] **Step 4: 运行全量测试验证通过**
  运行：`pnpm test run`
  预期：PASS
  运行：`cargo test`
  预期：PASS

- [ ] **Step 5: 提交应用装配代码**
  ```bash
  git add src/components/auth/ src/components/connection/ src/App.vue src/main.ts tests/frontend/
  git commit -m "feat: integrate master password modal and host connection modal"
  ```

---

### Task 8: 自动化文档、GitHub Actions 流水线与全量质量回归 (CI/CD, Docs & Full Verification)

**Files:**
- Create: `.github/workflows/release.yml`
- Create: `README.md`
- Modify: `README.md` (自动化新特性更新)

**Interfaces:**
- Consumes: 全项目代码、测试用例与配置
- Produces: 跨平台自动构建与 Release 发布工作流、详尽完整的 `README.md`

- [ ] **Step 1: 编写 GitHub Actions 多平台自动构建流**
  编写 `.github/workflows/release.yml`：
  - 监听 `v*` tag 推送
  - 矩阵构建：`macos-latest`, `ubuntu-latest`, `windows-latest`
  - 步骤：环境安装 (Rust, Node, pnpm) -> 运行 `cargo test` -> 运行 `pnpm test run` -> 执行 `tauri-action` 编译并发布 Release

- [ ] **Step 2: 撰写与更新 `README.md`**
  编写项目根目录 `README.md`（包含中文与英文指引）：
  - 项目简介与核心特性（Tauri 2 极速性能、现代分屏、混合 SFTP、主密码加密、多认证方式、中英双语、日夜模式）
  - 技术栈清单与架构简图
  - 开发环境要求与本地启动/构建步骤（`pnpm install`, `pnpm tauri dev`, `pnpm test`）
  - 自动化发版指引

- [ ] **Step 3: 运行全量单元测试与质量回归**
  运行：`cargo test`
  预期：PASS (100% 通过)
  运行：`pnpm test run`
  预期：PASS (100% 通过)

- [ ] **Step 4: 提交 CI/CD 与文档**
  ```bash
  git add .github/ README.md
  git commit -m "ci: add github actions release workflow and comprehensive readme documentation"
  ```
