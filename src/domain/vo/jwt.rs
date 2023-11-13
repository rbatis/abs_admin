use crate::error::Error;
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
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error> {
        let mut validation = Validation::default();
        validation.leeway=0;
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
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

    pub fn refresh(&self, secret: &str, jwt_exp: usize) -> Result<String, Error> {
        let mut jwt = self.clone();
        jwt.exp = jwt.exp + jwt_exp;
        jwt.create_token(&secret)
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
        let token = j.create_token("ssss").unwrap();
        assert_eq!(JWTToken::verify("ssss", &token).unwrap(), j);
    }
}
