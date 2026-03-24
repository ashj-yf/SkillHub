import { api } from './index'

export interface Group {
  id: string
  name: string
  description?: string
  parent_id: string | null
  created_at: string
  updated_at: string
}

/**
 * Group member response from backend
 * Note: Backend returns 'id' for user_id and 'is_primary' instead of 'role'
 */
export interface GroupMember {
  id: string // user's UUID
  username: string
  email: string
  is_primary: boolean // true = primary group, false = secondary group
}

export interface CreateGroupRequest {
  name: string
  description?: string
  parent_id?: string | null
}

export interface UpdateGroupRequest {
  name?: string
  description?: string
  parent_id?: string | null
}

/**
 * Request to add a member to a group
 * Note: Backend expects 'is_primary' (boolean) instead of 'role'
 */
export interface AddMemberRequest {
  user_id: string
  is_primary?: boolean // true = primary group membership
}

export async function listGroups(): Promise<Group[]> {
  const { data } = await api.get<Group[]>('/groups')
  return data
}

export async function getGroup(id: string): Promise<Group> {
  const { data } = await api.get<Group>(`/groups/${id}`)
  return data
}

export async function createGroup(group: CreateGroupRequest): Promise<Group> {
  const { data } = await api.post<Group>('/groups', group)
  return data
}

export async function updateGroup(id: string, group: UpdateGroupRequest): Promise<Group> {
  const { data } = await api.put<Group>(`/groups/${id}`, group)
  return data
}

export async function deleteGroup(id: string): Promise<void> {
  await api.delete(`/groups/${id}`)
}

export async function getGroupMembers(id: string): Promise<GroupMember[]> {
  const { data } = await api.get<GroupMember[]>(`/groups/${id}/members`)
  return data
}

export async function addGroupMember(id: string, member: AddMemberRequest): Promise<void> {
  await api.post(`/groups/${id}/members`, member)
}

export async function removeGroupMember(id: string, userId: string): Promise<void> {
  await api.delete(`/groups/${id}/members/${userId}`)
}