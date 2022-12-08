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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(usize)]
pub enum ServiceCode {
    Success = 0,
    BusinessError = 1000,
    AccessTokenIsEmpty = 1001,
}

impl From<ServiceCode> for String {
    fn from(code: ServiceCode) -> Self {
        Self::from(&code)
    }
}

impl From<&ServiceCode> for String {
    fn from(code: &ServiceCode) -> Self {
        match code {
            ServiceCode::Success => "SUCCESS".to_string(),
            ServiceCode::BusinessError => "FAIL".to_string(),
            ServiceCode::AccessTokenIsEmpty => "access token is empty!".to_string(),
            #[allow(unreachable_patterns)]
            _ => "".to_string(),
        }
    }
}

/// The http interface returns the model structure, providing basic json data structures such as code, msg, and data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: ServiceCode,
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
                code: ServiceCode::Success,
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: ServiceCode::BusinessError,
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: ServiceCode::Success,
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(arg: &Error) -> Self {
        Self {
            code: ServiceCode::BusinessError,
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(info: &str) -> Self {
        RespVO::<T>::from_code_info(ServiceCode::BusinessError, info)
    }

    pub fn from_code_info(code: ServiceCode, info: &str) -> Self {
        let info = if info.is_empty() {
            String::from(&code)
        } else {
            info.to_string()
        };
        Self {
            code: code,
            msg: Some(info),
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
