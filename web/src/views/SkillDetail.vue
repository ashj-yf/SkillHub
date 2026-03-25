<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { getSkill, getSkillTags, getSkillByVersion, downloadSkill, type Skill, type SkillTag, type SkillDetailByVersion } from '@/api/skills'
import { extractErrorMessage } from '@/api/index'
import { useToastStore } from '@/stores/toast'
import AppLayout from '@/design-system/layouts/AppLayout.vue'
import Button from '@/design-system/elements/Button/Button.vue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const toast = useToastStore()

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

  loading.value = true
  error.value = ''

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

    // 下载成功提示
    toast.success(t('skill.downloadSuccess'))
  } catch (e) {
    const errorMsg = extractErrorMessage(e, t('skill.downloadFailed'))
    toast.error(errorMsg)
  } finally {
    downloading.value = false
  }
}

function handleRetry() {
  loadSkill()
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
  <AppLayout :title="skill?.name || t('skill.loading')" :show-sidebar="true">
    <div class="max-w-4xl mx-auto">
      <!-- Loading State -->
      <div v-if="loading" class="text-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-brand-500 mx-auto"></div>
        <p class="text-neutral-500 mt-4">{{ t('common.loading') }}</p>
      </div>

      <!-- Error State with Retry -->
      <div v-else-if="error" class="text-center py-12">
        <svg class="w-16 h-16 text-neutral-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
        </svg>
        <p class="text-red-500 mb-4">{{ error }}</p>
        <div class="flex justify-center gap-3">
          <Button type="secondary" @click="router.push('/')">
            {{ t('skill.backHome') }}
          </Button>
          <Button type="primary" @click="handleRetry">
            {{ t('common.retry') }}
          </Button>
        </div>
      </div>

      <!-- Skill Detail -->
      <template v-else-if="skill">
        <div class="bg-white shadow-sm rounded-lg border border-neutral-200 p-8">
          <!-- 基本信息 -->
          <div class="flex justify-between items-start mb-6">
            <div>
              <h1 class="text-3xl font-bold text-neutral-900">{{ skill.name }}</h1>
              <p class="text-neutral-500 mt-1">{{ skill.slug }}</p>
            </div>
            <span class="px-3 py-1 bg-brand-100 text-brand-800 rounded-full text-sm font-medium">
              v{{ skill.version }}
            </span>
          </div>

          <!-- 标签 -->
          <div class="flex flex-wrap gap-2 mb-6">
            <span
              v-for="tag in skill.tags"
              :key="tag"
              class="px-2 py-1 bg-neutral-100 text-neutral-600 text-sm rounded"
            >
              {{ tag }}
            </span>
          </div>

          <!-- 描述 -->
          <p class="text-neutral-600 mb-6">{{ skill.description || t('skill.noDescription') }}</p>

          <!-- 版本选择（Docker Tag 模式） -->
          <div class="border-t border-neutral-200 pt-6 mb-6">
            <h2 class="text-lg font-semibold text-neutral-900 mb-4">{{ t('skill.versionSelect') }}</h2>

            <div v-if="loadingTags" class="text-neutral-500">
              {{ t('skill.loadingVersions') }}
            </div>

            <div v-else-if="tags.length === 0" class="text-neutral-500">
              {{ t('skill.noVersions') }}
            </div>

            <div v-else class="space-y-4">
              <div class="flex items-center gap-4">
                <label for="tag-select" class="text-sm font-medium text-neutral-700 whitespace-nowrap">
                  {{ t('skill.selectVersion') }}
                </label>
                <select
                  id="tag-select"
                  v-model="selectedTag"
                  @change="handleTagChange"
                  class="flex-1 max-w-xs px-3 py-2 border border-neutral-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-brand-500"
                >
                  <option v-for="tag in tags" :key="tag.id" :value="tag.tag">
                    {{ tag.tag }} (v{{ tag.version }})
                  </option>
                </select>
              </div>

              <!-- 选中版本的信息 -->
              <div v-if="loadingVersion" class="text-neutral-500 text-sm">
                {{ t('skill.loadingDetails') }}
              </div>

              <div v-else-if="versionDetail" class="bg-neutral-50 rounded-lg p-4 text-sm">
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <div>
                    <span class="text-neutral-500">{{ t('skill.versionTag') }}:</span>
                    <span class="ml-2 font-medium">{{ selectedTag }}</span>
                  </div>
                  <div>
                    <span class="text-neutral-500">{{ t('skill.version') }}:</span>
                    <span class="ml-2 font-medium">v{{ versionDetail.version_info?.version }}</span>
                  </div>
                </div>

                <div v-if="versionDetail.version_info?.digest" class="mt-2">
                  <span class="text-neutral-500">Digest:</span>
                  <span class="ml-2 font-mono text-xs">{{ versionDetail.version_info.digest?.slice(0, 12) }}...</span>
                </div>

                <!-- 内容预览 -->
                <div v-if="versionDetail.content" class="mt-4">
                  <span class="text-neutral-500">{{ t('skill.contentPreview') }}:</span>
                  <pre class="mt-2 p-2 bg-white rounded text-xs overflow-x-auto max-h-32">{{ versionDetail.content?.slice(0, 500) }}{{ versionDetail.content && versionDetail.content.length > 500 ? '...' : '' }}</pre>
                </div>
              </div>

              <!-- 标签创建时间 -->
              <div v-if="selectedTag" class="text-sm text-neutral-500">
                <template v-for="tag in tags" :key="tag.id">
                  <span v-if="tag.tag === selectedTag">
                    {{ t('skill.updateTime') }}: {{ formatDate(tag.updated_at) }}
                  </span>
                </template>
              </div>
            </div>
          </div>

          <!-- README -->
          <div class="border-t border-neutral-200 pt-6">
            <h2 class="text-lg font-semibold text-neutral-900 mb-4">README</h2>
            <div class="prose prose-sm max-w-none">
              <pre class="whitespace-pre-wrap bg-neutral-50 p-4 rounded-lg text-sm">{{ skill.readme || t('skill.noReadme') }}</pre>
            </div>
          </div>

          <!-- 底部操作栏 -->
          <div class="mt-6 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 text-sm text-neutral-500 border-t border-neutral-200 pt-6">
            <span>{{ t('skill.downloads', { count: skill.download_count }) }}</span>
            <div class="flex gap-3">
              <Button type="secondary" @click="router.push('/')">
                {{ t('skill.backToList') }}
              </Button>
              <Button
                type="primary"
                :loading="downloading"
                :disabled="downloading"
                @click="handleDownload"
              >
                {{ downloading ? t('skill.downloading') : t('skill.downloadSkill') }}
              </Button>
            </div>
          </div>
        </div>
      </template>
    </div>
  </AppLayout>
</template>