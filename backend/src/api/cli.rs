use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use crate::state::AppState;

/// CLI 版本信息响应
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloads {
    pub linux_x86_64: String,
    pub linux_arm64: String,
    pub macos_x86_64: String,
    pub macos_arm64: String,
    pub windows_x86_64: String,
}

/// 获取 CLI 最新版本信息
pub async fn get_version() -> Json<CliVersionResponse> {
    // TODO: 从配置或数据库读取版本信息
    // 当前返回硬编码版本
    Json(CliVersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        release_date: "2026-03-30".to_string(),
        changelog: "## 新功能\n- 新增 login/logout 命令\n- 新增 local/update/remove/versions 命令\n- 新增 version 命令".to_string(),
        downloads: Downloads {
            linux_x86_64: "https://github.com/ashj-yf/SkillHub/releases/download/v0.1.0/skillhub-linux-x86_64".to_string(),
            linux_arm64: "https://github.com/ashj-yf/SkillHub/releases/download/v0.1.0/skillhub-linux-arm64".to_string(),
            macos_x86_64: "https://github.com/ashj-yf/SkillHub/releases/download/v0.1.0/skillhub-macos-x86_64".to_string(),
            macos_arm64: "https://github.com/ashj-yf/SkillHub/releases/download/v0.1.0/skillhub-macos-arm64".to_string(),
            windows_x86_64: "https://github.com/ashj-yf/SkillHub/releases/download/v0.1.0/skillhub-windows-x86_64.exe".to_string(),
        },
        min_version: "0.1.0".to_string(),
        force_update: false,
    })
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cli/version", get(get_version))
}