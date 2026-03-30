<script setup lang="ts">
/**
 * Skills Intelligence Hub - CLI Download Modal
 *
 * CLI 下载弹窗组件
 */
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { getCliVersion, type CliVersionResponse, type CliDownload } from '@/api/cli'
import { extractErrorMessage } from '@/api/index'
import Button from '@/design-system/elements/Button/Button.vue'

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const { t } = useI18n()

// 状态
const cliVersion = ref<CliVersionResponse | null>(null)
const loading = ref(false)
const error = ref('')

// 平台配置
const platformConfig = [
  { id: 'linux-x86_64', label: 'Linux x86_64', icon: '🐧' },
  { id: 'linux-arm64', label: 'Linux ARM64', icon: '🐧' },
  { id: 'macos-x86_64', label: 'macOS x86_64', icon: '🍎' },
  { id: 'macos-arm64', label: 'macOS ARM64', icon: '🍎' },
  { id: 'windows-x86_64', label: 'Windows x86_64', icon: '🪟' },
]

// 计算各平台的下载信息
const platformDownloads = computed(() => {
  if (!cliVersion.value?.downloads) return []

  return platformConfig.map((platform) => {
    const download = cliVersion.value!.downloads.find(
      (d: CliDownload) => d.platform === platform.id
    )
    return {
      ...platform,
      download,
      available: !!download,
    }
  })
})

// 格式化文件大小
function formatSize(bytes?: number): string {
  if (!bytes) return ''
  if (bytes >= 1024 * 1024) {
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
  }
  return `${(bytes / 1024).toFixed(2)} KB`
}

// 加载 CLI 版本信息
async function loadCliVersion() {
  loading.value = true
  error.value = ''

  try {
    cliVersion.value = await getCliVersion()
  } catch (e) {
    error.value = extractErrorMessage(e, '获取 CLI 版本信息失败')
  } finally {
    loading.value = false
  }
}

// 关闭弹窗
function handleClose() {
  emit('close')
}

// 阻止背景点击
function handleBackdropClick(event: MouseEvent) {
  if (event.target === event.currentTarget) {
    handleClose()
  }
}

// 下载按钮点击
function handleDownload(url: string) {
  window.open(url, '_blank')
}

// 初始化
onMounted(() => {
  if (props.visible) {
    loadCliVersion()
  }
})
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="visible"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 p-4"
        @click="handleBackdropClick"
      >
        <div class="bg-white rounded-xl shadow-xl w-full max-w-lg">
          <!-- Header -->
          <div class="flex items-center justify-between px-6 py-4 border-b border-neutral-200">
            <h3 class="text-lg font-semibold text-neutral-800">
              下载 CLI 工具
            </h3>
            <button
              @click="handleClose"
              class="text-neutral-400 hover:text-neutral-600"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <!-- Body -->
          <div class="px-6 py-4 space-y-4">
            <!-- Error Message -->
            <div
              v-if="error"
              class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg text-sm"
            >
              {{ error }}
            </div>

            <!-- Loading State -->
            <div v-if="loading" class="flex items-center justify-center py-8">
              <svg
                class="animate-spin h-8 w-8 text-brand-500"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  class="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  stroke-width="4"
                />
                <path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
              </svg>
            </div>

            <!-- Version Info -->
            <div v-if="!loading && cliVersion" class="space-y-4">
              <!-- Version Badge -->
              <div class="flex items-center gap-3">
                <span class="inline-flex items-center px-3 py-1 rounded-full bg-brand-100 text-brand-600 text-sm font-medium">
                  v{{ cliVersion.version }}
                </span>
                <span class="text-sm text-neutral-500">
                  发布日期: {{ cliVersion.release_date }}
                </span>
              </div>

              <!-- Download Buttons -->
              <div class="space-y-2">
                <h4 class="text-sm font-medium text-neutral-700">选择平台下载</h4>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                  <button
                    v-for="platform in platformDownloads"
                    :key="platform.id"
                    :disabled="!platform.available"
                    :class="[
                      'flex items-center gap-3 px-4 py-3 rounded-lg border transition-all text-left',
                      platform.available
                        ? 'border-neutral-200 hover:border-brand-500 hover:bg-brand-50 cursor-pointer'
                        : 'border-neutral-100 bg-neutral-50 cursor-not-allowed opacity-50'
                    ]"
                    @click="platform.download && handleDownload(platform.download.url)"
                  >
                    <span class="text-2xl">{{ platform.icon }}</span>
                    <div class="flex-1">
                      <div class="text-sm font-medium text-neutral-700">
                        {{ platform.label }}
                      </div>
                      <div v-if="platform.download" class="text-xs text-neutral-500">
                        {{ formatSize(platform.download.size) }}
                      </div>
                      <div v-else class="text-xs text-neutral-400">
                        暂不可用
                      </div>
                    </div>
                    <svg
                      v-if="platform.available"
                      class="w-5 h-5 text-brand-500"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                    </svg>
                  </button>
                </div>
              </div>

              <!-- Changelog -->
              <div v-if="cliVersion.changelog" class="space-y-2">
                <h4 class="text-sm font-medium text-neutral-700">更新日志</h4>
                <div class="bg-neutral-50 rounded-lg p-3 text-sm text-neutral-600 whitespace-pre-wrap">
                  {{ cliVersion.changelog }}
                </div>
              </div>
            </div>
          </div>

          <!-- Footer -->
          <div class="flex justify-end gap-3 px-6 py-4 border-t border-neutral-200 bg-neutral-50 rounded-b-xl">
            <Button type="secondary" @click="handleClose">
              关闭
            </Button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .bg-white,
.modal-leave-active .bg-white {
  transition: transform 0.2s ease;
}

.modal-enter-from .bg-white,
.modal-leave-to .bg-white {
  transform: scale(0.95);
}
</style>