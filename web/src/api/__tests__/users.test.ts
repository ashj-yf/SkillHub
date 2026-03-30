import { describe, it, expect, vi, beforeEach } from 'vitest'
import { api } from '../index'
import {
  listUsers,
  getUser,
  updateUser,
  deleteUser,
  assignRole,
  removeRole,
  getCurrentUser,
  getUserRoles,
  type User,
  type UpdateUserRequest,
  type UserInfo,
} from '../users'

// Mock the api module
vi.mock('../index', () => ({
  api: {
    get: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
    post: vi.fn(),
  },
}))

describe('Users API', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('listUsers', () => {
    it('should call GET /users and return users list', async () => {
      const mockUsers: User[] = [
        {
          id: '1',
          username: 'john',
          email: 'john@example.com',
          is_active: true,
          roles: ['user'],
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
        {
          id: '2',
          username: 'jane',
          email: 'jane@example.com',
          is_active: true,
          roles: ['admin', 'user'],
          created_at: '2024-01-01T00:00:00Z',
          updated_at: '2024-01-01T00:00:00Z',
        },
      ]

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockUsers })

      const result = await listUsers()

      expect(api.get).toHaveBeenCalledWith('/users')
      expect(result).toEqual(mockUsers)
    })
  })

  describe('getUser', () => {
    it('should call GET /users/:id and return user', async () => {
      const mockUser: User = {
        id: '1',
        username: 'john',
        email: 'john@example.com',
        is_active: true,
        roles: ['user'],
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-01T00:00:00Z',
      }

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockUser })

      const result = await getUser('1')

      expect(api.get).toHaveBeenCalledWith('/users/1')
      expect(result).toEqual(mockUser)
    })
  })

  describe('updateUser', () => {
    it('should call PUT /users/:id with update data and return updated user', async () => {
      const updateData: UpdateUserRequest = {
        is_active: false,
      }

      const mockUpdatedUser: User = {
        id: '1',
        username: 'john',
        email: 'john@example.com',
        is_active: false,
        roles: ['user'],
        created_at: '2024-01-01T00:00:00Z',
        updated_at: '2024-01-03T00:00:00Z',
      }

      vi.mocked(api.put).mockResolvedValueOnce({ data: mockUpdatedUser })

      const result = await updateUser('1', updateData)

      expect(api.put).toHaveBeenCalledWith('/users/1', updateData)
      expect(result).toEqual(mockUpdatedUser)
    })
  })

  describe('deleteUser', () => {
    it('should call DELETE /users/:id', async () => {
      vi.mocked(api.delete).mockResolvedValueOnce({})

      await deleteUser('1')

      expect(api.delete).toHaveBeenCalledWith('/users/1')
    })
  })

  describe('assignRole', () => {
    it('should call POST /users/:id/roles with role name', async () => {
      vi.mocked(api.post).mockResolvedValueOnce({})

      await assignRole('1', 'admin')

      expect(api.post).toHaveBeenCalledWith('/users/1/roles', { role: 'admin' })
    })
  })

  describe('removeRole', () => {
    it('should call DELETE /users/:id/roles/:role', async () => {
      vi.mocked(api.delete).mockResolvedValueOnce({})

      await removeRole('1', 'admin')

      expect(api.delete).toHaveBeenCalledWith('/users/1/roles/admin')
    })
  })

  describe('getCurrentUser', () => {
    it('should call GET /users/me and return current user info', async () => {
      const mockUserInfo: UserInfo = {
        id: '1',
        username: 'john',
        email: 'john@example.com',
        role: 'admin',
        roles: ['admin', 'user'],
      }

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockUserInfo })

      const result = await getCurrentUser()

      expect(api.get).toHaveBeenCalledWith('/users/me')
      expect(result).toEqual(mockUserInfo)
    })
  })

  describe('getUserRoles', () => {
    it('should call GET /users/:id/roles and return role names', async () => {
      const mockRoles = ['admin', 'user']

      vi.mocked(api.get).mockResolvedValueOnce({ data: mockRoles })

      const result = await getUserRoles('1')

      expect(api.get).toHaveBeenCalledWith('/users/1/roles')
      expect(result).toEqual(mockRoles)
    })
  })
})