use crate::error::Result;
use crate::service::ICacheService;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::Sub;
use std::time::{Duration, Instant};
use futures_util::future::BoxFuture;
use parking_lot::Mutex;

///内存缓存服务
pub struct MemService {
    pub cache: Mutex<HashMap<String, (String, Option<(Instant, Duration)>), RandomState>>,
}

impl MemService {
    pub fn recycling(&self) {
        let mut map_lock_guard = self.cache.lock();
        let mut need_removed = vec![];
        for (k, v) in map_lock_guard.iter() {
            if let Some((i, d)) = v.1 {
                if i.elapsed() >= d {
                    //out of time
                    need_removed.push(k.to_string());
                }
            }
        }
        for x in need_removed {
            map_lock_guard.remove(&x);
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
        Box::pin(async move {
            let mut guard = self.cache.lock();
            guard.insert(k.to_string(), (v.to_string(), None));
            return Ok(v.to_string());
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        Box::pin(async move {
            let guard = self.cache.lock();
            let v = guard.get(&k);
            match v {
                Some((v, _)) => {
                    return Ok(v.to_string());
                }
                _ => {
                    return Ok("".to_string());
                }
            }
        })
    }

    fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            let mut locked = self.cache.lock();
            let mut e = Option::None;
            if let Some(ex) = t {
                e = Some((Instant::now(), ex));
            }
            let inserted = locked.insert(k, (v.clone(), e));
            if inserted.is_some() {
                return Ok(v.to_string());
            }
            return Result::Err(crate::error::Error::E(format!(
                "[abs_admin][mem_service]insert fail!"
            )));
        })
    }

    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        self.recycling();
        let k = k.to_string();
        Box::pin(async move {
            let locked = self.cache.lock();
            let v = locked.get(&k).cloned();
            drop(locked);
            return match v {
                None => Ok(-2),
                Some((r, o)) => match o {
                    None => Ok(-1),
                    Some((i, d)) => {
                        let use_time = i.elapsed();
                        if d > use_time {
                            return Ok(d.sub(use_time).as_secs() as i64);
                        }
                        Ok(0)
                    }
                },
            };
        })
    }
}
