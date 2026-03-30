use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{CreateUser, User};

pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, payload: &CreateUser, password_hash: &str) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password_hash, role, is_active)
            VALUES ($1, $2, $3, 'user', true)
            RETURNING *
            "#
        )
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// 创建用户（管理员创建，可指定 is_active）
    pub async fn create_with_active(
        &self,
        payload: &CreateUser,
        password_hash: &str,
        is_active: bool,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password_hash, role, is_active)
            VALUES ($1, $2, $3, 'user', $4)
            RETURNING *
            "#
        )
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(password_hash)
        .bind(is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_last_login(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE users SET last_login_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn set_active(&self, id: Uuid, is_active: bool) -> Result<()> {
        sqlx::query(
            "UPDATE users SET is_active = $1 WHERE id = $2"
        )
        .bind(is_active)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 获取所有用户
    pub async fn find_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    /// 更新用户信息
    pub async fn update(&self, id: Uuid, username: Option<&str>, email: Option<&str>, is_active: Option<bool>) -> Result<Option<User>> {
        // 构建动态更新语句
        let mut updates = Vec::new();
        let mut param_count = 1;

        if username.is_some() {
            updates.push(format!("username = ${}", param_count));
            param_count += 1;
        }
        if email.is_some() {
            updates.push(format!("email = ${}", param_count));
            param_count += 1;
        }
        if is_active.is_some() {
            updates.push(format!("is_active = ${}", param_count));
            param_count += 1;
        }

        if updates.is_empty() {
            return self.find_by_id(id).await;
        }

        let sql = format!(
            "UPDATE users SET {} WHERE id = ${} RETURNING *",
            updates.join(", "),
            param_count
        );

        let mut query = sqlx::query_as::<_, User>(&sql);

        if let Some(u) = username {
            query = query.bind(u);
        }
        if let Some(e) = email {
            query = query.bind(e);
        }
        if let Some(a) = is_active {
            query = query.bind(a);
        }
        query = query.bind(id);

        let user = query.fetch_optional(&self.pool).await?;

        Ok(user)
    }

    /// 删除用户
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "DELETE FROM users WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 更新用户密码
    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind(password_hash)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}