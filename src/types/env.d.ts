/**
 * 全局环境变量与窗口扩展声明
 */
export {}

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown
  }
}
