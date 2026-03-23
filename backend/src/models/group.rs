use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 用户组模型（原 Department）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub path: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建用户组请求
#[derive(Debug, Deserialize)]
pub struct CreateGroup {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

/// 更新用户组请求
#[derive(Debug, Deserialize)]
pub struct UpdateGroup {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

/// 用户组树节点
#[derive(Debug, Serialize)]
pub struct GroupTreeNode {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub path: Option<String>,
    pub children: Vec<GroupTreeNode>,
}

/// 用户组详情（包含成员数量）
#[derive(Debug, Serialize)]
pub struct GroupDetail {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub path: Option<String>,
    pub member_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 用户组关联
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserGroup {
    pub id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub is_primary: bool,
    pub joined_at: DateTime<Utc>,
}

/// 添加用户到组请求
#[derive(Debug, Deserialize)]
pub struct AddUserToGroup {
    pub user_id: Uuid,
    pub is_primary: Option<bool>,
}