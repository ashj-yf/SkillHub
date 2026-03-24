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

// Supported locales
export type SupportedLocale = 'zh-CN' | 'en-US'

// Get saved locale from localStorage, default to Chinese
const getSavedLocale = (): SupportedLocale => {
  if (typeof window === 'undefined') return 'zh-CN'
  const saved = localStorage.getItem('locale')
  if (saved === 'zh-CN' || saved === 'en-US') return saved
  return 'zh-CN'
}

const i18n = createI18n<[MessageSchema], SupportedLocale>({
  legacy: false, // Use Composition API mode
  locale: getSavedLocale(),
  fallbackLocale: 'zh-CN',
  messages,
})

export default i18n

// Type for accessing locale in Composition API mode
type I18nGlobalLocale = { value: SupportedLocale }

/**
 * Switch language
 * @param locale - Target locale ('zh-CN' | 'en-US')
 */
export function setLocale(locale: SupportedLocale): void {
  if (typeof window === 'undefined') return

  // In legacy: false mode, locale is a Ref<SupportedLocale>
  ;(i18n.global.locale as unknown as I18nGlobalLocale).value = locale
  localStorage.setItem('locale', locale)
  document.documentElement.lang = locale
}

/**
 * Get current locale
 */
export function getLocale(): SupportedLocale {
  // In legacy: false mode, locale is a Ref<SupportedLocale>
  return (i18n.global.locale as unknown as I18nGlobalLocale).value
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