<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { listMySkills, deleteSkill, type Skill } from '@/api/skills'
import { extractErrorMessage } from '@/api/index'

const router = useRouter()
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
    error.value = extractErrorMessage(e, '加载技能列表失败')
    console.error(e)
  } finally {
    loading.value = false
  }
}

async function handleDelete(skill: Skill) {
  if (!confirm(`确定要删除技能 "${skill.name}" 吗？此操作不可撤销。`)) {
    return
  }

  deleting.value = skill.id
  try {
    await deleteSkill(skill.id)
    mySkills.value = mySkills.value.filter(s => s.id !== skill.id)
  } catch (e) {
    alert(extractErrorMessage(e, '删除失败'))
  } finally {
    deleting.value = null
  }
}

function handleLogout() {
  userStore.logout()
  router.push('/')
}

onMounted(() => {
  loadMySkills()
})
</script>

<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold text-gray-900">管理面板</h1>
      <button
        @click="handleLogout"
        class="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium border border-gray-300 hover:bg-gray-50"
      >
        退出登录
      </button>
    </div>

    <!-- 账户信息 -->
    <div class="bg-white shadow-sm rounded-lg border border-gray-200 p-6 mb-6">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">账户信息</h2>
      <div v-if="userStore.user" class="grid grid-cols-1 sm:grid-cols-3 gap-4 text-sm">
        <div>
          <span class="text-gray-500">用户名:</span>
          <span class="ml-2 font-medium">{{ userStore.user.username }}</span>
        </div>
        <div>
          <span class="text-gray-500">邮箱:</span>
          <span class="ml-2">{{ userStore.user.email }}</span>
        </div>
        <div>
          <span class="text-gray-500">角色:</span>
          <span class="ml-2 px-2 py-1 bg-indigo-100 text-indigo-800 rounded text-xs">
            {{ userStore.user.role }}
          </span>
        </div>
      </div>
    </div>

    <!-- 我的技能 -->
    <div class="bg-white shadow-sm rounded-lg border border-gray-200 p-6">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-semibold text-gray-900">我的技能</h2>
        <router-link
          to="/"
          class="text-sm text-indigo-600 hover:text-indigo-800"
        >
          浏览技能市场
        </router-link>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="text-center py-8">
        <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-indigo-600 mx-auto"></div>
        <p class="text-gray-500 mt-2">加载中...</p>
      </div>

      <!-- 错误状态 -->
      <div v-else-if="error" class="text-center py-8">
        <p class="text-red-500">{{ error }}</p>
        <button
          @click="loadMySkills"
          class="mt-2 text-indigo-600 hover:text-indigo-800"
        >
          重试
        </button>
      </div>

      <!-- 空状态 -->
      <div v-else-if="mySkills.length === 0" class="text-center py-8">
        <svg class="w-12 h-12 text-gray-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
        </svg>
        <p class="text-gray-500">暂无创建的技能</p>
        <p class="text-gray-400 text-sm mt-1">使用 CLI 工具发布您的第一个技能</p>
      </div>

      <!-- 技能列表 -->
      <div v-else class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="border-b border-gray-200">
              <th class="text-left py-3 px-4 text-sm font-medium text-gray-500">名称</th>
              <th class="text-left py-3 px-4 text-sm font-medium text-gray-500">Slug</th>
              <th class="text-left py-3 px-4 text-sm font-medium text-gray-500">版本</th>
              <th class="text-left py-3 px-4 text-sm font-medium text-gray-500">下载量</th>
              <th class="text-right py-3 px-4 text-sm font-medium text-gray-500">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="skill in mySkills"
              :key="skill.id"
              class="border-b border-gray-100 hover:bg-gray-50"
            >
              <td class="py-3 px-4">
                <router-link
                  :to="`/skill/${skill.slug}`"
                  class="text-indigo-600 hover:text-indigo-800 font-medium"
                >
                  {{ skill.name }}
                </router-link>
              </td>
              <td class="py-3 px-4 text-gray-500 text-sm">{{ skill.slug }}</td>
              <td class="py-3 px-4">
                <span class="px-2 py-1 bg-gray-100 text-gray-600 rounded text-sm">
                  v{{ skill.version }}
                </span>
              </td>
              <td class="py-3 px-4 text-gray-500 text-sm">{{ skill.download_count }}</td>
              <td class="py-3 px-4 text-right">
                <div class="flex justify-end gap-2">
                  <router-link
                    :to="`/skill/${skill.slug}`"
                    class="text-indigo-600 hover:text-indigo-800 text-sm"
                  >
                    查看
                  </router-link>
                  <button
                    @click="handleDelete(skill)"
                    :disabled="deleting === skill.id"
                    class="text-red-600 hover:text-red-800 text-sm disabled:opacity-50"
                  >
                    {{ deleting === skill.id ? '删除中...' : '删除' }}
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 快捷操作 -->
    <div class="mt-6 bg-white shadow-sm rounded-lg border border-gray-200 p-6">
      <h2 class="text-lg font-semibold text-gray-900 mb-4">快捷操作</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="p-4 bg-gray-50 rounded-lg">
          <h3 class="font-medium text-gray-900 mb-2">使用 CLI 发布技能</h3>
          <code class="text-sm text-gray-600 bg-gray-100 px-2 py-1 rounded">
            skillhub push ./my-skill
          </code>
        </div>
        <div class="p-4 bg-gray-50 rounded-lg">
          <h3 class="font-medium text-gray-900 mb-2">安装技能到项目</h3>
          <code class="text-sm text-gray-600 bg-gray-100 px-2 py-1 rounded">
            skillhub pull skill-slug
          </code>
        </div>
      </div>
    </div>
  </div>
</template>