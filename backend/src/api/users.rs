use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::middleware::permission::{
    check_permission_or_forbidden, resources, actions,
};
use crate::models::skill::Skill;
use crate::models::user::{CreateUser, User};
use crate::repos::skill::SkillRepo;
use crate::repos::user::UserRepo;
use crate::repos::role::RoleRepo;
use crate::services::auth::{hash_password, AuthService};
use crate::state::AppState;
use crate::utils::error::ApiError;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub roles: Vec<String>,  // 用户角色列表
}

/// 用户详情响应（包含角色列表）
#[derive(Debug, Serialize)]
pub struct UserDetail {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub roles: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

/// 分配角色请求（使用角色名称）
#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role: String,
}

/// 管理员创建用户请求
#[derive(Debug, Deserialize)]
pub struct AdminCreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    /// 是否激活，默认 true
    pub is_active: Option<bool>,
    /// 角色列表，默认 ["user"]
    pub roles: Option<Vec<String>>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        // 当前用户信息
        .route("/users/me", get(get_current_user))
        .route("/users/me/profile", get(get_my_profile))
        .route("/users/me/skills", get(get_my_skills))
        // 用户管理（管理员）
        .route("/users", get(list_users).post(admin_create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        // 用户角色管理
        .route("/users/:id/roles", get(get_user_roles).post(assign_role))
        .route("/users/:id/roles/:role", delete(remove_role))
}

/// 从 Header 获取当前用户
pub async fn get_current_user(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserInfo>, ApiError> {
    // 从请求头获取 token
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError::Unauthorized);
    }

    let token = &auth_header[7..];

    let service = AuthService::new(state.db.clone(), state.jwt_secret, 24);

    let user = service.validate_token(token).await?;

    // 获取用户的角色列表
    let role_repo = RoleRepo::new(state.db);
    let roles = role_repo.get_user_roles(user.id).await?;
    let role_names: Vec<String> = roles.into_iter().map(|r| r.name).collect();

    Ok(Json(UserInfo {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        role: user.role,
        roles: role_names,
    }))
}

/// 使用 AuthUser 中间件获取当前用户信息
pub async fn get_my_profile(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
) -> Result<Json<User>, ApiError> {
    let service = AuthService::new(state.db, state.jwt_secret, 24);

    // 通过用户 ID 获取完整的用户信息
    let user = service.get_user_by_id(current_user.id).await?;

    Ok(Json(user))
}

/// 获取当前用户创建的技能列表
pub async fn get_my_skills(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
) -> Result<Json<Vec<Skill>>, ApiError> {
    let repo = SkillRepo::new(state.db);
    let skills = repo.find_by_author(current_user.id).await?;

    Ok(Json(skills))
}

/// 获取所有用户列表
pub async fn list_users(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
) -> Result<Json<Vec<UserDetail>>, ApiError> {
    // 权限检查：需要 users:read 权限
    check_permission_or_forbidden(&state, current_user.id, resources::USERS, actions::READ).await?;

    let user_repo = UserRepo::new(state.db.clone());
    let role_repo = RoleRepo::new(state.db.clone());

    let users = user_repo.find_all().await?;

    let mut user_details = Vec::new();
    for user in users {
        // 获取用户的角色列表
        let roles = role_repo.get_user_roles(user.id).await?;
        let role_names: Vec<String> = roles.into_iter().map(|r| r.name).collect();

        user_details.push(UserDetail {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            is_active: user.is_active,
            roles: role_names,
            created_at: user.created_at,
            updated_at: user.updated_at,
        });
    }

    Ok(Json(user_details))
}

/// 获取用户详情
pub async fn get_user(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UserDetail>, ApiError> {
    // 权限检查：用户本人或 users:read 权限
    if current_user.id != id {
        check_permission_or_forbidden(&state, current_user.id, resources::USERS, actions::READ).await?;
    }

    let user_repo = UserRepo::new(state.db.clone());
    let role_repo = RoleRepo::new(state.db.clone());

    let user = user_repo.find_by_id(id).await?
        .ok_or_else(|| ApiError::NotFound("用户不存在".into()))?;

    // 获取用户的角色列表
    let roles = role_repo.get_user_roles(user.id).await?;
    let role_names: Vec<String> = roles.into_iter().map(|r| r.name).collect();

    Ok(Json(UserDetail {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        is_active: user.is_active,
        roles: role_names,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}

/// 更新用户
pub async fn update_user(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserDetail>, ApiError> {
    // 权限检查：用户本人或 users:update 权限
    if current_user.id != id {
        check_permission_or_forbidden(&state, current_user.id, resources::USERS, actions::UPDATE).await?;
    }

    let user_repo = UserRepo::new(state.db.clone());
    let role_repo = RoleRepo::new(state.db.clone());

    // 如果更新用户名，检查是否已存在
    if let Some(ref username) = payload.username {
        if let Some(existing) = user_repo.find_by_username(username).await? {
            if existing.id != id {
                return Err(ApiError::Conflict("用户名已存在".into()));
            }
        }
    }

    // 如果更新邮箱，检查是否已存在
    if let Some(ref email) = payload.email {
        if let Some(existing) = user_repo.find_by_email(email).await? {
            if existing.id != id {
                return Err(ApiError::Conflict("邮箱已存在".into()));
            }
        }
    }

    let user = user_repo.update(
        id,
        payload.username.as_deref(),
        payload.email.as_deref(),
        payload.is_active,
    ).await?
        .ok_or_else(|| ApiError::NotFound("用户不存在".into()))?;

    // 获取用户的角色列表
    let roles = role_repo.get_user_roles(user.id).await?;
    let role_names: Vec<String> = roles.into_iter().map(|r| r.name).collect();

    Ok(Json(UserDetail {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        is_active: user.is_active,
        roles: role_names,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}

/// 删除用户
pub async fn delete_user(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 users:delete 权限
    check_permission_or_forbidden(&state, current_user.id, resources::USERS, actions::DELETE).await?;

    let repo = UserRepo::new(state.db);

    let deleted = repo.delete(id).await?;
    if !deleted {
        return Err(ApiError::NotFound("用户不存在".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// 获取用户的角色列表
pub async fn get_user_roles(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<String>>, ApiError> {
    // 权限检查：用户本人或 users:read 权限
    if current_user.id != id {
        check_permission_or_forbidden(&state, current_user.id, resources::USERS, actions::READ).await?;
    }

    let user_repo = UserRepo::new(state.db.clone());
    let role_repo = RoleRepo::new(state.db.clone());

    // 检查用户是否存在
    if user_repo.find_by_id(id).await?.is_none() {
        return Err(ApiError::NotFound("用户不存在".into()));
    }

    let roles = role_repo.get_user_roles(id).await?;
    let role_names: Vec<String> = roles.into_iter().map(|r| r.name).collect();

    Ok(Json(role_names))
}

/// 为用户分配角色（使用角色名称）
pub async fn assign_role(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<AssignRoleRequest>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 roles:manage 权限
    check_permission_or_forbidden(&state, current_user.id, resources::ROLES, "manage").await?;

    let user_repo = UserRepo::new(state.db.clone());
    let role_repo = RoleRepo::new(state.db.clone());

    // 检查用户是否存在
    if user_repo.find_by_id(id).await?.is_none() {
        return Err(ApiError::NotFound("用户不存在".into()));
    }

    // 根据角色名称查找角色
    let role = role_repo.find_by_name(&payload.role).await?
        .ok_or_else(|| ApiError::NotFound(format!("角色 '{}' 不存在", payload.role)))?;

    role_repo.assign_to_user(id, role.id, Some(current_user.id)).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 移除用户角色（使用角色名称）
pub async fn remove_role(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Path((id, role_name)): Path<(Uuid, String)>,
) -> Result<StatusCode, ApiError> {
    // 权限检查：需要 roles:manage 权限
    check_permission_or_forbidden(&state, current_user.id, resources::ROLES, "manage").await?;

    let user_repo = UserRepo::new(state.db.clone());
    let role_repo = RoleRepo::new(state.db.clone());

    // 检查用户是否存在
    if user_repo.find_by_id(id).await?.is_none() {
        return Err(ApiError::NotFound("用户不存在".into()));
    }

    // 根据角色名称查找角色
    let role = role_repo.find_by_name(&role_name).await?
        .ok_or_else(|| ApiError::NotFound(format!("角色 '{}' 不存在", role_name)))?;

    role_repo.remove_from_user(id, role.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 管理员创建用户
pub async fn admin_create_user(
    State(state): State<AppState>,
    AuthUser(current_user): AuthUser,
    Json(payload): Json<AdminCreateUserRequest>,
) -> Result<(StatusCode, Json<UserDetail>), ApiError> {
    debug!(username = %payload.username, email = %payload.email, "Admin creating user");

    // 权限检查：需要 users:create 权限或管理员
    check_permission_or_forbidden(&state, current_user.id, resources::USERS, actions::CREATE).await?;

    let user_repo = UserRepo::new(state.db.clone());
    let role_repo = RoleRepo::new(state.db.clone());

    // 验证用户名长度（3-50字符）
    let username_len = payload.username.trim().len();
    if username_len < 3 || username_len > 50 {
        warn!(username = %payload.username, len = username_len, "Username validation failed");
        return Err(ApiError::BadRequest("用户名长度需在 3-50 个字符之间".into()));
    }

    // 验证邮箱格式（简单检查）
    let email = payload.email.trim();
    if !email.contains('@') || email.len() < 5 {
        warn!(email = %email, "Email validation failed");
        return Err(ApiError::BadRequest("邮箱格式不正确".into()));
    }

    // 验证密码长度（至少8位）
    if payload.password.len() < 8 {
        warn!("Password validation failed: too short");
        return Err(ApiError::BadRequest("密码长度至少为 8 位".into()));
    }

    // 检查邮箱是否已存在
    if user_repo.find_by_email(email).await?.is_some() {
        warn!(email = %email, "Email already registered");
        return Err(ApiError::Conflict("邮箱已被注册".into()));
    }

    // 检查用户名是否已存在
    let username = payload.username.trim();
    if user_repo.find_by_username(username).await?.is_some() {
        warn!(username = %username, "Username already taken");
        return Err(ApiError::Conflict("用户名已被使用".into()));
    }

    // 哈希密码
    debug!("Hashing password");
    let password_hash = hash_password(&payload.password)
        .map_err(|e| {
            warn!(error = %e, "Password hashing failed");
            ApiError::InternalServerError
        })?;

    // 设置 is_active，默认 true
    let is_active = payload.is_active.unwrap_or(true);

    // 创建用户
    let create_payload = CreateUser {
        username: username.to_string(),
        email: email.to_string(),
        password: payload.password.clone(),
    };

    let user = user_repo.create_with_active(&create_payload, &password_hash, is_active).await
        .map_err(|e| {
            warn!(error = %e, "User creation failed");
            ApiError::InternalServerError
        })?;

    info!(user_id = %user.id, username = %user.username, "User created by admin");

    // 分配角色
    let roles_to_assign = payload.roles.unwrap_or_else(|| vec!["user".to_string()]);
    let mut assigned_roles = Vec::new();

    for role_name in &roles_to_assign {
        // 查找角色
        let role = role_repo.find_by_name(role_name).await?;
        if let Some(role) = role {
            role_repo.assign_to_user(user.id, role.id, Some(current_user.id)).await
                .map_err(|e| {
                    warn!(error = %e, role_name = %role_name, "Role assignment failed");
                    ApiError::InternalServerError
                })?;
            assigned_roles.push(role_name.clone());
        } else {
            warn!(role_name = %role_name, "Role not found, skipping");
        }
    }

    // 如果没有分配任何角色，确保至少有 user 角色
    if assigned_roles.is_empty() {
        if let Some(user_role) = role_repo.find_by_name("user").await? {
            role_repo.assign_to_user(user.id, user_role.id, Some(current_user.id)).await
                .map_err(|e| {
                    warn!(error = %e, "Default role assignment failed");
                    ApiError::InternalServerError
                })?;
            assigned_roles.push("user".to_string());
        }
    }

    Ok((StatusCode::CREATED, Json(UserDetail {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        is_active: user.is_active,
        roles: assigned_roles,
        created_at: user.created_at,
        updated_at: user.updated_at,
    })))
}