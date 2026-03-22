<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  skill: {
    id: string
    name: string
    slug: string
    description: string
    version: string
    tags: string[]
    download_count: number
  }
}>()

const truncatedDescription = computed(() => {
  if (!props.skill.description) return ''
  const desc = props.skill.description
  if (desc.length > 100) {
    return desc.slice(0, 100) + '...'
  }
  return desc
})

const displayTags = computed(() => {
  return (props.skill.tags || []).slice(0, 3)
})
</script>

<template>
  <router-link
    :to="`/skill/${skill.slug}`"
    class="block bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow"
  >
    <div class="flex justify-between items-start">
      <h3 class="text-lg font-semibold text-gray-900">{{ skill.name }}</h3>
      <span class="text-sm text-gray-500">v{{ skill.version }}</span>
    </div>

    <p class="mt-2 text-gray-600 text-sm">{{ truncatedDescription }}</p>

    <div class="mt-4 flex items-center justify-between">
      <div class="flex flex-wrap gap-2">
        <span
          v-for="tag in displayTags"
          :key="tag"
          class="px-2 py-1 bg-gray-100 text-gray-600 text-xs rounded"
        >
          {{ tag }}
        </span>
        <span
          v-if="skill.tags && skill.tags.length > 3"
          class="px-2 py-1 bg-gray-100 text-gray-600 text-xs rounded"
        >
          +{{ skill.tags.length - 3 }}
        </span>
      </div>

      <span class="text-xs text-gray-400">{{ skill.download_count }} 次下载</span>
    </div>
  </router-link>
</template>