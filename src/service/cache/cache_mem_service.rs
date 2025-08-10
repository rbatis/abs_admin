use crate::error::Result;
use crate::service::ICacheService;
use rbatis::dark_std::sync::SyncHashMap;
use std::ops::Sub;
use std::time::{Duration, Instant};
use async_trait::async_trait;

///Memory Cache Service
#[derive(Debug)]
pub struct MemCacheService {
    //Map<Key,(Value,Option<Instant(remain time), Duration(time to live)>)>
    pub cache: SyncHashMap<String, (String, Option<(Instant, Duration)>)>,
}

impl MemCacheService {
    pub fn recycling(&self) {
        let mut need_removed = Vec::with_capacity(self.cache.len());
        for (k, v) in self.cache.iter() {
            if let Some((i, d)) = v.1 {
                if i.elapsed() >= d {
                    //out of time
                    need_removed.push(k.to_string());
                }
            }
        }
        if need_removed.len() != 0 {
            for x in need_removed {
                self.cache.remove(&x);
            }
            self.cache.shrink_to_fit();
        }
    }
}

impl Default for MemCacheService {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

#[async_trait]
impl ICacheService for MemCacheService {
    async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        self.cache.insert(k.to_string(), (v.clone(), None));
        Ok(v.to_string())
    }

    async fn get_string(&self, k: &str) -> Result<String> {
        self.recycling();
        let k = k.to_string();
        let mut v = String::new();
        if let Some(r) = self.cache.get(&k) {
            v = r.0.to_string();
        }
        Ok(v)
    }

    async fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> Result<String> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let mut e = None;
        if let Some(ex) = t {
            e = Some((Instant::now(), ex));
        }
        _ = self.cache.insert(k.to_string(), (v.clone(), e));
        Ok(v.to_string())
    }

    async fn ttl(&self, k: &str) -> Result<i64> {
        self.recycling();
        let v = self.cache.get(k).cloned();
        let v = match v {
            None => -2,
            Some((_r, o)) => match o {
                None => -1,
                Some((i, d)) => {
                    let use_time = i.elapsed();
                    if d > use_time {
                        d.sub(use_time).as_secs() as i64
                    } else {
                        0
                    }
                }
            },
        };
        Ok(v)
    }
}

