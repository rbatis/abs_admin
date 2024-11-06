use crate::domain::vo::JWTToken;
use crate::error::Error;
use crate::middleware::auth::{checked_token, is_white_list_api};
use crate::context::CONTEXT;
use axum::{
    extract::Request,
    http,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// token key name
pub const TOKEN_KEY: &'static str = "access_token";

pub async fn auth(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    if !CONTEXT.config.debug {
        if !is_white_list_api(&path) {
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
