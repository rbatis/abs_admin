use log::error;
use log::info;
use rbatis::core::{Error, Result};
use redis::aio::Connection;
use serde::de::DeserializeOwned;
use serde::Serialize;

///缓存服务
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).unwrap();
        info!("connect redis success!");
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(rbatis::core::Error::from(err));
        }
        return Ok(conn.unwrap());
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
    where
        T: Serialize,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(rbatis::core::Error::from(format!(
                "RedisService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string(k, data.unwrap().as_str()).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(rbatis::core::Error::from(format!(
                "RedisService get_json fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let mut conn = self.get_conn().await?;
        match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
            Ok(v) => {
                return Ok(v);
            }
            Err(e) => {
                return Err(Error::from(format!(
                    "RedisService set_string fail:{}",
                    e.to_string()
                )));
            }
        }
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        let mut conn = self.get_conn().await?;
        match redis::cmd("GET").arg(&[k]).query_async(&mut conn).await {
            Ok(v) => {
                return Ok(v);
            }
            Err(e) => {
                return Err(Error::from(format!(
                    "RedisService get_string fail:{}",
                    e.to_string()
                )));
            }
        }
    }
}
