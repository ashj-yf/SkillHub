use axum::{extract::State, Json, Router, routing::get};
use crate::state::AppState;
use crate::repos::cli_version::CliVersionRepo;
use crate::utils::error::ApiError;

/// CLI 下载链接（数组格式，与前端类型定义一致）
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CliDownloadItem {
    pub platform: String,
    pub filename: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

/// CLI 版本信息响应
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CliVersionResponse {
    /// 版本号
    pub version: String,
    /// 发布日期
    pub release_date: String,
    /// 更新日志
    pub changelog: String,
    /// 各平台下载链接（数组格式）
    pub downloads: Vec<CliDownloadItem>,
    /// 最低支持版本
    pub min_version: String,
    /// 是否强制更新
    pub force_update: bool,
}

/// 版本列表响应
#[derive(Debug, serde::Serialize)]
pub struct VersionListResponse {
    pub versions: Vec<VersionListItem>,
}

#[derive(Debug, serde::Serialize)]
pub struct VersionListItem {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    pub force_update: bool,
}

/// 获取 CLI 最新版本信息（从数据库读取）
pub async fn get_version(State(state): State<AppState>) -> Result<Json<CliVersionResponse>, ApiError> {
    let repo = CliVersionRepo::new(state.db);

    // 获取最新版本
    let version = repo.get_latest().await?
        .ok_or_else(|| ApiError::NotFound("No CLI version found in database".into()))?;

    // 获取下载链接
    let downloads = repo.get_downloads(version.id).await?;

    // 转换为数组格式
    let download_items: Vec<CliDownloadItem> = downloads
        .into_iter()
        .map(|d| CliDownloadItem {
            platform: d.platform,
            filename: d.filename,
            url: d.url,
            size: d.size,
            checksum: d.checksum,
        })
        .collect();

    Ok(Json(CliVersionResponse {
        version: version.version,
        release_date: version.release_date
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_default(),
        changelog: version.changelog.unwrap_or_default(),
        downloads: download_items,
        min_version: version.min_version.unwrap_or_default(),
        force_update: version.force_update,
    }))
}

/// 获取 CLI 版本列表
pub async fn list_versions(State(state): State<AppState>) -> Result<Json<VersionListResponse>, ApiError> {
    let repo = CliVersionRepo::new(state.db);

    let versions = repo.list_all().await?;

    let items = versions.into_iter().map(|v| {
        VersionListItem {
            version: v.version,
            release_date: v.release_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
            changelog: v.changelog.unwrap_or_default(),
            force_update: v.force_update,
        }
    }).collect();

    Ok(Json(VersionListResponse { versions: items }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cli/version", get(get_version))
        .route("/cli/versions", get(list_versions))
}