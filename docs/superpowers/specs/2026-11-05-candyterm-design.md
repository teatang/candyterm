# Candyterm 软件设计规格说明书 (Design Specification)

## 1. 项目背景与意图 (Intent & The Why)
现有的主流 SSH 终端工具（如 Electerm、Tabby 等）多基于 Electron 构建，虽然生态丰富，但在多会话并行、大量日志输出和高并发分屏场景下，容易出现明显的内存占用高、启动耗时长以及界面打字与滚屏卡顿等性能瓶颈。

**Candyterm** 旨在基于 **Tauri 2.x + Rust 异步核心** 与 **Vue 3 + TypeScript + TailwindCSS** 打造一款极致轻量、启动迅速、连接与输出极速、界面现代简约的高性能跨平台桌面 SSH / SFTP 客户端。重点解决日常运维开发中高频使用的**快速连接**、**灵活多窗格分屏**与**无缝 SFTP 文件管理**。

---

## 2. 目标、非目标与反目标 (Goals, Non-Goals, Anti-Goals)

### 2.1 核心目标 (Goals)
1. **高性能与极速连接**：底层由 Rust 异步驱动（基于 Tokio 异步网络与并发通道），具备极低的内存基线与微秒级网络吞吐响应；前端采用 xterm.js 配合 WebGL/Canvas 硬件加速渲染，保障 60fps 极限滚屏不掉帧。
2. **极简现代工作区与灵活分屏 (Option A)**：
   - 极简侧边导航栏（会话列表、SFTP管理、快捷命令、设置）。
   - 强大灵活的分屏系统：支持单会话/多会话横向切分、纵向切分与网格切分，支持快捷键一键切分、鼠标拖拽调整窗格比例，以及活动窗格高亮与命令输入广播。
3. **混合协同 SFTP 管理 (Option C)**：
   - **随动抽屉模式**：任意分屏窗格一键滑出该服务器专属的轻量 SFTP 侧边栏，支持直接把本地文件拖入终端或面板进行快速上传。
   - **全功能独立标签页**：适合深度运维，提供双栏文件浏览、大文件多任务传输队列、断点续传、文件权限查看与在线编辑。
4. **主密码级凭据安全**：所有服务器连接配置与私钥、密码均由用户设置的单一主密码（Master Password）加密保护。使用 Argon2id 进行密钥派生，AES-256-GCM 进行对称加密，内存与本地存储均杜绝明文凭据泄漏。
5. **完备的 SSH 认证体系**：支持密码、私钥（带 Passphrase 解密）、SSH Agent 转发、以及 Jump Host（跳板机/ProxyJump）。
6. **日夜模式与国际化 (i18n)**：支持暗黑模式、明亮模式以及跟随系统切换；支持中英双语，首发默认跟随系统语言。
7. **全中文注释与严格质量保障**：项目代码覆盖完整中文注释；为 Rust 后端与前端核心功能编写单元测试，修改后全量回归验证。
8. **自动化工程化规范**：开发完成新功能后自动同步更新 `README.md`；配置跨平台（macOS/Linux/Windows）GitHub Actions 自动构建与 Release 发版流水线。

### 2.2 非目标 (Non-Goals)
1. 不实现重型的云端同步商业后端（所有配置默认本端加密存储，支持本地导入导出 JSON 备份）。
2. 不做膨胀的多合一功能（如臃肿的数据库客户端、K8s 图形化集群浏览器等），专注于纯粹优秀的 SSH/SFTP 体验。

### 2.3 反目标 (Anti-Goals)
1. **绝不退化为臃肿缓慢的 Electron 式应用**：杜绝加载大量笨重前端库，内存与启动速度必须保持在原生轻量水准。
2. **杜绝不安全明文凭据保存**：严禁明文保存主机密码或未加密私钥口令。
3. **杜绝卡死主线程的同步阻塞 IO**：无论是大文件 SFTP 传输还是大量终端输出，必须完全由异步后台任务承载，界面绝不掉帧卡死。

---

## 3. 技术约束与栈选型 (Constraints & Stack)

- **桌面框架**：Tauri 2.x
- **后端语言**：Rust (edition 2021)
  - 核心异步运行时：`tokio`
  - SSH / SFTP 核心通信引擎：基于 Rust 异步 SSH 实现（`russh` / `ssh2` 异步通道封装）
  - 加密体系：`argon2` (KDF) + `aes-gcm` (AES-256-GCM)
  - 数据持久化：本地 SQLite 或加密 JSON 文件存储
- **前端技术栈**：
  - 框架：Vue 3 (Composition API, `<script setup lang="ts">`)
  - 语言：TypeScript (严格模式)
  - 包管理工具：`pnpm`
  - 状态管理：`Pinia`
  - 样式方案：`TailwindCSS`
  - UI 组件库：`Naive UI`（极简高质感，深度支持 Dark 模式，与 Tailwind 零冲突）
  - 终端渲染引擎：`xterm` + `@xterm/addon-fit` + `@xterm/addon-webgl` (带 Canvas 降级)
  - 国际化：`vue-i18n` (zh-CN, en-US)
  - 单元测试：Rust `cargo test` (后端) + `vitest` (前端)

---

## 4. 架构与交互设计 (Architecture & Interaction)

### 4.1 核心界面布局 (Modern Workspace Layout)
- **极简图标侧边栏 (Icon Sidebar)**：
  - 顶部：Logo / App 状态指示。
  - 主功能区：会话管理器（Sessions）、SFTP 传输面板（Transfers）、快捷指令库（Snippets）。
  - 底部：日夜模式切换按钮（Light / Dark / Auto）、国际化切换（中/英）、应用设置（Settings）。
- **多标签栏 (Tab Bar)**：
  - 支持多工作区标签、独立全屏 SFTP 标签页、设置标签页。
  - 每个终端标签页内包含一个独立的多窗格网格（Pane Grid）。
- **分屏窗格系统 (Split-Pane Grid)**：
  - **切分动作**：支持快捷键或窗格菜单快速执行“向右拆分”、“向下拆分”。
  - **比例调整**：各窗格间提供自适应拖拽分割条（Splitter）。
  - **窗格状态**：当前聚焦窗格具有清晰的主题高亮边框；右上角具备最小化/最大化当前分屏、呼出 SFTP 抽屉与关闭窗格按钮。
  - **输入广播 (Broadcast Input)**：可一键开启“广播输入”，将当前键盘按键同步发送至同一工作区标签下的所有分屏。

### 4.2 SFTP 协同交互机制 (Hybrid SFTP Mode)
1. **随动轻量抽屉 (Quick Drawer)**：
   - 在任意分屏窗格点击 📁 按钮，当前窗格右侧平滑滑出该主机的目录树。
   - 自动获取该 SSH 会话当前 working directory（或用户主目录）。
   - 本地桌面文件直接拖拽到窗格或抽屉，即时触发后台上传并显示极简进度条。
2. **全功能独立 SFTP 标签页 (Dedicated Full SFTP Tab)**：
   - 双栏对比视图：左侧本地文件系统，右侧远程主机文件系统。
   - 文件操作：新建、重命名、删除、修改权限属性（chmod）、直接在线文本编辑。
   - 传输队列监控：支持大文件分片断点续传、并发任务限制、实时速率与剩余时间统计。

### 4.3 安全与主密码机制 (Security & Vault Model)
1. **初始化**：首次启动时引导用户设置主密码（Master Password）。
2. **派生与加密**：
   - 随机生成 16 字节 Salt，通过 Argon2id 算法将主密码派生出 256-bit Key。
   - 服务器密码、私钥内容、私钥密码均采用 AES-256-GCM 加密，并存储在本地数据文件中。
3. **解锁与会话状态**：
   - 应用每次冷启动时展示解锁窗口。
   - 解锁成功后密钥常驻 Rust 内存中，前端仅持连接会话 ID，敏感密码私钥不暴露给前端内存。
   - 支持空闲超时自动锁定保险箱。

### 4.4 认证方式与会话生命周期
- **密码认证 (Password)**：支持保存或每次连接时弹窗提示输入。
- **密钥认证 (Private Key)**：支持 RSA, Ed25519, ECDSA 等标准 PEM/OpenSSH 格式密钥，支持带密码的私钥解密。
- **SSH 代理 (SSH Agent)**：集成系统标准 SSH Agent (macOS/Linux UNIX socket, Windows Pageant/OpenSSH named pipe)。
- **跳板机 (Jump Host / ProxyJump)**：支持选择已有保存的主机作为前置代理节点，通过内部通道建立多跳代理连接。

---

## 5. 工程质量与自动化流程 (Engineering & Automation)

### 5.1 完整中文注释规范
- Rust 代码：所有模块、结构体、枚举、公共方法、Tauri 命令均包含规范的 `///` 中文文档注释，复杂逻辑内部标注流程注释。
- 前端 TypeScript / Vue：所有组件 Props、Emits、Pinia Store、工具函数均添加 JSDoc 中文注释，组件内部关键逻辑配详细解释。

### 5.2 单元测试与测试驱动保障
- **Rust 后端测试**：
  - 加密/解密逻辑单元测试（Argon2id KDF 派生正确性、AES-256-GCM 加密与解密回环测试）。
  - 主机配置解析与序列化测试。
  - SSH 配置与代理跳板机链式构建测试。
- **前端单元测试**：
  - Pinia Stores 状态管理逻辑测试（ThemeStore、SessionStore、SettingsStore）。
  - 分屏树结构操作算法测试（分屏分裂、合并、焦点移动、广播路由算法）。
  - 国际化文案完整性校验测试。
- **全量执行规范**：每次代码修改完成后，必须依次执行 `cargo test` 与 `pnpm test`，确保 100% 通过无回归。

### 5.3 自动化更新 README.md
- 当新增或调整核心功能模块时，必须同步更新根目录 `README.md`，包括：
  - 功能特性列表 (Feature Matrix)。
  - 架构图或使用指引。
  - 构建运行命令与测试指引。

### 5.4 GitHub Actions 持续集成与发布
- 配置 `.github/workflows/release.yml`：
  - 触发条件：推送新版本 tag（例如 `v*`）时触发。
  - 矩阵构建：`macos-latest`, `ubuntu-latest`, `windows-latest`。
  - 流程：环境依赖安装（Rust, Node.js, pnpm, 各平台系统开发库） -> 运行全量单元测试 -> 使用 `tauri-action` 编译并自动创建 GitHub Release，上传对应架构的应用安装包（.dmg, .AppImage/.deb, .msi/.exe）。

---

## 6. 构建者决断与实现细节 (Builder's Calls & Implementation Details)
经过构建审查（Builder Check），以下技术实现细节已由构建者确立：
1. **SSH 核心库选型 (Builder's Call)**：优先选用纯 Rust 异步实现的 `russh`（结合 `tokio`）。相较于绑定 C 语言动态库的 `libssh2`，`russh` 在 macOS、Linux 以及 Windows 上的交叉编译和 GitHub Actions CI 中无需安装复杂的 OpenSSL / vcpkg 依赖，具备天然的跨平台稳定性和无锁并发吞吐性能。
2. **凭据与配置持久化 (Builder's Call)**：采用应用安全数据目录中的原子加密存储文件（`vault.enc`，AES-256-GCM 封装）。写入时通过临时文件写入后原子替换（Atomic Rename），并附带 HMAC/校验码，既避免了轻量客户端引入额外 SQLite 依赖的冗余，又规避了多进程文件锁冲突风险，方便用户导出加密备份。
3. **前端与后端高吞吐流传输 (Builder's Call)**：利用 Tauri 2.x 的 `Channel<Vec<u8>>` 二进制通道直接传输终端 PTY/SSH 输出流与 SFTP 分片，绕过传统 JSON 事件总线的字符串编码开销，确保高刷日志输出时达到原生 60fps。
4. **UI 组件库与样式系统 (Builder's Call)**：确立采用 Naive UI + TailwindCSS，配合 `@xterm/addon-fit` 和 `@xterm/addon-webgl`（在 WebGL 不可用时优雅降级为 Canvas 渲染），主题配色遵循 Slate / Zinc 极简质感。
5. **测试用例与质量门禁 (Builder's Call)**：Rust 后端通过 `cargo test` 覆盖 KDF、AES-GCM 加解密及 SSH 配置解析；前端通过 `vitest` + `@vue/test-utils` 覆盖 Pinia Store、分屏网格算法与 i18n 字典完整性；自动化构建通过 GitHub Actions 矩阵测试覆盖。
