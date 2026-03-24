<script setup lang="ts">
/**
 * Skills Intelligence Hub - Roles Admin Page
 *
 * Role management page with CRUD operations
 */
import { ref, onMounted } from 'vue'
import {
  listRoles,
  createRole,
  updateRole,
  deleteRole,
  type Role,
  type CreateRoleRequest,
} from '@/api/roles'
import { extractErrorMessage } from '@/api/index'
import Button from '@/design-system/elements/Button/Button.vue'
import Input from '@/design-system/elements/Input/Input.vue'
import Tag from '@/design-system/elements/Tag/Tag.vue'
import AppLayout from '@/design-system/layouts/AppLayout.vue'

// State
const roles = ref<Role[]>([])
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
const newPermission = ref('')

// Available permissions (would come from backend in real app)
const availablePermissions = [
  'skills:read',
  'skills:write',
  'skills:delete',
  'users:read',
  'users:write',
  'users:delete',
  'groups:read',
  'groups:write',
  'groups:delete',
  'roles:read',
  'roles:write',
  'roles:delete',
  'admin',
]

// Methods
async function loadRoles() {
  loading.value = true
  error.value = ''
  try {
    roles.value = await listRoles()
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

function addPermission() {
  if (newPermission.value && !formData.value.permissions.includes(newPermission.value)) {
    formData.value.permissions.push(newPermission.value)
    newPermission.value = ''
  }
}

function removePermission(permission: string) {
  formData.value.permissions = formData.value.permissions.filter((p) => p !== permission)
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
  <AppLayout title="Role Management" :show-sidebar="true">
    <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-neutral-800">Role Management</h1>
        <p class="text-neutral-500 mt-1">Manage roles and permissions</p>
      </div>
      <Button @click="openCreateModal">
        <template #icon-left>
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </template>
        Create Role
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
      <p class="text-neutral-500 mt-4">Loading roles...</p>
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
              title="Edit"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
            </button>
            <button
              @click="handleDelete(role)"
              class="p-1 text-neutral-400 hover:text-red-600 rounded"
              title="Delete"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>

        <div>
          <h4 class="text-sm font-medium text-neutral-500 mb-2">Permissions</h4>
          <div v-if="role.permissions.length === 0" class="text-sm text-neutral-400">
            No permissions assigned
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
        No roles yet. Click "Create Role" to add one.
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div
      v-if="isModalOpen"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    >
      <div class="bg-white rounded-lg shadow-xl w-full max-w-md p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">
          {{ isEditMode ? 'Edit Role' : 'Create Role' }}
        </h2>

        <div class="space-y-4">
          <Input
            v-model="formData.name"
            label="Name"
            required
            placeholder="Role name"
          />
          <Input
            v-model="formData.description"
            label="Description"
            placeholder="Role description"
          />

          <div>
            <label class="block text-sm font-medium text-neutral-700 mb-1">
              Permissions
            </label>
            <div class="flex gap-2 mb-2">
              <select
                v-model="newPermission"
                class="flex-1 px-3 py-2 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-brand-500 focus:border-brand-500"
              >
                <option value="">Select a permission</option>
                <option
                  v-for="perm in availablePermissions.filter((p) => !formData.permissions.includes(p))"
                  :key="perm"
                  :value="perm"
                >
                  {{ perm }}
                </option>
              </select>
              <Button type="secondary" size="sm" @click="addPermission">Add</Button>
            </div>
            <div v-if="formData.permissions.length > 0" class="flex flex-wrap gap-1">
              <Tag
                v-for="permission in formData.permissions"
                :key="permission"
                closable
                @close="removePermission(permission)"
              >
                {{ permission }}
              </Tag>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-3 mt-6">
          <Button type="secondary" @click="closeModal">Cancel</Button>
          <Button
            :loading="operationLoading"
            :disabled="!formData.name"
            @click="handleSubmit"
          >
            {{ isEditMode ? 'Update' : 'Create' }}
          </Button>
        </div>
      </div>
    </div>
  </div>
  </AppLayout>
</template>