<script setup lang="ts">
import ToastContainer from './design-system/elements/Toast/ToastContainer.vue'
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <router-view />
    <!-- 全局 Toast 通知容器 -->
    <ToastContainer />
  </div>
</template>