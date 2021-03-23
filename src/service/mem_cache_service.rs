use std::collections::hash_map::RandomState;

use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::{Error, Result};

///缓存服务
pub struct MemCacheService {
    pub cache: DashMap<String, String>,
}

impl Default for MemCacheService {
    fn default() -> Self {
        Self {
            cache: Default::default()
        }
    }
}

impl MemCacheService {
    pub fn set_string(&self, k: &str, v: &str) -> Result<String> {
        return self.cache.insert(k.to_string(), v.to_string()).ok_or(Error::E("save cache none".to_string()));
    }
    pub fn get_string(&self, k: &str) -> Result<String> {
        let v = self.cache.get(k);
        match v {
            Some(v) => {
                return Ok(v.value().to_string());
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
}