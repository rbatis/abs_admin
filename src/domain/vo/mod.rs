pub mod dict;
pub mod jwt;
pub mod res;
pub mod role;
pub mod sign_in;
pub mod user;

use axum::response::IntoResponse;
pub use dict::*;
pub use jwt::*;
pub use res::*;
pub use role::*;
pub use sign_in::*;

use crate::error::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub const CODE_SUCCESS: &str = "0";
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
            Ok(data) => Self {
                code: Some(CODE_SUCCESS.to_string()),
                msg: None,
                data: Some(data),
            },
            Err(e) => Self::from_error(e),
        }
    }

    pub fn from(data: T) -> Self {
        Self {
            code: Some(CODE_SUCCESS.to_string()),
            msg: None,
            data: Some(data),
        }
    }

    pub fn from_error(error: Error) -> Self {
        match error {
            Error::E(msg) => Self {
                code: Some(CODE_FAIL.to_string()),
                msg: Some(msg),
                data: None,
            },
            Error::CE(code,msg) => Self {
                code: Some(code.to_string()),
                msg: Some(msg),
                data: None,
            },
        }

        // let code = CONTEXT
        //     .config
        //     .error_infos
        //     .as_ref()
        //     .unwrap()
        //     .get(&error)
        //     .map(|v| v.to_string())
        //     .unwrap_or_else(|| CODE_FAIL.to_string());
        // Self {
        //     code: Some(code),
        //     msg: Some(error),
        //     data: None,
        // }
    }

    pub fn json(self) -> axum::Json<RespVO<T>> {
        axum::Json(self)
    }
}

impl<T> IntoResponse for RespVO<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
