use crate::error::Result;
use crate::service::{MemService, RedisService};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ops::Deref;
use std::time::Duration;
use crate::config::app_config::ApplicationConfig;

#[async_trait]
pub trait ICacheService: Sync + Send {
    async fn set_string(&self, k: &str, v: &str) -> Result<String>;

    async fn get_string(&self, k: &str) -> Result<String>;

    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String>;

    async fn ttl(&self, k: &str) -> Result<i64>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    pub fn new(cfg:&ApplicationConfig) -> Self {
        Self {
            inner: match cfg.cache_type.as_str() {
                "redis" => Box::new(RedisService::new(&cfg.redis_url)),
                //"mem"
                _ => Box::new(MemService::default()),
            },
        }
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
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "MemCacheService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string(k, data.unwrap().as_str()).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "MemCacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        self.inner.set_string_ex(k, v, ex).await
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        self.inner.ttl(k).await
    }
}
