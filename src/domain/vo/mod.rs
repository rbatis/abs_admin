pub mod sign_in;

pub use sign_in::*;

pub mod res;

pub use res::*;

pub mod jwt;

pub use jwt::*;

pub mod role;

pub use role::*;

use crate::config::CONFIG;
use actix_http::Response;
use actix_web::HttpResponse;
use rbatis::core::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub const CODE_SUCCESS: &str = "SUCCESS";
pub const CODE_FAIL: &str = "FAIL";

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(CODE_SUCCESS.to_string()),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some(CODE_FAIL.to_string()),
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some(CODE_SUCCESS.to_string()),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(code: &str, arg: &Error) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = CODE_FAIL.to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: &str, info: &str) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = CODE_FAIL.to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(info.to_string()),
            data: None,
        }
    }

    pub fn resp_json(&self) -> Response {
        if CONFIG.debug {
            println!("[abs_admin][debug] resp:{}", self.to_string());
        }
        return HttpResponse::Ok()
            .set_header("Access-Control-Allow-Origin", "*")
            .set_header("Cache-Control", "no-cache")
            .content_type("json")
            .body(self.to_string());
    }
}

impl<T> ToString for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
