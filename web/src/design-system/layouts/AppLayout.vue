<script setup lang="ts">
/**
 * Skills Intelligence Hub - App Layout 应用主布局
 *
 * 包含顶部导航栏、侧边栏（可选）和内容区域
 */
import { computed, ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'

// Props 定义
interface Props {
  /** 是否显示侧边栏 */
  showSidebar?: boolean
  /** 页面标题 */
  title?: string
}

const props = withDefaults(defineProps<Props>(), {
  showSidebar: false,
})

// Emits 定义
const emit = defineEmits<{
  (e: 'toggleSidebar'): void
}>()

// i18n
const { t } = useI18n()

// 路由和状态
const route = useRoute()
const router = useRouter()
const userStore = useUserStore()

// 内部状态
const isMobileMenuOpen = ref(false)
const isUserMenuOpen = ref(false)
const sidebarCollapsed = ref(false) // 内部管理折叠状态
const isUserLoading = ref(false) // 用户信息加载状态

// 计算当前用户信息和登录状态
const currentUser = computed(() => userStore.user)
const isLoggedIn = computed(() => userStore.isLoggedIn)
const isAdmin = computed(() => userStore.isAdmin)

// 计算主内容区域类名
const mainClasses = computed(() => {
  const classes = ['flex-1', 'min-h-screen', 'transition-all', 'duration-300']

  if (props.showSidebar) {
    if (sidebarCollapsed.value) {
      classes.push('lg:ml-16')
    } else {
      classes.push('lg:ml-64')
    }
  }

  return classes.join(' ')
})

// 导航菜单项 - 根据权限过滤
const navItems = computed(() => {
  const items = [
    { name: 'market', label: t('nav.market'), path: '/', icon: 'home' },
    { name: 'my-skills', label: t('appLayout.mySkills'), path: '/my-skills', icon: 'folder' },
    { name: 'admin', label: t('nav.admin'), path: '/admin', icon: 'settings', admin: true },
  ]
  // 过滤：未登录不显示 my-skills，非管理员不显示 admin
  return items.filter(item => {
    if (item.name === 'my-skills' && !isLoggedIn.value) return false
    if (item.admin && !isAdmin.value) return false
    return true
  })
})

// Admin 二级菜单项
const adminSubItems = computed(() => [
  { name: 'admin-users', label: t('users.title'), path: '/admin/users' },
  { name: 'admin-groups', label: t('groups.title'), path: '/admin/groups' },
  { name: 'admin-roles', label: t('roles.title'), path: '/admin/roles' },
])

// 计算当前激活的菜单项
const activeNavItem = computed(() => {
  return navItems.value.find(item => item.path === route.path)
})

// 判断是否在 admin 路由下
const isAdminRoute = computed(() => {
  return route.path.startsWith('/admin')
})

// 计算 admin 二级菜单激活状态
const activeSubItem = computed(() => {
  return adminSubItems.value.find(item => item.path === route.path)
})

// 退出登录
const handleLogout = async () => {
  await userStore.logout()
  router.push('/login')
}

// 切换移动端菜单
const toggleMobileMenu = () => {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}

// 切换用户菜单
const toggleUserMenu = () => {
  isUserMenuOpen.value = !isUserMenuOpen.value
}

// 切换侧边栏折叠状态
const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value
  emit('toggleSidebar')
}

// 通知按钮处理
const handleNotifications = () => {
  // TODO: 实现通知面板
  alert(t('appLayout.notificationsComingSoon') || '通知功能即将推出')
}

// 初始化用户信息
onMounted(async () => {
  // 如果有 token 但没有用户信息，尝试获取
  if (userStore.isTokenValid() && !userStore.user) {
    isUserLoading.value = true
    try {
      await userStore.fetchUser()
    } catch {
      // 获取失败，token 无效，用户会被重定向到登录页
    } finally {
      isUserLoading.value = false
    }
  }
})
</script>

<template>
  <div class="min-h-screen bg-neutral-50">
    <!-- 顶部导航栏 -->
    <header class="fixed top-0 left-0 right-0 h-16 bg-white border-b border-neutral-200 z-50">
      <div class="h-full px-4 flex items-center justify-between">
        <!-- 左侧：Logo 和菜单按钮 -->
        <div class="flex items-center space-x-4">
          <!-- 移动端菜单按钮 -->
          <button
            v-if="showSidebar"
            class="lg:hidden p-2 rounded-lg hover:bg-neutral-100 transition-colors"
            @click="toggleMobileMenu"
          >
            <svg class="w-6 h-6 text-neutral-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>

          <!-- Logo -->
          <router-link to="/" class="flex items-center space-x-2">
            <div class="w-8 h-8 bg-brand-500 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
              </svg>
            </div>
            <span class="hidden sm:block text-lg font-semibold text-neutral-800">
              {{ t('nav.brand') }}
            </span>
          </router-link>
        </div>

        <!-- 中间：页面标题 -->
        <div v-if="title" class="hidden md:block absolute left-1/2 -translate-x-1/2">
          <h1 class="text-lg font-semibold text-neutral-800">{{ title }}</h1>
        </div>

        <!-- 右侧：用户菜单 -->
        <div class="flex items-center space-x-4">
          <!-- 已登录：显示通知按钮和用户菜单 -->
          <template v-if="isLoggedIn">
            <!-- 通知按钮 -->
            <button
              class="p-2 rounded-lg hover:bg-neutral-100 transition-colors relative"
              @click="handleNotifications"
            >
              <svg class="w-5 h-5 text-neutral-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
              </svg>
              <!-- 通知徽章 -->
              <span class="absolute top-1 right-1 w-2 h-2 bg-semantic-error-dark rounded-full"></span>
            </button>

            <!-- 用户头像和菜单 -->
            <div class="relative">
              <button
                class="flex items-center space-x-2 p-1 rounded-lg hover:bg-neutral-100 transition-colors"
                @click="toggleUserMenu"
              >
                <div class="w-8 h-8 bg-brand-100 rounded-full flex items-center justify-center">
                  <span class="text-brand-600 font-medium text-sm">
                    {{ isUserLoading ? '...' : (currentUser?.username?.charAt(0).toUpperCase() || 'U') }}
                  </span>
                </div>
                <span class="hidden sm:block text-sm font-medium text-neutral-700">
                  {{ isUserLoading ? t('common.loading') : (currentUser?.username || 'User') }}
                </span>
                <svg class="w-4 h-4 text-neutral-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </button>

              <!-- 用户下拉菜单 -->
              <div
                v-if="isUserMenuOpen"
                class="absolute right-0 mt-2 w-48 bg-white rounded-lg shadow-lg border border-neutral-200 py-1 z-50"
              >
                <router-link
                  to="/profile"
                  class="block px-4 py-2 text-sm text-neutral-700 hover:bg-neutral-50"
                  @click="isUserMenuOpen = false"
                >
                  {{ t('appLayout.profile') }}
                </router-link>
                <router-link
                  to="/settings"
                  class="block px-4 py-2 text-sm text-neutral-700 hover:bg-neutral-50"
                  @click="isUserMenuOpen = false"
                >
                  {{ t('appLayout.settings') }}
                </router-link>
                <hr class="my-1 border-neutral-200" />
                <button
                  class="w-full text-left px-4 py-2 text-sm text-semantic-error-dark hover:bg-neutral-50"
                  @click="handleLogout"
                >
                  {{ t('auth.nav.logout') }}
                </button>
              </div>
            </div>
          </template>

          <!-- 未登录：显示登录/注册按钮 -->
          <template v-else>
            <router-link
              to="/login"
              class="text-neutral-600 hover:text-neutral-900 px-3 py-2 rounded-md text-sm font-medium"
            >
              {{ t('auth.nav.login') }}
            </router-link>
            <router-link
              to="/register"
              class="bg-brand-500 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-brand-600"
            >
              {{ t('auth.nav.register') }}
            </router-link>
          </template>
        </div>
      </div>
    </header>

    <!-- 侧边栏 -->
    <aside
      v-if="showSidebar"
      :class="[
        'fixed top-16 left-0 h-[calc(100vh-4rem)] bg-white border-r border-neutral-200 z-40 transition-all duration-300',
        sidebarCollapsed ? 'w-16' : 'w-64',
        isMobileMenuOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'
      ]"
    >
      <!-- 导航菜单 -->
      <nav class="p-4 space-y-1">
        <router-link
          v-for="item in navItems"
          :key="item.name"
          :to="item.path"
          :class="[
            'flex items-center px-3 py-2 rounded-lg transition-colors',
            activeNavItem?.name === item.name
              ? 'bg-brand-50 text-brand-600'
              : 'text-neutral-600 hover:bg-neutral-50 hover:text-neutral-800'
          ]"
          @click="isMobileMenuOpen = false"
        >
          <!-- 图标 -->
          <svg v-if="item.icon === 'home'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          <svg v-else-if="item.icon === 'folder'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          <svg v-else-if="item.icon === 'settings'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>

          <!-- 标签文字 -->
          <span v-if="!sidebarCollapsed" class="ml-3">{{ item.label }}</span>
        </router-link>

        <!-- Admin 二级菜单 -->
        <div v-if="isAdminRoute && !sidebarCollapsed" class="ml-6 mt-2 space-y-1 border-l border-neutral-200 pl-3">
          <router-link
            v-for="subItem in adminSubItems"
            :key="subItem.name"
            :to="subItem.path"
            :class="[
              'flex items-center px-3 py-2 rounded-lg text-sm transition-colors',
              activeSubItem?.name === subItem.name
                ? 'bg-brand-50 text-brand-600'
                : 'text-neutral-500 hover:bg-neutral-50 hover:text-neutral-700'
            ]"
            @click="isMobileMenuOpen = false"
          >
            {{ subItem.label }}
          </router-link>
        </div>
      </nav>

      <!-- 折叠按钮 -->
      <div class="absolute bottom-4 left-0 right-0 px-4">
        <button
          class="w-full p-2 rounded-lg hover:bg-neutral-100 transition-colors text-neutral-500"
          @click="toggleSidebar"
        >
          <svg
            :class="['w-5 h-5 mx-auto transition-transform', sidebarCollapsed && 'rotate-180']"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
          </svg>
        </button>
      </div>
    </aside>

    <!-- 移动端遮罩 -->
    <div
      v-if="isMobileMenuOpen && showSidebar"
      class="fixed inset-0 bg-black/50 z-30 lg:hidden"
      @click="isMobileMenuOpen = false"
    ></div>

    <!-- 主内容区域 -->
    <main :class="mainClasses" class="pt-16">
      <div class="p-6">
        <slot />
      </div>
    </main>

    <!-- 页脚 -->
    <footer class="border-t border-neutral-200 bg-white py-4 px-6 mt-auto">
      <div class="flex flex-col sm:flex-row justify-between items-center text-sm text-neutral-500">
        <p>{{ t('appLayout.copyright', { year: new Date().getFullYear() }) }}</p>
        <div class="flex space-x-4 mt-2 sm:mt-0">
          <router-link to="/admin" class="hover:text-neutral-700">{{ t('appLayout.help') }}</router-link>
          <router-link to="/admin" class="hover:text-neutral-700">{{ t('appLayout.privacy') }}</router-link>
          <router-link to="/admin" class="hover:text-neutral-700">{{ t('appLayout.terms') }}</router-link>
        </div>
      </div>
    </footer>
  </div>
</template>