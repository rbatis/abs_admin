use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::Result;
use std::time::Duration;
use std::sync::Mutex;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

///内存缓存服务
pub struct MemService {
    pub cache: Mutex<HashMap<String, String, RandomState>>,
}

impl Default for MemService {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

impl MemService {
    pub fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let mut guard = self.cache.lock().unwrap();
        guard.insert(k.to_string(), v.to_string());
        return Ok(v.to_string());
    }
    pub fn get_string(&self, k: &str) -> Result<String> {
        let guard = self.cache.lock().unwrap();
        let v = guard.get(k);
        match v {
            Some(v) => {
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


    pub fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        unimplemented!()
    }

    pub fn ttl(&self, k: &str) -> Result<i64> {
        unimplemented!()
    }
}
