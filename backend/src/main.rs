use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use skillhub_backend::api;
use skillhub_backend::config::Config;
use skillhub_backend::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    // 连接数据库
    tracing::info!("Connecting to database...");
    let db = PgPoolOptions::new()
        .max_connections(config.max_db_connections)
        .connect(&config.database_url)
        .await?;
    tracing::info!("Database connected");

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
                .allow_methods([hyper::Method::GET, hyper::Method::POST, hyper::Method::PUT, hyper::Method::DELETE])
                .allow_headers([hyper::header::AUTHORIZATION, hyper::header::CONTENT_TYPE])
        )
        .with_state(state);

    // 启动服务器
    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port).parse()?;
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}