use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use tracing::warn;
use uuid::Uuid;

use crate::middleware::auth::{AuthUser, CurrentUser};
use crate::services::permission::PermissionService;
use crate::state::AppState;
use crate::utils::error::ApiError;

// ==================== 资源和操作常量 ====================

/// 资源类型
pub mod resources {
    pub const SKILLS: &str = "skills";
    pub const USERS: &str = "users";
    pub const ROLES: &str = "roles";
    pub const GROUPS: &str = "groups";
}

/// 操作类型
pub mod actions {
    pub const CREATE: &str = "create";
    pub const READ: &str = "read";
    pub const UPDATE: &str = "update";
    pub const DELETE: &str = "delete";
    pub const MANAGE: &str = "manage";
}

// ==================== 权限提取器 ====================

/// 权限检查提取器（已弃用，保留向后兼容）
/// 用于验证用户是否有特定资源和操作的权限
#[deprecated(note = "Use WithPermission<RESOURCE, ACTION> instead")]
pub struct RequirePermission {
    pub user: CurrentUser,
}

#[allow(deprecated)]
impl RequirePermission {
    /// 创建权限检查器
    pub fn new(resource: &'static str, action: &'static str) -> PermissionGuard {
        PermissionGuard { resource, action }
    }
}

/// 权限守卫，用于在路由中检查权限
pub struct PermissionGuard {
    pub resource: &'static str,
    pub action: &'static str,
}

#[allow(deprecated)]
#[async_trait]
impl FromRequestParts<AppState> for RequirePermission {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // 首先获取认证用户
        let AuthUser(current_user) = AuthUser::from_request_parts(parts, state).await?;

        Ok(RequirePermission { user: current_user })
    }
}

/// 类型安全的权限提取器
/// 使用 const generics 实现编译时类型检查
///
/// # 示例
/// ```ignore
/// // 在 handler 中使用
/// async fn create_skill(
///     WithPermission<SKILLS, CREATE>: WithPermission<resources::SKILLS, actions::CREATE>,
/// ) -> ... {
///     // 用户已有 skills:create 权限
/// }
/// ```
pub struct WithPermission<const RESOURCE: &'static str, const ACTION: &'static str> {
    pub user: CurrentUser,
}

#[async_trait]
impl<const RESOURCE: &'static str, const ACTION: &'static str> FromRequestParts<AppState>
    for WithPermission<RESOURCE, ACTION>
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // 首先获取认证用户
        let AuthUser(current_user) = AuthUser::from_request_parts(parts, state).await?;

        // 检查权限
        check_permission_or_forbidden(state, current_user.id, RESOURCE, ACTION).await?;

        Ok(WithPermission { user: current_user })
    }
}

// ==================== 权限检查函数 ====================

/// 检查用户是否有指定权限
pub async fn check_permission(
    state: &AppState,
    user_id: Uuid,
    resource: &str,
    action: &str,
) -> Result<bool, ApiError> {
    let service = PermissionService::new(state.db.clone());

    let has_permission = service.check_permission(user_id, resource, action).await
        .map_err(|e| {
            warn!(error = %e, "Permission check failed");
            ApiError::InternalServerError
        })?;

    Ok(has_permission)
}

/// 检查用户是否有指定权限，如果没有则返回 403 Forbidden
pub async fn check_permission_or_forbidden(
    state: &AppState,
    user_id: Uuid,
    resource: &str,
    action: &str,
) -> Result<(), ApiError> {
    // 先检查是否是管理员（管理员绕过所有权限检查）
    if is_admin(state, user_id).await? {
        return Ok(());
    }

    // 检查具体权限
    if check_permission(state, user_id, resource, action).await? {
        return Ok(());
    }

    warn!(
        user_id = %user_id,
        resource = %resource,
        action = %action,
        "Permission denied"
    );
    Err(ApiError::Forbidden)
}

/// 检查用户是否是管理员
pub async fn is_admin(state: &AppState, user_id: Uuid) -> Result<bool, ApiError> {
    let service = PermissionService::new(state.db.clone());

    let is_admin = service.is_admin(user_id).await
        .map_err(|e| {
            warn!(error = %e, "Admin check failed");
            ApiError::InternalServerError
        })?;

    Ok(is_admin)
}

/// 检查用户是否是管理员或具有指定权限
pub async fn is_admin_or_has_permission(
    state: &AppState,
    user_id: Uuid,
    resource: &str,
    action: &str,
) -> Result<bool, ApiError> {
    if is_admin(state, user_id).await? {
        return Ok(true);
    }
    check_permission(state, user_id, resource, action).await
}

// ==================== 管理员提取器 ====================

/// 管理员权限提取器
/// 验证用户是否是管理员
pub struct AdminUser(pub CurrentUser);

#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let AuthUser(current_user) = AuthUser::from_request_parts(parts, state).await?;

        // 检查是否是管理员
        let is_admin = is_admin(state, current_user.id).await?;

        if !is_admin {
            warn!(user_id = %current_user.id, "Non-admin user attempted to access admin-only resource");
            return Err(ApiError::Forbidden);
        }

        Ok(AdminUser(current_user))
    }
}

// ==================== 所有权检查 ====================

/// 可拥有资源的 trait
/// 用于检查用户是否是资源的所有者
pub trait Ownable {
    /// 获取资源所有者 ID
    fn owner_id(&self) -> Option<Uuid>;
}

/// 检查用户是否是资源的所有者
///
/// # 参数
/// - `state`: 应用状态
/// - `user_id`: 当前用户 ID
/// - `resource`: 资源对象（实现 Ownable trait）
///
/// # 返回
/// - `true`: 用户是所有者
/// - `false`: 用户不是所有者
pub fn check_ownership<T: Ownable>(user_id: Uuid, resource: &T) -> bool {
    resource.owner_id().map(|id| id == user_id).unwrap_or(false)
}

/// 检查用户是否是所有者或管理员
///
/// # 参数
/// - `state`: 应用状态
/// - `user_id`: 当前用户 ID
/// - `resource`: 资源对象（实现 Ownable trait）
///
/// # 返回
/// - `Ok(true)`: 是所有者或管理员
/// - `Ok(false)`: 不是所有者也不是管理员
/// - `Err`: 数据库错误
pub async fn is_owner_or_admin<T: Ownable>(
    state: &AppState,
    user_id: Uuid,
    resource: &T,
) -> Result<bool, ApiError> {
    // 检查是否是管理员
    if is_admin(state, user_id).await? {
        return Ok(true);
    }

    // 检查所有权
    Ok(check_ownership(user_id, resource))
}

/// 检查所有权或指定权限
/// 用于"所有者或有权限"的场景
///
/// # 参数
/// - `state`: 应用状态
/// - `user_id`: 当前用户 ID
/// - `resource`: 资源对象
/// - `resource_name`: 资源名称（如 "skills"）
/// - `action`: 操作名称（如 "update"）
///
/// # 返回
/// - `Ok(())`: 有权限（是所有者、管理员或有指定权限）
/// - `Err(ApiError::Forbidden)`: 无权限
pub async fn check_ownership_or_permission<T: Ownable>(
    state: &AppState,
    user_id: Uuid,
    resource: &T,
    resource_name: &str,
    action: &str,
) -> Result<(), ApiError> {
    // 检查是否是管理员
    if is_admin(state, user_id).await? {
        return Ok(());
    }

    // 检查所有权
    if check_ownership(user_id, resource) {
        return Ok(());
    }

    // 检查权限
    if check_permission(state, user_id, resource_name, action).await? {
        return Ok(());
    }

    warn!(
        user_id = %user_id,
        resource = %resource_name,
        action = %action,
        "Neither owner nor has permission"
    );
    Err(ApiError::Forbidden)
}

// ==================== 宏辅助 ====================

/// 权限检查宏辅助
/// 用法: require_permission!(state, user_id, "skills", "create")
#[macro_export]
macro_rules! require_permission {
    ($state:expr, $user_id:expr, $resource:expr, $action:expr) => {{
        let service = $crate::services::permission::PermissionService::new($state.db.clone());
        let has = service.check_permission($user_id, $resource, $action).await?;
        if !has {
            return Err($crate::utils::error::ApiError::Forbidden);
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_name() {
        assert_eq!(
            $crate::models::permission::permission_name("skills", "read"),
            "skills:read"
        );
    }
}