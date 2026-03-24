/**
 * Skills Intelligence Hub - i18n Configuration
 */
import { createI18n } from 'vue-i18n'
import zhCN from './zh-CN'
import enUS from './en-US'

// Supported locales
export type SupportedLocale = 'zh-CN' | 'en-US'

const messages: Record<SupportedLocale, typeof zhCN> = {
  'zh-CN': zhCN,
  'en-US': enUS as typeof zhCN,
}

// Get saved locale from localStorage, default to Chinese
const getSavedLocale = (): SupportedLocale => {
  if (typeof window === 'undefined') return 'zh-CN'
  const saved = localStorage.getItem('locale')
  if (saved === 'zh-CN' || saved === 'en-US') return saved
  return 'zh-CN'
}

const i18n = createI18n({
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
export function setLocale(locale: SupportedLocale): void {
  if (typeof window === 'undefined') return

  // In legacy: false mode, locale is a Ref<SupportedLocale>
  i18n.global.locale.value = locale
  localStorage.setItem('locale', locale)
  document.documentElement.lang = locale
}

/**
 * Get current locale
 */
export function getLocale(): SupportedLocale {
  // In legacy: false mode, locale is a Ref<SupportedLocale>
  return i18n.global.locale.value as SupportedLocale
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