use axum::{
    extract::{Path, State},
    routing::{delete, get},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::middleware::permission::{
    check_permission_or_forbidden, resources, actions,
};
use crate::models::role::{CreateRole, Role, RoleDetail, UpdateRole};
use crate::models::permission::Permission;
use crate::repos::permission::PermissionRepo;
use crate::repos::role::RoleRepo;
use crate::state::AppState;
use crate::utils::error::ApiError;

/// 添加权限请求
#[derive(Debug, Deserialize)]
pub struct AddPermissionRequest {
    pub permission_id: Uuid,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        // 角色管理
        .route("/roles", get(list_roles).post(create_role))
        .route("/roles/{id}", get(get_role).put(update_role).delete(delete_role))
        // 角色权限管理
        .route("/roles/{id}/permissions", get(get_role_permissions).post(add_permission))
        .route("/roles/{id}/permissions/{permission_id}", delete(remove_permission))
        // 权限列表
        .route("/permissions", get(list_permissions))
}

/// 获取所有角色
pub async fn list_roles(
    State(state): State<AppState>,
) -> Result<Json<Vec<Role>>, ApiError> {
    let repo = RoleRepo::new(state.db);
    let roles = repo.find_all().await?;
    Ok(Json(roles))
}

/// 创建角色
pub async fn create_role(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Json(payload): Json<CreateRole>,
) -> Result<Json<Role>, ApiError> {
    // 权限检查：需要 roles:create 权限
    check_permission_or_forbidden(&state, current_user.id, resources::ROLES, actions::CREATE).await?;

    // 验证
    if payload.name.is_empty() || payload.name.len() > 50 {
        return Err(ApiError::BadRequest("角色名称长度应为 1-50 个字符".into()));
    }

    let repo = RoleRepo::new(state.db);

    // 检查名称是否已存在
    if repo.find_by_name(&payload.name).await?.is_some() {
        return Err(ApiError::Conflict("角色名称已存在".into()));
    }

    let role = repo.create(&payload).await?;
    Ok(Json(role))
}

/// 获取角色详情
pub async fn get_role(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<RoleDetail>, ApiError> {
    let repo = RoleRepo::new(state.db);
    let detail = repo.get_detail(id).await?
        .ok_or_else(|| ApiError::NotFound("角色不存在".into()))?;

    Ok(Json(detail))
}

/// 更新角色
pub async fn update_role(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRole>,
) -> Result<Json<Role>, ApiError> {
    // 权限检查：需要 roles:update 权限
    check_permission_or_forbidden(&state, current_user.id, resources::ROLES, actions::UPDATE).await?;

    let repo = RoleRepo::new(state.db);

    // 如果更新名称，检查是否已存在
    if let Some(ref name) = payload.name {
        if let Some(existing) = repo.find_by_name(name).await? {
            if existing.id != id {
                return Err(ApiError::Conflict("角色名称已存在".into()));
            }
        }
    }

    let role = repo.update(id, &payload).await?
        .ok_or_else(|| ApiError::NotFound("角色不存在或为系统角色".into()))?;

    Ok(Json(role))
}

/// 删除角色
pub async fn delete_role(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 roles:delete 权限
    check_permission_or_forbidden(&state, current_user.id, resources::ROLES, actions::DELETE).await?;

    let repo = RoleRepo::new(state.db);

    let deleted = repo.delete(id).await?;
    if !deleted {
        return Err(ApiError::NotFound("角色不存在或为系统角色".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// 获取角色的权限列表
pub async fn get_role_permissions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<String>>, ApiError> {
    let repo = RoleRepo::new(state.db);

    // 检查角色是否存在
    if repo.find_by_id(id).await?.is_none() {
        return Err(ApiError::NotFound("角色不存在".into()));
    }

    let permissions = repo.get_role_permissions(id).await?;
    Ok(Json(permissions))
}

/// 为角色添加权限
pub async fn add_permission(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<AddPermissionRequest>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 roles:update 权限（管理角色权限视为更新角色）
    check_permission_or_forbidden(&state, current_user.id, resources::ROLES, actions::UPDATE).await?;

    let role_repo = RoleRepo::new(state.db.clone());
    let permission_repo = PermissionRepo::new(state.db);

    // 检查角色是否存在
    if role_repo.find_by_id(id).await?.is_none() {
        return Err(ApiError::NotFound("角色不存在".into()));
    }

    // 检查权限是否存在
    if permission_repo.find_by_id(payload.permission_id).await?.is_none() {
        return Err(ApiError::NotFound("权限不存在".into()));
    }

    role_repo.add_permission(id, payload.permission_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 从角色移除权限
pub async fn remove_permission(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path((id, permission_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 roles:update 权限
    check_permission_or_forbidden(&state, current_user.id, resources::ROLES, actions::UPDATE).await?;

    let repo = RoleRepo::new(state.db);
    repo.remove_permission(id, permission_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 获取所有权限
pub async fn list_permissions(
    State(state): State<AppState>,
) -> Result<Json<Vec<Permission>>, ApiError> {
    let repo = PermissionRepo::new(state.db);
    let permissions = repo.find_all().await?;
    Ok(Json(permissions))
}

use axum::http::StatusCode;