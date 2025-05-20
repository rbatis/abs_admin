use std::future::{ready, Ready};
use std::ops::{Deref, DerefMut};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;

use crate::context::CONTEXT;
use crate::domain::vo::JWTToken;
use crate::middleware::auth::{checked_token, is_white_list_api};

/// token key name
pub const TOKEN_KEY: &str = "Authorization";

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let debug = CONTEXT.config.debug;
        
        // Early return for debug mode or whitelisted paths
        if debug || is_white_list_api(&path) {
            return Box::pin(self.service.call(req));
        }
        
        // Check if token exists
        let token_opt = req.headers().get(TOKEN_KEY).and_then(|token_header| {
            token_header.to_str().ok().and_then(|token_str| token_is_valid(token_str))
        });
        
        // If no token or invalid token
        if token_opt.is_none() {
            return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))));
        }
        
        // We have a valid token
        let token_data = token_opt.unwrap();
        
        // Check if token needs refresh
        let now = rbatis::rbdc::DateTime::now().unix_timestamp() as usize;
        if (token_data.exp - now) < CONTEXT.config.jwt_refresh_token {
            // Try to refresh token
            if let Ok(new_token) = token_data.refresh(
                &CONTEXT.config.jwt_secret, 
                CONTEXT.config.jwt_exp
            ) {
                if let Ok(new_header) = HeaderValue::from_str(&new_token) {
                    // Add the refreshed token to the request
                    req.headers_mut().insert(
                        HeaderName::from_static(TOKEN_KEY),
                        new_header,
                    );
                }
            }
        }
        
        // Continue with the modified request
        Box::pin(self.service.call(req))
    }
}

fn token_is_valid(token: &str) -> Option<JWTToken> {
    match checked_token(token) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

// JwtAuth for extracting from requests
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
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get(TOKEN_KEY) {
            if let Ok(auth_str) = auth_header.to_str() {
                match checked_token(auth_str) {
                    Ok(token) => ready(Ok(JwtAuth(token))),
                    Err(e) => ready(Err(actix_web::error::ErrorUnauthorized(e.to_string()))),
                }
            } else {
                ready(Err(actix_web::error::ErrorUnauthorized(
                    "Invalid authorization header",
                )))
            }
        } else {
            ready(Err(actix_web::error::ErrorUnauthorized(
                "Authorization header missing",
            )))
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