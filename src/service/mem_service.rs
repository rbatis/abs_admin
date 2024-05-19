use crate::error::Result;
use crate::service::ICacheService;
use futures_util::future::BoxFuture;
use std::ops::Sub;
use std::time::{Duration, Instant};
use rbatis::dark_std::sync::SyncHashMap;

///Memory Cache Service
#[derive(Debug)]
pub struct MemService {
    //Map<Key,(Value,Option<Instant, Duration>)>
    pub cache: SyncHashMap<String, (String, Option<(Instant, Duration)>)>,
}

impl MemService {
    pub fn recycling(&self) {
        let mut need_removed = vec![];
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

impl Default for MemService {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

impl ICacheService for MemService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        self.cache.insert(k.to_string(), (v.clone(), None));
        Box::pin(async move {
            return Ok(v.to_string());
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let mut v = String::new();
        if let Some(r) = self.cache.get(&k) {
            v = r.0.to_string();
        }
        Box::pin(async move { Ok(v) })
    }

    fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        let mut e = None;
        if let Some(ex) = t {
            e = Some((Instant::now(), ex));
        }
        _ = self.cache.insert(k.to_string(), (v.clone(), e));
        Box::pin(async move {
            return Ok(v.to_string());
        })
    }

    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
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
        Box::pin(async move { Ok(v) })
    }
}
