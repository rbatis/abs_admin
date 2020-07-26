use redis::AsyncCommands;
use log::info;
pub struct CacheService {
    pub client: redis::Client
}

impl CacheService {
    pub fn new(url:&str) -> Self {
        let client = redis::Client::open(url).unwrap();
        info!("connect redis success!");
        Self {
            client
        }
    }

    pub async fn put(&self, k: &str, v: &str) -> rbatis_core::Result<String> {
        let mut conn = self.client.get_async_connection().await.unwrap();
       let r:String = redis::cmd("SET")
            .arg(&[k, v])
            .query_async(&mut conn)
            .await.unwrap();
        Ok(r)
    }
}