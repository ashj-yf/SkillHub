/**
 * Skills Intelligence Hub - 响应式断点设计令牌
 *
 * 断点系统用于响应式布局
 * 采用移动优先 (Mobile First) 策略
 */

// 基础断点值
const breakpointValues = {
  sm: '640px',    // 小屏手机 (横屏)
  md: '768px',    // 平板
  lg: '1024px',   // 小屏笔记本
  xl: '1280px',   // 桌面显示器
  '2xl': '1536px', // 大屏显示器
} as const

export const breakpoints = {
  // ========================================
  // 断点定义 (Breakpoints)
  // 最小宽度断点
  // ========================================
  ...breakpointValues,

  // ========================================
  // 设备类型断点 (Device Breakpoints)
  // 用于媒体查询
  // ========================================
  device: {
    mobile: {
      min: '0',
      max: '767px',
    },
    tablet: {
      min: '768px',
      max: '1023px',
    },
    desktop: {
      min: '1024px',
      max: '1279px',
    },
    largeDesktop: {
      min: '1280px',
      max: '1535px',
    },
    wideScreen: {
      min: '1536px',
      max: '9999px',
    },
  },

  // ========================================
  // 容器最大宽度 (Container Max Widths)
  // ========================================
  container: {
    sm: '640px',
    md: '768px',
    lg: '1024px',
    xl: '1280px',
    '2xl': '1536px',
  },
} as const

// 断点类型
export type Breakpoint = keyof typeof breakpointValues

// CSS 变量转换
export function breakpointsToCssVars(): Record<string, string> {
  return {
    '--breakpoint-sm': breakpointValues.sm,
    '--breakpoint-md': breakpointValues.md,
    '--breakpoint-lg': breakpointValues.lg,
    '--breakpoint-xl': breakpointValues.xl,
    '--breakpoint-2xl': breakpointValues['2xl'],
  }
}

// Tailwind 配置转换
export function breakpointsToTailwind(): Record<string, string> {
  return { ...breakpointValues }
}

// 响应式对象类型
export type ResponsiveValue<T> = {
  base?: T
  sm?: T
  md?: T
  lg?: T
  xl?: T
  '2xl'?: T
}

// 辅助函数：获取响应式值
export function getResponsiveValue<T>(
  responsive: ResponsiveValue<T>,
  breakpoint: keyof ResponsiveValue<T>
): T | undefined {
  return responsive[breakpoint]
}

export default breakpoints