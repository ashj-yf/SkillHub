import { describe, it, expect, vi, beforeEach } from 'vitest'
import { api } from '../index'
import {
  listGroups,
  getGroup,
  createGroup,
  updateGroup,
  deleteGroup,
  getGroupMembers,
  addGroupMember,
  removeGroupMember,
  type Group,
  type CreateGroupRequest,
  type UpdateGroupRequest,
  type GroupMember,
} from '../groups'

// Mock the api module
vi.mock('../index', () => ({
  api: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
  },
}))

describe('Groups API', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('listGroups', () => {
    it('should call GET /groups and return groups list', async () => {
      const mockGroups: Group[] = [
        {
          id: '1',
          name: 'Engineering',
          description: 'Engineering department',
          parent_id: null,
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
        {
          id: '2',
          name: 'Frontend Team',
          description: 'Frontend development team',
          parent_id: '1',
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
      ]

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockGroups })

      const result = await listGroups()

      expect(api.get).toHaveBeenCalledWith('/groups')
      expect(result).toEqual(mockGroups)
    })
  })

  describe('getGroup', () => {
    it('should call GET /groups/:id and return group', async () => {
      const mockGroup: Group = {
        id: '1',
        name: 'Engineering',
        description: 'Engineering department',
        parent_id: null,
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      }

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockGroup })

      const result = await getGroup('1')

      expect(api.get).toHaveBeenCalledWith('/groups/1')
      expect(result).toEqual(mockGroup)
    })
  })

  describe('createGroup', () => {
    it('should call POST /groups with group data and return created group', async () => {
      const newGroup: CreateGroupRequest = {
        name: 'Design Team',
        description: 'Design department',
        parent_id: null,
      }

      const mockCreatedGroup: Group = {
        id: '3',
        name: newGroup.name,
        description: newGroup.description,
        parent_id: newGroup.parent_id ?? null,
        created_at: '2024-01-02T00:00:00Z',
        updated_at: '2024-01-02T00:00:00Z',
      }

      vi.mocked(api.post).mockResolvedValueOnce({ data: mockCreatedGroup })

      const result = await createGroup(newGroup)

      expect(api.post).toHaveBeenCalledWith('/groups', newGroup)
      expect(result).toEqual(mockCreatedGroup)
    })
  })

  describe('updateGroup', () => {
    it('should call PUT /groups/:id with update data and return updated group', async () => {
      const updateData: UpdateGroupRequest = {
        description: 'Updated description',
      }

      const mockUpdatedGroup: Group = {
        id: '1',
        name: 'Engineering',
        description: 'Updated description',
        parent_id: null,
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-03T00:00:00Z',
      }

      vi.mocked(api.put).mockResolvedValueOnce({ data: mockUpdatedGroup })

      const result = await updateGroup('1', updateData)

      expect(api.put).toHaveBeenCalledWith('/groups/1', updateData)
      expect(result).toEqual(mockUpdatedGroup)
    })
  })

  describe('deleteGroup', () => {
    it('should call DELETE /groups/:id', async () => {
      vi.mocked(api.delete).mockResolvedValueOnce({})

      await deleteGroup('1')

      expect(api.delete).toHaveBeenCalledWith('/groups/1')
    })
  })

  describe('getGroupMembers', () => {
    it('should call GET /groups/:id/members and return members list', async () => {
      const mockMembers: GroupMember[] = [
        {
          user_id: '1',
          username: 'john',
          email: 'john@example.com',
          role: 'admin',
          joined_at: '2024-01-01T00:00:00Z',
        },
        {
          user_id: '2',
          username: 'jane',
          email: 'jane@example.com',
          role: 'member',
          joined_at: '2024-01-02T00:00:00Z',
        },
      ]

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockMembers })

      const result = await getGroupMembers('1')

      expect(api.get).toHaveBeenCalledWith('/groups/1/members')
      expect(result).toEqual(mockMembers)
    })
  })

  describe('addGroupMember', () => {
    it('should call POST /groups/:id/members with user data', async () => {
      vi.mocked(api.post).mockResolvedValueOnce({})

      await addGroupMember('1', { user_id: '3', role: 'member' })

      expect(api.post).toHaveBeenCalledWith('/groups/1/members', {
        user_id: '3',
        role: 'member',
      })
    })
  })

  describe('removeGroupMember', () => {
    it('should call DELETE /groups/:id/members/:userId', async () => {
      vi.mocked(api.delete).mockResolvedValueOnce({})

      await removeGroupMember('1', '3')

      expect(api.delete).toHaveBeenCalledWith('/groups/1/members/3')
    })
  })
})