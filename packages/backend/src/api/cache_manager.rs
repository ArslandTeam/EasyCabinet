use crate::BackendError;
use redis::AsyncCommands;

#[derive(Clone)]
enum CacheType {
    Local(moka::future::Cache<String, String>),
    Redis(redis::aio::MultiplexedConnection),
}

#[derive(Clone)]
pub struct CacheManager {
    cache_type: CacheType,
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
                    .time_to_live(std::time::Duration::from_secs(2592000))
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
                    cache_type: CacheType::Redis(conn),
                }
            }
            _ => panic!("CACHE manager not correct set"),
        }
    }

    pub async fn set(&self, key: String, value: String) -> Result<(), BackendError> {
        match &self.cache_type {
            CacheType::Local(cache) => {
                cache.insert(key, value).await;
                Ok(())
            }
            CacheType::Redis(conn) => {
                let mut cache = conn.clone();
                cache
                    .set(key, value)
                    .await
                    .map_err(|_| BackendError::InternalError)
            }
        }
    }

    pub async fn get(&self, key: String) -> Option<String> {
        match &self.cache_type {
            CacheType::Local(cache) => cache.get(&key).await,
            CacheType::Redis(conn) => {
                let mut cache = conn.clone();
                cache.get(&key).await.ok()
            }
        }
    }

    pub async fn delete(&self, key: String) -> Result<(), BackendError> {
        match &self.cache_type {
            CacheType::Local(cache) => {
                cache.invalidate(&key).await;
                Ok(())
            }
            CacheType::Redis(conn) => {
                let mut cache = conn.clone();
                let _: usize = cache
                    .del(&key)
                    .await
                    .map_err(|_| BackendError::InternalError)?;
                Ok(())
            }
        }
    }
}
