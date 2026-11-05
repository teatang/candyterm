import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useI18nStore } from '@/stores/i18n'
import zhCN from '@/locales/zh-CN.json'
import enUS from '@/locales/en-US.json'

/**
 * 验证国际化字典完整性及语言切换逻辑
 */
describe('国际化系统测试 (i18n Store Test)', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorage.clear()
  })

  it('zh-CN 与 en-US 词典必须具备完全一致的键名结构', () => {
    const zhKeys = Object.keys(zhCN).sort()
    const enKeys = Object.keys(enUS).sort()
    expect(zhKeys).toEqual(enKeys)

    // 递归检查二级结构
    for (const key of zhKeys) {
      const zhVal = (zhCN as Record<string, unknown>)[key]
      const enVal = (enUS as Record<string, unknown>)[key]
      if (typeof zhVal === 'object' && zhVal !== null) {
        expect(typeof enVal).toBe('object')
        expect(Object.keys(zhVal).sort()).toEqual(Object.keys(enVal as object).sort())
      }
    }
  })

  it('支持在 zh-CN 与 en-US 之间切换并持久化', () => {
    const i18nStore = useI18nStore()
    i18nStore.setLocale('en-US')
    expect(i18nStore.locale).toBe('en-US')
    expect(localStorage.getItem('candyterm_locale')).toBe('en-US')

    i18nStore.setLocale('zh-CN')
    expect(i18nStore.locale).toBe('zh-CN')
  })
})
