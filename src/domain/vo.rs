use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use rbatis_core::Error;
use crate::domain::domain::BizAdminUser;

#[derive(Debug, Serialize, Deserialize)]
struct JWTToken {
    id: String,
    account: String,
    permissions: Vec<String>,
    role_ids: Vec<String>,
    exp: usize,
}

impl JWTToken {
    pub fn create_token(&self, secret: &str) -> Result<String,Error> {
         match encode(&Header::default(), self, &EncodingKey::from_secret(secret.as_ref())) {
            Ok(t) => return Ok(t),
            Err(_) =>  return Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
         }
    }

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

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInVO {
    pub user:Option<BizAdminUser>,
    pub permissions:Vec<String>,
}

impl ToString for SignInVO{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
