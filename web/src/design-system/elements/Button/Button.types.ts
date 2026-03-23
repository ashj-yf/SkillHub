/**
 * Skills Intelligence Hub - Button 组件类型定义
 */

// 按钮类型
export type ButtonType = 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger' | 'success'

// 按钮尺寸
export type ButtonSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl'

// 按钮状态
export type ButtonState = 'default' | 'hover' | 'active' | 'disabled' | 'loading'

// 按钮形状
export type ButtonShape = 'default' | 'circle' | 'square'

// 按钮属性接口
export interface ButtonProps {
  /** 按钮类型 */
  type?: ButtonType
  /** 按钮尺寸 */
  size?: ButtonSize
  /** 按钮形状 */
  shape?: ButtonShape
  /** 是否禁用 */
  disabled?: boolean
  /** 是否加载中 */
  loading?: boolean
  /** 是否块级按钮 */
  block?: boolean
  /** 原生按钮类型 */
  nativeType?: 'button' | 'submit' | 'reset'
  /** 图标位置 */
  iconPosition?: 'left' | 'right'
  /** href 属性，设置后渲染为 a 标签 */
  href?: string
  /** 链接的 target 属性 */
  target?: string
}

// 按钮事件接口
export interface ButtonEmits {
  (e: 'click', event: MouseEvent): void
}

// 按钮插槽接口
export interface ButtonSlots {
  default?: () => unknown
  icon?: () => unknown
  'icon-left'?: () => unknown
  'icon-right'?: () => unknown
}

// 尺寸对应的样式映射
export const buttonSizeMap: Record<ButtonSize, {
  height: string
  padding: string
  fontSize: string
  iconSize: string
}> = {
  xs: {
    height: '1.5rem',    // 24px
    padding: '0.375rem', // 6px
    fontSize: '0.75rem', // 12px
    iconSize: '0.75rem', // 12px
  },
  sm: {
    height: '2rem',      // 32px
    padding: '0.5rem',   // 8px
    fontSize: '0.875rem', // 14px
    iconSize: '0.875rem', // 14px
  },
  md: {
    height: '2.5rem',    // 40px
    padding: '0.75rem',  // 12px
    fontSize: '0.875rem', // 14px
    iconSize: '1rem',    // 16px
  },
  lg: {
    height: '3rem',      // 48px
    padding: '1rem',     // 16px
    fontSize: '1rem',    // 16px
    iconSize: '1.125rem', // 18px
  },
  xl: {
    height: '3.5rem',    // 56px
    padding: '1.25rem',  // 20px
    fontSize: '1rem',    // 16px
    iconSize: '1.25rem', // 20px
  },
}

// 类型对应的样式映射
export const buttonTypeMap: Record<ButtonType, {
  base: string
  hover: string
  active: string
  disabled: string
}> = {
  primary: {
    base: 'bg-brand-500 text-white hover:bg-brand-600 active:bg-brand-700',
    hover: 'bg-brand-600',
    active: 'bg-brand-700',
    disabled: 'bg-neutral-200 text-neutral-400 cursor-not-allowed',
  },
  secondary: {
    base: 'bg-neutral-100 text-neutral-700 hover:bg-neutral-200 active:bg-neutral-300',
    hover: 'bg-neutral-200',
    active: 'bg-neutral-300',
    disabled: 'bg-neutral-100 text-neutral-400 cursor-not-allowed',
  },
  outline: {
    base: 'border-2 border-brand-500 text-brand-500 bg-transparent hover:bg-brand-50 active:bg-brand-100',
    hover: 'bg-brand-50',
    active: 'bg-brand-100',
    disabled: 'border-neutral-200 text-neutral-400 cursor-not-allowed bg-transparent',
  },
  ghost: {
    base: 'text-neutral-600 bg-transparent hover:bg-neutral-100 active:bg-neutral-200',
    hover: 'bg-neutral-100',
    active: 'bg-neutral-200',
    disabled: 'text-neutral-400 cursor-not-allowed',
  },
  danger: {
    base: 'bg-semantic-error-dark text-white hover:opacity-90 active:opacity-80',
    hover: 'opacity-90',
    active: 'opacity-80',
    disabled: 'bg-neutral-200 text-neutral-400 cursor-not-allowed',
  },
  success: {
    base: 'bg-semantic-success-dark text-white hover:opacity-90 active:opacity-80',
    hover: 'opacity-90',
    active: 'opacity-80',
    disabled: 'bg-neutral-200 text-neutral-400 cursor-not-allowed',
  },
}