<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { useRouter } from 'vue-router'

const userStore = useUserStore()
const router = useRouter()

function handleLogout() {
  userStore.logout()
  router.push('/')
}
</script>

<template>
  <nav class="bg-white shadow-sm border-b border-neutral-200">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between h-16">
        <div class="flex items-center">
          <router-link to="/" class="text-xl font-bold text-neutral-900">
            Skills Hub
          </router-link>
        </div>

        <div class="flex items-center space-x-4">
          <router-link
            to="/"
            class="text-neutral-600 hover:text-neutral-900 px-3 py-2 rounded-md text-sm font-medium"
          >
            技能市场
          </router-link>

          <template v-if="userStore.isLoggedIn">
            <router-link
              to="/admin"
              class="text-neutral-600 hover:text-neutral-900 px-3 py-2 rounded-md text-sm font-medium"
            >
              管理
            </router-link>
            <button
              @click="handleLogout"
              class="text-neutral-600 hover:text-neutral-900 px-3 py-2 rounded-md text-sm font-medium"
            >
              退出
            </button>
          </template>

          <template v-else>
            <router-link
              to="/login"
              class="text-neutral-600 hover:text-neutral-900 px-3 py-2 rounded-md text-sm font-medium"
            >
              登录
            </router-link>
            <router-link
              to="/register"
              class="bg-brand-500 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-brand-600"
            >
              注册
            </router-link>
          </template>
        </div>
      </div>
    </div>
  </nav>
</template>