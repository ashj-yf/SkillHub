<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { getSkill, getSkillTags, getSkillByVersion, downloadSkill, type Skill, type SkillTag, type SkillDetailByVersion } from '@/api/skills'
import { extractErrorMessage } from '@/api/index'

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
    const latest = tags.value.find(t => t.name === 'latest')
    return latest ? latest.name : tags.value[0].name
  }
  return ''
})

async function loadSkill() {
  const slug = route.params.slug as string

  if (!slug) {
    error.value = '无效的技能标识'
    loading.value = false
    return
  }

  try {
    skill.value = await getSkill(slug)
    // 加载完成后获取标签列表
    await loadTags()
  } catch (e) {
    error.value = extractErrorMessage(e, '加载失败')
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
    alert(extractErrorMessage(e, '下载失败'))
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

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
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
      <p class="text-gray-500 mt-4">加载中...</p>
    </div>

    <div v-else-if="error" class="text-center py-12">
      <p class="text-red-500">{{ error }}</p>
      <button
        @click="router.push('/')"
        class="mt-4 px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
      >
        返回首页
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
        <p class="text-gray-600 mb-6">{{ skill.description }}</p>

        <!-- 版本选择（Docker Tag 模式） -->
        <div class="border-t border-gray-200 pt-6 mb-6">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">版本选择</h2>

          <div v-if="loadingTags" class="text-gray-500">
            加载版本列表...
          </div>

          <div v-else-if="tags.length === 0" class="text-gray-500">
            暂无可用版本
          </div>

          <div v-else class="space-y-4">
            <div class="flex items-center gap-4">
              <label for="tag-select" class="text-sm font-medium text-gray-700 whitespace-nowrap">
                选择版本:
              </label>
              <select
                id="tag-select"
                v-model="selectedTag"
                @change="handleTagChange"
                class="flex-1 max-w-xs px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
              >
                <option v-for="tag in tags" :key="tag.name" :value="tag.name">
                  {{ tag.name }}
                </option>
              </select>
            </div>

            <!-- 选中版本的信息 -->
            <div v-if="loadingVersion" class="text-gray-500 text-sm">
              加载版本详情...
            </div>

            <div v-else-if="versionDetail" class="bg-gray-50 rounded-lg p-4 text-sm">
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                  <span class="text-gray-500">版本标签:</span>
                  <span class="ml-2 font-medium">{{ versionDetail.tag }}</span>
                </div>
                <div>
                  <span class="text-gray-500">Digest:</span>
                  <span class="ml-2 font-mono text-xs">{{ versionDetail.manifest?.slice(0, 12) }}...</span>
                </div>
              </div>

              <!-- 文件列表 -->
              <div v-if="versionDetail.files && versionDetail.files.length > 0" class="mt-4">
                <span class="text-gray-500">文件列表:</span>
                <ul class="mt-2 space-y-1">
                  <li
                    v-for="file in versionDetail.files"
                    :key="file.name"
                    class="flex justify-between items-center py-1 px-2 bg-white rounded"
                  >
                    <span class="font-mono text-xs">{{ file.name }}</span>
                    <span class="text-gray-400 text-xs">{{ formatSize(file.size) }}</span>
                  </li>
                </ul>
              </div>
            </div>

            <!-- 标签创建时间 -->
            <div v-if="selectedTag" class="text-sm text-gray-500">
              <template v-for="tag in tags" :key="tag.name">
                <span v-if="tag.name === selectedTag">
                  创建时间: {{ formatDate(tag.created_at) }}
                </span>
              </template>
            </div>
          </div>
        </div>

        <!-- README -->
        <div class="border-t border-gray-200 pt-6">
          <h2 class="text-lg font-semibold text-gray-900 mb-4">README</h2>
          <div class="prose prose-sm max-w-none">
            <pre class="whitespace-pre-wrap bg-gray-50 p-4 rounded-lg text-sm">{{ skill.readme || '暂无说明' }}</pre>
          </div>
        </div>

        <!-- 底部操作栏 -->
        <div class="mt-6 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 text-sm text-gray-500 border-t border-gray-200 pt-6">
          <span>{{ skill.download_count }} 次下载</span>
          <div class="flex gap-3">
            <button
              @click="router.push('/')"
              class="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50"
            >
              返回列表
            </button>
            <button
              @click="handleDownload"
              :disabled="downloading"
              class="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ downloading ? '下载中...' : '下载技能' }}
            </button>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>