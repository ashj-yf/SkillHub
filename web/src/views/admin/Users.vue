<script setup lang="ts">
/**
 * Skills Intelligence Hub - Users Admin Page
 *
 * User management page with list, role assignment, and enable/disable functionality
 *
 * API 状态（参考 docs/api-spec.yaml）:
 * - GET /users - 已实现
 * - GET /users/{id} - 已实现
 * - PUT /users/{id} - 已实现
 * - DELETE /users/{id} - 已实现
 * - POST /users/{id}/roles - 已实现（使用角色名称，非 UUID）
 * - DELETE /users/{id}/roles/{role} - 已实现（使用角色名称，非 UUID）
 * - GET /users/{id}/groups - 已实现（用户所属组）
 */
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { listRoles, type Role } from '@/api/roles'
import {
  listUsers,
  updateUser,
  assignRole,
  removeRole,
  getUserGroups,
  type User,
  type UserGroup,
} from '@/api/users'
import { extractErrorMessage } from '@/api/index'
import Button from '@/design-system/elements/Button/Button.vue'
import Input from '@/design-system/elements/Input/Input.vue'
import Tag from '@/design-system/elements/Tag/Tag.vue'
import AppLayout from '@/design-system/layouts/AppLayout.vue'

const { t } = useI18n()

// State
const users = ref<User[]>([])
const roles = ref<Role[]>([])
const loading = ref(true)
const error = ref('')
const searchQuery = ref('')
const selectedUser = ref<User | null>(null)
const isRoleModalOpen = ref(false)
const selectedRoleId = ref('')
const operationLoading = ref(false)

// 用户组相关状态
const isGroupModalOpen = ref(false)
const userGroups = ref<UserGroup[]>([])
const groupLoading = ref(false)

// Map role IDs to role names for display
const roleIdToName = computed(() => {
  const map = new Map<string, string>()
  for (const role of roles.value) {
    map.set(role.id, role.name)
  }
  return map
})

// Computed
const filteredUsers = computed(() => {
  if (!searchQuery.value) return users.value
  const query = searchQuery.value.toLowerCase()
  return users.value.filter(
    (user) =>
      user.username.toLowerCase().includes(query) ||
      user.email.toLowerCase().includes(query)
  )
})

// Methods
async function loadData() {
  loading.value = true
  error.value = ''
  try {
    const [usersData, rolesData] = await Promise.all([listUsers(), listRoles()])
    users.value = usersData
    roles.value = rolesData
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to load data. Note: User management endpoints may not be implemented yet.')
  } finally {
    loading.value = false
  }
}

async function toggleUserStatus(user: User) {
  try {
    const updated = await updateUser(user.id, { is_active: !user.is_active })
    const index = users.value.findIndex((u) => u.id === user.id)
    if (index !== -1) {
      users.value[index] = updated
    }
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to update user status')
  }
}

function openRoleModal(user: User) {
  selectedUser.value = user
  isRoleModalOpen.value = true
}

function closeRoleModal() {
  selectedUser.value = null
  isRoleModalOpen.value = false
  selectedRoleId.value = ''
}

async function handleAssignRole() {
  if (!selectedUser.value || !selectedRoleId.value) return

  operationLoading.value = true
  try {
    // API 使用角色名称（非 UUID），需要从 ID 获取名称
    const roleName = roleIdToName.value.get(selectedRoleId.value)
    if (!roleName) {
      error.value = 'Cannot find role name'
      return
    }
    await assignRole(selectedUser.value.id, roleName)
    // 更新本地状态
    const index = users.value.findIndex((u) => u.id === selectedUser.value!.id)
    if (index !== -1 && !users.value[index].roles.includes(roleName)) {
      users.value[index].roles.push(roleName)
    }
    closeRoleModal()
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to assign role')
  } finally {
    operationLoading.value = false
  }
}

async function handleRemoveRole(user: User, roleName: string) {
  try {
    // API 使用角色名称（非 UUID）
    await removeRole(user.id, roleName)
    const index = users.value.findIndex((u) => u.id === user.id)
    if (index !== -1) {
      users.value[index].roles = users.value[index].roles.filter((r) => r !== roleName)
    }
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to remove role')
  }
}

// 用户组相关方法
async function openGroupModal(user: User) {
  selectedUser.value = user
  isGroupModalOpen.value = true
  groupLoading.value = true
  userGroups.value = []

  try {
    userGroups.value = await getUserGroups(user.id)
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to load user groups')
  } finally {
    groupLoading.value = false
  }
}

function closeGroupModal() {
  isGroupModalOpen.value = false
  userGroups.value = []
}

// Lifecycle
onMounted(() => {
  loadData()
})
</script>

<template>
  <AppLayout :title="t('users.title')" :show-sidebar="true">
    <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-neutral-800">{{ t('users.title') }}</h1>
        <p class="text-neutral-500 mt-1">{{ t('users.subtitle') }}</p>
      </div>
    </div>

    <!-- Search Bar -->
    <div class="max-w-md">
      <Input
        v-model="searchQuery"
        :placeholder="t('users.searchPlaceholder')"
        clearable
      >
        <template #prefix>
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </template>
      </Input>
    </div>

    <!-- Error Alert -->
    <div
      v-if="error"
      class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg"
    >
      {{ error }}
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="text-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-brand-500 mx-auto"></div>
      <p class="text-neutral-500 mt-4">{{ t('users.loading') }}</p>
    </div>

    <!-- Users Table -->
    <div v-else class="bg-white rounded-lg border border-neutral-200 overflow-hidden">
      <table class="min-w-full divide-y divide-neutral-200">
        <thead class="bg-neutral-50">
          <tr>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-neutral-500 uppercase tracking-wider"
            >
              {{ t('users.user') }}
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-neutral-500 uppercase tracking-wider"
            >
              {{ t('users.email') }}
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-neutral-500 uppercase tracking-wider"
            >
              {{ t('users.roles') }}
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-neutral-500 uppercase tracking-wider"
            >
              {{ t('users.status') }}
            </th>
            <th
              class="px-6 py-3 text-right text-xs font-medium text-neutral-500 uppercase tracking-wider"
            >
              {{ t('common.actions') }}
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-neutral-200">
          <tr v-for="user in filteredUsers" :key="user.id" class="hover:bg-neutral-50">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="flex items-center">
                <div
                  class="w-10 h-10 rounded-full bg-brand-100 flex items-center justify-center"
                >
                  <span class="text-brand-600 font-medium">
                    {{ user.username.charAt(0).toUpperCase() }}
                  </span>
                </div>
                <div class="ml-4">
                  <div class="text-sm font-medium text-neutral-900">
                    {{ user.username }}
                  </div>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-neutral-500">
              {{ user.email }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="flex flex-wrap gap-1">
                <Tag
                  v-for="role in user.roles"
                  :key="role"
                  :closable="role !== 'user'"
                  @close="handleRemoveRole(user, role)"
                >
                  {{ role }}
                </Tag>
                <button
                  @click="openRoleModal(user)"
                  class="inline-flex items-center px-2 py-1 text-xs text-brand-600 hover:text-brand-700 hover:bg-brand-50 rounded"
                >
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                  </svg>
                  {{ t('users.addRole') }}
                </button>
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                :class="[
                  'px-2 py-1 text-xs font-medium rounded-full',
                  user.is_active
                    ? 'bg-green-100 text-green-700'
                    : 'bg-red-100 text-red-700',
                ]"
              >
                {{ user.is_active ? t('users.active') : t('users.disabled') }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <div class="flex items-center justify-end gap-2">
                <Button
                  type="secondary"
                  size="sm"
                  @click="openGroupModal(user)"
                >
                  {{ t('users.viewGroups') }}
                </Button>
                <Button
                  :type="user.is_active ? 'danger' : 'success'"
                  size="sm"
                  @click="toggleUserStatus(user)"
                >
                  {{ user.is_active ? t('users.disable') : t('users.enable') }}
                </Button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>

      <!-- Empty State -->
      <div v-if="filteredUsers.length === 0" class="text-center py-12">
        <p class="text-neutral-500">{{ t('users.noUsers') }}</p>
      </div>
    </div>

    <!-- Role Assignment Modal -->
    <div
      v-if="isRoleModalOpen"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    >
      <div class="bg-white rounded-lg shadow-xl w-full max-w-md p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">{{ t('users.assignRole') }}</h2>
        <p class="text-neutral-500 mb-4">
          {{ t('users.selectRole', { username: selectedUser?.username }) }}
        </p>

        <div class="space-y-2 mb-6">
          <label
            v-for="role in roles"
            :key="role.id"
            :class="[
              'flex items-center p-3 border rounded-lg cursor-pointer transition-colors',
              selectedRoleId === role.id
                ? 'border-brand-500 bg-brand-50'
                : 'border-neutral-200 hover:border-neutral-300',
            ]"
          >
            <input
              type="radio"
              :value="role.id"
              v-model="selectedRoleId"
              class="sr-only"
            />
            <div class="flex-1">
              <div class="font-medium text-neutral-800">{{ role.name }}</div>
              <div class="text-sm text-neutral-500">{{ role.description }}</div>
            </div>
          </label>
        </div>

        <div class="flex justify-end gap-3">
          <Button type="secondary" @click="closeRoleModal">{{ t('common.cancel') }}</Button>
          <Button
            :disabled="!selectedRoleId || operationLoading"
            :loading="operationLoading"
            @click="handleAssignRole"
          >
            {{ t('users.assign') }}
          </Button>
        </div>
      </div>
    </div>

    <!-- User Groups Modal -->
    <div
      v-if="isGroupModalOpen"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    >
      <div class="bg-white rounded-lg shadow-xl w-full max-w-md p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">
          {{ t('users.userGroups') }} - {{ selectedUser?.username }}
        </h2>

        <div v-if="groupLoading" class="text-center py-8">
          <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-brand-500 mx-auto"></div>
        </div>

        <div v-else-if="userGroups.length === 0" class="text-center py-8 text-neutral-500">
          {{ t('users.noGroups') }}
        </div>

        <div v-else class="space-y-3">
          <div
            v-for="group in userGroups"
            :key="group.id"
            class="flex items-center justify-between p-3 bg-neutral-50 rounded-lg"
          >
            <div>
              <div class="font-medium text-neutral-800">{{ group.name }}</div>
              <div v-if="group.description" class="text-sm text-neutral-500">
                {{ group.description }}
              </div>
            </div>
            <span
              v-if="group.is_primary"
              class="px-2 py-1 text-xs font-medium bg-brand-100 text-brand-700 rounded"
            >
              {{ t('users.primaryGroup') }}
            </span>
          </div>
        </div>

        <div class="flex justify-end mt-6">
          <Button type="secondary" @click="closeGroupModal">
            {{ t('common.close') }}
          </Button>
        </div>
      </div>
    </div>
    </div>
  </AppLayout>
</template>