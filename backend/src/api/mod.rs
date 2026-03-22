pub mod auth;
pub mod skills;
pub mod users;

use axum::Router;
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .merge(auth::routes())
        .merge(skills::routes())
        .merge(users::routes())
}