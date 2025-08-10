use crate::config::config::ApplicationConfig;
use crate::error::{Error, Result};
use crate::service::MemCacheService;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::time::Duration;
use async_trait::async_trait;

#[async_trait]
pub trait ICacheService: Sync + Send + Debug {
    /// set key-value
    async fn set_string(&self, k: &str, v: &str) -> Result<String>;

    /// get value from key
    async  fn get_string(&self, k: &str) -> Result<String>;

    /// set key  Time To Live(Duration)
    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String>;

    /// get key  Time To Live(secs)
    async fn ttl(&self, k: &str) -> Result<i64>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    pub fn new(cfg: &ApplicationConfig) -> Result<Self> {
        let cache = cfg.cache.as_str();
        if cache == "mem" {
            println!("[xuangyin] cache_type: mem");
            return Ok(Self {
                inner: Box::new(MemCacheService::default()),
            });
        } else if cache.starts_with("redis") {
            #[cfg(feature = "cache_redis")]
            {
                println!("[xuangyin] cache_type: redis");
                return Ok(Self {
                    inner: Box::new(crate::service::RedisCacheService::new(&cache)?),
                });
            }
        }
        Err(Error::from(format!(
            "[xuangyin] unknown of cache: \"{}\",current support 'mem' or 'redis'",
            cache
        )))
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.inner.set_string(k, v).await
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        self.inner.get_string(k).await
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
    where
        T: Serialize + Sync,
    {
        let data = serde_json::to_string(v);
        if let Err(e) = &data {
            return Err(crate::error::Error::from(format!(
                "MemCacheService set_json fail:{}",
                e
            )));
        }
        let value = data.map_err(|e| Error::from(e.to_string()))?;
        let result = self.set_string(k, &value).await?;
        Ok(result)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: T = serde_json::from_str(r.as_str())
            .map_err(|e| Error::from(format!("MemCacheService GET fail:{}", e)))?;
        Ok(data)
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        self.inner.set_string_ex(k, v, ex).await
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        self.inner.ttl(k).await
    }
}
