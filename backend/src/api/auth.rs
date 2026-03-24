use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::middleware::auth::AuthUser;
use crate::models::user::{CreateUser, LoginRequest};
use crate::services::auth::AuthService;
use crate::state::AppState;
use crate::utils::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .route("/health", get(health))
}

async fn health() -> &'static str {
    "OK"
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, ApiError> {
    let service = AuthService::new(state.db, state.jwt_secret, 24);

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
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let service = AuthService::new(state.db, state.jwt_secret, 24);

    let token = service.login(payload).await?;

    Ok(Json(serde_json::json!({ "token": token })))
}

/// 刷新 Token，获取新的 JWT
pub async fn refresh(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    let service = AuthService::new(state.db, state.jwt_secret, 24);

    // 使用当前用户 ID 刷新 Token
    let token = service.refresh_token(user.id).await?;

    Ok(Json(serde_json::json!({ "token": token })))
}