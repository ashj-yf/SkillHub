use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthUser;
use crate::middleware::permission::{
    check_ownership_or_permission, is_admin, resources, actions,
};
use crate::models::skill::{CreateSkill, CreateSkillTag, CreateSkillVersion, Skill, SkillTag, SkillTagResponse, SkillVersion, UpdateSkill, SkillManifest};
use crate::repos::skill::SkillRepo;
use crate::services::skill::SkillService;
use crate::state::AppState;
use crate::utils::error::ApiError;

/// 从 AppState 创建带缓存的 SkillService
fn create_service(state: &AppState) -> SkillService {
    SkillService::with_cache(state.db.clone(), state.storage.clone(), state.cache.clone())
}

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
        .route("/skills/:slug/download/:tag", get(download_file))
        .route("/skills/:slug", put(update))
        .route("/skills/:slug", delete(delete_skill))
        // 版本管理
        .route("/skills/:slug/versions", get(list_versions))
        .route("/skills/:slug/versions", post(create_version))
        .route("/skills/:slug/versions/upload", post(upload_version))
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
    let service = create_service(&state);

    let skills = service.list(query.q.as_deref(), query.tags.as_deref(), query.page, query.sort.as_deref()).await?;

    Ok(Json(skills))
}

pub async fn get_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Skill>, ApiError> {
    let service = create_service(&state);

    let skill = service.get_by_slug(&slug).await?;

    // 增加下载计数
    service.increment_download(skill.id).await?;

    Ok(Json(skill))
}

pub async fn get_by_tag(
    State(state): State<AppState>,
    Path((slug, tag)): Path<(String, String)>,
) -> Result<Json<SkillVersionResponse>, ApiError> {
    let service = create_service(&state);

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
    // 权限检查：需要 skills:create 权限
    // 注意：创建技能的权限检查暂时开放给所有已登录用户
    // 如需限制，取消下面的注释
    // check_permission_or_forbidden(&state, user.id, resources::SKILLS, actions::CREATE).await?;

    let service = create_service(&state);

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
    let service = create_service(&state);
    let repo = SkillRepo::new(state.db.clone());

    // 获取技能以检查所有权
    let skill = repo.find_by_slug(&slug).await?
        .ok_or_else(|| ApiError::NotFound("技能不存在".into()))?;

    // 权限检查：所有者或 skills:update 权限
    check_ownership_or_permission(&state, user.id, &skill, resources::SKILLS, actions::UPDATE).await?;

    let update_skill = UpdateSkill {
        name: payload.name,
        description: payload.description,
        readme: payload.readme,
        tags: payload.tags,
        is_public: payload.is_public,
    };

    let skill = service.update_by_slug(user.id, &slug, update_skill).await?;

    Ok(Json(skill))
}

pub async fn delete_skill(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<StatusCode, ApiError> {
    let service = create_service(&state);
    let repo = SkillRepo::new(state.db.clone());

    // 获取技能以检查所有权
    let skill = repo.find_by_slug(&slug).await?
        .ok_or_else(|| ApiError::NotFound("技能不存在".into()))?;

    // 权限检查：所有者或 skills:delete 权限
    check_ownership_or_permission(&state, user.id, &skill, resources::SKILLS, actions::DELETE).await?;

    service.delete_by_slug(user.id, &slug).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ==================== 版本管理 ====================

pub async fn list_versions(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<SkillVersion>>, ApiError> {
    let service = create_service(&state);

    let versions = service.list_versions(&slug).await?;

    Ok(Json(versions))
}

pub async fn create_version(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(payload): Json<CreateVersionRequest>,
) -> Result<(StatusCode, Json<SkillVersion>), ApiError> {
    let service = create_service(&state);
    let repo = SkillRepo::new(state.db.clone());

    // 获取技能以检查所有权
    let skill = repo.find_by_slug(&slug).await?
        .ok_or_else(|| ApiError::NotFound("技能不存在".into()))?;

    // 权限检查：所有者或 skills:update 权限（创建版本视为更新技能）
    check_ownership_or_permission(&state, user.id, &skill, resources::SKILLS, actions::UPDATE).await?;

    let create_version = CreateSkillVersion {
        version: payload.version,
        content: payload.content,
        changelog: payload.changelog,
    };

    let version = service.create_version(user.id, &slug, create_version).await?;

    Ok((StatusCode::CREATED, Json(version)))
}

// ==================== 标签管理 ====================

pub async fn list_tags(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<SkillTagResponse>>, ApiError> {
    let service = create_service(&state);

    let tags = service.list_tags_with_version(&slug).await?;

    Ok(Json(tags))
}

pub async fn create_tag(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    Json(payload): Json<CreateTagRequest>,
) -> Result<Json<SkillTag>, ApiError> {
    let service = create_service(&state);
    let repo = SkillRepo::new(state.db.clone());

    // 获取技能以检查所有权
    let skill = repo.find_by_slug(&slug).await?
        .ok_or_else(|| ApiError::NotFound("技能不存在".into()))?;

    // 权限检查：所有者或 skills:update 权限
    check_ownership_or_permission(&state, user.id, &skill, resources::SKILLS, actions::UPDATE).await?;

    let create_tag = CreateSkillTag {
        tag: payload.tag,
        version: payload.version,
    };

    let tag = service.create_tag(user.id, &slug, create_tag).await?;

    Ok(Json(tag))
}

pub async fn delete_tag(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path((slug, tag)): Path<(String, String)>,
) -> Result<StatusCode, ApiError> {
    let service = create_service(&state);
    let repo = SkillRepo::new(state.db.clone());

    // 获取技能以检查所有权
    let skill = repo.find_by_slug(&slug).await?
        .ok_or_else(|| ApiError::NotFound("技能不存在".into()))?;

    // 权限检查：所有者或 skills:update 权限
    check_ownership_or_permission(&state, user.id, &skill, resources::SKILLS, actions::UPDATE).await?;

    service.delete_tag(user.id, &slug, &tag).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ==================== Manifest ====================

pub async fn get_manifest(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<SkillManifest>, ApiError> {
    let service = create_service(&state);

    let manifest = service.get_manifest(&slug).await?;

    Ok(Json(manifest))
}

// ==================== 文件上传/下载 ====================

/// 通过 multipart/form-data 上传技能版本
pub async fn upload_version(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<SkillVersion>), ApiError> {
    let repo = SkillRepo::new(state.db.clone());

    // 获取技能以检查所有权
    let skill = repo.find_by_slug(&slug).await?
        .ok_or_else(|| ApiError::NotFound("技能不存在".into()))?;

    // 权限检查：所有者或 skills:update 权限
    check_ownership_or_permission(&state, user.id, &skill, resources::SKILLS, actions::UPDATE).await?;

    let mut version = None;
    let mut changelog = None;
    let mut file_data = None;
    let mut filename = None;

    // 解析 multipart 表单
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!(error = %e, "Failed to read multipart field");
        ApiError::BadRequest("Failed to read form data".into())
    })? {
        match field.name() {
            Some("version") => {
                version = Some(field.text().await.map_err(|e| {
                    tracing::error!(error = %e, "Failed to read version field");
                    ApiError::BadRequest("Invalid version field".into())
                })?);
            }
            Some("changelog") => {
                changelog = Some(field.text().await.map_err(|e| {
                    tracing::error!(error = %e, "Failed to read changelog field");
                    ApiError::BadRequest("Invalid changelog field".into())
                })?);
            }
            Some("file") => {
                filename = field.file_name().map(|s| s.to_string());
                let data = field.bytes().await.map_err(|e| {
                    tracing::error!(error = %e, "Failed to read file field");
                    ApiError::BadRequest("Invalid file field".into())
                })?;
                file_data = Some(data);
            }
            _ => {}
        }
    }

    // 验证必填字段
    let version = version.ok_or_else(|| ApiError::BadRequest("version is required".into()))?;
    let file_data = file_data.ok_or_else(|| ApiError::BadRequest("file is required".into()))?;

    // 验证版本号格式
    if !is_valid_version(&version) {
        return Err(ApiError::BadRequest("Invalid version format, should be like v1.0.0".into()));
    }

    // 验证文件类型（可选：检查 .tar.gz 扩展名）
    if let Some(ref fname) = filename {
        if !fname.ends_with(".tar.gz") && !fname.ends_with(".md") && !fname.ends_with(".txt") {
            tracing::warn!(filename = %fname, "File type validation: unexpected extension");
        }
    }

    // 将文件内容转换为字符串（对于 tar.gz 也可以存储为 base64 或直接存储二进制）
    let content = String::from_utf8_lossy(&file_data).to_string();

    // 创建版本
    let service = create_service(&state);
    let author_id = user.id;

    let create_version = CreateSkillVersion {
        version,
        content,
        changelog,
    };

    let skill_version = service.create_version(author_id, &slug, create_version).await?;

    Ok((StatusCode::CREATED, Json(skill_version)))
}

/// 下载技能版本文件
pub async fn download_file(
    State(state): State<AppState>,
    Path((slug, tag)): Path<(String, String)>,
) -> Result<Response, ApiError> {
    let service = create_service(&state);

    let (skill, version) = service.get_version(&slug, &tag).await?;

    let content = version.content.ok_or_else(|| {
        ApiError::NotFound("Version content not found".into())
    })?;

    // 构建文件名
    let filename = format!("{}-{}.md", slug, tag);
    let body = Body::from(content);

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .header(header::CONTENT_TYPE, "text/markdown; charset=utf-8")
        .body(body)
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to build download response");
            ApiError::InternalServerError
        })
}

/// 验证版本号格式
fn is_valid_version(version: &str) -> bool {
    let version_regex = regex::Regex::new(r"^v\d+\.\d+\.\d+(-[a-zA-Z0-9]+(\.[a-zA-Z0-9]+)*)?$").unwrap();
    version_regex.is_match(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_version() {
        // 有效的版本号
        assert!(is_valid_version("v1.0.0"));
        assert!(is_valid_version("v1.2.3"));
        assert!(is_valid_version("v0.0.1"));
        assert!(is_valid_version("v1.0.0-alpha"));
        assert!(is_valid_version("v1.0.0-beta.1"));
        assert!(is_valid_version("v2.1.0-rc.1"));

        // 无效的版本号
        assert!(!is_valid_version("1.0.0"));      // 缺少 v 前缀
        assert!(!is_valid_version("v1.0"));        // 缺少补丁版本
        assert!(!is_valid_version("v1"));          // 缺少次版本和补丁
        assert!(!is_valid_version("v1.0.0."));     // 末尾有点
        assert!(!is_valid_version("v1.0.0-"));     // 末尾有连字符
    }
}