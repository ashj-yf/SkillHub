use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// CLI 版本信息
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CliVersion {
    pub id: Uuid,
    pub version: String,
    pub changelog: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub min_version: Option<String>,
    pub force_update: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// CLI 下载链接
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CliDownload {
    pub id: Uuid,
    pub version_id: Uuid,
    pub platform: String,
    pub filename: String,
    pub url: String,
    pub size: Option<i64>,
    pub checksum: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// API 响应：CLI 版本信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CliVersionResponse {
    /// 版本号
    pub version: String,
    /// 发布日期
    pub release_date: Option<String>,
    /// 更新日志
    pub changelog: Option<String>,
    /// 各平台下载链接
    pub downloads: Downloads,
    /// 最低支持版本
    pub min_version: Option<String>,
    /// 是否强制更新
    pub force_update: bool,
}

/// 各平台下载链接
#[derive(Debug, Serialize, Deserialize)]
pub struct Downloads {
    pub linux_x86_64: Option<String>,
    pub linux_arm64: Option<String>,
    pub macos_x86_64: Option<String>,
    pub macos_arm64: Option<String>,
    pub windows_x86_64: Option<String>,
}

/// API 响应：版本列表项
#[derive(Debug, Serialize, Deserialize)]
pub struct CliVersionListItem {
    pub version: String,
    pub release_date: Option<String>,
    pub changelog: Option<String>,
    pub force_update: bool,
}

/// 将数据库模型转换为 API 响应
impl CliVersion {
    pub fn to_response(&self, downloads: Vec<CliDownload>) -> CliVersionResponse {
        let downloads_map = Downloads::from_downloads(downloads);

        CliVersionResponse {
            version: self.version.clone(),
            release_date: self.release_date.map(|d| d.format("%Y-%m-%d").to_string()),
            changelog: self.changelog.clone(),
            downloads: downloads_map,
            min_version: self.min_version.clone(),
            force_update: self.force_update,
        }
    }

    pub fn to_list_item(&self) -> CliVersionListItem {
        CliVersionListItem {
            version: self.version.clone(),
            release_date: self.release_date.map(|d| d.format("%Y-%m-%d").to_string()),
            changelog: self.changelog.clone(),
            force_update: self.force_update,
        }
    }
}

impl Downloads {
    /// 从下载链接列表构建 Downloads 结构
    pub fn from_downloads(downloads: Vec<CliDownload>) -> Self {
        let mut result = Downloads {
            linux_x86_64: None,
            linux_arm64: None,
            macos_x86_64: None,
            macos_arm64: None,
            windows_x86_64: None,
        };

        for download in downloads {
            match download.platform.as_str() {
                "linux-x86_64" => result.linux_x86_64 = Some(download.url),
                "linux-arm64" => result.linux_arm64 = Some(download.url),
                "macos-x86_64" => result.macos_x86_64 = Some(download.url),
                "macos-arm64" => result.macos_arm64 = Some(download.url),
                "windows-x86_64" => result.windows_x86_64 = Some(download.url),
                _ => {}
            }
        }

        result
    }
}