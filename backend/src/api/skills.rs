use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthUser;
use crate::models::skill::{CreateSkill, CreateSkillTag, CreateSkillVersion, Skill, SkillTag, SkillTagResponse, SkillVersion, UpdateSkill, SkillManifest};
use crate::services::skill::SkillService;
use crate::state::AppState;
use crate::utils::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub tags: Option<String>,
    pub page: Option<u32>,
    /// 排序方式：downloads, recent, name
    pub sort: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSkillRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub tags: Vec<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSkillRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub version: String,
    pub content: String,
    pub changelog: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub tag: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct SkillVersionResponse {
    #[serde(flatten)]
    pub skill: Skill,
    pub content: Option<String>,
    pub version_info: SkillVersion,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/skills", get(list))
        .route("/skills", post(create))
        .route("/skills/:slug", get(get_by_slug))
        .route("/skills/:slug/:tag", get(get_by_tag))
        .route("/skills/:slug", put(update))
        .route("/skills/:slug", delete(delete_skill))
        // 版本管理
        .route("/skills/:slug/versions", get(list_versions))
        .route("/skills/:slug/versions", post(create_version))
        // 标签管理
        .route("/skills/:slug/tags", get(list_tags))
        .route("/skills/:slug/tags", post(create_tag))
        .route("/skills/:slug/tags/:tag", delete(delete_tag))
        // Manifest
        .route("/skills/:slug/manifest", get(get_manifest))
}

// ==================== 公开 API ====================

pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<Skill>>, ApiError> {
    let service = SkillService::new(state.db);

    let skills = service.list(query.q.as_deref(), query.tags.as_deref(), query.page, query.sort.as_deref()).await?;

    Ok(Json(skills))
}

pub async fn get_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Skill>, ApiError> {
    let service = SkillService::new(state.db);

    let skill = service.get_by_slug(&slug).await?;

    // 增加下载计数
    service.increment_download(skill.id).await?;

    Ok(Json(skill))
}

pub async fn get_by_tag(
    State(state): State<AppState>,
    Path((slug, tag)): Path<(String, String)>,
) -> Result<Json<SkillVersionResponse>, ApiError> {
    let service = SkillService::new(state.db);

    let (skill, version) = service.get_version(&slug, &tag).await?;

    // 增加下载计数
    service.increment_download(skill.id).await?;

    Ok(Json(SkillVersionResponse {
        skill,
        content: version.content.clone(),
        version_info: version,
    }))
}

// ==================== 需要认证的 API ====================

pub async fn create(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateSkillRequest>,
) -> Result<(StatusCode, Json<Skill>), ApiError> {
    let service = SkillService::new(state.db);

    // 验证输入
    if payload.name.is_empty() || payload.name.len() > 100 {
        return Err(ApiError::BadRequest("技能名称长度应为 1-100 个字符".into()));
    }
    if payload.slug.is_empty() || payload.slug.len() > 100 {
        return Err(ApiError::BadRequest("Slug 长度应为 1-100 个字符".into()));
    }

    let create_skill = CreateSkill {
        name: payload.name,
        slug: payload.slug,
        description: payload.description,
        readme: payload.readme,
        tags: payload.tags,
        is_public: payload.is_public,
    };

    // 从 JWT 获取实际用户 ID
    let author_id = user.id;

    let skill = service.create(author_id, create_skill).await?;

    Ok((StatusCode::CREATED, Json(skill)))
}

pub async fn update(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(payload): Json<UpdateSkillRequest>,
) -> Result<Json<Skill>, ApiError> {
    let service = SkillService::new(state.db);

    let update_skill = UpdateSkill {
        name: payload.name,
        description: payload.description,
        readme: payload.readme,
        tags: payload.tags,
        is_public: payload.is_public,
    };

    // 从 JWT 获取实际用户 ID
    let author_id = user.id;

    let skill = service.update_by_slug(author_id, &slug, update_skill).await?;

    Ok(Json(skill))
}

pub async fn delete_skill(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<StatusCode, ApiError> {
    let service = SkillService::new(state.db);

    // 从 JWT 获取实际用户 ID
    let author_id = user.id;

    service.delete_by_slug(author_id, &slug).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ==================== 版本管理 ====================

pub async fn list_versions(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<SkillVersion>>, ApiError> {
    let service = SkillService::new(state.db);

    let versions = service.list_versions(&slug).await?;

    Ok(Json(versions))
}

pub async fn create_version(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(payload): Json<CreateVersionRequest>,
) -> Result<(StatusCode, Json<SkillVersion>), ApiError> {
    let service = SkillService::new(state.db);

    let create_version = CreateSkillVersion {
        version: payload.version,
        content: payload.content,
        changelog: payload.changelog,
    };

    // 从 JWT 获取实际用户 ID
    let author_id = user.id;

    let version = service.create_version(author_id, &slug, create_version).await?;

    Ok((StatusCode::CREATED, Json(version)))
}

// ==================== 标签管理 ====================

pub async fn list_tags(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<SkillTagResponse>>, ApiError> {
    let service = SkillService::new(state.db);

    let tags = service.list_tags_with_version(&slug).await?;

    Ok(Json(tags))
}

pub async fn create_tag(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(payload): Json<CreateTagRequest>,
) -> Result<Json<SkillTag>, ApiError> {
    let service = SkillService::new(state.db);

    let create_tag = CreateSkillTag {
        tag: payload.tag,
        version: payload.version,
    };

    // 从 JWT 获取实际用户 ID
    let author_id = user.id;

    let tag = service.create_tag(author_id, &slug, create_tag).await?;

    Ok(Json(tag))
}

pub async fn delete_tag(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path((slug, tag)): Path<(String, String)>,
) -> Result<StatusCode, ApiError> {
    let service = SkillService::new(state.db);

    // 从 JWT 获取实际用户 ID
    let author_id = user.id;

    service.delete_tag(author_id, &slug, &tag).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ==================== Manifest ====================

pub async fn get_manifest(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<SkillManifest>, ApiError> {
    let service = SkillService::new(state.db);

    let manifest = service.get_manifest(&slug).await?;

    Ok(Json(manifest))
}