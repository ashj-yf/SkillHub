use axum::{
    extract::{Path, Query, State},
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::middleware::permission::{
    check_permission_or_forbidden, resources, actions,
};
use crate::models::group::{AddUserToGroup, CreateGroup, Group, GroupDetail, GroupTreeNode, UpdateGroup};
use crate::repos::group::GroupRepo;
use crate::repos::user::UserRepo;
use crate::state::AppState;
use crate::utils::error::ApiError;

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub parent_id: Option<Uuid>,
}

/// 用户组成员响应
#[derive(Debug, Serialize)]
pub struct GroupMember {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub is_primary: bool,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        // 用户组管理
        .route("/groups", get(list_groups).post(create_group))
        .route("/groups/tree", get(get_group_tree))
        .route("/groups/{id}", get(get_group).put(update_group).delete(delete_group))
        // 用户组成员管理
        .route("/groups/{id}/members", get(get_group_members).post(add_member))
        .route("/groups/{id}/members/{user_id}", delete(remove_member))
        // 用户所属组
        .route("/users/{id}/groups", get(get_user_groups))
}

/// 获取用户组列表
pub async fn list_groups(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Group>>, ApiError> {
    let repo = GroupRepo::new(state.db);

    let groups = if let Some(parent_id) = query.parent_id {
        repo.find_children(parent_id).await?
    } else {
        repo.find_all().await?
    };

    Ok(Json(groups))
}

/// 获取用户组树
pub async fn get_group_tree(
    State(state): State<AppState>,
) -> Result<Json<Vec<GroupTreeNode>>, ApiError> {
    let repo = GroupRepo::new(state.db);
    let tree = repo.build_tree().await?;
    Ok(Json(tree))
}

/// 创建用户组
pub async fn create_group(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Json(payload): Json<CreateGroup>,
) -> Result<Json<Group>, ApiError> {
    // 权限检查：需要 groups:create 权限
    check_permission_or_forbidden(&state, current_user.id, resources::GROUPS, actions::CREATE).await?;

    // 验证
    if payload.name.is_empty() || payload.name.len() > 100 {
        return Err(ApiError::BadRequest("用户组名称长度应为 1-100 个字符".into()));
    }

    let repo = GroupRepo::new(state.db.clone());

    // 检查名称是否已存在
    if repo.find_by_name(&payload.name).await?.is_some() {
        return Err(ApiError::Conflict("用户组名称已存在".into()));
    }

    // 检查父组是否存在
    if let Some(parent_id) = payload.parent_id {
        if repo.find_by_id(parent_id).await?.is_none() {
            return Err(ApiError::BadRequest("父用户组不存在".into()));
        }
    }

    let group = repo.create(&payload, Some(current_user.id)).await?;
    Ok(Json(group))
}

/// 获取用户组详情
pub async fn get_group(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GroupDetail>, ApiError> {
    let repo = GroupRepo::new(state.db);
    let detail = repo.get_detail(id).await?
        .ok_or_else(|| ApiError::NotFound("用户组不存在".into()))?;

    Ok(Json(detail))
}

/// 更新用户组
pub async fn update_group(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateGroup>,
) -> Result<Json<Group>, ApiError> {
    // 权限检查：需要 groups:update 权限
    check_permission_or_forbidden(&state, current_user.id, resources::GROUPS, actions::UPDATE).await?;

    let repo = GroupRepo::new(state.db);

    // 如果更新名称，检查是否已存在
    if let Some(ref name) = payload.name {
        if let Some(existing) = repo.find_by_name(name).await? {
            if existing.id != id {
                return Err(ApiError::Conflict("用户组名称已存在".into()));
            }
        }
    }

    // 如果更新父组，检查是否存在且不会形成循环
    if let Some(parent_id) = payload.parent_id {
        if parent_id == id {
            return Err(ApiError::BadRequest("不能将自己设为父组".into()));
        }

        if repo.find_by_id(parent_id).await?.is_none() {
            return Err(ApiError::BadRequest("父用户组不存在".into()));
        }
    }

    let group = repo.update(id, &payload).await?
        .ok_or_else(|| ApiError::NotFound("用户组不存在".into()))?;

    Ok(Json(group))
}

/// 删除用户组
pub async fn delete_group(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 groups:delete 权限
    check_permission_or_forbidden(&state, current_user.id, resources::GROUPS, actions::DELETE).await?;

    let repo = GroupRepo::new(state.db);

    let deleted = repo.delete(id).await?;
    if !deleted {
        return Err(ApiError::NotFound("用户组不存在".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// 获取用户组成员
pub async fn get_group_members(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<GroupMember>>, ApiError> {
    let group_repo = GroupRepo::new(state.db.clone());
    let user_repo = UserRepo::new(state.db.clone());

    // 检查组是否存在
    if group_repo.find_by_id(id).await?.is_none() {
        return Err(ApiError::NotFound("用户组不存在".into()));
    }

    let member_ids = group_repo.get_group_members(id).await?;

    let mut members = Vec::new();
    for user_id in member_ids {
        if let Some(user) = user_repo.find_by_id(user_id).await? {
            // 检查是否为主组
            let is_primary = sqlx::query_scalar::<_, bool>(
                "SELECT is_primary FROM user_groups WHERE user_id = $1 AND group_id = $2"
            )
            .bind(user_id)
            .bind(id)
            .fetch_one(&state.db)
            .await
            .unwrap_or(false);

            members.push(GroupMember {
                id: user.id,
                username: user.username,
                email: user.email,
                is_primary,
            });
        }
    }

    Ok(Json(members))
}

/// 添加成员到用户组
pub async fn add_member(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<AddUserToGroup>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 groups:update 权限
    check_permission_or_forbidden(&state, current_user.id, resources::GROUPS, actions::UPDATE).await?;

    let group_repo = GroupRepo::new(state.db.clone());
    let user_repo = UserRepo::new(state.db);

    // 检查组是否存在
    if group_repo.find_by_id(id).await?.is_none() {
        return Err(ApiError::NotFound("用户组不存在".into()));
    }

    // 检查用户是否存在
    if user_repo.find_by_id(payload.user_id).await?.is_none() {
        return Err(ApiError::NotFound("用户不存在".into()));
    }

    let is_primary = payload.is_primary.unwrap_or(false);
    group_repo.add_user(id, payload.user_id, is_primary).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// 从用户组移除成员
pub async fn remove_member(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 groups:update 权限
    check_permission_or_forbidden(&state, current_user.id, resources::GROUPS, actions::UPDATE).await?;

    let repo = GroupRepo::new(state.db);
    repo.remove_user(id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 获取用户所属的用户组
pub async fn get_user_groups(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<Group>>, ApiError> {
    let group_repo = GroupRepo::new(state.db.clone());
    let user_repo = UserRepo::new(state.db);

    // 检查用户是否存在
    if user_repo.find_by_id(id).await?.is_none() {
        return Err(ApiError::NotFound("用户不存在".into()));
    }

    let groups = group_repo.get_user_groups(id).await?;
    Ok(Json(groups))
}

use axum::http::StatusCode;