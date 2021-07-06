use serde::de::DeserializeOwned;
use crate::error::Result;
use crate::service::CONTEXT;
use crate::service::cache_service::ProxyType::Mem;
use std::time::Duration;
use async_trait::async_trait;
use serde::{Serialize,Deserialize};

pub enum ProxyType {
    Mem,
    Redis,
}

#[async_trait]
pub trait ICacheService {
    async fn set_string(&self, k: &str, v: &str) -> Result<String>;

    async fn get_string(&self, k: &str) -> Result<String>;

    async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>  where T: Serialize+Sync;

    async fn get_json<T>(&self, k: &str) -> Result<T> where T: DeserializeOwned+Sync;

    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String>;

    async fn ttl(&self, k: &str) -> Result<i64>;
}


///内存缓存服务
pub struct CacheService {
    pub inner: ProxyType,
}

impl Default for CacheService {
    fn default() -> Self {
        Self {
            inner: Mem,
        }
    }
}

#[async_trait]
impl ICacheService for CacheService {
    async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.set_string(k, v)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.set_string(k, v).await
            }
        };
    }

    async fn get_string(&self, k: &str) -> Result<String> {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.get_string(k)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.get_string(k).await
            }
        };
    }

    async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
        where
            T: Serialize+Sync,
    {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.set_json::<T>(k, v)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.set_json::<T>(k, v).await
            }
        };
    }

    async fn get_json<T>(&self, k: &str) -> Result<T>
        where
            T: DeserializeOwned+Sync,
    {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.get_json(k)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.get_json(k).await
            }
        };
    }

    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.set_string_ex(k, v, ex)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.set_string_ex(k, v, ex).await
            }
        };
    }

    async fn ttl(&self, k: &str) -> Result<i64> {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.ttl(k)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.ttl(k).await
            }
        };
    }
}
