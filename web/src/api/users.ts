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

export interface UserInfo {
  id: string
  username: string
  email: string
  role: string
  roles: string[]
}

export interface UpdateUserRequest {
  username?: string
  email?: string
  is_active?: boolean
}

// User Management API - Backend now implements full user management

/**
 * 获取当前登录用户信息
 * @returns 用户基本信息（包含角色列表）
 */
export async function getCurrentUser(): Promise<UserInfo> {
  const { data } = await api.get<UserInfo>('/users/me')
  return data
}

/**
 * 获取指定用户的角色列表
 * @param userId - 用户 ID
 * @returns 角色名称列表
 */
export async function getUserRoles(userId: string): Promise<string[]> {
  const { data } = await api.get<string[]>(`/users/${userId}/roles`)
  return data
}

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
 * @param roleName - The role name (e.g., 'admin', 'user')
 *
 * API 使用角色名称（非 UUID）分配角色
 * 后端接口: POST /users/{id}/roles { "role": "admin" }
 */
export async function assignRole(userId: string, roleName: string): Promise<void> {
  await api.post(`/users/${userId}/roles`, { role: roleName })
}

/**
 * Remove a role from a user
 * @param userId - The user's UUID
 * @param roleName - The role name (e.g., 'admin', 'user')
 *
 * API 使用角色名称（非 UUID）移除角色
 * 后端接口: DELETE /users/{id}/roles/{role}
 */
export async function removeRole(userId: string, roleName: string): Promise<void> {
  await api.delete(`/users/${userId}/roles/${roleName}`)
}

/**
 * 用户所属组信息
 */
export interface UserGroup {
  id: string
  name: string
  description?: string
  parent_id: string | null
  is_primary: boolean
}

/**
 * 获取用户所属的用户组列表
 * @param userId - 用户 ID
 * @returns 用户所属组列表
 */
export async function getUserGroups(userId: string): Promise<UserGroup[]> {
  const { data } = await api.get<UserGroup[]>(`/users/${userId}/groups`)
  return data
}