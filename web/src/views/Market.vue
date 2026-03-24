<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { listSkills, type Skill, type SkillListParams } from '@/api/skills'
import { extractErrorMessage } from '@/api/index'
import AppLayout from '@/design-system/layouts/AppLayout.vue'
import SkillCard from '@/components/SkillCard.vue'
import SearchBar from '@/components/SearchBar.vue'
import Button from '@/design-system/elements/Button/Button.vue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const allSkills = ref<Skill[]>([])
const popularSkills = ref<Skill[]>([])
const latestSkills = ref<Skill[]>([])
const loading = ref(true)
const loadingMore = ref(false)
const error = ref('')

// Search and filter state
const searchQuery = ref('')
const selectedTag = ref('')
const currentPage = ref(1)
const pageSize = 12
const hasMore = ref(true)

// Sync from URL
const urlQuery = computed(() => route.query.q as string || '')
const urlTag = computed(() => route.query.tag as string || '')

// Get all available tags
const availableTags = computed(() => {
  const tagSet = new Set<string>()
  allSkills.value.forEach(skill => {
    skill.tags?.forEach(tag => tagSet.add(tag))
  })
  return Array.from(tagSet).sort()
})

// Currently displayed skills (search/filter results)
const displayedSkills = computed(() => {
  let result = allSkills.value

  // Filter by search query
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(skill =>
      skill.name.toLowerCase().includes(query) ||
      skill.description?.toLowerCase().includes(query) ||
      skill.slug.toLowerCase().includes(query)
    )
  }

  // Filter by tag
  if (selectedTag.value) {
    result = result.filter(skill => skill.tags?.includes(selectedTag.value))
  }

  return result
})

// Whether to show featured sections
const showFeatured = computed(() => {
  return !searchQuery.value && !selectedTag.value
})

async function loadSkills(append = false) {
  if (append) {
    loadingMore.value = true
  } else {
    loading.value = true
    error.value = ''
  }

  try {
    const params: SkillListParams = {
      page: currentPage.value,
      page_size: pageSize,
    }

    const skills = await listSkills(params)

    if (append) {
      allSkills.value = [...allSkills.value, ...skills]
    } else {
      allSkills.value = skills

      // Set popular and latest skills
      popularSkills.value = [...skills]
        .sort((a, b) => b.download_count - a.download_count)
        .slice(0, 6)

      latestSkills.value = [...skills]
        .sort((a, b) => {
          const dateA = a.created_at ? new Date(a.created_at).getTime() : 0
          const dateB = b.created_at ? new Date(b.created_at).getTime() : 0
          return dateB - dateA
        })
        .slice(0, 6)
    }

    hasMore.value = skills.length === pageSize
  } catch (e) {
    error.value = extractErrorMessage(e, 'Failed to load')
    console.error(e)
  } finally {
    loading.value = false
    loadingMore.value = false
  }
}

function handleSearch(query: string) {
  searchQuery.value = query
  currentPage.value = 1
  updateUrlParams()
}

function handleTagSelect(tag: string) {
  selectedTag.value = selectedTag.value === tag ? '' : tag
  currentPage.value = 1
  updateUrlParams()
}

function clearFilters() {
  searchQuery.value = ''
  selectedTag.value = ''
  currentPage.value = 1
  router.push({ path: '/' })
}

function updateUrlParams() {
  const query: Record<string, string> = {}
  if (searchQuery.value) query.q = searchQuery.value
  if (selectedTag.value) query.tag = selectedTag.value
  router.push({ path: '/', query })
}

async function loadMore() {
  currentPage.value++
  await loadSkills(true)
}

// Initialize filters from URL
function initFromUrl() {
  if (urlQuery.value) {
    searchQuery.value = urlQuery.value
  }
  if (urlTag.value) {
    selectedTag.value = urlTag.value
  }
}

// Watch URL changes
watch([urlQuery, urlTag], ([newQuery, newTag]) => {
  searchQuery.value = newQuery
  selectedTag.value = newTag
})

onMounted(() => {
  initFromUrl()
  loadSkills()
})
</script>

<template>
  <AppLayout :title="t('market.title')">
    <div class="max-w-7xl mx-auto">
      <!-- Page Header -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-neutral-900 mb-4">{{ t('market.title') }}</h1>
        <SearchBar @search="handleSearch" :initial-query="urlQuery" />
      </div>

      <!-- Tag Filters -->
      <div v-if="availableTags.length > 0" class="mb-6">
        <div class="flex flex-wrap gap-2">
          <button
            v-for="tag in availableTags"
            :key="tag"
            @click="handleTagSelect(tag)"
            :class="[
              'px-3 py-1 rounded-full text-sm transition-colors',
              selectedTag === tag
                ? 'bg-brand-500 text-white'
                : 'bg-neutral-100 text-neutral-700 hover:bg-neutral-200'
            ]"
          >
            {{ tag }}
          </button>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="text-center py-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-brand-500 mx-auto"></div>
        <p class="text-neutral-500 mt-4">{{ t('common.loading') }}</p>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="text-center py-12">
        <p class="text-red-500">{{ error }}</p>
        <Button type="primary" class="mt-4" @click="loadSkills()">
          {{ t('common.retry') }}
        </Button>
      </div>

      <!-- Featured Section: Popular and Latest Skills -->
      <template v-else-if="showFeatured">
        <!-- Popular Skills -->
        <section v-if="popularSkills.length > 0" class="mb-12">
          <h2 class="text-xl font-semibold text-neutral-900 mb-4 flex items-center">
            <svg class="w-5 h-5 mr-2 text-orange-500" fill="currentColor" viewBox="0 0 20 20">
              <path d="M12.395 2.553a1 1 0 00-1.45-.385c-.345.23-.614.558-.822.88-.214.33-.403.713-.57 1.116-.334.804-.614 1.768-.84 2.734a31.365 31.365 0 00-.613 3.58 2.64 2.64 0 01-.945-1.067c-.328-.68-.398-1.534-.398-2.654A1 1 0 005.05 6.05 6.981 6.981 0 003 11a7 7 0 1011.95-4.95c-.592-.591-.98-.985-1.348-1.467-.363-.476-.724-1.063-1.207-2.03zM12.12 15.12A3 3 0 017 13s.879.5 2.5.5c0-1 .5-4 1.25-4.5.5 1 .786 1.293 1.371 1.879A2.99 2.99 0 0113 13a2.99 2.99 0 01-.879 2.121z" />
            </svg>
            {{ t('market.popularSkills') }}
          </h2>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <SkillCard v-for="skill in popularSkills" :key="skill.id" :skill="skill" />
          </div>
        </section>

        <!-- Latest Skills -->
        <section v-if="latestSkills.length > 0" class="mb-12">
          <h2 class="text-xl font-semibold text-neutral-900 mb-4 flex items-center">
            <svg class="w-5 h-5 mr-2 text-green-500" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
            </svg>
            {{ t('market.latestReleases') }}
          </h2>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <SkillCard v-for="skill in latestSkills" :key="skill.id" :skill="skill" />
          </div>
        </section>

        <!-- All Skills -->
        <section v-if="allSkills.length > 0">
          <h2 class="text-xl font-semibold text-neutral-900 mb-4">{{ t('market.allSkills') }}</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <SkillCard v-for="skill in allSkills" :key="skill.id" :skill="skill" />
          </div>
        </section>

        <!-- Empty State -->
        <div v-if="allSkills.length === 0" class="text-center py-12">
          <p class="text-neutral-500">{{ t('market.noSkills') }}</p>
        </div>
      </template>

      <!-- Search/Filter Results -->
      <template v-else>
        <div class="mb-4">
          <div class="flex items-center justify-between">
            <p class="text-neutral-600">
              {{ t('market.foundSkills', { count: displayedSkills.length }) }}
              <template v-if="searchQuery">, {{ t('market.searchingFor', { query: searchQuery }) }}</template>
              <template v-if="selectedTag">, {{ t('market.tagFiltering', { tag: selectedTag }) }}</template>
            </p>
            <button
              @click="clearFilters"
              class="text-brand-500 hover:text-brand-700 text-sm"
            >
              {{ t('common.clearFilters') }}
            </button>
          </div>
        </div>

        <div v-if="displayedSkills.length === 0" class="text-center py-12">
          <p class="text-neutral-500">{{ t('market.noMatching') }}</p>
          <button
            @click="clearFilters"
            class="mt-4 text-brand-500 hover:text-brand-700"
          >
            {{ t('common.viewAll') }}
          </button>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <SkillCard v-for="skill in displayedSkills" :key="skill.id" :skill="skill" />
        </div>

        <!-- Load More -->
        <div v-if="hasMore" class="text-center mt-8">
          <Button
            type="secondary"
            :loading="loadingMore"
            @click="loadMore"
          >
            {{ loadingMore ? t('common.loading') : t('market.loadMore') }}
          </Button>
        </div>
      </template>
    </div>
  </AppLayout>
</template>