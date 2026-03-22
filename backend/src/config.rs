use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub max_db_connections: u32,
    pub jwt_expiration_hours: i64,
    pub storage_endpoint: String,
    pub storage_access_key: String,
    pub storage_secret_key: String,
    pub storage_bucket: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable is required")?;
        let jwt_secret = std::env::var("JWT_SECRET")
            .context("JWT_SECRET environment variable is required. Please set a secure random string.")?;

        Ok(Config {
            database_url,
            jwt_secret,
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .context("SERVER_PORT must be a valid port number (1-65535)")?,
            max_db_connections: std::env::var("MAX_DB_CONNECTIONS")
                .unwrap_or_else(|_| "5".into())
                .parse()
                .context("MAX_DB_CONNECTIONS must be a positive number")?,
            jwt_expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".into())
                .parse()
                .context("JWT_EXPIRATION_HOURS must be a valid number")?,
            storage_endpoint: std::env::var("STORAGE_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".into()),
            storage_access_key: std::env::var("STORAGE_ACCESS_KEY")
                .context("STORAGE_ACCESS_KEY environment variable is required")?,
            storage_secret_key: std::env::var("STORAGE_SECRET_KEY")
                .context("STORAGE_SECRET_KEY environment variable is required")?,
            storage_bucket: std::env::var("STORAGE_BUCKET")
                .unwrap_or_else(|_| "skills".into()),
        })
    }
}