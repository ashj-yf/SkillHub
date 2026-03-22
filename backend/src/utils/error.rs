use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound(String),
    Conflict(String),
    InternalServerError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: String,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "未授权，请先登录".into()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN", "禁止访问".into()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg),
            ApiError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "服务器内部错误".into())
            }
        };

        let body = Json(ErrorResponse {
            error: ErrorDetail {
                code: code.into(),
                message,
            },
        });

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound("记录不存在".into()),
            sqlx::Error::Database(db_err) if db_err.constraint().is_some() => {
                ApiError::Conflict("数据冲突，可能是重复的值".into())
            }
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        // 检查是否是业务错误（通过错误消息判断）
        let msg = err.to_string();

        if msg.contains("已被") || msg.contains("不存在") || msg.contains("无效") {
            ApiError::BadRequest(msg)
        } else if msg.contains("无权") {
            ApiError::Forbidden
        } else {
            // 记录详细错误日志，但不暴露给客户端
            tracing::error!("Internal error: {}", msg);
            ApiError::InternalServerError
        }
    }
}