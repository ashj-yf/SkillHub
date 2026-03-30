<script setup lang="ts">
/**
 * Skills Intelligence Hub - Version Upload Modal
 *
 * 技能版本上传弹窗组件
 */
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { uploadSkillVersion, type SkillVersion } from '@/api/skills'
import { extractErrorMessage } from '@/api/index'
import Button from '@/design-system/elements/Button/Button.vue'
import Input from '@/design-system/elements/Input/Input.vue'

const props = defineProps<{
  slug: string
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
  success: [version: SkillVersion]
}>()

const { t } = useI18n()

// 表单数据
const version = ref('')
const changelog = ref('')
const selectedFile = ref<File | null>(null)
const uploading = ref(false)
const error = ref('')
const uploadProgress = ref(0)

// 计算属性
const isValidVersion = computed(() => {
  return /^v\d+\.\d+\.\d+$/.test(version.value)
})

const canSubmit = computed(() => {
  return isValidVersion.value && selectedFile.value && !uploading.value
})

const fileName = computed(() => {
  return selectedFile.value?.name || ''
})

const fileSize = computed(() => {
  if (!selectedFile.value) return ''
  const size = selectedFile.value.size
  if (size >= 1024 * 1024) {
    return `${(size / (1024 * 1024)).toFixed(2)} MB`
  }
  return `${(size / 1024).toFixed(2)} KB`
})

// 文件选择处理
function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    const file = target.files[0]

    // 验证文件类型
    if (!file.name.endsWith('.tar.gz')) {
      error.value = t('upload.onlyTarGz')
      return
    }

    // 验证文件大小 (10MB)
    if (file.size > 10 * 1024 * 1024) {
      error.value = t('upload.fileTooLarge')
      return
    }

    selectedFile.value = file
    error.value = ''
  }
}

// 上传处理
async function handleUpload() {
  if (!canSubmit.value || !selectedFile.value) return

  uploading.value = true
  error.value = ''
  uploadProgress.value = 0

  try {
    const result = await uploadSkillVersion(props.slug, {
      version: version.value,
      file: selectedFile.value,
      changelog: changelog.value || undefined,
    })

    emit('success', result)
    resetForm()
  } catch (e) {
    error.value = extractErrorMessage(e, t('upload.failed'))
  } finally {
    uploading.value = false
    uploadProgress.value = 0
  }
}

// 重置表单
function resetForm() {
  version.value = ''
  changelog.value = ''
  selectedFile.value = null
  error.value = ''
  uploadProgress.value = 0
}

// 关闭弹窗
function handleClose() {
  if (!uploading.value) {
    resetForm()
    emit('close')
  }
}

// 阻止背景点击
function handleBackdropClick(event: MouseEvent) {
  if (event.target === event.currentTarget) {
    handleClose()
  }
}
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="visible"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 p-4"
        @click="handleBackdropClick"
      >
        <div class="bg-white rounded-xl shadow-xl w-full max-w-md">
          <!-- Header -->
          <div class="flex items-center justify-between px-6 py-4 border-b border-neutral-200">
            <h3 class="text-lg font-semibold text-neutral-800">
              {{ t('upload.title') }}
            </h3>
            <button
              @click="handleClose"
              :disabled="uploading"
              class="text-neutral-400 hover:text-neutral-600 disabled:opacity-50"
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

            <!-- Version Input -->
            <Input
              v-model="version"
              :label="t('upload.version')"
              :placeholder="t('upload.versionPlaceholder')"
              :hint="t('upload.versionHint')"
              :disabled="uploading"
              :state="version && !isValidVersion ? 'error' : 'default'"
              :error-message="version && !isValidVersion ? t('upload.versionFormat') : ''"
            />

            <!-- File Upload -->
            <div>
              <label class="block text-sm font-medium text-neutral-700 mb-1">
                {{ t('upload.file') }}
              </label>
              <div
                class="border-2 border-dashed border-neutral-300 rounded-lg p-4 text-center hover:border-brand-500 transition-colors cursor-pointer"
                :class="{ 'border-brand-500 bg-brand-50': selectedFile }"
              >
                <input
                  type="file"
                  accept=".tar.gz"
                  :disabled="uploading"
                  class="hidden"
                  @change="handleFileSelect"
                  id="file-upload"
                />
                <label for="file-upload" class="cursor-pointer">
                  <svg
                    class="w-8 h-8 mx-auto text-neutral-400 mb-2"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                    />
                  </svg>
                  <template v-if="selectedFile">
                    <p class="text-sm text-brand-600 font-medium">{{ fileName }}</p>
                    <p class="text-xs text-neutral-500">{{ fileSize }}</p>
                  </template>
                  <template v-else>
                    <p class="text-sm text-neutral-600">{{ t('upload.selectFile') }}</p>
                    <p class="text-xs text-neutral-400 mt-1">{{ t('upload.fileHint') }}</p>
                  </template>
                </label>
              </div>
            </div>

            <!-- Changelog -->
            <div>
              <label class="block text-sm font-medium text-neutral-700 mb-1">
                {{ t('upload.changelog') }}
                <span class="text-neutral-400 font-normal">{{ t('common.optional') }}</span>
              </label>
              <textarea
                v-model="changelog"
                :disabled="uploading"
                :placeholder="t('upload.changelogPlaceholder')"
                rows="3"
                class="w-full px-3 py-2 border border-neutral-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-brand-500 focus:border-transparent resize-none"
              ></textarea>
            </div>

            <!-- Upload Progress -->
            <div v-if="uploading" class="space-y-2">
              <div class="flex items-center justify-between text-sm">
                <span class="text-neutral-600">{{ t('upload.uploading') }}</span>
                <span class="text-brand-600">{{ uploadProgress }}%</span>
              </div>
              <div class="w-full bg-neutral-200 rounded-full h-2">
                <div
                  class="bg-brand-500 h-2 rounded-full transition-all duration-300"
                  :style="{ width: `${uploadProgress}%` }"
                ></div>
              </div>
            </div>
          </div>

          <!-- Footer -->
          <div class="flex justify-end gap-3 px-6 py-4 border-t border-neutral-200 bg-neutral-50 rounded-b-xl">
            <Button
              type="secondary"
              :disabled="uploading"
              @click="handleClose"
            >
              {{ t('common.cancel') }}
            </Button>
            <Button
              type="primary"
              :loading="uploading"
              :disabled="!canSubmit"
              @click="handleUpload"
            >
              {{ t('upload.upload') }}
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