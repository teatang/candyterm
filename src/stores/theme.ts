import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type ThemeMode = 'light' | 'dark' | 'auto'

/**
 * 主题状态管理器
 * 支持暗黑模式、明亮模式以及跟随操作系统切换
 */
export const useThemeStore = defineStore('theme', () => {
  // 从本地存储中恢复主题，默认跟随系统 auto
  const initialMode = (typeof localStorage !== 'undefined'
    ? localStorage.getItem('candyterm_theme') as ThemeMode
    : null) || 'auto'

  const mode = ref<ThemeMode>(initialMode)
  const systemPrefersDark = ref(false)

  // 监听操作系统色彩模式
  if (typeof window !== 'undefined' && window.matchMedia) {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    systemPrefersDark.value = mediaQuery.matches
    mediaQuery.addEventListener('change', (e) => {
      systemPrefersDark.value = e.matches
      applyDocumentTheme()
    })
  }

  // 计算当前是否为暗黑模式
  const isDark = computed(() => {
    if (mode.value === 'dark') return true
    if (mode.value === 'light') return false
    return systemPrefersDark.value
  })

  // 同步主题样式到 HTML 根元素
  function applyDocumentTheme() {
    if (typeof document !== 'undefined') {
      if (isDark.value) {
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }
    }
  }

  // 切换主题模式并持久化
  function setThemeMode(newMode: ThemeMode) {
    mode.value = newMode
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('candyterm_theme', newMode)
    }
    applyDocumentTheme()
  }

  // 初始应用
  applyDocumentTheme()

  return {
    mode,
    isDark,
    setThemeMode
  }
})
