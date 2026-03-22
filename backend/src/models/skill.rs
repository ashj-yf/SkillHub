use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub author_id: Option<Uuid>,
    pub version: String,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub download_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSkill {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub tags: Vec<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSkill {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SkillVersion {
    pub id: Uuid,
    pub skill_id: Uuid,
    pub version: String,
    pub storage_path: String,
    pub content: Option<String>,
    pub changelog: Option<String>,
    pub digest: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSkillVersion {
    pub version: String,
    pub content: String,
    pub changelog: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SkillTag {
    pub id: Uuid,
    pub skill_id: Uuid,
    pub tag: String,
    pub version_id: Uuid,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}

/// 标签响应，包含版本号而非仅 version_id
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SkillTagResponse {
    pub id: Uuid,
    pub skill_id: Uuid,
    pub tag: String,
    pub version: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSkillTag {
    pub tag: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct SkillManifest {
    pub skill_id: String,
    pub name: String,
    pub tags: std::collections::HashMap<String, String>,
    pub versions: Vec<String>,
    pub updated_at: DateTime<Utc>,
}