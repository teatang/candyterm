import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useThemeStore } from '@/stores/theme'

/**
 * 验证暗黑模式、明亮模式以及系统跟随逻辑
 */
describe('主题系统测试 (Theme Store Test)', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorage.clear()
  })

  it('默认状态为跟随系统 (auto)', () => {
    const themeStore = useThemeStore()
    expect(themeStore.mode).toBe('auto')
  })

  it('支持手动切换为 dark 模式并持久化', () => {
    const themeStore = useThemeStore()
    themeStore.setThemeMode('dark')
    expect(themeStore.mode).toBe('dark')
    expect(themeStore.isDark).toBe(true)
    expect(localStorage.getItem('candyterm_theme')).toBe('dark')
  })

  it('支持手动切换为 light 模式并更新 isDark 状态', () => {
    const themeStore = useThemeStore()
    themeStore.setThemeMode('light')
    expect(themeStore.mode).toBe('light')
    expect(themeStore.isDark).toBe(false)
  })
})
