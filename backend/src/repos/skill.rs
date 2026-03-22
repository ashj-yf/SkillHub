use anyhow::Result;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::skill::{
    CreateSkill, CreateSkillTag, CreateSkillVersion, Skill, SkillTag, SkillTagResponse, SkillVersion, SkillManifest,
};
use std::collections::HashMap;

pub struct SkillRepo {
    pool: PgPool,
}

impl SkillRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, author_id: Uuid, payload: &CreateSkill) -> Result<Skill> {
        let skill = sqlx::query_as::<_, Skill>(
            r#"
            INSERT INTO skills (name, slug, description, readme, author_id, tags, is_public)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(&payload.name)
        .bind(&payload.slug)
        .bind(&payload.description)
        .bind(&payload.readme)
        .bind(author_id)
        .bind(&payload.tags)
        .bind(payload.is_public.unwrap_or(true))
        .fetch_one(&self.pool)
        .await?;

        Ok(skill)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Skill>> {
        let skill = sqlx::query_as::<_, Skill>(
            "SELECT * FROM skills WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(skill)
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Skill>> {
        let skill = sqlx::query_as::<_, Skill>(
            "SELECT * FROM skills WHERE slug = $1"
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(skill)
    }

    pub async fn list(&self, q: Option<&str>, tags: Option<&str>, page: u32, per_page: u32, sort: Option<&str>) -> Result<Vec<Skill>> {
        let offset = page.saturating_sub(1) * per_page;

        // 确定排序方式
        let order_clause = match sort {
            Some("downloads") => "download_count DESC",
            Some("name") => "name ASC",
            Some("recent") | None => "created_at DESC",
            _ => "created_at DESC",
        };

        let skills = if let Some(query) = q {
            let pattern = format!("%{}%", query);
            let sql = format!(
                r#"
                SELECT * FROM skills
                WHERE is_public = true
                AND (name ILIKE $1 OR description ILIKE $1)
                ORDER BY {}
                LIMIT $2 OFFSET $3
                "#,
                order_clause
            );
            sqlx::query_as::<_, Skill>(&sql)
            .bind(&pattern)
            .bind(per_page as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        } else if let Some(tag_filter) = tags {
            let tags_vec: Vec<String> = tag_filter.split(',').map(|s| s.trim().to_string()).collect();
            let sql = format!(
                r#"
                SELECT * FROM skills
                WHERE is_public = true
                AND tags && $1
                ORDER BY {}
                LIMIT $2 OFFSET $3
                "#,
                order_clause
            );
            sqlx::query_as::<_, Skill>(&sql)
            .bind(&tags_vec)
            .bind(per_page as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        } else {
            let sql = format!(
                r#"
                SELECT * FROM skills
                WHERE is_public = true
                ORDER BY {}
                LIMIT $1 OFFSET $2
                "#,
                order_clause
            );
            sqlx::query_as::<_, Skill>(&sql)
            .bind(per_page as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(skills)
    }

    pub async fn update(&self, id: Uuid, payload: &crate::models::skill::UpdateSkill) -> Result<Skill> {
        let skill = sqlx::query_as::<_, Skill>(
            r#"
            UPDATE skills
            SET name = COALESCE($1, name),
                description = COALESCE($2, description),
                readme = COALESCE($3, readme),
                tags = COALESCE($4, tags),
                is_public = COALESCE($5, is_public),
                updated_at = NOW()
            WHERE id = $6
            RETURNING *
            "#
        )
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&payload.readme)
        .bind(&payload.tags)
        .bind(&payload.is_public)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(skill)
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM skills WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn increment_download_count(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE skills SET download_count = download_count + 1 WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn find_by_author(&self, author_id: Uuid) -> Result<Vec<Skill>> {
        let skills = sqlx::query_as::<_, Skill>(
            "SELECT * FROM skills WHERE author_id = $1 ORDER BY created_at DESC"
        )
        .bind(author_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(skills)
    }

    // ==================== 版本管理 ====================

    pub async fn create_version(&self, skill_id: Uuid, created_by: Uuid, payload: &CreateSkillVersion) -> Result<SkillVersion> {
        // 计算内容哈希
        let mut hasher = Sha256::new();
        hasher.update(payload.content.as_bytes());
        let digest = format!("{:x}", hasher.finalize());
        let storage_path = format!("skills/{}/versions/{}", skill_id, payload.version);

        let version = sqlx::query_as::<_, SkillVersion>(
            r#"
            INSERT INTO skill_versions (skill_id, version, storage_path, content, changelog, digest, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(skill_id)
        .bind(&payload.version)
        .bind(&storage_path)
        .bind(&payload.content)
        .bind(&payload.changelog)
        .bind(&digest)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;

        // 更新技能版本号
        sqlx::query("UPDATE skills SET version = $1, updated_at = NOW() WHERE id = $2")
            .bind(&payload.version)
            .bind(skill_id)
            .execute(&self.pool)
            .await?;

        Ok(version)
    }

    pub async fn find_version(&self, skill_id: Uuid, version: &str) -> Result<Option<SkillVersion>> {
        let version = sqlx::query_as::<_, SkillVersion>(
            "SELECT * FROM skill_versions WHERE skill_id = $1 AND version = $2"
        )
        .bind(skill_id)
        .bind(version)
        .fetch_optional(&self.pool)
        .await?;

        Ok(version)
    }

    pub async fn list_versions(&self, skill_id: Uuid) -> Result<Vec<SkillVersion>> {
        let versions = sqlx::query_as::<_, SkillVersion>(
            "SELECT * FROM skill_versions WHERE skill_id = $1 ORDER BY created_at DESC"
        )
        .bind(skill_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(versions)
    }

    // ==================== 标签管理 ====================

    pub async fn create_tag(&self, skill_id: Uuid, tag: &str, version_id: Uuid, updated_by: Uuid) -> Result<SkillTag> {
        // 使用 upsert：如果标签存在则更新，不存在则创建
        let skill_tag = sqlx::query_as::<_, SkillTag>(
            r#"
            INSERT INTO skill_tags (skill_id, tag, version_id, updated_by)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (skill_id, tag) DO UPDATE SET
                version_id = EXCLUDED.version_id,
                updated_by = EXCLUDED.updated_by,
                updated_at = NOW()
            RETURNING *
            "#
        )
        .bind(skill_id)
        .bind(tag)
        .bind(version_id)
        .bind(updated_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(skill_tag)
    }

    pub async fn find_tag(&self, skill_id: Uuid, tag: &str) -> Result<Option<SkillTag>> {
        let skill_tag = sqlx::query_as::<_, SkillTag>(
            "SELECT * FROM skill_tags WHERE skill_id = $1 AND tag = $2"
        )
        .bind(skill_id)
        .bind(tag)
        .fetch_optional(&self.pool)
        .await?;

        Ok(skill_tag)
    }

    pub async fn list_tags(&self, skill_id: Uuid) -> Result<Vec<SkillTag>> {
        let tags = sqlx::query_as::<_, SkillTag>(
            "SELECT * FROM skill_tags WHERE skill_id = $1 ORDER BY tag"
        )
        .bind(skill_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    /// 获取标签列表，包含版本号而非仅 version_id
    pub async fn list_tags_with_version(&self, skill_id: Uuid) -> Result<Vec<SkillTagResponse>> {
        let tags = sqlx::query_as::<_, SkillTagResponse>(
            r#"
            SELECT st.id, st.skill_id, st.tag, sv.version, st.updated_at, st.updated_by
            FROM skill_tags st
            JOIN skill_versions sv ON st.version_id = sv.id
            WHERE st.skill_id = $1
            ORDER BY st.tag
            "#
        )
        .bind(skill_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    pub async fn delete_tag(&self, skill_id: Uuid, tag: &str) -> Result<()> {
        sqlx::query("DELETE FROM skill_tags WHERE skill_id = $1 AND tag = $2")
            .bind(skill_id)
            .bind(tag)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 解析标签获取版本（支持 v1 自动匹配 v1.x.x 最新）
    pub async fn resolve_version(&self, skill_id: Uuid, tag: &str) -> Result<Option<SkillVersion>> {
        // 先尝试精确匹配标签
        if let Some(skill_tag) = self.find_tag(skill_id, tag).await? {
            let version = sqlx::query_as::<_, SkillVersion>(
                "SELECT * FROM skill_versions WHERE id = $1"
            )
            .bind(skill_tag.version_id)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(v) = version {
                return Ok(Some(v));
            }
        }

        // 如果标签是版本前缀（如 v1），尝试匹配最新版本
        if tag.starts_with('v') {
            let pattern = format!("{}.%", tag);
            let version = sqlx::query_as::<_, SkillVersion>(
                r#"
                SELECT * FROM skill_versions
                WHERE skill_id = $1 AND version LIKE $2
                ORDER BY version DESC
                LIMIT 1
                "#
            )
            .bind(skill_id)
            .bind(&pattern)
            .fetch_optional(&self.pool)
            .await?;

            if version.is_some() {
                return Ok(version);
            }
        }

        // 尝试直接作为版本号查找
        self.find_version(skill_id, tag).await
    }

    /// 获取技能的完整 manifest
    pub async fn get_manifest(&self, skill_id: Uuid) -> Result<Option<SkillManifest>> {
        let skill = self.find_by_id(skill_id).await?;
        if skill.is_none() {
            return Ok(None);
        }
        let skill = skill.unwrap();

        let tags = self.list_tags(skill_id).await?;
        let versions = self.list_versions(skill_id).await?;

        let tags_map: HashMap<String, String> = tags
            .iter()
            .filter_map(|t| {
                versions.iter()
                    .find(|v| v.id == t.version_id)
                    .map(|v| (t.tag.clone(), v.version.clone()))
            })
            .collect();

        let version_names: Vec<String> = versions.iter().map(|v| v.version.clone()).collect();

        Ok(Some(SkillManifest {
            skill_id: skill_id.to_string(),
            name: skill.name,
            tags: tags_map,
            versions: version_names,
            updated_at: skill.updated_at,
        }))
    }
}