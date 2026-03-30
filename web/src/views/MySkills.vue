<script setup lang="ts">
/**
 * Skills Intelligence Hub - My Skills Page
 *
 * 用户管理自己创建的技能
 */
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { listMySkills, deleteSkill, type Skill, type SkillVersion } from '@/api/skills'
import { extractErrorMessage } from '@/api/index'
import AppLayout from '@/design-system/layouts/AppLayout.vue'
import Button from '@/design-system/elements/Button/Button.vue'
import VersionUploadModal from '@/components/VersionUploadModal.vue'

const { t } = useI18n()

const mySkills = ref<Skill[]>([])
const loading = ref(false)
const deleting = ref<string | null>(null)
const error = ref('')

// 上传弹窗状态
const showUploadModal = ref(false)
const selectedSkillSlug = ref('')

async function loadMySkills() {
  loading.value = true
  error.value = ''
  try {
    mySkills.value = await listMySkills()
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to load skills')
    console.error(e)
  } finally {
    loading.value = false
  }
}

async function handleDelete(skill: Skill) {
  if (!confirm(t('admin.confirmDelete', { name: skill.name }))) {
    return
  }

  deleting.value = skill.id
  try {
    await deleteSkill(skill.slug)
    mySkills.value = mySkills.value.filter(s => s.id !== skill.id)
  } catch (e) {
    alert(extractErrorMessage(e, 'Failed to delete'))
  } finally {
    deleting.value = null
  }
}

function openUploadModal(skill: Skill) {
  selectedSkillSlug.value = skill.slug
  showUploadModal.value = true
}

function handleUploadSuccess(version: SkillVersion) {
  showUploadModal.value = false
  // 刷新列表
  loadMySkills()
}

onMounted(() => {
  loadMySkills()
})
</script>

<template>
  <AppLayout :title="t('appLayout.mySkills')" :show-sidebar="true">
    <div class="space-y-6">
      <!-- Page Header -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-neutral-800">{{ t('appLayout.mySkills') }}</h1>
          <p class="text-neutral-500 mt-1">{{ t('admin.useCliTip') }}</p>
        </div>
        <router-link
          to="/"
          class="text-sm text-brand-500 hover:text-brand-700"
        >
          {{ t('admin.browseMarket') }}
        </router-link>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="text-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-brand-500 mx-auto"></div>
        <p class="text-neutral-500 mt-4">{{ t('common.loading') }}</p>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
        {{ error }}
        <Button type="secondary" size="sm" class="ml-2" @click="loadMySkills">
          {{ t('common.retry') }}
        </Button>
      </div>

      <!-- Empty State -->
      <div v-else-if="mySkills.length === 0" class="text-center py-12 bg-white rounded-lg border border-neutral-200">
        <svg class="w-16 h-16 text-neutral-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
        </svg>
        <p class="text-neutral-500">{{ t('admin.noSkills') }}</p>
        <p class="text-neutral-400 text-sm mt-1">{{ t('admin.useCliTip') }}</p>
      </div>

      <!-- Skills Table -->
      <div v-else class="bg-white rounded-lg border border-neutral-200 overflow-hidden">
        <table class="w-full">
          <thead>
            <tr class="border-b border-neutral-200 bg-neutral-50">
              <th class="text-left py-3 px-4 text-sm font-medium text-neutral-500">{{ t('common.name') }}</th>
              <th class="text-left py-3 px-4 text-sm font-medium text-neutral-500">{{ t('admin.slug') }}</th>
              <th class="text-left py-3 px-4 text-sm font-medium text-neutral-500">{{ t('skill.version') }}</th>
              <th class="text-left py-3 px-4 text-sm font-medium text-neutral-500">{{ t('admin.downloads') }}</th>
              <th class="text-right py-3 px-4 text-sm font-medium text-neutral-500">{{ t('common.actions') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="skill in mySkills"
              :key="skill.id"
              class="border-b border-neutral-100 hover:bg-neutral-50"
            >
              <td class="py-3 px-4">
                <router-link
                  :to="`/skill/${skill.slug}`"
                  class="text-brand-500 hover:text-brand-700 font-medium"
                >
                  {{ skill.name }}
                </router-link>
              </td>
              <td class="py-3 px-4 text-neutral-500 text-sm">{{ skill.slug }}</td>
              <td class="py-3 px-4">
                <span class="px-2 py-1 bg-neutral-100 text-neutral-600 rounded text-sm">
                  v{{ skill.version }}
                </span>
              </td>
              <td class="py-3 px-4 text-neutral-500 text-sm">{{ skill.download_count }}</td>
              <td class="py-3 px-4 text-right">
                <div class="flex justify-end gap-2">
                  <router-link
                    :to="`/skill/${skill.slug}`"
                    class="text-brand-500 hover:text-brand-700 text-sm"
                  >
                    {{ t('admin.view') }}
                  </router-link>
                  <button
                    @click="openUploadModal(skill)"
                    class="text-brand-500 hover:text-brand-700 text-sm"
                  >
                    {{ t('upload.upload') }}
                  </button>
                  <button
                    @click="handleDelete(skill)"
                    :disabled="deleting === skill.id"
                    class="text-red-500 hover:text-red-700 text-sm disabled:opacity-50"
                  >
                    {{ deleting === skill.id ? t('admin.deleting') : t('common.delete') }}
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Quick Actions Card -->
      <div class="bg-white rounded-lg border border-neutral-200 p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">{{ t('admin.quickActions') }}</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div class="p-4 bg-neutral-50 rounded-lg">
            <h3 class="font-medium text-neutral-800 mb-2">{{ t('admin.publishSkillCmd') }}</h3>
            <code class="text-sm text-neutral-600 bg-neutral-100 px-2 py-1 rounded">
              skillhub push ./my-skill
            </code>
          </div>
          <div class="p-4 bg-neutral-50 rounded-lg">
            <h3 class="font-medium text-neutral-800 mb-2">{{ t('admin.installSkillCmd') }}</h3>
            <code class="text-sm text-neutral-600 bg-neutral-100 px-2 py-1 rounded">
              skillhub pull skill-slug
            </code>
          </div>
        </div>
      </div>
    </div>

    <!-- Upload Modal -->
    <VersionUploadModal
      v-if="selectedSkillSlug"
      :slug="selectedSkillSlug"
      :visible="showUploadModal"
      @close="showUploadModal = false"
      @success="handleUploadSuccess"
    />
  </AppLayout>
</template>