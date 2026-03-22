<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  initialQuery?: string
}>()

const emit = defineEmits<{
  search: [query: string]
}>()

const query = ref(props.initialQuery || '')

// 监听初始值变化
watch(() => props.initialQuery, (newVal) => {
  if (newVal) {
    query.value = newVal
  }
})

function handleSearch() {
  emit('search', query.value.trim())
}

function clearSearch() {
  query.value = ''
  emit('search', '')
}
</script>

<template>
  <div class="relative">
    <input
      v-model="query"
      type="text"
      placeholder="搜索技能名称、描述..."
      class="w-full px-4 py-2 pr-24 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
      @keyup.enter="handleSearch"
    />
    <div class="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-2">
      <button
        v-if="query"
        @click="clearSearch"
        class="text-gray-400 hover:text-gray-600"
      >
        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
        </svg>
      </button>
      <button
        @click="handleSearch"
        class="px-4 py-1 bg-indigo-600 text-white rounded-md text-sm hover:bg-indigo-700"
      >
        搜索
      </button>
    </div>
  </div>
</template>