use axum::extract::FromRef;
use sqlx::PgPool;

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> PgPool {
        state.db.clone()
    }
}

impl FromRef<AppState> for String {
    fn from_ref(state: &AppState) -> String {
        state.jwt_secret.clone()
    }
}