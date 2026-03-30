import { describe, it, expect, vi, beforeEach } from 'vitest'
import { api } from '../index'
import {
  createSkillTag,
  deleteSkillTag,
  type SkillTag,
  type CreateSkillTagRequest,
} from '../skills'

// Mock the api module
vi.mock('../index', () => ({
  api: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
  },
}))

describe('Skills API - Tag Management', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('createSkillTag', () => {
    it('should call POST /skills/:slug/tags with tag data and return created tag', async () => {
      const request: CreateSkillTagRequest = {
        tag: 'stable',
        version: 'v1.0.0',
      }

      const mockCreatedTag: SkillTag = {
        id: '1',
        skill_id: 'skill-1',
        tag: 'stable',
        version: 'v1.0.0',
        updated_at: '2024-01-01T00:00:00Z',
        updated_by: null,
      }

      vi.mocked(api.post).mockResolvedValueOnce({ data: mockCreatedTag })

      const result = await createSkillTag('my-skill', request)

      expect(api.post).toHaveBeenCalledWith('/skills/my-skill/tags', request)
      expect(result).toEqual(mockCreatedTag)
    })
  })

  describe('deleteSkillTag', () => {
    it('should call DELETE /skills/:slug/tags/:tag', async () => {
      vi.mocked(api.delete).mockResolvedValueOnce({})

      await deleteSkillTag('my-skill', 'stable')

      expect(api.delete).toHaveBeenCalledWith('/skills/my-skill/tags/stable')
    })
  })
})