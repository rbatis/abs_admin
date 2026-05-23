use std::future::{ready, Ready};
use crate::context::CONTEXT;
use crate::domain::vo::JWTToken;
use crate::error::Error;
use crate::middleware::auth::checked_token;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::{HeaderMap, HeaderName};
use actix_web::middleware::Next;
use futures_util::future::LocalBoxFuture;
use std::ops::{Deref, DerefMut};
use actix_web::{FromRequest, HttpRequest};

pub const TOKEN_KEY: &'static str = "Authorization";

pub fn auth<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> LocalBoxFuture<'static, Result<ServiceResponse<B>, actix_web::Error>>
where
    B: MessageBody + 'static,
{
    Box::pin(async move {
        if req.path().starts_with("/admin/") && !CONTEXT.config.debug() {
            if let Ok(token_str) = get_token(&req.headers()) {
                if let Some(token) = token_is_valid(token_str) {
                    let now = rbatis::rbdc::DateTime::now().unix_timestamp() as usize;
                    if (token.exp - now) < CONTEXT.config.jwt_refresh_token {
                        if let Ok(new_token) =
                            token.refresh(&CONTEXT.config.jwt_secret, CONTEXT.config.jwt_exp)
                        {
                            if let Ok(new_header) =
                                actix_web::http::header::HeaderValue::from_str(&new_token)
                            {
                                let mut res = next.call(req).await?;
                                res.headers_mut().insert(
                                    HeaderName::from_static(TOKEN_KEY),
                                    new_header,
                                );
                                return Ok(res);
                            }
                        }
                    }
                    return next.call(req).await;
                }
                return Err(ErrorUnauthorized("Unauthorized"));
            }
            return Err(ErrorUnauthorized("Unauthorized"));
        }
        next.call(req).await
    })
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

impl FromRequest for JwtAuth {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());
        match auth_header {
            Some(auth_str) => {
                if let Ok(jwt)= checked_token(auth_str) {
                    ready(Ok(JwtAuth(jwt)))
                } else {
                    ready(Err(ErrorUnauthorized("Invalid authorization format")))
                }
            }
            None => ready(Err(ErrorUnauthorized("Missing authorization header"))),
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
