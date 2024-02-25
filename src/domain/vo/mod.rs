pub mod dict;
pub mod jwt;
pub mod res;
pub mod role;
pub mod sign_in;
pub mod user;

pub use dict::*;
pub use jwt::*;
pub use res::*;
pub use role::*;
pub use sign_in::*;

use crate::error::Error;
use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::service::CONTEXT;

pub const CODE_SUCCESS: &str = "SUCCESS";
pub const CODE_FAIL: &str = "-1";

/// The http interface returns the model structure, providing basic json data structures such as code, msg, and data
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
    pub fn from_result(result: Result<T, Error>) -> Self {
        match result {
            Ok(data) => {
                Self {
                    code: Some(CODE_SUCCESS.to_string()),
                    msg: None,
                    data: Some(data),
                }
            }
            Err(e) => {
                Self::from_error(e.to_string())
            }
        }
    }

    pub fn from(data: T) -> Self {
        Self {
            code: Some(CODE_SUCCESS.to_string()),
            msg: None,
            data: Some(data),
        }
    }

    pub fn from_error(msg: String) -> Self {
        let mut error = msg.to_string();
        if error.contains("{}") {
            error = error[0..error.find("{}").unwrap()].to_string();
        }
        let code = CONTEXT.config.error_infos.as_ref().unwrap().get(&error).map(|v| v.to_string()).unwrap_or_else(|| { CODE_FAIL.to_string() });
        Self {
            code: Some(code),
            msg: Some(msg.to_string()),
            data: None,
        }
    }

    pub fn resp_json(&self) -> HttpResponse {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/json;charset=UTF-8"))
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
