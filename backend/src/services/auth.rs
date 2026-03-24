use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use tracing::{debug, info, warn};

use crate::models::user::{CreateUser, LoginRequest, User};
use crate::repos::user::UserRepo;
use crate::utils::jwt::create_token;

pub struct AuthService {
    user_repo: UserRepo,
    jwt_secret: String,
    jwt_expiration_hours: i64,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String, jwt_expiration_hours: i64) -> Self {
        Self {
            user_repo: UserRepo::new(pool),
            jwt_secret,
            jwt_expiration_hours,
        }
    }

    pub async fn register(&self, payload: CreateUser) -> Result<User> {
        debug!(username = %payload.username, email = %payload.email, "Processing registration request");

        // 检查邮箱是否已存在
        if self.user_repo.find_by_email(&payload.email).await?.is_some() {
            warn!(email = %payload.email, "Registration failed: email already registered");
            return Err(anyhow!("邮箱已被注册"));
        }

        // 检查用户名是否已存在
        if self.user_repo.find_by_username(&payload.username).await?.is_some() {
            warn!(username = %payload.username, "Registration failed: username already taken");
            return Err(anyhow!("用户名已被使用"));
        }

        // 验证密码长度
        if payload.password.len() < 8 {
            return Err(anyhow!("密码长度至少为 8 位"));
        }

        // 哈希密码
        debug!(username = %payload.username, "Hashing password");
        let password_hash = hash_password(&payload.password)?;

        // 创建用户
        let user = self.user_repo.create(&payload, &password_hash).await?;

        info!(user_id = %user.id, username = %user.username, "User registered successfully");

        Ok(user)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<String> {
        debug!(email = %request.email, "Processing login request");

        // 查找用户
        let user = self.user_repo
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| {
                warn!(email = %request.email, "Login failed: user not found");
                anyhow!("邮箱或密码错误")
            })?;

        // 检查用户是否激活
        if !user.is_active {
            warn!(user_id = %user.id, "Login failed: account disabled");
            return Err(anyhow!("账户已被禁用"));
        }

        // 验证密码
        debug!(user_id = %user.id, "Verifying password");
        let valid = verify_password(&request.password, &user.password_hash)?;
        if !valid {
            warn!(user_id = %user.id, "Login failed: invalid password");
            return Err(anyhow!("邮箱或密码错误"));
        }

        // 更新最后登录时间
        self.user_repo.update_last_login(user.id).await?;

        // 生成 JWT
        debug!(user_id = %user.id, role = %user.role, "Generating JWT token");
        let token = create_token(
            &user.id.to_string(),
            &user.role,
            &self.jwt_secret,
            self.jwt_expiration_hours,
        )?;

        info!(user_id = %user.id, username = %user.username, role = %user.role, "User logged in successfully");

        Ok(token)
    }

    pub async fn validate_token(&self, token: &str) -> Result<User> {
    let claims = crate::utils::jwt::verify_token(token, &self.jwt_secret)?;
    let user_id = uuid::Uuid::parse_str(&claims.sub)?;

    let user = self.user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow!("用户不存在"))?;

    if !user.is_active {
        return Err(anyhow!("账户已被禁用"));
    }

    Ok(user)
    }

    /// 通过用户 ID 获取用户信息
    pub async fn get_user_by_id(&self, user_id: uuid::Uuid) -> Result<User> {
        let user = self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        if !user.is_active {
            return Err(anyhow!("账户已被禁用"));
        }

        Ok(user)
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    Ok(verify(password, hash)?)
}

impl AuthService {
    /// 刷新 Token，返回新的 JWT
    pub async fn refresh_token(&self, user_id: uuid::Uuid) -> Result<String> {
        debug!(user_id = %user_id, "Refreshing token");

        // 验证用户是否存在且激活
        let user = self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        if !user.is_active {
            warn!(user_id = %user_id, "Token refresh failed: account disabled");
            return Err(anyhow!("账户已被禁用"));
        }

        // 生成新 Token
        let token = create_token(
            &user.id.to_string(),
            &user.role,
            &self.jwt_secret,
            self.jwt_expiration_hours,
        )?;

        info!(user_id = %user.id, "Token refreshed successfully");

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::jwt::verify_token;

    #[test]
    fn test_refresh_token_creates_valid_token() {
        // 测试 refresh_token 方法生成的 token 可以被验证
        let secret = "test-secret-key-for-refresh-token-test";
        let user_id = uuid::Uuid::new_v4();

        // 直接调用 create_token 模拟 refresh 逻辑
        let token = create_token(
            &user_id.to_string(),
            "user",
            secret,
            24,
        ).unwrap();

        // 验证生成的 token
        let claims = verify_token(&token, secret).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.role, "user");
    }

    #[test]
    fn test_refresh_token_different_from_original() {
        // 测试刷新后的 token 与原 token 不同（因为 iat 不同）
        let secret = "test-secret-key";
        let user_id = uuid::Uuid::new_v4();

        let token1 = create_token(
            &user_id.to_string(),
            "user",
            secret,
            24,
        ).unwrap();

        // 模拟时间流逝（实际中 iat 会不同）
        std::thread::sleep(std::time::Duration::from_secs(1));

        let token2 = create_token(
            &user_id.to_string(),
            "user",
            secret,
            24,
        ).unwrap();

        // 两个 token 应该不同（因为 iat 时间戳不同）
        assert_ne!(token1, token2);
    }
}