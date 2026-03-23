/**
 * Skills Intelligence Hub - 设计令牌统一导出
 *
 * 集中管理所有设计令牌，提供统一的访问入口
 */

// 导出所有令牌
export { colors, withOpacity } from './colors'
export type { BrandColor, NeutralColor, SemanticType, SkillType } from './colors'

export { typography, getFontSizeClass, getFontWeightClass } from './typography'
export type { FontSize, FontWeight, LineHeight, TextStyle } from './typography'

export { spacing, getPaddingClass, getGapClass } from './spacing'
export type { SpacingValue, PaddingSize, MarginSize, GapSize } from './spacing'

export { shadows, getShadowClass, getSemanticShadow } from './shadows'
export type { ShadowSize } from './shadows'

export { breakpoints, breakpointsToCssVars, breakpointsToTailwind, getResponsiveValue } from './breakpoints'
export type { Breakpoint, ResponsiveValue } from './breakpoints'

// ========================================
// 设计令牌集合
// ========================================
import colors from './colors'
import typography from './typography'
import spacing from './spacing'
import shadows from './shadows'
import breakpoints, { breakpointsToCssVars, breakpointsToTailwind } from './breakpoints'

export const tokens = {
  colors,
  typography,
  spacing,
  shadows,
  breakpoints,
} as const

// ========================================
// 生成 CSS 变量
// ========================================
export function generateCssVariables(): Record<string, string> {
  const vars: Record<string, string> = {}

  // 颜色变量
  Object.entries(colors).forEach(([category, values]) => {
    if (typeof values === 'object' && values !== null) {
      Object.entries(values).forEach(([key, value]) => {
        if (typeof value === 'string') {
          vars[`--color-${category}-${key}`] = value
        } else if (typeof value === 'object' && value !== null) {
          Object.entries(value).forEach(([subKey, subValue]) => {
            vars[`--color-${category}-${key}-${subKey}`] = subValue as string
          })
        }
      })
    }
  })

  // 间距变量
  Object.entries(spacing).forEach(([key, value]) => {
    if (typeof value === 'string' && key !== 'semantic' && key !== 'layout') {
      vars[`--spacing-${key}`] = value
    }
  })

  // 字体变量
  Object.entries(typography.fontSize).forEach(([key, value]) => {
    vars[`--font-size-${key}`] = value
  })

  // 阴影变量
  Object.entries(shadows).forEach(([key, value]) => {
    if (typeof value === 'string' && key !== 'semantic' && key !== 'special') {
      vars[`--shadow-${key}`] = value
    }
  })

  // 断点变量
  Object.assign(vars, breakpointsToCssVars())

  return vars
}

// ========================================
// Tailwind 预设配置
// ========================================
export function getTailwindPreset() {
  return {
    theme: {
      colors: {
        brand: colors.brand,
        semantic: colors.semantic,
        neutral: colors.neutral,
        role: colors.role,
        skillType: colors.skillType,
      },
      fontFamily: typography.fontFamily,
      fontSize: typography.fontSize,
      fontWeight: typography.fontWeight,
      lineHeight: typography.lineHeight,
      letterSpacing: typography.letterSpacing,
      spacing: spacing,
      boxShadow: shadows,
      screens: breakpointsToTailwind(),
    },
  }
}

export default tokens