use crate::BackendError;
use redis::AsyncCommands;

#[derive(Clone)]
enum CacheType {
    Local(moka::future::Cache<String, MokaCacheTTL>),
    Redis(std::sync::Arc<tokio::sync::Mutex<redis::aio::MultiplexedConnection>>),
}

#[derive(Clone)]
pub struct CacheManager {
    cache_type: CacheType,
}

#[derive(Clone)]
pub struct MokaCacheTTL {
    value: String,
    ttl: std::time::Duration,
}

struct PerEntryExpiry;

/// Это реализация кастомного TTL для каждого элемента в кеше.
/// Пример взят с документации moka
/// - <https://docs.rs/moka/latest/moka/future/struct.Cache.html#per-entry-expiration-policy>
impl moka::Expiry<String, MokaCacheTTL> for PerEntryExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &MokaCacheTTL,
        _current_time: std::time::Instant,
    ) -> Option<std::time::Duration> {
        Some(value.ttl)
    }
}

/// Инициализирует тип кеша.
/// Из .env извлекает значение и присваивает его переменной cache_type с типом `CacheType`
impl CacheManager {
    pub async fn cache_init() -> Self {
        match std::env::var("CACHE")
            .expect("CACHE key not set in .env")
            .as_str()
        {
            "local" => {
                let cache = moka::future::Cache::builder()
                    .max_capacity(1000)
                    .expire_after(PerEntryExpiry)
                    .build();
                CacheManager {
                    cache_type: CacheType::Local(cache),
                }
            }
            "redis" => {
                let redis_url = redis::Client::open(std::env::var("REDIS_URL").unwrap())
                    .expect("Invalid Redis URL");
                let conn = redis_url
                    .get_multiplexed_async_connection()
                    .await
                    .expect("Error connecting to Redis");
                CacheManager {
                    cache_type: CacheType::Redis(std::sync::Arc::new(tokio::sync::Mutex::new(
                        conn,
                    ))),
                }
            }
            _ => panic!("CACHE manager not correct set"),
        }
    }

    pub async fn set(&self, key: &str, value: &str, ttl: u64) -> Result<(), BackendError> {
        match &self.cache_type {
            CacheType::Local(cache) => {
                cache
                    .insert(
                        key.to_string(),
                        MokaCacheTTL {
                            value: value.to_string(),
                            ttl: std::time::Duration::from_secs(ttl),
                        },
                    )
                    .await;
                Ok(())
            }
            CacheType::Redis(conn) => {
                let mut conn = conn.lock().await;
                conn.set_ex(key, value, ttl)
                    .await
                    .map_err(|_| BackendError::InternalError)
            }
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        match &self.cache_type {
            CacheType::Local(cache) => cache.get(key).await.map(|v| v.value),
            CacheType::Redis(conn) => {
                let mut conn = conn.lock().await;
                conn.get(key).await.ok()
            }
        }
    }

    pub async fn delete(&self, key: &str) -> Result<(), BackendError> {
        match &self.cache_type {
            CacheType::Local(cache) => {
                cache.invalidate(key).await;
                Ok(())
            }
            CacheType::Redis(conn) => {
                let mut conn = conn.lock().await;
                let _: usize = conn
                    .del(key)
                    .await
                    .map_err(|_| BackendError::InternalError)?;
                Ok(())
            }
        }
    }
}
