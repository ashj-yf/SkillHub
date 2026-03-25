/**
 * Skills Intelligence Hub - Toast Store
 *
 * 全局 Toast 通知状态管理
 */
import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Toast {
  id: string
  message: string
  type: 'success' | 'error' | 'warning' | 'info'
  duration: number
}

export const useToastStore = defineStore('toast', () => {
  const toasts = ref<Toast[]>([])

  /**
   * 添加一个 Toast 通知
   */
  function addToast(
    message: string,
    type: Toast['type'] = 'info',
    duration: number = 3000
  ): string {
    const id = `toast-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`

    toasts.value.push({
      id,
      message,
      type,
      duration,
    })

    // 自动移除
    if (duration > 0) {
      setTimeout(() => {
        removeToast(id)
      }, duration)
    }

    return id
  }

  /**
   * 移除一个 Toast 通知
   */
  function removeToast(id: string) {
    const index = toasts.value.findIndex((t) => t.id === id)
    if (index !== -1) {
      toasts.value.splice(index, 1)
    }
  }

  /**
   * 清除所有 Toast 通知
   */
  function clearAll() {
    toasts.value = []
  }

  /**
   * 快捷方法：成功通知
   */
  function success(message: string, duration?: number): string {
    return addToast(message, 'success', duration)
  }

  /**
   * 快捷方法：错误通知
   */
  function error(message: string, duration?: number): string {
    return addToast(message, 'error', duration)
  }

  /**
   * 快捷方法：警告通知
   */
  function warning(message: string, duration?: number): string {
    return addToast(message, 'warning', duration)
  }

  /**
   * 快捷方法：信息通知
   */
  function info(message: string, duration?: number): string {
    return addToast(message, 'info', duration)
  }

  return {
    toasts,
    addToast,
    removeToast,
    clearAll,
    success,
    error,
    warning,
    info,
  }
})