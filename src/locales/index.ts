import { createI18n } from 'vue-i18n'
import zhCN from './zh-CN.json'
import enUS from './en-US.json'

/**
 * 获取初始语言：优先读取本地缓存，其次跟随系统语言
 */
export function getDefaultLocale(): 'zh-CN' | 'en-US' {
  const cached = typeof localStorage !== 'undefined' ? localStorage.getItem('candyterm_locale') : null
  if (cached === 'zh-CN' || cached === 'en-US') {
    return cached
  }
  const systemLang = typeof navigator !== 'undefined' ? navigator.language : 'zh-CN'
  if (systemLang.startsWith('zh')) {
    return 'zh-CN'
  }
  return 'en-US'
}

/**
 * 导出 vue-i18n 实例
 */
export const i18n = createI18n({
  legacy: false,
  locale: getDefaultLocale(),
  fallbackLocale: 'en-US',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  }
})
