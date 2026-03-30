use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::cli_version::{CliVersion, CliDownload};

pub struct CliVersionRepo {
    pool: PgPool,
}

impl CliVersionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 获取最新版本信息
    pub async fn get_latest(&self) -> Result<Option<CliVersion>> {
        let version = sqlx::query_as::<_, CliVersion>(
            r#"
            SELECT * FROM cli_versions
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(version)
    }

    /// 获取所有版本列表
    pub async fn list_all(&self) -> Result<Vec<CliVersion>> {
        let versions = sqlx::query_as::<_, CliVersion>(
            r#"
            SELECT * FROM cli_versions
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(versions)
    }

    /// 获取指定版本的下载链接
    pub async fn get_downloads(&self, version_id: Uuid) -> Result<Vec<CliDownload>> {
        let downloads = sqlx::query_as::<_, CliDownload>(
            r#"
            SELECT * FROM cli_downloads
            WHERE version_id = $1
            ORDER BY platform
            "#
        )
        .bind(version_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(downloads)
    }

    /// 获取指定版本信息
    pub async fn get_by_version(&self, version: &str) -> Result<Option<CliVersion>> {
        let cli_version = sqlx::query_as::<_, CliVersion>(
            "SELECT * FROM cli_versions WHERE version = $1"
        )
        .bind(version)
        .fetch_optional(&self.pool)
        .await?;

        Ok(cli_version)
    }
}