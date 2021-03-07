use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::domain::vo::RespVO;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::HeaderValue;
use actix_web::{error, Error};
use futures::future::{ok, Ready};
use futures::Future;
use crate::service::CONTEXT;

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
            if is_white_list_api(req.path().to_string()) || checked_token(token) {
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
        if path.contains(x) {
            return true;
        }
    }
    return false;
}

///校验token是否有效，未过期
fn checked_token(token: &HeaderValue) -> bool {
    //TODO check token alive
    return true;
}