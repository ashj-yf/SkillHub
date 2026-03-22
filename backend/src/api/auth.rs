use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::user::{CreateUser, LoginRequest};
use crate::services::auth::AuthService;
use crate::utils::error::ApiError;
use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/health", get(health))
}

async fn health() -> &'static str {
    "OK"
}

pub async fn register(
    State(db): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, ApiError> {
    let config = Config::from_env()?;
    let service = AuthService::new(db, config.jwt_secret, config.jwt_expiration_hours);

    // 验证输入
    if payload.username.is_empty() || payload.username.len() > 50 {
        return Err(ApiError::BadRequest("用户名长度应为 1-50 个字符".into()));
    }
    if payload.email.is_empty() || !payload.email.contains('@') {
        return Err(ApiError::BadRequest("请输入有效的邮箱地址".into()));
    }
    if payload.password.len() < 8 {
        return Err(ApiError::BadRequest("密码长度至少为 8 位".into()));
    }

    let create_user = CreateUser {
        username: payload.username,
        email: payload.email,
        password: payload.password,
    };

    service.register(create_user).await?;

    Ok(StatusCode::CREATED)
}

pub async fn login(
    State(db): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let config = Config::from_env()?;
    let service = AuthService::new(db, config.jwt_secret, config.jwt_expiration_hours);

    let token = service.login(payload).await?;

    Ok(Json(serde_json::json!({ "token": token })))
}