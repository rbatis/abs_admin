use std::ops::{Deref, DerefMut};
use crate::domain::vo::JWTToken;
use crate::error::Error;
use crate::middleware::auth::{checked_token};
use crate::context::CONTEXT;
use axum::{
    extract::Request,
    http,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

/// token key name
pub const TOKEN_KEY: &'static str = "Authorization";

pub async fn auth(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    if !CONTEXT.config.debug() {
        if let Ok(token) = get_token(&request.headers()) {
            if let Some(token) = token_is_valid(&token) {
                //Jwt resolution determines whether the expiration time is less than 10 minutes and automatically renews the contract.
                let now = rbatis::rbdc::DateTime::now().unix_timestamp() as usize;
                if (token.exp - now) < CONTEXT.config.jwt_refresh_token {
                    if let Ok(new_token) = token
                        .refresh(&CONTEXT.config.jwt_secret, CONTEXT.config.jwt_exp) {
                        if let Ok(new_header) = http::HeaderValue::from_str(&new_token) {
                            request.headers_mut().insert(
                                TOKEN_KEY,
                                new_header,
                            );
                        }
                    }
                }
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            }
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
    let response = next.run(request).await;
    Ok(response)
}

fn token_is_valid(token: &str) -> Option<JWTToken> {
    match checked_token(token) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

fn get_token(h: &HeaderMap) -> Result<&str, Error> {
    Ok(h.get(TOKEN_KEY)
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default())
}


///Put to Axum Handle
pub struct JwtAuth(pub JWTToken);

impl Deref for JwtAuth {
    type Target = JWTToken;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for JwtAuth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<S:Sync> FromRequestParts<S> for JwtAuth {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // take token
        if let Some(auth_header) = parts.headers.get(TOKEN_KEY) {
            if let Ok(auth_str) = auth_header.to_str() {
                match checked_token(auth_str) {
                    Ok(v) => Ok(JwtAuth(v)),
                    Err(e) => Err((
                        StatusCode::UNAUTHORIZED,
                        format!("Invalid authorization header={}", e),
                    )),
                }
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid authorization header".to_string(),
                ))
            }
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                "Authorization header missing".to_string(),
            ))
        }
    }
}

impl From<JwtAuth> for JWTToken {
    fn from(jwt: JwtAuth) -> Self {
        JWTToken {
            id: jwt.id.clone(),
            account: jwt.account.clone(),
            permissions: jwt.permissions.clone(),
            role_ids: jwt.role_ids.clone(),
            exp: jwt.exp,
        }
    }
}
