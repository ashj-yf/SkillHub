import { api } from './index'

export interface User {
  id: string
  username: string
  email: string
  is_active: boolean
  roles: string[]
  created_at: string
  updated_at: string
}

export interface UpdateUserRequest {
  username?: string
  email?: string
  is_active?: boolean
}

// Note: Backend currently only has /users/me endpoints, not full user management
// These functions are kept for future use when backend implements admin user management

export async function listUsers(): Promise<User[]> {
  const { data } = await api.get<User[]>('/users')
  return data
}

export async function getUser(id: string): Promise<User> {
  const { data } = await api.get<User>(`/users/${id}`)
  return data
}

export async function updateUser(id: string, user: UpdateUserRequest): Promise<User> {
  const { data } = await api.put<User>(`/users/${id}`, user)
  return data
}

export async function deleteUser(id: string): Promise<void> {
  await api.delete(`/users/${id}`)
}

/**
 * Assign a role to a user
 * @param userId - The user's UUID
 * @param roleId - The role's UUID (not role name)
 */
export async function assignRole(userId: string, roleId: string): Promise<void> {
  await api.post(`/users/${userId}/roles`, { role_id: roleId })
}

/**
 * Remove a role from a user
 * @param userId - The user's UUID
 * @param roleId - The role's UUID (not role name)
 */
export async function removeRole(userId: string, roleId: string): Promise<void> {
  await api.delete(`/users/${userId}/roles/${roleId}`)
}