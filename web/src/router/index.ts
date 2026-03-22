import { createRouter, createWebHistory } from 'vue-router'
import { useUserStore } from '@/stores/user'

const routes = [
  { path: '/login', name: 'Login', component: () => import('@/views/Login.vue') },
  { path: '/register', name: 'Register', component: () => import('@/views/Register.vue') },
  { path: '/', name: 'Market', component: () => import('@/views/Market.vue') },
  { path: '/skill/:slug', name: 'SkillDetail', component: () => import('@/views/SkillDetail.vue') },
  {
    path: '/admin',
    name: 'Admin',
    component: () => import('@/views/Admin.vue'),
    meta: { requiresAuth: true }
  },
  // 404 处理
  { path: '/:pathMatch(.*)*', redirect: '/' },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

// 路由守卫
router.beforeEach(async (to, _from, next) => {
  const userStore = useUserStore()

  // 检查是否需要认证
  if (to.meta.requiresAuth) {
    if (!userStore.isTokenValid()) {
      // 保存目标路径，登录后重定向
      next({ name: 'Login', query: { redirect: to.fullPath } })
      return
    }

    // 如果有 token 但没有用户信息，尝试获取
    if (!userStore.user) {
      try {
        await userStore.fetchUser()
      } catch {
        next({ name: 'Login', query: { redirect: to.fullPath } })
        return
      }
    }
  }

  // 已登录用户访问登录/注册页，重定向到首页
  if ((to.name === 'Login' || to.name === 'Register') && userStore.isTokenValid()) {
    next({ name: 'Market' })
    return
  }

  next()
})