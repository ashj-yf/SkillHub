use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use tracing::{info, warn};

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
        let password_hash = hash_password(&payload.password)?;

        // 创建用户
        let user = self.user_repo.create(&payload, &password_hash).await?;

        info!(user_id = %user.id, username = %user.username, "User registered successfully");

        Ok(user)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<String> {
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
        let valid = verify_password(&request.password, &user.password_hash)?;
        if !valid {
            warn!(user_id = %user.id, "Login failed: invalid password");
            return Err(anyhow!("邮箱或密码错误"));
        }

        // 更新最后登录时间
        self.user_repo.update_last_login(user.id).await?;

        // 生成 JWT
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