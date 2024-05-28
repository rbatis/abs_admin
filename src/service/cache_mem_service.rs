use crate::error::Result;
use crate::service::ICacheService;
use futures_util::future::BoxFuture;
use std::ops::Sub;
use std::time::{Duration, Instant};
use rbatis::dark_std::sync::SyncHashMap;

type Velue = (String, Option<(Instant, Duration)>);

///Memory Cache Service
#[derive(Debug,Clone)]
pub struct MemCacheService {
    //Map<Key,(Value,Option<Instant(remain time), Duration(time to live)>)>
    pub cache: SyncHashMap<String, Velue>,
    is_run: bool,
}

impl MemCacheService {
    pub fn new() -> Self {
        let s = Self {
            cache: Default::default(),
            is_run: true,
        };
        let s1 = s.clone();
        tokio::spawn(async move {
            loop {
                if !s1.is_run {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
                s1.recycling();
            }
        });
        s
    }

    pub fn recycling(&self) {
        let need_removed = self.cache.iter().filter(|(_, v)| {
            if let Some((i, d)) = v.1 {
                //out of time
                i.elapsed() >= d
            } else {
                false
            }
        }).map(|(k, _)| k.to_string()).collect::<Vec<String>>();

        if !need_removed.is_empty() {
            need_removed.iter().for_each(|x| {
                self.cache.remove(x);
            });
            self.cache.shrink_to_fit();
        }
    }
}

impl Default for MemCacheService {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MemCacheService {
    fn drop(&mut self) {
        self.is_run = false;
    }
}

impl ICacheService for MemCacheService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        // self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        self.cache.insert(k, (v.clone(), None));
        Box::pin(async move {
            Ok(v)
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        // self.recycling();
        let mut v = String::new();
        if let Some(r) = self.cache.get(k) {
            v = r.0.to_string();
        }
        Box::pin(async move { Ok(v) })
    }

    fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> BoxFuture<Result<String>> {
        // self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let e = t.map(|d| (Instant::now(), d));
        _ = self.cache.insert(k, (v.clone(), e));
        Box::pin(async move {
            Ok(v)
        })
    }

    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        // self.recycling();
        let v = self.cache.get(k);
        let v = match v {
            None => -2,
            Some((_r, o)) => match o {
                None => -1,
                Some((i, d)) => {
                    let use_time = i.elapsed();
                    if *d > use_time {
                        d.sub(use_time).as_secs() as i64
                    } else {
                        0
                    }
                }
            },
        };
        Box::pin(async move { Ok(v) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mem_cache_get_time() {
        let m = MemCacheService::new();
        for i in 0..100000 {
            m.set_string(&i.to_string(), &i.to_string()).await.unwrap();
            
        }
        println!("{:?}", m.get_string("1").await);
        
        let start = std::time::Instant::now();
        for _i in 0..10000 {
            m.get_string(&_i.to_string()).await.unwrap();
        } 
        println!("no recycling time:   {:?}", start.elapsed());
        let start = std::time::Instant::now();
        for _i in 0..100 {
            m.recycling();
            m.get_string(&_i.to_string()).await.unwrap();
        } 
        println!("need recycling time: {:?}", start.elapsed());


    }
}