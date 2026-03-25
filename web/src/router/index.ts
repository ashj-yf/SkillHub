import { createRouter, createWebHistory } from 'vue-router'
import { useUserStore } from '@/stores/user'

const routes = [
  // Auth routes - use AuthLayout
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/Login.vue'),
    meta: { layout: 'auth' }
  },
  {
    path: '/register',
    name: 'Register',
    component: () => import('@/views/Register.vue'),
    meta: { layout: 'auth' }
  },

  // Main routes - use AppLayout
  {
    path: '/',
    name: 'Market',
    component: () => import('@/views/Market.vue'),
    meta: { layout: 'app' }
  },
  {
    path: '/skill/:slug',
    name: 'SkillDetail',
    component: () => import('@/views/SkillDetail.vue'),
    meta: { layout: 'app' }
  },

  // My Skills route
  {
    path: '/my-skills',
    name: 'MySkills',
    component: () => import('@/views/MySkills.vue'),
    meta: { requiresAuth: true, layout: 'app' }
  },

  // Profile and Settings routes (placeholder - redirect to admin for now)
  {
    path: '/profile',
    name: 'Profile',
    redirect: '/admin'
  },
  {
    path: '/settings',
    name: 'Settings',
    redirect: '/admin'
  },

  // Admin routes - use AppLayout with sidebar
  {
    path: '/admin',
    name: 'Admin',
    component: () => import('@/views/Admin.vue'),
    meta: { requiresAuth: true, requiresAdmin: true, layout: 'admin' }
  },
  {
    path: '/admin/users',
    name: 'AdminUsers',
    component: () => import('@/views/admin/Users.vue'),
    meta: { requiresAuth: true, requiresAdmin: true, layout: 'admin' }
  },
  {
    path: '/admin/groups',
    name: 'AdminGroups',
    component: () => import('@/views/admin/Groups.vue'),
    meta: { requiresAuth: true, requiresAdmin: true, layout: 'admin' }
  },
  {
    path: '/admin/roles',
    name: 'AdminRoles',
    component: () => import('@/views/admin/Roles.vue'),
    meta: { requiresAuth: true, requiresAdmin: true, layout: 'admin' }
  },

  // 404 handling
  { path: '/:pathMatch(.*)*', redirect: '/' },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

// Route guard
router.beforeEach(async (to, _from, next) => {
  const userStore = useUserStore()

  // Check if authentication is required
  if (to.meta.requiresAuth) {
    if (!userStore.isTokenValid()) {
      // Save target path for redirect after login
      next({ name: 'Login', query: { redirect: to.fullPath } })
      return
    }

    // If token exists but no user info, try to fetch
    if (!userStore.user) {
      try {
        await userStore.fetchUser()
      } catch {
        next({ name: 'Login', query: { redirect: to.fullPath } })
        return
      }
    }

    // Check admin permission
    if (to.meta.requiresAdmin && !userStore.isAdmin) {
      next({ name: 'Market' })
      return
    }
  }

  // Logged in user accessing login/register pages, redirect to home
  if ((to.name === 'Login' || to.name === 'Register') && userStore.isTokenValid()) {
    next({ name: 'Market' })
    return
  }

  next()
})