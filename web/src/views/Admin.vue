<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUserStore } from '@/stores/user'
import { listMySkills, deleteSkill, type Skill } from '@/api/skills'
import { extractErrorMessage } from '@/api/index'
import AppLayout from '@/design-system/layouts/AppLayout.vue'
import Button from '@/design-system/elements/Button/Button.vue'

const userStore = useUserStore()

const mySkills = ref<Skill[]>([])
const loading = ref(false)
const deleting = ref<string | null>(null)
const error = ref('')

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
  if (!confirm(`Are you sure you want to delete "${skill.name}"? This action cannot be undone.`)) {
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

onMounted(() => {
  loadMySkills()
})
</script>

<template>
  <AppLayout title="Admin Panel" :show-sidebar="true">
    <div class="space-y-6">
      <!-- Page Header -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-neutral-800">Admin Panel</h1>
          <p class="text-neutral-500 mt-1">Manage your skills and settings</p>
        </div>
      </div>

      <!-- Account Info Card -->
      <div class="bg-white rounded-lg border border-neutral-200 p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">Account Information</h2>
        <div v-if="userStore.user" class="grid grid-cols-1 sm:grid-cols-3 gap-4 text-sm">
          <div>
            <span class="text-neutral-500">Username:</span>
            <span class="ml-2 font-medium text-neutral-800">{{ userStore.user.username }}</span>
          </div>
          <div>
            <span class="text-neutral-500">Email:</span>
            <span class="ml-2 text-neutral-800">{{ userStore.user.email }}</span>
          </div>
          <div>
            <span class="text-neutral-500">Role:</span>
            <span class="ml-2 px-2 py-1 bg-brand-100 text-brand-700 rounded text-xs font-medium">
              {{ userStore.user.role || 'user' }}
            </span>
          </div>
        </div>
      </div>

      <!-- Quick Navigation Cards -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <router-link
          to="/admin/users"
          class="bg-white rounded-lg border border-neutral-200 p-6 hover:border-brand-300 hover:shadow-sm transition-all"
        >
          <div class="flex items-center">
            <div class="w-12 h-12 bg-brand-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-brand-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            </div>
            <div class="ml-4">
              <h3 class="font-semibold text-neutral-800">User Management</h3>
              <p class="text-sm text-neutral-500">Manage users and roles</p>
            </div>
          </div>
        </router-link>

        <router-link
          to="/admin/groups"
          class="bg-white rounded-lg border border-neutral-200 p-6 hover:border-brand-300 hover:shadow-sm transition-all"
        >
          <div class="flex items-center">
            <div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
              </svg>
            </div>
            <div class="ml-4">
              <h3 class="font-semibold text-neutral-800">Department Management</h3>
              <p class="text-sm text-neutral-500">Organize teams and departments</p>
            </div>
          </div>
        </router-link>

        <router-link
          to="/admin/roles"
          class="bg-white rounded-lg border border-neutral-200 p-6 hover:border-brand-300 hover:shadow-sm transition-all"
        >
          <div class="flex items-center">
            <div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
            </div>
            <div class="ml-4">
              <h3 class="font-semibold text-neutral-800">Role Management</h3>
              <p class="text-sm text-neutral-500">Configure roles and permissions</p>
            </div>
          </div>
        </router-link>
      </div>

      <!-- My Skills Section -->
      <div class="bg-white rounded-lg border border-neutral-200 p-6">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-semibold text-neutral-800">My Skills</h2>
          <router-link
            to="/"
            class="text-sm text-brand-500 hover:text-brand-700"
          >
            Browse Market
          </router-link>
        </div>

        <!-- Loading State -->
        <div v-if="loading" class="text-center py-8">
          <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-brand-500 mx-auto"></div>
          <p class="text-neutral-500 mt-2">Loading...</p>
        </div>

        <!-- Error State -->
        <div v-else-if="error" class="text-center py-8">
          <p class="text-red-500">{{ error }}</p>
          <Button type="secondary" size="sm" class="mt-2" @click="loadMySkills">
            Retry
          </Button>
        </div>

        <!-- Empty State -->
        <div v-else-if="mySkills.length === 0" class="text-center py-8">
          <svg class="w-12 h-12 text-neutral-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
          </svg>
          <p class="text-neutral-500">No skills created yet</p>
          <p class="text-neutral-400 text-sm mt-1">Use the CLI tool to publish your first skill</p>
        </div>

        <!-- Skills Table -->
        <div v-else class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="border-b border-neutral-200">
                <th class="text-left py-3 px-4 text-sm font-medium text-neutral-500">Name</th>
                <th class="text-left py-3 px-4 text-sm font-medium text-neutral-500">Slug</th>
                <th class="text-left py-3 px-4 text-sm font-medium text-neutral-500">Version</th>
                <th class="text-left py-3 px-4 text-sm font-medium text-neutral-500">Downloads</th>
                <th class="text-right py-3 px-4 text-sm font-medium text-neutral-500">Actions</th>
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
                      View
                    </router-link>
                    <button
                      @click="handleDelete(skill)"
                      :disabled="deleting === skill.id"
                      class="text-red-500 hover:text-red-700 text-sm disabled:opacity-50"
                    >
                      {{ deleting === skill.id ? 'Deleting...' : 'Delete' }}
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Quick Actions Card -->
      <div class="bg-white rounded-lg border border-neutral-200 p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">Quick Actions</h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div class="p-4 bg-neutral-50 rounded-lg">
            <h3 class="font-medium text-neutral-800 mb-2">Publish a skill with CLI</h3>
            <code class="text-sm text-neutral-600 bg-neutral-100 px-2 py-1 rounded">
              skillhub push ./my-skill
            </code>
          </div>
          <div class="p-4 bg-neutral-50 rounded-lg">
            <h3 class="font-medium text-neutral-800 mb-2">Install a skill to project</h3>
            <code class="text-sm text-neutral-600 bg-neutral-100 px-2 py-1 rounded">
              skillhub pull skill-slug
            </code>
          </div>
        </div>
      </div>
    </div>
  </AppLayout>
</template>