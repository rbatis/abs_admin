use redis::AsyncCommands;
use log::info;
use serde::de::DeserializeOwned;
use serde::Serialize;
use redis::aio::Connection;

///缓存服务
pub struct CacheService {
    pub client: redis::Client
}

impl CacheService {
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).unwrap();
        info!("connect redis success!");
        Self {
            client
        }
    }

    pub async fn get_conn(&self)->rbatis_core::Result<Connection>{
        let mut conn = self.client.get_async_connection().await;
        if conn.is_err(){
            return Err(rbatis_core::Error::from(conn.err().unwrap().to_string()));
        }
        return Ok(conn.unwrap());
    }



    pub async fn set_json<T>(&self, k: &str, v: &T) -> rbatis_core::Result<String>
    where T:Serialize{
        let mut conn = self.get_conn().await?;
        let data=serde_json::to_string(v);
        if data.is_err(){
            return Err(rbatis_core::Error::from(data.err().unwrap().to_string()));
        }
        let data=data.unwrap();
        let r: String = redis::cmd("SET")
            .arg(&[k, data.as_str()])
            .query_async(&mut conn)
            .await.unwrap_or(String::new());
        Ok(r)
    }

    pub async fn get_json<T>(&self, k: &str) -> rbatis_core::Result<T> where T: DeserializeOwned {
        let mut conn = self.get_conn().await?;
        let r: String = redis::cmd("GET")
            .arg(&[k])
            .query_async(&mut conn)
            .await.unwrap();
        if r.is_empty(){
            return Err(rbatis_core::Error::from("cache data is empty!"));
        }
        let data:serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err(){
            return Err(rbatis_core::Error::from(data.err().unwrap().to_string()));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string(&self, k: &str, v: &str) -> rbatis_core::Result<String>{
        let mut conn = self.get_conn().await?;
        let r: String = redis::cmd("SET")
            .arg(&[k, v])
            .query_async(&mut conn)
            .await.unwrap_or(String::new());
        Ok(r)
    }

    pub async fn get_string(&self, k: &str) -> rbatis_core::Result<String> {
        let mut conn = self.get_conn().await?;
        let r: String = redis::cmd("GET")
            .arg(&[k])
            .query_async(&mut conn)
            .await.unwrap_or(String::new());
        if r.is_empty(){
            return Err(rbatis_core::Error::from("cache data is empty!"));
        }
        Ok(r)
    }
}