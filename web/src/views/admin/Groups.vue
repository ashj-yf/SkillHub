<script setup lang="ts">
/**
 * Skills Intelligence Hub - Groups Admin Page
 *
 * Department/group management page with tree view, CRUD, and member management
 *
 * NOTE: This page requires GET /users endpoint for listing users to add as members.
 * If that endpoint is not available, the "Add Member" feature will show an error.
 * The member role is mapped to 'is_primary' boolean (admin=true, member=false).
 */
import { ref, computed, onMounted } from 'vue'
import {
  listGroups,
  createGroup,
  updateGroup,
  deleteGroup,
  getGroupMembers,
  addGroupMember,
  removeGroupMember,
  type Group,
  type GroupMember,
} from '@/api/groups'
import { listUsers, type User } from '@/api/users'
import { extractErrorMessage } from '@/api/index'
import Button from '@/design-system/elements/Button/Button.vue'
import Input from '@/design-system/elements/Input/Input.vue'
import Tag from '@/design-system/elements/Tag/Tag.vue'

// State
const groups = ref<Group[]>([])
const users = ref<User[]>([])
const loading = ref(true)
const error = ref('')
const selectedGroup = ref<Group | null>(null)
const members = ref<GroupMember[]>([])

// Modal states
const isCreateModalOpen = ref(false)
const isEditModalOpen = ref(false)
const isMemberModalOpen = ref(false)
const operationLoading = ref(false)

// Form data
const formData = ref({
  name: '',
  description: '',
  parent_id: null as string | null,
})
const editingGroup = ref<Group | null>(null)
const selectedMemberId = ref('')
const selectedMemberRole = ref<'admin' | 'member'>('member')

// Computed - build tree structure
const groupTree = computed(() => {
  const buildTree = (parentId: string | null): (Group & { children: Group[] })[] => {
    return groups.value
      .filter((g) => g.parent_id === parentId)
      .map((g) => ({
        ...g,
        children: buildTree(g.id),
      }))
  }
  return buildTree(null)
})

const rootGroups = computed(() => groups.value.filter((g) => g.parent_id === null))

// Methods
async function loadData() {
  loading.value = true
  error.value = ''
  try {
    const [groupsData, usersData] = await Promise.all([listGroups(), listUsers()])
    groups.value = groupsData
    users.value = usersData
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to load data. Note: Some endpoints may not be implemented yet.')
  } finally {
    loading.value = false
  }
}

async function selectGroup(group: Group) {
  selectedGroup.value = group
  try {
    members.value = await getGroupMembers(group.id)
  } catch {
    members.value = []
  }
}

function openCreateModal(parentId: string | null = null) {
  formData.value = {
    name: '',
    description: '',
    parent_id: parentId,
  }
  isCreateModalOpen.value = true
}

function openEditModal(group: Group) {
  editingGroup.value = group
  formData.value = {
    name: group.name,
    description: group.description || '',
    parent_id: group.parent_id,
  }
  isEditModalOpen.value = true
}

function openMemberModal() {
  selectedMemberId.value = ''
  selectedMemberRole.value = 'member'
  isMemberModalOpen.value = true
}

function closeModals() {
  isCreateModalOpen.value = false
  isEditModalOpen.value = false
  isMemberModalOpen.value = false
  editingGroup.value = null
}

async function handleCreate() {
  if (!formData.value.name) return

  operationLoading.value = true
  try {
    const newGroup = await createGroup(formData.value)
    groups.value.push(newGroup)
    closeModals()
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to create group')
  } finally {
    operationLoading.value = false
  }
}

async function handleUpdate() {
  if (!editingGroup.value || !formData.value.name) return

  operationLoading.value = true
  try {
    const updated = await updateGroup(editingGroup.value.id, formData.value)
    const index = groups.value.findIndex((g) => g.id === updated.id)
    if (index !== -1) {
      groups.value[index] = updated
    }
    closeModals()
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to update group')
  } finally {
    operationLoading.value = false
  }
}

async function handleDelete(group: Group) {
  if (!confirm(`Are you sure you want to delete "${group.name}"?`)) return

  try {
    await deleteGroup(group.id)
    groups.value = groups.value.filter((g) => g.id !== group.id)
    if (selectedGroup.value?.id === group.id) {
      selectedGroup.value = null
    }
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to delete group')
  }
}

async function handleAddMember() {
  if (!selectedGroup.value || !selectedMemberId.value) return

  operationLoading.value = true
  try {
    await addGroupMember(selectedGroup.value.id, {
      user_id: selectedMemberId.value,
      is_primary: selectedMemberRole.value === 'admin',
    })
    members.value = await getGroupMembers(selectedGroup.value.id)
    closeModals()
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to add member')
  } finally {
    operationLoading.value = false
  }
}

async function handleRemoveMember(member: GroupMember) {
  if (!selectedGroup.value) return

  try {
    await removeGroupMember(selectedGroup.value.id, member.id)
    members.value = members.value.filter((m) => m.id !== member.id)
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to remove member')
  }
}

// Initialize
onMounted(() => {
  loadData()
})
</script>

<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-neutral-800">Department Management</h1>
        <p class="text-neutral-500 mt-1">Manage departments and their members</p>
      </div>
      <Button @click="openCreateModal()">
        <template #icon-left>
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </template>
        Create Department
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
      <p class="text-neutral-500 mt-4">Loading departments...</p>
    </div>

    <!-- Main Content -->
    <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Group Tree -->
      <div class="lg:col-span-1 bg-white rounded-lg border border-neutral-200 p-4">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">Departments</h2>

        <div v-if="groups.length === 0" class="text-center py-8 text-neutral-500">
          No departments yet
        </div>

        <div v-else class="space-y-1">
          <!-- Recursive tree rendering -->
          <template v-for="group in groupTree" :key="group.id">
            <div
              :class="[
                'group flex items-center justify-between p-2 rounded-lg cursor-pointer transition-colors',
                selectedGroup?.id === group.id
                  ? 'bg-brand-50 text-brand-600'
                  : 'hover:bg-neutral-50',
              ]"
              @click="selectGroup(group)"
            >
              <div class="flex items-center">
                <svg class="w-5 h-5 mr-2 text-neutral-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                </svg>
                <span>{{ group.name }}</span>
              </div>
              <div class="hidden group-hover:flex items-center gap-1">
                <button
                  @click.stop="openCreateModal(group.id)"
                  class="p-1 text-neutral-400 hover:text-neutral-600 rounded"
                  title="Add sub-department"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                  </svg>
                </button>
                <button
                  @click.stop="openEditModal(group)"
                  class="p-1 text-neutral-400 hover:text-neutral-600 rounded"
                  title="Edit"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                  </svg>
                </button>
                <button
                  @click.stop="handleDelete(group)"
                  class="p-1 text-neutral-400 hover:text-red-600 rounded"
                  title="Delete"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
            </div>

            <!-- Children -->
            <div v-if="group.children.length > 0" class="ml-6 space-y-1 border-l border-neutral-200 pl-2">
              <div
                v-for="child in group.children"
                :key="child.id"
                :class="[
                  'group flex items-center justify-between p-2 rounded-lg cursor-pointer transition-colors',
                  selectedGroup?.id === child.id
                    ? 'bg-brand-50 text-brand-600'
                    : 'hover:bg-neutral-50',
                ]"
                @click="selectGroup(child)"
              >
                <div class="flex items-center">
                  <svg class="w-5 h-5 mr-2 text-neutral-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                  </svg>
                  <span>{{ child.name }}</span>
                </div>
                <div class="hidden group-hover:flex items-center gap-1">
                  <button
                    @click.stop="openEditModal(child)"
                    class="p-1 text-neutral-400 hover:text-neutral-600 rounded"
                    title="Edit"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button
                    @click.stop="handleDelete(child)"
                    class="p-1 text-neutral-400 hover:text-red-600 rounded"
                    title="Delete"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </div>
            </div>
          </template>
        </div>
      </div>

      <!-- Selected Group Details -->
      <div class="lg:col-span-2 bg-white rounded-lg border border-neutral-200 p-4">
        <div v-if="!selectedGroup" class="text-center py-12 text-neutral-500">
          Select a department to view details
        </div>

        <template v-else>
          <div class="flex items-center justify-between mb-6">
            <div>
              <h2 class="text-xl font-semibold text-neutral-800">{{ selectedGroup.name }}</h2>
              <p v-if="selectedGroup.description" class="text-neutral-500 mt-1">
                {{ selectedGroup.description }}
              </p>
            </div>
            <Button type="primary" size="sm" @click="openMemberModal">
              Add Member
            </Button>
          </div>

          <h3 class="text-lg font-medium text-neutral-800 mb-4">Members</h3>

          <div v-if="members.length === 0" class="text-center py-8 text-neutral-500">
            No members in this department
          </div>

          <div v-else class="space-y-2">
            <div
              v-for="member in members"
              :key="member.id"
              class="flex items-center justify-between p-3 bg-neutral-50 rounded-lg"
            >
              <div class="flex items-center">
                <div
                  class="w-10 h-10 rounded-full bg-brand-100 flex items-center justify-center"
                >
                  <span class="text-brand-600 font-medium">
                    {{ member.username.charAt(0).toUpperCase() }}
                  </span>
                </div>
                <div class="ml-3">
                  <div class="font-medium text-neutral-800">{{ member.username }}</div>
                  <div class="text-sm text-neutral-500">{{ member.email }}</div>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <Tag :type="member.is_primary ? 'primary' : 'default'">
                  {{ member.is_primary ? 'Primary' : 'Member' }}
                </Tag>
                <button
                  @click="handleRemoveMember(member)"
                  class="text-neutral-400 hover:text-red-600 p-1"
                  title="Remove member"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div
      v-if="isCreateModalOpen || isEditModalOpen"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    >
      <div class="bg-white rounded-lg shadow-xl w-full max-w-md p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">
          {{ isEditModalOpen ? 'Edit Department' : 'Create Department' }}
        </h2>

        <div class="space-y-4">
          <Input
            v-model="formData.name"
            label="Name"
            required
            placeholder="Department name"
          />
          <Input
            v-model="formData.description"
            label="Description"
            placeholder="Department description"
          />
          <div v-if="rootGroups.length > 0">
            <label class="block text-sm font-medium text-neutral-700 mb-1">
              Parent Department
            </label>
            <select
              v-model="formData.parent_id"
              class="w-full px-3 py-2 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-brand-500 focus:border-brand-500"
            >
              <option :value="null">None (Root)</option>
              <option
                v-for="group in rootGroups"
                :key="group.id"
                :value="group.id"
                :disabled="isEditModalOpen && editingGroup?.id === group.id"
              >
                {{ group.name }}
              </option>
            </select>
          </div>
        </div>

        <div class="flex justify-end gap-3 mt-6">
          <Button type="secondary" @click="closeModals">Cancel</Button>
          <Button
            :loading="operationLoading"
            :disabled="!formData.name"
            @click="isEditModalOpen ? handleUpdate() : handleCreate()"
          >
            {{ isEditModalOpen ? 'Update' : 'Create' }}
          </Button>
        </div>
      </div>
    </div>

    <!-- Add Member Modal -->
    <div
      v-if="isMemberModalOpen"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    >
      <div class="bg-white rounded-lg shadow-xl w-full max-w-md p-6">
        <h2 class="text-lg font-semibold text-neutral-800 mb-4">Add Member</h2>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-neutral-700 mb-1">User</label>
            <select
              v-model="selectedMemberId"
              class="w-full px-3 py-2 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-brand-500 focus:border-brand-500"
            >
              <option value="">Select a user</option>
              <option v-for="user in users" :key="user.id" :value="user.id">
                {{ user.username }} ({{ user.email }})
              </option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-neutral-700 mb-1">Role</label>
            <select
              v-model="selectedMemberRole"
              class="w-full px-3 py-2 border border-neutral-300 rounded-lg focus:ring-2 focus:ring-brand-500 focus:border-brand-500"
            >
              <option value="member">Member</option>
              <option value="admin">Admin</option>
            </select>
          </div>
        </div>

        <div class="flex justify-end gap-3 mt-6">
          <Button type="secondary" @click="closeModals">Cancel</Button>
          <Button
            :loading="operationLoading"
            :disabled="!selectedMemberId"
            @click="handleAddMember"
          >
            Add
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>