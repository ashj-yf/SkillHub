use anyhow::Result;
use sqlx::PgPool;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::models::permission::permission_name;
use crate::repos::permission::PermissionRepo;
use crate::repos::role::RoleRepo;

/// 权限服务
pub struct PermissionService {
    role_repo: RoleRepo,
    permission_repo: PermissionRepo,
}

impl PermissionService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            role_repo: RoleRepo::new(pool.clone()),
            permission_repo: PermissionRepo::new(pool),
        }
    }

    /// 检查用户是否有特定资源的操作权限
    pub async fn check_permission(
        &self,
        user_id: Uuid,
        resource: &str,
        action: &str,
    ) -> Result<bool> {
        let perm_name = permission_name(resource, action);
        debug!(user_id = %user_id, permission = %perm_name, "Checking permission");

        let has_permission = self.role_repo.user_has_permission(user_id, &perm_name).await?;

        // 如果没有特定权限，检查是否有 manage 权限
        if !has_permission && action != "manage" {
            let manage_perm = permission_name(resource, "manage");
            let has_manage = self.role_repo.user_has_permission(user_id, &manage_perm).await?;

            if has_manage {
                debug!(user_id = %user_id, resource = %resource, "User has manage permission");
                return Ok(true);
            }
        }

        if has_permission {
            debug!(user_id = %user_id, permission = %perm_name, "Permission granted");
        } else {
            warn!(user_id = %user_id, permission = %perm_name, "Permission denied");
        }

        Ok(has_permission)
    }

    /// 检查用户是否是管理员
    pub async fn is_admin(&self, user_id: Uuid) -> Result<bool> {
        let roles = self.role_repo.get_user_roles(user_id).await?;
        Ok(roles.iter().any(|r| r.name == "admin"))
    }

    /// 获取用户的所有权限
    pub async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<String>> {
        debug!(user_id = %user_id, "Getting user permissions");
        self.role_repo.get_user_permissions(user_id).await
    }

    /// 获取用户的所有角色
    pub async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>> {
        debug!(user_id = %user_id, "Getting user roles");
        let roles = self.role_repo.get_user_roles(user_id).await?;
        Ok(roles.into_iter().map(|r| r.name).collect())
    }

    /// 为用户分配角色
    pub async fn assign_role(
        &self,
        user_id: Uuid,
        role_name: &str,
        assigned_by: Option<Uuid>,
    ) -> Result<bool> {
        debug!(user_id = %user_id, role_name = %role_name, "Assigning role to user");

        let role = self.role_repo.find_by_name(role_name).await?
            .ok_or_else(|| anyhow::anyhow!("角色不存在: {}", role_name))?;

        self.role_repo.assign_to_user(user_id, role.id, assigned_by).await?;

        info!(user_id = %user_id, role_name = %role_name, "Role assigned successfully");
        Ok(true)
    }

    /// 移除用户角色
    pub async fn remove_role(&self, user_id: Uuid, role_name: &str) -> Result<bool> {
        debug!(user_id = %user_id, role_name = %role_name, "Removing role from user");

        let role = self.role_repo.find_by_name(role_name).await?
            .ok_or_else(|| anyhow::anyhow!("角色不存在: {}", role_name))?;

        let removed = self.role_repo.remove_from_user(user_id, role.id).await?;

        if removed {
            info!(user_id = %user_id, role_name = %role_name, "Role removed successfully");
        }

        Ok(removed)
    }

    /// 获取所有可用权限
    pub async fn get_all_permissions(&self) -> Result<Vec<(String, String)>> {
        let permissions = self.permission_repo.find_all().await?;
        Ok(permissions.into_iter().map(|p| (p.name, p.description.unwrap_or_default())).collect())
    }

    /// 为角色添加权限
    pub async fn add_permission_to_role(
        &self,
        role_name: &str,
        permission_name: &str,
    ) -> Result<bool> {
        debug!(role_name = %role_name, permission_name = %permission_name, "Adding permission to role");

        let role = self.role_repo.find_by_name(role_name).await?
            .ok_or_else(|| anyhow::anyhow!("角色不存在: {}", role_name))?;

        let permission = self.permission_repo.find_by_name(permission_name).await?
            .ok_or_else(|| anyhow::anyhow!("权限不存在: {}", permission_name))?;

        let added = self.role_repo.add_permission(role.id, permission.id).await?;

        if added {
            info!(role_name = %role_name, permission_name = %permission_name, "Permission added to role");
        }

        Ok(added)
    }

    /// 从角色移除权限
    pub async fn remove_permission_from_role(
        &self,
        role_name: &str,
        permission_name: &str,
    ) -> Result<bool> {
        debug!(role_name = %role_name, permission_name = %permission_name, "Removing permission from role");

        let role = self.role_repo.find_by_name(role_name).await?
            .ok_or_else(|| anyhow::anyhow!("角色不存在: {}", role_name))?;

        let permission = self.permission_repo.find_by_name(permission_name).await?
            .ok_or_else(|| anyhow::anyhow!("权限不存在: {}", permission_name))?;

        let removed = self.role_repo.remove_permission(role.id, permission.id).await?;

        if removed {
            info!(role_name = %role_name, permission_name = %permission_name, "Permission removed from role");
        }

        Ok(removed)
    }
}