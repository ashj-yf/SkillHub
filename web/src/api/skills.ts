import { api } from './index'

export interface Skill {
  id: string
  name: string
  slug: string
  description?: string
  readme?: string
  author_id?: string
  version: string
  tags: string[]
  is_public: boolean
  download_count: number
  created_at: string
  updated_at: string
}

export interface SkillTag {
  id: string
  skill_id: string
  tag: string
  version: string
  updated_at: string
  updated_by: string | null
}

export interface SkillVersion {
  id: string
  skill_id: string
  version: string
  storage_path: string
  content?: string
  changelog?: string
  digest?: string
  created_at: string
  created_by?: string
}

export interface SkillDetailByVersion {
  id: string
  name: string
  slug: string
  description?: string
  readme?: string
  author_id?: string
  version: string
  tags: string[]
  is_public: boolean
  download_count: number
  created_at: string
  updated_at: string
  content?: string
  version_info: SkillVersion
}

export interface SkillFile {
  name: string
  size: number
  digest: string
}

export interface CreateSkillRequest {
  name: string
  slug: string
  description?: string
  readme?: string
  tags: string[]
  is_public?: boolean
}

export interface UpdateSkillRequest {
  name?: string
  description?: string
  readme?: string
  tags?: string[]
  is_public?: boolean
}

export interface SkillListParams {
  q?: string
  tags?: string
  page?: number
  page_size?: number
  sort?: 'popular' | 'latest'
}

export interface SkillListResponse {
  items: Skill[]
  total: number
  page: number
  page_size: number
  total_pages: number
}

export async function listSkills(params?: SkillListParams): Promise<Skill[]> {
  const { data } = await api.get<Skill[]>('/skills', { params })
  return data
}

export async function listSkillsPaginated(params?: SkillListParams): Promise<SkillListResponse> {
  const { data } = await api.get<SkillListResponse>('/skills', { params })
  return data
}

export async function getSkill(slug: string): Promise<Skill> {
  const { data } = await api.get<Skill>(`/skills/${slug}`)
  return data
}

export async function getSkillByVersion(slug: string, tag: string): Promise<SkillDetailByVersion> {
  const { data } = await api.get<SkillDetailByVersion>(`/skills/${slug}/${tag}`)
  return data
}

export async function getSkillVersions(slug: string): Promise<SkillVersion[]> {
  const { data } = await api.get<SkillVersion[]>(`/skills/${slug}/versions`)
  return data
}

export async function getSkillTags(slug: string): Promise<SkillTag[]> {
  const { data } = await api.get<SkillTag[]>(`/skills/${slug}/tags`)
  return data
}

export async function downloadSkill(slug: string, tag?: string): Promise<Blob> {
  const url = tag ? `/skills/${slug}/${tag}/download` : `/skills/${slug}/download`
  const { data } = await api.get<Blob>(url, { responseType: 'blob' })
  return data
}

export async function createSkill(skill: CreateSkillRequest): Promise<Skill> {
  const { data } = await api.post<Skill>('/skills', skill)
  return data
}

export async function updateSkill(slug: string, skill: UpdateSkillRequest): Promise<Skill> {
  const { data } = await api.put<Skill>(`/skills/${slug}`, skill)
  return data
}

export async function deleteSkill(slug: string): Promise<void> {
  await api.delete(`/skills/${slug}`)
}

// 获取当前用户创建的技能
export async function listMySkills(): Promise<Skill[]> {
  const { data } = await api.get<Skill[]>('/users/me/skills')
  return data
}