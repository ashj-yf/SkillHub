<script setup lang="ts">
/**
 * Skills Intelligence Hub - Auth Layout 认证页布局
 *
 * 用于登录、注册等认证相关页面的布局组件
 * 包含品牌展示区域和表单区域
 */
import { computed } from 'vue'
import { useRoute } from 'vue-router'

// Props 定义
interface Props {
  /** 页面标题 */
  title?: string
  /** 页面副标题 */
  subtitle?: string
  /** 是否显示品牌 Logo */
  showLogo?: boolean
  /** 自定义背景图片 */
  backgroundImg?: string
}

withDefaults(defineProps<Props>(), {
  title: 'Skills Intelligence Hub',
  subtitle: '让技能主动找到你',
  showLogo: true,
})

// 获取当前路由
const route = useRoute()

// 判断是否为登录页
const isLoginPage = computed(() => route.name === 'login')

// 判断是否为注册页
const isRegisterPage = computed(() => route.name === 'register')
</script>

<template>
  <div class="min-h-screen flex">
    <!-- 左侧品牌展示区 - 桌面端可见 -->
    <div class="hidden lg:flex lg:w-1/2 bg-brand-600 relative overflow-hidden">
      <!-- 背景装饰 -->
      <div class="absolute inset-0 bg-gradient-to-br from-brand-500 to-brand-700"></div>

      <!-- 背景图案 -->
      <div class="absolute inset-0 opacity-10">
        <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
              <path d="M 40 0 L 0 0 0 40" fill="none" stroke="white" stroke-width="1"/>
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#grid)" />
        </svg>
      </div>

      <!-- 内容 -->
      <div class="relative z-10 flex flex-col justify-center items-center w-full px-12">
        <!-- Logo -->
        <div v-if="showLogo" class="mb-8">
          <div class="w-20 h-20 bg-white rounded-2xl flex items-center justify-center shadow-2xl">
            <svg class="w-12 h-12 text-brand-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
            </svg>
          </div>
        </div>

        <!-- 标题 -->
        <h1 class="text-4xl font-bold text-white mb-4 text-center">
          {{ title }}
        </h1>

        <!-- 副标题 -->
        <p class="text-xl text-white/80 mb-12 text-center max-w-md">
          {{ subtitle }}
        </p>

        <!-- 特性列表 -->
        <div class="space-y-4 max-w-md">
          <div class="flex items-center text-white/90">
            <svg class="w-6 h-6 mr-3 text-brand-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span>支持多种 AI 编程工具的技能模板</span>
          </div>
          <div class="flex items-center text-white/90">
            <svg class="w-6 h-6 mr-3 text-brand-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span>企业级权限管理和版本控制</span>
          </div>
          <div class="flex items-center text-white/90">
            <svg class="w-6 h-6 mr-3 text-brand-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span>智能推荐，让技能主动找到你</span>
          </div>
        </div>
      </div>

      <!-- 底部版权 -->
      <div class="absolute bottom-6 left-0 right-0 text-center text-white/60 text-sm">
        &copy; {{ new Date().getFullYear() }} Skills Intelligence Hub. All rights reserved.
      </div>
    </div>

    <!-- 右侧表单区域 -->
    <div class="w-full lg:w-1/2 flex flex-col justify-center items-center px-6 py-12 bg-neutral-50">
      <!-- 移动端 Logo -->
      <div v-if="showLogo" class="lg:hidden mb-8">
        <div class="w-16 h-16 bg-brand-500 rounded-xl flex items-center justify-center">
          <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
          </svg>
        </div>
      </div>

      <!-- 表单容器 -->
      <div class="w-full max-w-md">
        <!-- 页面标题 - 移动端显示 -->
        <div class="lg:hidden text-center mb-8">
          <h1 class="text-2xl font-bold text-neutral-800">{{ title }}</h1>
          <p class="text-neutral-500 mt-2">{{ subtitle }}</p>
        </div>

        <!-- 表单卡片 -->
        <div class="bg-white rounded-2xl shadow-sm border border-neutral-200 p-8">
          <slot />
        </div>

        <!-- 底部链接 -->
        <div class="mt-6 text-center">
          <slot name="footer">
            <p v-if="isLoginPage" class="text-neutral-500 text-sm">
              还没有账号？
              <router-link to="/register" class="text-brand-500 hover:text-brand-600 font-medium">
                立即注册
              </router-link>
            </p>
            <p v-else-if="isRegisterPage" class="text-neutral-500 text-sm">
              已有账号？
              <router-link to="/login" class="text-brand-500 hover:text-brand-600 font-medium">
                立即登录
              </router-link>
            </p>
          </slot>
        </div>
      </div>

      <!-- 移动端版权 -->
      <div class="lg:hidden mt-8 text-center text-neutral-400 text-sm">
        &copy; {{ new Date().getFullYear() }} Skills Intelligence Hub
      </div>
    </div>
  </div>
</template>