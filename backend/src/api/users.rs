use axum::{
    extract::State,
    http::HeaderMap,
    routing::get,
    Json, Router,
};
use serde::Serialize;

use crate::middleware::auth::AuthUser;
use crate::models::user::User;
use crate::services::auth::AuthService;
use crate::state::AppState;
use crate::utils::error::ApiError;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/me/profile", get(get_my_profile))
}

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

    let service = AuthService::new(state.db, state.jwt_secret, 24);

    let user = service.validate_token(token).await?;

    Ok(Json(UserInfo {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        role: user.role,
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