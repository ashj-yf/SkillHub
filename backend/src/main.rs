use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use skillhub_backend::api;
use skillhub_backend::cache::RedisCache;
use skillhub_backend::config::Config;
use skillhub_backend::state::AppState;
use skillhub_backend::storage::Storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    // 日志级别通过 RUST_LOG 环境变量控制，支持以下格式：
    // - "info"          - 全局 info 级别
    // - "debug"         - 全局 debug 级别
    // - "skillhub_backend=debug,info" - 指定模块级别
    // - "skillhub_backend::services=trace,info" - 指定子模块级别
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    tracing::info!(
        log_level = %log_level,
        server_host = %config.server_host,
        server_port = %config.server_port,
        "Starting Skills Hub server"
    );

    // 连接数据库
    tracing::debug!("Connecting to database...");
    let db = PgPoolOptions::new()
        .max_connections(config.max_db_connections)
        .connect(&config.database_url)
        .await?;
    tracing::info!(
        max_connections = config.max_db_connections,
        "Database connected"
    );

    // 初始化对象存储
    tracing::debug!(
        endpoint = %config.storage_endpoint,
        bucket = %config.storage_bucket,
        "Connecting to object storage..."
    );
    let storage = Storage::new(
        &config.storage_endpoint,
        &config.storage_access_key,
        &config.storage_secret_key,
        &config.storage_bucket,
    )
    .await?;
    tracing::info!(
        bucket = %config.storage_bucket,
        "Object storage connected"
    );

    // 初始化 Redis 缓存（可选）
    tracing::debug!(
        redis_url = %config.redis_url,
        "Connecting to Redis..."
    );
    let cache = match RedisCache::new(&config.redis_url) {
        Ok(cache) => {
            // 验证连接
            match cache.ping().await {
                Ok(true) => {
                    tracing::info!("Redis cache connected");
                    Some(cache)
                }
                Ok(false) => {
                    tracing::warn!("Redis ping returned unexpected response, cache disabled");
                    None
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Redis connection failed, cache disabled");
                    None
                }
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to create Redis client, cache disabled");
            None
        }
    };

    // 配置 CORS（从环境变量读取允许的来源）
    let allowed_origins: Vec<String> = std::env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".into())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // 构建应用状态
    let state = AppState {
        db: db.clone(),
        jwt_secret: config.jwt_secret.clone(),
        storage,
        cache,
    };

    // 构建路由
    let app = Router::new()
        .nest("/api", api::routes())
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(move |origin, _| {
                    origin
                        .to_str()
                        .map(|o| allowed_origins.contains(&o.to_string()))
                        .unwrap_or(false)
                }))
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::DELETE])
                .allow_headers([axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE])
        )
        .with_state(state);

    // 启动服务器
    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port).parse()?;
    tracing::info!(
        addr = %addr,
        "Server started, ready to accept connections"
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}