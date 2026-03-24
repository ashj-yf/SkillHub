use axum::extract::FromRef;
use sqlx::PgPool;

use crate::cache::RedisCache;
use crate::storage::Storage;

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub storage: Storage,
    pub cache: Option<RedisCache>,
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

impl FromRef<AppState> for Storage {
    fn from_ref(state: &AppState) -> Storage {
        state.storage.clone()
    }
}

impl FromRef<AppState> for Option<RedisCache> {
    fn from_ref(state: &AppState) -> Option<RedisCache> {
        state.cache.clone()
    }
}