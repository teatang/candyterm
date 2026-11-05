import { defineStore } from 'pinia'
import { ref } from 'vue'
import { i18n, getDefaultLocale } from '@/locales'

export type LocaleType = 'zh-CN' | 'en-US'

/**
 * 国际化状态管理器
 * 支持中英文即时切换并持久化保存
 */
export const useI18nStore = defineStore('i18n', () => {
  const locale = ref<LocaleType>(getDefaultLocale())

  // 设置当前语言
  function setLocale(newLocale: LocaleType) {
    locale.value = newLocale
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('candyterm_locale', newLocale)
    }
    // 同步到 vue-i18n 实例
    if (i18n.global) {
      i18n.global.locale.value = newLocale
    }
  }

  return {
    locale,
    setLocale
  }
})
