use std::time::Duration;

use crate::error::{Error, Result};
use crate::service::ICacheService;
use futures_util::future::BoxFuture;
use log::error;
use redis::aio::MultiplexedConnection;
use redis::RedisResult;
///Redis Cache service
#[derive(Debug)]
pub struct RedisCacheService {
    pub client: redis::Client,
}

impl RedisCacheService {
    pub fn new(url: &str) -> Result<Self> {
        println!("[xuangyin] connect redis ({})...", url);
        let client = redis::Client::open(url)
            .map_err(|e| Error::from(format!("open redis client failed={}", e)))?;
        println!("[xuangyin] connect redis success!");
        Ok(Self { client })
    }

    pub async fn get_conn(&self) -> Result<MultiplexedConnection> {
        let conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("RedisService connect fail:{}", e))?;
        Ok(conn)
    }
}

impl ICacheService for RedisCacheService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            return self.set_string_ex(&k, &v, None).await;
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let result: RedisResult<Option<String>> =
                redis::cmd("GET").arg(&[&k]).query_async(&mut conn).await;
            return match result {
                Ok(v) => Ok(v.unwrap_or_default()),
                Err(e) => Err(Error::from(format!(
                    "RedisService get_string({}) fail:{}",
                    k,
                    e.to_string()
                ))),
            };
        })
    }

    ///set_string Automatically expire
    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return if ex.is_none() {
                match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "RedisService set_string_ex fail:{}",
                        e.to_string()
                    ))),
                }
            } else {
                match redis::cmd("SET")
                    .arg(&[&k, &v, "EX", &ex.unwrap_or_default().as_secs().to_string()])
                    .query_async(&mut conn)
                    .await
                {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "RedisService set_string_ex fail:{}",
                        e.to_string()
                    ))),
                }
            };
        })
    }

    ///set_string Automatically expire
    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService ttl fail:{}",
                    e.to_string()
                ))),
            };
        })
    }
}
