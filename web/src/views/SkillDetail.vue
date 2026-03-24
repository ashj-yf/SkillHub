<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { getSkill, getSkillTags, getSkillByVersion, downloadSkill, type Skill, type SkillTag, type SkillDetailByVersion } from '@/api/skills'
import { extractErrorMessage } from '@/api/index'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const skill = ref<Skill | null>(null)
const tags = ref<SkillTag[]>([])
const selectedTag = ref<string>('')
const versionDetail = ref<SkillDetailByVersion | null>(null)
const loading = ref(true)
const loadingTags = ref(false)
const loadingVersion = ref(false)
const downloading = ref(false)
const error = ref('')

const defaultTag = computed(() => {
  if (tags.value.length > 0) {
    // 优先选择 latest 或第一个标签
    const latest = tags.value.find(t => t.tag === 'latest')
    return latest ? latest.tag : tags.value[0].tag
  }
  return ''
})

async function loadSkill() {
  const slug = route.params.slug as string

  if (!slug) {
    error.value = t('skill.invalidSlug')
    loading.value = false
    return
  }

  try {
    skill.value = await getSkill(slug)
    // 加载完成后获取标签列表
    await loadTags()
  } catch (e) {
    error.value = extractErrorMessage(e, t('skill.loadFailed'))
    console.error(e)
  } finally {
    loading.value = false
  }
}

async function loadTags() {
  if (!skill.value) return

  loadingTags.value = true
  try {
    tags.value = await getSkillTags(skill.value.slug)
    // 设置默认选中的标签
    if (tags.value.length > 0 && !selectedTag.value) {
      selectedTag.value = defaultTag.value
      // 加载默认版本的详细信息
      await loadVersionDetail()
    }
  } catch (e) {
    console.error('Failed to load tags:', e)
  } finally {
    loadingTags.value = false
  }
}

async function loadVersionDetail() {
  if (!skill.value || !selectedTag.value) return

  loadingVersion.value = true
  try {
    versionDetail.value = await getSkillByVersion(skill.value.slug, selectedTag.value)
  } catch (e) {
    console.error('Failed to load version detail:', e)
    versionDetail.value = null
  } finally {
    loadingVersion.value = false
  }
}

async function handleTagChange() {
  await loadVersionDetail()
}

async function handleDownload() {
  if (!skill.value) return

  downloading.value = true
  try {
    const blob = await downloadSkill(skill.value.slug, selectedTag.value || undefined)

    // 创建下载链接
    const url = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${skill.value.slug}${selectedTag.value ? `-${selectedTag.value}` : ''}.tar.gz`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    window.URL.revokeObjectURL(url)
  } catch (e) {
    alert(extractErrorMessage(e, t('skill.loadFailed')))
  } finally {
    downloading.value = false
  }
}

function formatDate(dateStr: string): string {
  if (!dateStr) return ''
  try {
    return new Date(dateStr).toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    })
  } catch {
    return dateStr
  }
}

// 监听路由参数变化
watch(() => route.params.slug, () => {
  if (route.params.slug) {
    loading.value = true
    error.value = ''
    skill.value = null
    tags.value = []
    selectedTag.value = ''
    versionDetail.value = null
    loadSkill()
  }
})

onMounted(() => {
  loadSkill()
})
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div v-if="loading" class="text-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600 mx-auto"></div>
      <p class="text-gray-500 mt-4">{{ t('common.loading') }}</p>
    </div>

    <div v-else-if="error" class="text-center py-12">
      <p class="text-red-500">{{ error }}</p>
      <button
        @click="router.push('/')"
        class="mt-4 px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
      >
        {{ t('skill.backHome') }}
      </button>
    </div>

    <template v-else-if="skill">
      <div class="bg-white shadow-sm rounded-lg border border-gray-200 p-8">
        <!-- 基本信息 -->
        <div class="flex justify-between items-start mb-6">
          <div>
            <h1 class="text-3xl font-bold text-gray-900">{{ skill.name }}</h1>
            <p class="text-gray-500 mt-1">{{ skill.slug }}</p>
          </div>
          <span class="px-3 py-1 bg-indigo-100 text-indigo-800 rounded-full text-sm font-medium">
            v{{ skill.version }}
          </span>
        </div>

        <!-- 标签 -->
        <div class="flex flex-wrap gap-2 mb-6">
          <span
            v-for="tag in skill.tags"
            :key="tag"
            class="px-2 py-1 bg-gray-100 text-gray-600 text-sm rounded"
          >
            {{ tag }}
          </span>
        </div>

        <!-- 描述 -->
        <p class="text-gray-600 mb-6">{{ skill.description || t('skill.noDescription') }}</p>

        <!-- 版本选择（Docker Tag 模式） -->
        <div class="border-t border-gray-200 pt-6 mb-6">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">{{ t('skill.versionSelect') }}</h2>

          <div v-if="loadingTags" class="text-gray-500">
            {{ t('skill.loadingVersions') }}
          </div>

          <div v-else-if="tags.length === 0" class="text-gray-500">
            {{ t('skill.noVersions') }}
          </div>

          <div v-else class="space-y-4">
            <div class="flex items-center gap-4">
              <label for="tag-select" class="text-sm font-medium text-gray-700 whitespace-nowrap">
                {{ t('skill.selectVersion') }}
              </label>
              <select
                id="tag-select"
                v-model="selectedTag"
                @change="handleTagChange"
                class="flex-1 max-w-xs px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
              >
                <option v-for="tag in tags" :key="tag.id" :value="tag.tag">
                  {{ tag.tag }} (v{{ tag.version }})
                </option>
              </select>
            </div>

            <!-- 选中版本的信息 -->
            <div v-if="loadingVersion" class="text-gray-500 text-sm">
              {{ t('skill.loadingDetails') }}
            </div>

            <div v-else-if="versionDetail" class="bg-gray-50 rounded-lg p-4 text-sm">
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                  <span class="text-gray-500">{{ t('skill.versionTag') }}:</span>
                  <span class="ml-2 font-medium">{{ selectedTag }}</span>
                </div>
                <div>
                  <span class="text-gray-500">{{ t('skill.version') }}:</span>
                  <span class="ml-2 font-medium">v{{ versionDetail.version_info?.version }}</span>
                </div>
              </div>

              <div v-if="versionDetail.version_info?.digest" class="mt-2">
                <span class="text-gray-500">Digest:</span>
                <span class="ml-2 font-mono text-xs">{{ versionDetail.version_info.digest?.slice(0, 12) }}...</span>
              </div>

              <!-- 内容预览 -->
              <div v-if="versionDetail.content" class="mt-4">
                <span class="text-gray-500">{{ t('skill.contentPreview') }}:</span>
                <pre class="mt-2 p-2 bg-white rounded text-xs overflow-x-auto max-h-32">{{ versionDetail.content?.slice(0, 500) }}{{ versionDetail.content && versionDetail.content.length > 500 ? '...' : '' }}</pre>
              </div>
            </div>

            <!-- 标签创建时间 -->
            <div v-if="selectedTag" class="text-sm text-gray-500">
              <template v-for="tag in tags" :key="tag.id">
                <span v-if="tag.tag === selectedTag">
                  {{ t('skill.updateTime') }}: {{ formatDate(tag.updated_at) }}
                </span>
              </template>
            </div>
          </div>
        </div>

        <!-- README -->
        <div class="border-t border-gray-200 pt-6">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">README</h2>
          <div class="prose prose-sm max-w-none">
            <pre class="whitespace-pre-wrap bg-gray-50 p-4 rounded-lg text-sm">{{ skill.readme || t('skill.noReadme') }}</pre>
          </div>
        </div>

        <!-- 底部操作栏 -->
        <div class="mt-6 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 text-sm text-gray-500 border-t border-gray-200 pt-6">
          <span>{{ t('skill.downloads', { count: skill.download_count }) }}</span>
          <div class="flex gap-3">
            <button
              @click="router.push('/')"
              class="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
            >
              {{ t('skill.backToList') }}
            </button>
            <button
              @click="handleDownload"
              :disabled="downloading"
              class="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ downloading ? t('skill.downloading') : t('skill.downloadSkill') }}
            </button>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>