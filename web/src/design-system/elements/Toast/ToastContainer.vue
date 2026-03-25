<script setup lang="ts">
/**
 * Skills Intelligence Hub - Toast Container
 *
 * 全局 Toast 通知容器，需要在 App.vue 中引入
 */
import { useToastStore } from '@/stores/toast'
import Toast from './Toast.vue'

const toastStore = useToastStore()
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed top-20 right-4 z-[100] flex flex-col gap-2 max-w-sm w-full pointer-events-none"
    >
      <TransitionGroup name="toast">
        <Toast
          v-for="toast in toastStore.toasts"
          :key="toast.id"
          v-bind="toast"
          class="pointer-events-auto"
          @close="toastStore.removeToast(toast.id)"
        />
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%);
}
</style>