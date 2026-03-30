/**
 * Skills Intelligence Hub - CLI 类型定义
 */

/**
 * CLI 下载信息
 */
export interface CliDownload {
  /** 平台名称 */
  platform: string
  /** 文件名 */
  filename: string
  /** 下载 URL */
  url: string
  /** 文件大小（字节） */
  size?: number
  /** 校验和 */
  checksum?: string
}

/**
 * CLI 版本响应（最新版本）
 */
export interface CliVersionResponse {
  /** 版本号 */
  version: string
  /** 更新日志 */
  changelog: string
  /** 发布日期 */
  release_date: string
  /** 最小兼容版本 */
  min_version: string
  /** 是否强制更新 */
  force_update: boolean
  /** 下载列表 */
  downloads: CliDownload[]
}

/**
 * CLI 版本信息（历史版本）
 */
export interface CliVersion {
  /** 版本 ID */
  id: string
  /** 版本号 */
  version: string
  /** 更新日志 */
  changelog?: string
  /** 发布日期 */
  release_date?: string
  /** 最小兼容版本 */
  min_version?: string
  /** 是否强制更新 */
  force_update: boolean
  /** 创建时间 */
  created_at: string
}