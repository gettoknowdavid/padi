use anyhow::{Context, Result};
use deadpool_redis::{Config as RedisConfig, Pool, Runtime};
pub fn cache_redis_pool(redis_url: &str) -> Result<Pool> {
    let cfg = RedisConfig::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1))
        .context("Failed to create Redis connection pool")
}
