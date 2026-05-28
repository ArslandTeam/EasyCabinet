use crate::{BackendError, generate_config::CONFIG};
use futures::TryStreamExt;
use redis::AsyncCommands;

pub struct CacheManager {
    redis: redis::aio::MultiplexedConnection,
}

impl CacheManager {
    pub async fn cache_init() -> Self {
        let redis_url = redis::Client::open(&*CONFIG.redis_url).expect("Invalid Redis URL");
        let conn = redis_url
            .get_multiplexed_async_connection()
            .await
            .expect("Error connecting to Redis");

        Self { redis: conn }
    }

    pub async fn set(&self, key: &str, value: &str, ttl: u64) -> Result<(), BackendError> {
        let mut conn = self.redis.clone();
        conn.set_ex(key, value, ttl)
            .await
            .map_err(|_| BackendError::InternalError)
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, BackendError> {
        let mut conn = self.redis.clone();
        conn.get(key).await.map_err(|_| BackendError::InternalError)
    }

    pub async fn get_del(&self, key: &str) -> Result<Option<String>, BackendError> {
        let mut conn = self.redis.clone();
        conn.get_del(key)
            .await
            .map_err(|_| BackendError::InternalError)
    }

    pub async fn delete<T: std::marker::Send + std::marker::Sync + redis::ToRedisArgs>(
        &self,
        key: T,
    ) -> Result<(), BackendError> {
        let mut conn = self.redis.clone();
        conn.del::<_, usize>(key)
            .await
            .map_err(|_| BackendError::InternalError)?;
        Ok(())
    }

    async fn scan_match(&self, pattern: &str) -> Result<Vec<String>, BackendError> {
        let mut conn = self.redis.clone();

        let iter = conn.scan_match::<_, String>(pattern).await;

        let keys = iter
            .map_err(|_| BackendError::InternalError)?
            .into_stream()
            .try_collect::<Vec<String>>()
            .await
            .map_err(|_| BackendError::InternalError)?;

        Ok(keys)
    }

    pub async fn get_pattern(&self, pattern: &str) -> Result<Vec<String>, BackendError> {
        let keys = self.scan_match(pattern).await?;

        if keys.is_empty() {
            return Ok(Vec::new());
        }

        let mut conn = self.redis.clone();

        let values: Vec<String> = conn
            .mget(keys)
            .await
            .map_err(|_| BackendError::InternalError)?;

        Ok(values)
    }

    pub async fn delete_pattern(&self, pattern: &str) -> Result<(), BackendError> {
        let keys = self.scan_match(pattern).await?;

        if keys.is_empty() {
            return Ok(());
        }

        self.delete(keys).await?;

        Ok(())
    }
}
