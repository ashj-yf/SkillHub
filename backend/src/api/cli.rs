use axum::{extract::State, Json, Router, routing::get};
use crate::state::AppState;
use crate::repos::cli_version::CliVersionRepo;
use crate::utils::error::ApiError;

/// CLI 版本信息响应（保留原有结构用于兼容）
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CliVersionResponse {
    /// 版本号
    pub version: String,
    /// 发布日期
    pub release_date: String,
    /// 更新日志
    pub changelog: String,
    /// 各平台下载链接
    pub downloads: Downloads,
    /// 最低支持版本
    pub min_version: String,
    /// 是否强制更新
    pub force_update: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Downloads {
    pub linux_x86_64: String,
    pub linux_arm64: String,
    pub macos_x86_64: String,
    pub macos_arm64: String,
    pub windows_x86_64: String,
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

    // 构建下载链接映射
    let downloads_resp = build_downloads_response(downloads);

    Ok(Json(CliVersionResponse {
        version: version.version,
        release_date: version.release_date
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_default(),
        changelog: version.changelog.unwrap_or_default(),
        downloads: downloads_resp,
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

/// 构建下载链接响应
fn build_downloads_response(downloads: Vec<crate::models::cli_version::CliDownload>) -> Downloads {
    let mut result = Downloads {
        linux_x86_64: String::new(),
        linux_arm64: String::new(),
        macos_x86_64: String::new(),
        macos_arm64: String::new(),
        windows_x86_64: String::new(),
    };

    for download in downloads {
        match download.platform.as_str() {
            "linux-x86_64" => result.linux_x86_64 = download.url,
            "linux-arm64" => result.linux_arm64 = download.url,
            "macos-x86_64" => result.macos_x86_64 = download.url,
            "macos-arm64" => result.macos_arm64 = download.url,
            "windows-x86_64" => result.windows_x86_64 = download.url,
            _ => {}
        }
    }

    result
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cli/version", get(get_version))
        .route("/cli/versions", get(list_versions))
}