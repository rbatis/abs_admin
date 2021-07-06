use dashmap::DashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::Result;
use crate::service::redis_service::RedisService;
use crate::service::CONTEXT;
use crate::service::cache_service::ProxyType::Mem;
use std::time::Duration;

pub enum ProxyType {
    Mem,
    Redis,
}

///内存缓存服务
pub struct MemService {
    pub inner: ProxyType,
}

impl Default for MemService {
    fn default() -> Self {
        Self {
            inner: Mem,
        }
    }
}

impl MemService {
    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.set_string(k, v)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.set_string(k, v).await
            }
        };
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.get_string(k)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.get_string(k).await
            }
        };
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
        where
            T: Serialize,
    {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.set_json(k, v)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.set_json(k, v).await
            }
        };
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
        where
            T: DeserializeOwned,
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

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        return match self.inner {
            Mem => {
                CONTEXT.mem_service.set_string_ex(k, v, ex)
            }
            ProxyType::Redis => {
                CONTEXT.redis_service.set_string_ex(k, v, ex).await
            }
        };
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
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
