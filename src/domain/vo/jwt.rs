use std::sync::OnceLock;

use crate::error::Error;
use crate::service::CONTEXT;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};


/// JWT authentication Token structure
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct JWTToken {
    pub id: String,
    pub account: String,
    pub permissions: Vec<String>,
    pub role_ids: Vec<String>,
    pub exp: usize,
}

impl JWTToken {
    /// create token
    /// secret: your secret string
    pub fn create_token(&self) -> Result<String, Error> {
        match encode(
            &Header::default(),
            self,
            &get_key().encoding_key,
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        }
    }
    /// verify token invalid
    pub fn verify(token: &str) -> Result<JWTToken, Error> {
        let mut validation = Validation::default();
        validation.leeway = 0;
        return match decode::<JWTToken>(
            token,
            &get_key().decoding_key,
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                ErrorKind::ExpiredSignature => return Err(Error::from("ExpiredSignature")),
                _ => return Err(Error::from("InvalidToken other errors")),
            },
        };
    }

    /// refresh exp seconds
    pub fn refresh(&mut self, jwt_exp: usize) -> Result<String, Error> {
        self.exp += jwt_exp;
        self.create_token()
    }
}


fn get_key()-> &'static Keys {
    static KEYS: OnceLock<Keys> = OnceLock::new();
    KEYS.get_or_init(|| {
        // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let secret = CONTEXT.config.jwt_secret.as_str();
        Keys::new(secret)

    })
}

struct Keys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Keys {
    fn new(secret: impl AsRef<[u8]>) -> Self {
        let secret = secret.as_ref();
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }
}

// impl FromRequest 
#[axum::async_trait]
impl<S> FromRequestParts<S> for JWTToken
where
    S: Send + Sync,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        log::info!("JWTToken from_request_parts");
        if let Some(auth) = parts.extensions.get::<Self>() {
            log::info!("JWTToken from_request_parts extensions");
            return Ok(auth.clone());
        }

        let token = parts.headers.get("access_token").ok_or("access_token not found")?;
        let token = token.to_str().unwrap_or(""); //.trim_start_matches("Bearer ");
        JWTToken::verify(token).map_err(|e| e.to_string())
        
    }
}

#[cfg(test)]
mod test {
    use crate::domain::vo::JWTToken;
    use rbatis::rbdc::types::DateTime;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_jwt() {
        let j = JWTToken {
            id: "1".to_string(),
            account: "189".to_string(),
            permissions: vec![],
            role_ids: vec![],
            exp: DateTime::now().unix_timestamp() as usize,
        };
        sleep(Duration::from_secs(5));
        let token = j.create_token().unwrap();
        assert_eq!(JWTToken::verify( &token).unwrap(), j);
    }
}
