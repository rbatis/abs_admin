use log::error;
use log::info;
use redis::aio::Connection;
use serde::de::DeserializeOwned;
use serde::Serialize;
use rbatis_core::Result;

///缓存服务
pub struct RedisService {
    pub client: redis::Client
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).unwrap();
        info!("connect redis success!");
        Self {
            client
        }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = conn.err().unwrap().to_string();
            error!("CacheService get_conn fail! {}",err.as_str());
            return Err(rbatis_core::Error::from(err));
        }
        return Ok(conn.unwrap());
    }


    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
        where T: Serialize {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(rbatis_core::Error::from(data.err().unwrap().to_string()));
        }
        let data = self.set_string(k,data.unwrap().as_str()).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T> where T: DeserializeOwned {
        let r = self.get_string(k).await?;
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(rbatis_core::Error::from(data.err().unwrap().to_string()));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let mut conn = self.get_conn().await?;
        let r: String = redis::cmd("SET")
            .arg(&[k, v])
            .query_async(&mut conn)
            .await.unwrap_or(String::new());
        Ok(r)
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        let mut conn = self.get_conn().await?;
        let r: String = redis::cmd("GET")
            .arg(&[k])
            .query_async(&mut conn)
            .await.unwrap_or(String::new());
        if r.is_empty() {
            return Err(rbatis_core::Error::from("cache data is empty!"));
        }
        Ok(r)
    }
}