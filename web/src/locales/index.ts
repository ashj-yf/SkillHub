/**
 * Skills Intelligence Hub - i18n Configuration
 */
import { createI18n } from 'vue-i18n'
import zhCN from './zh-CN'
import enUS from './en-US'

export type MessageSchema = typeof zhCN

const messages = {
  'zh-CN': zhCN,
  'en-US': enUS,
}

// Get saved locale from localStorage, default to Chinese
const getSavedLocale = (): string => {
  if (typeof window === 'undefined') return 'zh-CN'
  return localStorage.getItem('locale') || 'zh-CN'
}

const i18n = createI18n<[MessageSchema], 'zh-CN' | 'en-US'>({
  legacy: false, // Use Composition API mode
  locale: getSavedLocale(),
  fallbackLocale: 'zh-CN',
  messages,
})

export default i18n

/**
 * Switch language
 * @param locale - Target locale ('zh-CN' | 'en-US')
 */
export function setLocale(locale: 'zh-CN' | 'en-US'): void {
  if (typeof window === 'undefined') return

  i18n.global.locale.value = locale
  localStorage.setItem('locale', locale)
  document.documentElement.lang = locale
}

/**
 * Get current locale
 */
export function getLocale(): string {
  return i18n.global.locale.value
}

/**
 * Get available locales
 */
export function getAvailableLocales(): Array<{ code: string; name: string }> {
  return [
    { code: 'zh-CN', name: '中文' },
    { code: 'en-US', name: 'English' },
  ]
}