<script setup lang="ts">
/**
 * Skills Intelligence Hub - Roles Admin Page
 *
 * Role management page with CRUD operations
 *
 * API 状态（参考 docs/api-spec.yaml）:
 * - GET /roles - 已实现
 * - POST /roles - 已实现
 * - GET /roles/{id} - 已实现
 * - PUT /roles/{id} - 已实现
 * - DELETE /roles/{id} - 已实现（系统角色不可删除）
 * - GET /roles/{id}/permissions - 已实现
 * - POST /roles/{id}/permissions - 已实现
 * - DELETE /roles/{id}/permissions/{permission_id} - 已实现
 * - GET /permissions - 已实现
 */
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  listRoles,
  listPermissions,
  createRole,
  updateRole,
  deleteRole,
  type Role,
  type Permission,
  type CreateRoleRequest,
} from '@/api/roles'
import { extractErrorMessage } from '@/api/index'
import Button from '@/design-system/elements/Button/Button.vue'
import Input from '@/design-system/elements/Input/Input.vue'
import Tag from '@/design-system/elements/Tag/Tag.vue'
import AppLayout from '@/design-system/layouts/AppLayout.vue'

const { t } = useI18n()

// State
const roles = ref<Role[]>([])
const permissions = ref<Permission[]>([])
const loading = ref(true)
const error = ref('')
const isModalOpen = ref(false)
const isEditMode = ref(false)
const editingRole = ref<Role | null>(null)
const operationLoading = ref(false)

// Form data
const formData = ref<CreateRoleRequest>({
  name: '',
  description: '',
  permissions: [],
})

// 按资源分组的权限
const permissionsByResource = computed(() => {
  const grouped: Record<string, Permission[]> = {}
  for (const perm of permissions.value) {
    if (!grouped[perm.resource]) {
      grouped[perm.resource] = []
    }
    grouped[perm.resource].push(perm)
  }
  return grouped
})

// Methods
async function loadRoles() {
  loading.value = true
  error.value = ''
  try {
    const [rolesData, permissionsData] = await Promise.all([
      listRoles(),
      listPermissions(),
    ])
    roles.value = rolesData
    permissions.value = permissionsData
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to load roles')
  } finally {
    loading.value = false
  }
}

function openCreateModal() {
  isEditMode.value = false
  editingRole.value = null
  formData.value = {
    name: '',
    description: '',
    permissions: [],
  }
  isModalOpen.value = true
}

function openEditModal(role: Role) {
  isEditMode.value = true
  editingRole.value = role
  formData.value = {
    name: role.name,
    description: role.description || '',
    permissions: [...role.permissions],
  }
  isModalOpen.value = true
}

function closeModal() {
  isModalOpen.value = false
  editingRole.value = null
}

async function handleSubmit() {
  if (!formData.value.name) return

  operationLoading.value = true
  try {
    if (isEditMode.value && editingRole.value) {
      const updated = await updateRole(editingRole.value.id, formData.value)
      const index = roles.value.findIndex((r) => r.id === updated.id)
      if (index !== -1) {
        roles.value[index] = updated
      }
    } else {
      const created = await createRole(formData.value)
      roles.value.push(created)
    }
    closeModal()
  } catch (e) {
    error.value = extractErrorMessage(e, `Failed to ${isEditMode.value ? 'update' : 'create'} role`)
  } finally {
    operationLoading.value = false
  }
}

async function handleDelete(role: Role) {
  if (!confirm(`Are you sure you want to delete "${role.name}"?`)) return

  try {
    await deleteRole(role.id)
    roles.value = roles.value.filter((r) => r.id !== role.id)
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to delete role')
  }
}

// Lifecycle
onMounted(() => {
  loadRoles()
})
</script>

<template>
  <AppLayout :title="t('roles.title')" :show-sidebar="true">
    <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-neutral-800">{{ t('roles.title') }}</h1>
        <p class="text-neutral-500 mt-1">{{ t('roles.subtitle') }}</p>
      </div>
      <Button @click="openCreateModal">
        <template #icon-left>
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </template>
        {{ t('roles.createRole') }}
      </Button>
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
      <p class="text-neutral-500 mt-4">{{ t('roles.loading') }}</p>
    </div>

    <!-- Roles Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="role in roles"
        :key="role.id"
        class="bg-white rounded-lg border border-neutral-200 p-6 hover:border-neutral-300 transition-colors"
      >
        <div class="flex items-start justify-between mb-4">
          <div>
            <h3 class="text-lg font-semibold text-neutral-800">{{ role.name }}</h3>
            <p v-if="role.description" class="text-neutral-500 text-sm mt-1">
              {{ role.description }}
            </p>
          </div>
          <div class="flex items-center gap-1">
            <button
              @click="openEditModal(role)"
              class="p-1 text-neutral-400 hover:text-neutral-600 rounded"
              :title="t('roles.edit')"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
            </button>
            <button
              @click="handleDelete(role)"
              class="p-1 text-neutral-400 hover:text-red-600 rounded"
              :title="t('common.delete')"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>

        <div>
          <h4 class="text-sm font-medium text-neutral-500 mb-2">{{ t('roles.permissions') }}</h4>
          <div v-if="role.permissions.length === 0" class="text-sm text-neutral-400">
            {{ t('roles.noPermissions') }}
          </div>
          <div v-else class="flex flex-wrap gap-1">
            <Tag v-for="permission in role.permissions" :key="permission" size="sm">
              {{ permission }}
            </Tag>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="roles.length === 0" class="col-span-full text-center py-12 text-neutral-500">
        {{ t('roles.noRoles') }}
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div
      v-if="isModalOpen"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    >
      <div class="bg-white rounded-lg shadow-xl w-full max-w-md p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">
          {{ isEditMode ? t('roles.editRole') : t('roles.createRole') }}
        </h2>

        <div class="space-y-4">
          <Input
            v-model="formData.name"
            :label="t('common.name')"
            required
            :placeholder="t('roles.namePlaceholder')"
          />
          <Input
            v-model="formData.description"
            :label="t('common.description')"
            :placeholder="t('roles.descPlaceholder')"
          />

          <div>
            <label class="block text-sm font-medium text-neutral-700 mb-2">
              {{ t('roles.permissions') }}
            </label>
            <!-- 权限分组选择器 -->
            <div class="max-h-64 overflow-y-auto border border-neutral-200 rounded-lg p-3 space-y-4">
              <div
                v-for="(perms, resource) in permissionsByResource"
                :key="resource"
                class="space-y-2"
              >
                <h5 class="text-sm font-semibold text-neutral-600 uppercase tracking-wide">
                  {{ resource }}
                </h5>
                <div class="grid grid-cols-2 gap-2 pl-2">
                  <label
                    v-for="perm in perms"
                    :key="perm.id"
                    class="flex items-center gap-2 text-sm text-neutral-700 cursor-pointer hover:bg-neutral-50 p-1 rounded"
                  >
                    <input
                      type="checkbox"
                      :value="perm.name"
                      v-model="formData.permissions"
                      class="w-4 h-4 text-brand-500 border-neutral-300 rounded focus:ring-brand-500"
                    />
                    <span class="flex-1">
                      <span class="font-medium">{{ perm.action }}</span>
                      <span v-if="perm.description" class="text-neutral-400 text-xs ml-1">
                        ({{ perm.description }})
                      </span>
                    </span>
                  </label>
                </div>
              </div>
            </div>
            <!-- 已选权限数量提示 -->
            <p class="text-xs text-neutral-400 mt-2">
              已选择 {{ formData.permissions.length }} 个权限
            </p>
          </div>
        </div>

        <div class="flex justify-end gap-3 mt-6">
          <Button type="secondary" @click="closeModal">{{ t('common.cancel') }}</Button>
          <Button
            :loading="operationLoading"
            :disabled="!formData.name"
            @click="handleSubmit"
          >
            {{ isEditMode ? t('roles.update') : t('roles.create') }}
          </Button>
        </div>
      </div>
    </div>
  </div>
  </AppLayout>
</template>