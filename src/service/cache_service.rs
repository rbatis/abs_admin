use redis::AsyncCommands;
use log::info;
use serde::de::DeserializeOwned;
use serde::Serialize;

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

    pub async fn put_json<T>(&self, k: &str, v: &T) -> rbatis_core::Result<String>
    where T:Serialize{
        let mut conn = self.client.get_async_connection().await.unwrap();
        let data=serde_json::to_string(v);
        if data.is_err(){
            return Err(rbatis_core::Error::from(data.err().unwrap().to_string()));
        }
        let data=data.unwrap();
        let r: String = redis::cmd("SET")
            .arg(&[k, data.as_str()])
            .query_async(&mut conn)
            .await.unwrap();
        Ok(r)
    }

    pub async fn get_json<T>(&self, k: &str) -> rbatis_core::Result<T> where T: DeserializeOwned {
        let mut conn = self.client.get_async_connection().await.unwrap();
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
}