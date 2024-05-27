use crate::middleware::auth::is_white_list_api;
use crate::service::CONTEXT;
use crate::{domain::vo::JWTToken, middleware::auth::check_auth};
use axum::{extract::Request, http, http::StatusCode, middleware::Next, response::Response};

pub async fn auth(jwt: Option<JWTToken>, mut request: Request, next: Next) -> Result<Response, StatusCode> {
    log::info!("middleware-auth path: {:?} jwt: {:?}", request.uri(), jwt);
    let path = request.uri().path().to_string();
    if !is_white_list_api(&path) {
        match jwt {
            Some(mut token) => {
                let now = rbatis::rbdc::DateTime::now().unix_timestamp() as usize;
                log::info!("check_auth now: {}, exp: {}", now, token.exp-now);
                if (token.exp - now) < 1 {
                    log::info!("check_auth expired");
                    return Err(StatusCode::UNAUTHORIZED);
                }
                if check_auth(&token, &path).await.is_err() {
                    log::info!("check_auth is_err");
                    return Err(StatusCode::UNAUTHORIZED);
                }
                //Jwt resolution determines whether the expiration time is less than 10 minutes and automatically renews the contract.
                if (token.exp - now) < CONTEXT.config.jwt_refresh_token {
                    let new_token = token.refresh(CONTEXT.config.jwt_exp).unwrap();
                    request
                        .headers_mut()
                        .insert("access_token", http::HeaderValue::from_str(&new_token).unwrap());
                   
                }
                if path == "/admin/sys_user_info" {
                    request.extensions_mut().insert(token);
                }
                
            },
            
            None => {
                return Err(StatusCode::UNAUTHORIZED)},
        };
      
        
    }
    let response = next.run(request).await;
    Ok(response)
}

