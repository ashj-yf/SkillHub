use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::cache::{CacheKey, RedisCache, ttl};
use crate::models::skill::{CreateSkill, CreateSkillTag, CreateSkillVersion, Skill, SkillTag, SkillTagResponse, SkillVersion, SkillManifest, UpdateSkill};
use crate::repos::skill::SkillRepo;
use crate::storage::StorageBackend;

/// Content size threshold for storing in object storage (10KB)
const STORAGE_THRESHOLD: usize = 10 * 1024;

pub struct SkillService {
    skill_repo: SkillRepo,
    storage: StorageBackend,
    cache: Option<RedisCache>,
}

impl SkillService {
    pub fn new(pool: PgPool, storage: StorageBackend) -> Self {
        Self {
            skill_repo: SkillRepo::new(pool),
            storage,
            cache: None,
        }
    }

    /// 创建带缓存的 SkillService
    pub fn with_cache(pool: PgPool, storage: StorageBackend, cache: Option<RedisCache>) -> Self {
        Self {
            skill_repo: SkillRepo::new(pool),
            storage,
            cache,
        }
    }

    pub async fn list(&self, q: Option<&str>, tags: Option<&str>, page: Option<u32>, sort: Option<&str>) -> Result<Vec<Skill>> {
        let page = page.unwrap_or(1);
        let per_page = 20;

        debug!(query = ?q, tags = ?tags, page = page, sort = ?sort, "Listing skills");

        self.skill_repo.list(q, tags, page, per_page, sort).await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Skill> {
        debug!(slug = %slug, "Fetching skill by slug");

        // 尝试从缓存获取
        if let Some(ref cache) = self.cache {
            let cache_key = CacheKey::skill_detail(slug);
            if let Ok(Some(cached)) = cache.get(&cache_key).await {
                if let Ok(skill) = serde_json::from_str::<Skill>(&cached) {
                    debug!(slug = %slug, "Cache hit for skill detail");
                    return Ok(skill);
                }
            }
        }

        // 从数据库获取
        let skill = self.skill_repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| anyhow!("技能不存在"))?;

        // 写入缓存
        if let Some(ref cache) = self.cache {
            let cache_key = CacheKey::skill_detail(slug);
            if let Ok(json) = serde_json::to_string(&skill) {
                let _ = cache.set(&cache_key, &json, ttl::SKILL_DETAIL).await;
            }
        }

        Ok(skill)
    }

    /// 获取技能的指定版本（支持 tag）
    pub async fn get_version(&self, slug: &str, tag: &str) -> Result<(Skill, SkillVersion)> {
        debug!(slug = %slug, tag = %tag, "Fetching skill version");

        let skill = self.get_by_slug(slug).await?;

        let mut version = self.skill_repo
            .resolve_version(skill.id, tag)
            .await?
            .ok_or_else(|| anyhow!("版本或标签 '{}' 不存在", tag))?;

        // If content is not in DB, fetch from object storage
        if version.content.is_none() && version.storage_path.starts_with("skills/") {
            debug!(skill_id = %skill.id, version = %version.version, storage_path = %version.storage_path, "Fetching content from object storage");
            let content = self.storage
                .download_skill_content(skill.id, &version.version)
                .await?
                .ok_or_else(|| anyhow!("版本内容在存储中不存在"))?;
            // Convert bytes to String (using lossy conversion for non-UTF8)
            version.content = Some(String::from_utf8_lossy(&content).to_string());
        }

        Ok((skill, version))
    }

    pub async fn create(&self, author_id: Uuid, payload: CreateSkill) -> Result<Skill> {
        debug!(author_id = %author_id, slug = %payload.slug, name = %payload.name, "Creating skill");

        // 验证 slug 格式
        if !is_valid_slug(&payload.slug) {
            warn!(slug = %payload.slug, author_id = %author_id, "Skill creation failed: invalid slug format");
            return Err(anyhow!("Slug 只能包含小写字母、数字和连字符"));
        }

        // 检查 slug 是否已存在
        if self.skill_repo.find_by_slug(&payload.slug).await?.is_some() {
            warn!(slug = %payload.slug, author_id = %author_id, "Skill creation failed: slug already exists");
            return Err(anyhow!("Slug 已被使用"));
        }

        let skill = self.skill_repo.create(author_id, &payload).await?;

        info!(skill_id = %skill.id, slug = %skill.slug, author_id = %author_id, "Skill created successfully");

        Ok(skill)
    }

    pub async fn update(&self, author_id: Uuid, skill_id: Uuid, payload: UpdateSkill) -> Result<Skill> {
        // 检查技能是否存在且属于该用户
        let skill = self.skill_repo
            .find_by_id(skill_id)
            .await?
            .ok_or_else(|| anyhow!("技能不存在"))?;

        if skill.author_id != Some(author_id) {
            warn!(skill_id = %skill_id, author_id = %author_id, actual_author = ?skill.author_id, "Skill update denied: not owner");
            return Err(anyhow!("无权修改此技能"));
        }

        let updated = self.skill_repo.update(skill_id, &payload).await?;

        info!(skill_id = %skill_id, author_id = %author_id, "Skill updated successfully");

        Ok(updated)
    }

    /// 通过 slug 更新技能
    pub async fn update_by_slug(&self, author_id: Uuid, slug: &str, payload: UpdateSkill) -> Result<Skill> {
        // 检查技能是否存在且属于该用户
        let skill = self.skill_repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| anyhow!("技能不存在"))?;

        if skill.author_id != Some(author_id) {
            warn!(slug = %slug, author_id = %author_id, actual_author = ?skill.author_id, "Skill update denied: not owner");
            return Err(anyhow!("无权修改此技能"));
        }

        let updated = self.skill_repo.update(skill.id, &payload).await?;

        info!(skill_id = %skill.id, slug = %slug, author_id = %author_id, "Skill updated successfully");

        Ok(updated)
    }

    pub async fn delete(&self, author_id: Uuid, skill_id: Uuid) -> Result<()> {
        // 检查技能是否存在且属于该用户
        let skill = self.skill_repo
            .find_by_id(skill_id)
            .await?
            .ok_or_else(|| anyhow!("技能不存在"))?;

        if skill.author_id != Some(author_id) {
            warn!(skill_id = %skill_id, author_id = %author_id, actual_author = ?skill.author_id, "Skill deletion denied: not owner");
            return Err(anyhow!("无权删除此技能"));
        }

        self.skill_repo.delete(skill_id).await?;

        info!(skill_id = %skill_id, author_id = %author_id, "Skill deleted successfully");

        Ok(())
    }

    /// 通过 slug 删除技能
    pub async fn delete_by_slug(&self, author_id: Uuid, slug: &str) -> Result<()> {
        // 检查技能是否存在且属于该用户
        let skill = self.skill_repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| anyhow!("技能不存在"))?;

        if skill.author_id != Some(author_id) {
            warn!(slug = %slug, author_id = %author_id, actual_author = ?skill.author_id, "Skill deletion denied: not owner");
            return Err(anyhow!("无权删除此技能"));
        }

        self.skill_repo.delete(skill.id).await?;

        info!(skill_id = %skill.id, slug = %slug, author_id = %author_id, "Skill deleted successfully");

        Ok(())
    }

    pub async fn increment_download(&self, skill_id: Uuid) -> Result<()> {
        self.skill_repo.increment_download_count(skill_id).await
    }

    pub async fn list_by_author(&self, author_id: Uuid) -> Result<Vec<Skill>> {
        self.skill_repo.find_by_author(author_id).await
    }

    // ==================== 版本管理 ====================

    pub async fn create_version(&self, author_id: Uuid, slug: &str, payload: CreateSkillVersion) -> Result<SkillVersion> {
        debug!(author_id = %author_id, slug = %slug, version = %payload.version, "Creating skill version");

        let skill = self.get_by_slug(slug).await?;

        if skill.author_id != Some(author_id) {
            warn!(slug = %slug, skill_id = %skill.id, author_id = %author_id, "Version creation denied: not owner");
            return Err(anyhow!("无权为此技能创建版本"));
        }

        // 验证版本号格式
        if !is_valid_version(&payload.version) {
            warn!(version = %payload.version, slug = %slug, "Version creation failed: invalid version format");
            return Err(anyhow!("版本号格式无效，应为 v1.0.0 格式"));
        }

        // 检查版本是否已存在
        if self.skill_repo.find_version(skill.id, &payload.version).await?.is_some() {
            warn!(skill_id = %skill.id, version = %payload.version, "Version creation failed: version already exists");
            return Err(anyhow!("版本 {} 已存在", payload.version));
        }

        // Calculate content hash
        debug!(skill_id = %skill.id, content_len = payload.content.len(), "Calculating content hash");
        let mut hasher = Sha256::new();
        hasher.update(payload.content.as_bytes());
        let digest = format!("{:x}", hasher.finalize());

        // Determine storage strategy based on content size
        let content_bytes = payload.content.as_bytes();
        let content_len = content_bytes.len();
        let (content, storage_path) = if content_len > STORAGE_THRESHOLD {
            // Store in object storage for large content
            debug!(skill_id = %skill.id, content_len = content_len, "Storing content in object storage (exceeds threshold)");
            let path = self.storage
                .upload_skill_content(skill.id, &payload.version, content_bytes)
                .await?;
            info!(
                skill_id = %skill.id,
                version = %payload.version,
                content_size = content_len,
                storage_path = %path,
                "Version content stored in object storage"
            );
            (None, path)
        } else {
            // Store in database for small content
            debug!(skill_id = %skill.id, content_len = content_len, "Storing content in database (within threshold)");
            let path = format!("db://skills/{}/versions/{}", skill.id, payload.version);
            (Some(payload.content.clone()), path)
        };

        let version = self.skill_repo
            .create_version(
                skill.id,
                author_id,
                &payload.version,
                content.as_deref(),
                &storage_path,
                payload.changelog.as_deref(),
                &digest,
            )
            .await?;

        // 如果是第一个版本，自动创建 latest 标签
        let versions = self.skill_repo.list_versions(skill.id).await?;
        if versions.len() == 1 {
            debug!(skill_id = %skill.id, version_id = %version.id, "Auto-creating 'latest' tag for first version");
            self.skill_repo.create_tag(skill.id, "latest", version.id, author_id).await?;
        }

        info!(
            skill_id = %skill.id,
            slug = %slug,
            version = %version.version,
            author_id = %author_id,
            "Skill version created successfully"
        );

        Ok(version)
    }

    pub async fn list_versions(&self, slug: &str) -> Result<Vec<SkillVersion>> {
        let skill = self.get_by_slug(slug).await?;
        self.skill_repo.list_versions(skill.id).await
    }

    // ==================== 标签管理 ====================

    pub async fn list_tags(&self, slug: &str) -> Result<Vec<SkillTag>> {
        let skill = self.get_by_slug(slug).await?;
        self.skill_repo.list_tags(skill.id).await
    }

    /// 获取标签列表，包含版本号而非仅 version_id
    pub async fn list_tags_with_version(&self, slug: &str) -> Result<Vec<SkillTagResponse>> {
        let skill = self.get_by_slug(slug).await?;
        self.skill_repo.list_tags_with_version(skill.id).await
    }

    pub async fn create_tag(&self, author_id: Uuid, slug: &str, payload: CreateSkillTag) -> Result<SkillTag> {
        debug!(author_id = %author_id, slug = %slug, tag = %payload.tag, version = %payload.version, "Creating skill tag");

        let skill = self.get_by_slug(slug).await?;

        if skill.author_id != Some(author_id) {
            warn!(slug = %slug, skill_id = %skill.id, author_id = %author_id, "Tag creation denied: not owner");
            return Err(anyhow!("无权为此技能创建标签"));
        }

        // 查找版本
        let version = self.skill_repo
            .find_version(skill.id, &payload.version)
            .await?
            .ok_or_else(|| {
                warn!(skill_id = %skill.id, version = %payload.version, "Tag creation failed: version not found");
                anyhow!("版本 {} 不存在", payload.version)
            })?;

        // 创建或更新标签
        let tag = self.skill_repo.create_tag(skill.id, &payload.tag, version.id, author_id).await?;

        info!(
            skill_id = %skill.id,
            slug = %slug,
            tag = %tag.tag,
            version = %payload.version,
            author_id = %author_id,
            "Skill tag created successfully"
        );

        Ok(tag)
    }

    pub async fn delete_tag(&self, author_id: Uuid, slug: &str, tag: &str) -> Result<()> {
        let skill = self.get_by_slug(slug).await?;

        if skill.author_id != Some(author_id) {
            warn!(slug = %slug, skill_id = %skill.id, tag = %tag, author_id = %author_id, "Tag deletion denied: not owner");
            return Err(anyhow!("无权删除此技能的标签"));
        }

        // 不允许删除 latest 标签
        if tag == "latest" {
            warn!(skill_id = %skill.id, "Tag deletion failed: cannot delete 'latest' tag");
            return Err(anyhow!("不能删除 latest 标签"));
        }

        self.skill_repo.delete_tag(skill.id, tag).await?;

        info!(skill_id = %skill.id, slug = %slug, tag = %tag, author_id = %author_id, "Skill tag deleted successfully");

        Ok(())
    }

    pub async fn get_manifest(&self, slug: &str) -> Result<SkillManifest> {
        debug!(slug = %slug, "Fetching skill manifest");

        let skill = self.get_by_slug(slug).await?;

        self.skill_repo
            .get_manifest(skill.id)
            .await?
            .ok_or_else(|| anyhow!("无法获取技能清单"))
    }
}

fn is_valid_slug(slug: &str) -> bool {
    !slug.is_empty() && slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn is_valid_version(version: &str) -> bool {
    // 支持语义版本：v1.0.0, v1.0.0-beta.1 等
    let version_regex = regex::Regex::new(r"^v\d+\.\d+\.\d+(-[a-zA-Z0-9]+(\.[a-zA-Z0-9]+)*)?$").unwrap();
    version_regex.is_match(version)
}