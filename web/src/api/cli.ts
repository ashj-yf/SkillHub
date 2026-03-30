/**
 * Skills Intelligence Hub - CLI API
 */
import { api } from './index'
import type { CliVersionResponse, CliVersion } from '@/types/cli'

/**
 * 获取最新 CLI 版本信息
 */
export async function getCliVersion(): Promise<CliVersionResponse> {
  const response = await api.get<CliVersionResponse>('/cli/version')
  return response.data
}

/**
 * 获取 CLI 版本历史列表
 */
export async function getCliVersions(): Promise<CliVersion[]> {
  const response = await api.get<CliVersion[]>('/cli/versions')
  return response.data
}