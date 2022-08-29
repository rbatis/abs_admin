use std::time::Duration;

use crate::error::{Error, Result};
use crate::service::ICacheService;
use futures_util::future::BoxFuture;
use log::error;
use redis::aio::Connection;
use redis::RedisResult;
///Redis缓存服务
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        println!("[abs_admin] conncect redis ({})...", url);
        let client = redis::Client::open(url).unwrap();
        println!("[abs_admin] conncect redis success!");
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(crate::error::Error::from(err));
        }
        return Ok(conn.unwrap());
    }
}

impl ICacheService for RedisService {
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

    ///set_string 自动过期
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
                    .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
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

    ///set_string 自动过期
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
