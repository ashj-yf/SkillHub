import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { login as apiLogin, register as apiRegister, type LoginRequest, type RegisterRequest } from '@/api/auth'
import { getCurrentUser as fetchCurrentUser, type UserInfo } from '@/api/users'

// Re-export UserInfo for backward compatibility
export type { UserInfo }

export const useUserStore = defineStore('user', () => {
  const token = ref<string | null>(localStorage.getItem('token'))
  const user = ref<UserInfo | null>(null)
  const loading = ref(false)

  const isLoggedIn = computed(() => !!token.value)

  // 判断是否为管理员
  const isAdmin = computed(() => {
    if (!user.value) return false
    // 检查 roles 数组中是否包含 admin
    if (user.value.roles?.includes('admin')) return true
    // 兼容旧的 role 字段
    if (user.value.role === 'admin') return true
    return false
  })

  async function login(data: LoginRequest) {
    loading.value = true
    try {
      const t = await apiLogin(data)
      token.value = t
      localStorage.setItem('token', t)
      // 登录成功后获取用户信息
      await fetchUser()
    } finally {
      loading.value = false
    }
  }

  async function register(data: RegisterRequest) {
    await apiRegister(data)
  }

  async function fetchUser() {
    if (!token.value) return

    try {
      const data = await fetchCurrentUser()
      user.value = data
    } catch (e) {
      // 获取用户信息失败，可能 token 无效
      logout()
      throw e
    }
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
  }

  // 检查 token 是否有效
  function isTokenValid(): boolean {
    if (!token.value) return false

    try {
      // 简单检查 JWT 格式和过期时间（不验证签名）
      const parts = token.value.split('.')
      if (parts.length !== 3) return false

      const payload = JSON.parse(atob(parts[1]))
      const exp = payload.exp as number

      if (exp && exp * 1000 < Date.now()) {
        logout()
        return false
      }

      return true
    } catch {
      logout()
      return false
    }
  }

  return {
    token,
    user,
    loading,
    isLoggedIn,
    isAdmin,
    login,
    register,
    logout,
    fetchUser,
    isTokenValid,
  }
})