use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::domain::vo::{RespVO, JWTToken};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::HeaderValue;
use actix_web::{error, Error};
use futures::future::{ok, Ready};
use futures::Future;
use crate::service::CONTEXT;
use chrono::{NaiveDateTime, Duration};
use rbatis::core::value::DateTimeNow;
use std::ops::Sub;

// custom request auth middleware
pub struct Auth;

impl<S, B> Transform<S> for Auth
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();

        Box::pin(async move {
            let value = HeaderValue::from_str("").unwrap();
            let token = req.headers().get("access_token").unwrap_or(&value);
            let is_white_list_api = is_white_list_api(req.path().to_string());
            let mut is_checked_token = false;
            if !is_white_list_api {
                //非白名单检查token是否有效
                match checked_token(token).await {
                    Ok(data) => {
                        is_checked_token = data;
                    }
                    Err(e) => {
                        let resp: RespVO<String> = RespVO {
                            code: Some("-1".to_string()),
                            msg: Some(format!("Unauthorized for:{}", e.to_string())),
                            data: None,
                        };
                        return Err(error::ErrorUnauthorized(
                            serde_json::json!(&resp).to_string(),
                        ));
                    }
                }
            }
            if is_white_list_api || is_checked_token {
                let resp = svc.call(req).await?;
                Ok(resp)
            } else {
                let resp: RespVO<String> = RespVO {
                    code: Some("-1".to_string()),
                    msg: Some("Unauthorized".to_string()),
                    data: None,
                };
                Err(error::ErrorUnauthorized(
                    serde_json::json!(&resp).to_string(),
                ))
            }
        })
    }
}

///是否处在白名单接口中
fn is_white_list_api(path: String) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if x.contains(&path) {
            return true;
        }
    }
    return false;
}

///校验token是否有效，未过期
async fn checked_token(token: &HeaderValue) -> Result<bool, crate::error::Error> {
    //check token alive
    let token_value = token.to_str().unwrap_or("");
    if CONTEXT.config.debug{
        log::info!("[abs_admin] token:{}",token_value);
    }
    let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token_value);
    match token {
        Ok(token) => {
            let token_create_time = CONTEXT.redis_service.get_string(&format!("login:token:{}", token.account)).await?;
            let time = NaiveDateTime::parse_from_str(&token_create_time,"%Y-%m-%dT%H:%M:%S");
            match time {
                Ok(time) => {
                    let sub = NaiveDateTime::now().sub(time);
                    if sub.gt(&Duration::milliseconds(token.exp as i64)) {
                        return Ok(false);
                    }
                    return Ok(true);
                }
                Err(e) => {
                    log::error!("[abs_admin] parse token.exp error:{}", e.to_string());
                    return Ok(false);
                }
            }
        }
        _ => {
            return Ok(false);
        }
    }
}