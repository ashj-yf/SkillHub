use axum::{
    extract::State,
    http::HeaderMap,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::services::auth::AuthService;
use crate::utils::error::ApiError;
use crate::config::Config;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

pub fn routes() -> Router<PgPool> {
    Router::new().route("/users/me", get(get_current_user))
}

pub async fn get_current_user(
    State(db): State<PgPool>,
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

    let config = Config::from_env()?;
    let service = AuthService::new(db, config.jwt_secret, config.jwt_expiration_hours);

    let user = service.validate_token(token).await?;

    Ok(Json(UserInfo {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        role: user.role,
    }))
}