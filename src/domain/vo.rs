use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use rbatis_core::Error;
use crate::domain::domain::BizAdminUser;
use serde::de::DeserializeOwned;
use actix_web::{HttpResponse, Responder, web};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JWTToken {
    id: String,
    account: String,
    permissions: Vec<String>,
    role_ids: Vec<String>,
    exp: usize,
}

impl JWTToken {
    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        return match encode(&Header::default(), self, &EncodingKey::from_secret(secret.as_ref())) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error> {
        let validation = Validation { ..Validation::default() };
        return match decode::<JWTToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                _ => return Err(Error::from("InvalidToken other errors"))
            },
        };
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T> where T: Serialize + DeserializeOwned + Clone {
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some("SUCCESS".to_string()),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some("FAIL".to_string()),
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some("SUCCESS".to_string()),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(code: &str, arg: &Error) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = "FAIL".to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn to_json_resp(&self) -> actix_http::Response {
        return HttpResponse::Ok().content_type("json").body(self.to_string());
    }
}

impl<T> ToString for RespVO<T> where T: Serialize + DeserializeOwned + Clone {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    pub user: Option<BizAdminUser>,
    pub permissions: Vec<String>,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
