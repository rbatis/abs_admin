use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::Result;
use std::time::{Duration, Instant};
use std::sync::{Mutex, MutexGuard};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::Sub;
use crate::service::CONTEXT;
use std::sync::atomic::{AtomicBool, Ordering};

///内存缓存服务
pub struct MemService {
    //因为Mutex不可重入，需要根据locked防止重入
    pub cache: Mutex<HashMap<String, (String, Option<(Instant, Duration)>), RandomState>>,
}


impl Default for MemService {
    fn default() -> Self {
        std::thread::spawn(|| {
            loop {
                match CONTEXT.mem_service.cache.lock() {
                    Ok(mut l) => {
                        let mut need_removed = vec![];
                        for (k, v) in l.iter() {
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
                            l.remove(&x);
                        }
                    }
                    Err(_) => {}
                }
                std::thread::sleep(Duration::from_secs(1));
            }
        });
        Self {
            cache: Default::default(),
        }
    }
}

impl MemService {
    pub fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let mut guard = self.cache.lock().unwrap();
        guard.insert(k.to_string(), (v.to_string(), None));
        return Ok(v.to_string());
    }
    pub fn get_string(&self, k: &str) -> Result<String> {
        let guard = self.cache.lock().unwrap();
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
    pub fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
        where
            T: Serialize,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "MemCacheService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string(k, data.unwrap().as_str())?;
        Ok(data)
    }

    pub fn get_json<T>(&self, k: &str) -> Result<T>
        where
            T: DeserializeOwned,
    {
        let mut r = self.get_string(k)?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "MemCacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    pub fn set_string_ex(&self, k: &str, v: &str, t: Option<Duration>) -> Result<String> {
        let mut locked = self.cache.lock().unwrap();
        let mut e = Option::None;
        if let Some(ex) = t {
            e = Some((Instant::now(), ex));
        }
        let inserted = locked.insert(k.to_string(), (v.to_string(), e));
        if inserted.is_some() {
            return Ok(v.to_string());
        }
        return Result::Err(crate::error::Error::E(format!("[abs_admin][mem_service]insert fail!")));
    }

    pub fn ttl(&self, k: &str) -> Result<i64> {
        let locked = self.cache.lock().unwrap();
        let v = locked.get(k);
        return match v {
            None => {
                Ok(-2)
            }
            Some((r, o)) => {
                match o {
                    None => {
                        Ok(-1)
                    }
                    Some((i, d)) => {
                        let use_time = i.elapsed();
                        if *d > use_time {
                            return Ok(d.sub(use_time).as_secs() as i64);
                        }
                        drop(locked);
                        //clean data
                        self.set_string(k, "")?;
                        Ok(0)
                    }
                }
            }
        };
    }
}
