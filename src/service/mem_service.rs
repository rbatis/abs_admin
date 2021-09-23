use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{Error, Result};
use crate::service::ICacheService;
use async_trait::async_trait;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::Sub;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

///内存缓存服务
pub struct MemService {
    pub cache: Mutex<HashMap<String, (String, Option<(Instant, Duration)>), RandomState>>,
}

impl MemService {
    pub fn recycling(&self) {
        if let Ok(mut map_lock_guard) = self.cache.lock() {
            let mut need_removed = vec![];
            for (k, v) in map_lock_guard.iter() {
                match v.1 {
                    None => {}
                    Some((i, d)) => {
                        if i.elapsed() >= d {
                            //out of time
                            need_removed.push(k.to_string());
                        }
                    }
                }
            }
            for x in need_removed {
                map_lock_guard.remove(&x);
            }
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

impl<T> std::convert::From<PoisonError<T>> for Error {
    fn from(arg: PoisonError<T>) -> Self {
        Error::E(arg.to_string())
    }
}

#[async_trait]
impl ICacheService for MemService {
    async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.recycling();
        let mut guard = self.cache.lock()?;
        guard.insert(k.to_string(), (v.to_string(), None));
        return Ok(v.to_string());
    }
    async fn get_string(&self, k: &str) -> Result<String> {
        self.recycling();
        let guard = self.cache.lock()?;
        let v = guard.get(k);
        match v {
            Some((v, _)) => {
                return Ok(v.to_string());
            }
            _ => {
                return Ok("".to_string());
            }
        }
    }

    async fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> Result<String> {
        self.recycling();
        let mut locked = self.cache.lock()?;
        let mut e = Option::None;
        if let Some(ex) = t {
            e = Some((Instant::now(), ex));
        }
        let inserted = locked.insert(k.to_string(), (v.to_string(), e));
        if inserted.is_some() {
            return Ok(v.to_string());
        }
        return Result::Err(crate::error::Error::E(format!(
            "[abs_admin][mem_service]insert fail!"
        )));
    }

    async fn ttl(&self, k: &str) -> Result<i64> {
        self.recycling();
        let locked = self.cache.lock()?;
        let v = locked.get(k).cloned();
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
    }
}
