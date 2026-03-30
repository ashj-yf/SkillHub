use anyhow::{anyhow, Result};
use redis::{AsyncCommands, Client};
use std::time::Duration;
use tracing::{debug, warn};

/// Redis 缓存客户端封装
#[derive(Clone)]
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    /// 创建 Redis 缓存实例
    pub fn new(url: &str) -> Result<Self> {
        let client = Client::open(url)
            .map_err(|e| anyhow!("Failed to create Redis client: {}", e))?;

        debug!("Redis client created successfully");
        Ok(Self { client })
    }

    /// 获取异步连接
    async fn get_connection(&self) -> Result<redis::aio::Connection> {
        self.client
            .get_async_connection()
            .await
            .map_err(|e| anyhow!("Failed to get Redis connection: {}", e))
    }

    /// 设置缓存值
    pub async fn set(&self, key: &str, value: &str, ttl: Duration) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let ttl_secs = ttl.as_secs();

        conn.set_ex::<_, _, ()>(key, value, ttl_secs)
            .await
            .map_err(|e| anyhow!("Failed to set cache: {}", e))?;

        debug!(key = %key, ttl_secs = ttl_secs, "Cache set successfully");
        Ok(())
    }

    /// 获取缓存值
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.get_connection().await?;

        let result: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| anyhow!("Failed to get cache: {}", e))?;

        if result.is_some() {
            debug!(key = %key, "Cache hit");
        } else {
            debug!(key = %key, "Cache miss");
        }

        Ok(result)
    }

    /// 删除缓存值
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;

        let deleted: i64 = conn
            .del(key)
            .await
            .map_err(|e| anyhow!("Failed to delete cache: {}", e))?;

        if deleted > 0 {
            debug!(key = %key, "Cache deleted successfully");
        }

        Ok(())
    }

    /// 删除匹配前缀的所有缓存
    pub async fn delete_pattern(&self, pattern: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;

        // 使用 SCAN 命令安全地查找并删除匹配的键
        let mut cursor: u64 = 0;
        let mut keys_to_delete: Vec<String> = Vec::new();

        loop {
            let (new_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await
                .map_err(|e| anyhow!("Failed to scan keys: {}", e))?;

            keys_to_delete.extend(keys);
            cursor = new_cursor;

            if cursor == 0 {
                break;
            }
        }

        if !keys_to_delete.is_empty() {
            let deleted: i64 = conn
                .del(&keys_to_delete)
                .await
                .map_err(|e| anyhow!("Failed to delete keys: {}", e))?;

            debug!(pattern = %pattern, deleted = deleted, "Pattern cache deleted");
        }

        Ok(())
    }

    /// 检查连接是否正常
    pub async fn ping(&self) -> Result<bool> {
        let mut conn = self.get_connection().await?;

        let result: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| {
                warn!("Redis ping failed: {}", e);
                anyhow!("Redis ping failed: {}", e)
            })?;

        Ok(result == "PONG")
    }
}

/// 缓存键生成工具
pub struct CacheKey;

impl CacheKey {
    /// 技能详情缓存键
    pub fn skill_detail(slug: &str) -> String {
        format!("skill:detail:{}", slug)
    }

    /// 技能列表缓存键
    pub fn skill_list(query_hash: &str) -> String {
        format!("skill:list:{}", query_hash)
    }

    /// 技能 Manifest 缓存键
    pub fn skill_manifest(slug: &str) -> String {
        format!("skill:manifest:{}", slug)
    }

    /// Token 黑名单缓存键
    pub fn token_blacklist(jti: &str) -> String {
        format!("token:blacklist:{}", jti)
    }
}

/// 默认 TTL 工具函数
pub mod ttl {
    use std::time::Duration;

    /// 技能详情缓存时间 (10分钟)
    pub fn skill_detail() -> Duration {
        Duration::from_secs(10 * 60)
    }

    /// 技能列表缓存时间 (5分钟)
    pub fn skill_list() -> Duration {
        Duration::from_secs(5 * 60)
    }

    /// Manifest 缓存时间 (10分钟)
    pub fn skill_manifest() -> Duration {
        Duration::from_secs(10 * 60)
    }

    /// Token 黑名单缓存时间 (24小时)
    pub fn token_blacklist() -> Duration {
        Duration::from_secs(24 * 60 * 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_generation() {
        assert_eq!(
            CacheKey::skill_detail("python-security"),
            "skill:detail:python-security"
        );

        assert_eq!(
            CacheKey::skill_list("abc123"),
            "skill:list:abc123"
        );

        assert_eq!(
            CacheKey::skill_manifest("rust-lint"),
            "skill:manifest:rust-lint"
        );

        assert_eq!(
            CacheKey::token_blacklist("jti-123"),
            "token:blacklist:jti-123"
        );
    }

    #[test]
    fn test_ttl_functions() {
        assert_eq!(ttl::skill_detail(), Duration::from_secs(600));
        assert_eq!(ttl::skill_list(), Duration::from_secs(300));
        assert_eq!(ttl::skill_manifest(), Duration::from_secs(600));
        assert_eq!(ttl::token_blacklist(), Duration::from_secs(86400));
    }
}