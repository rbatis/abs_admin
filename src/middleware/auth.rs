use std::cell::RefCell;
use std::future::ready;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use actix_http::header;
use actix_http::header::HeaderValue;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error;
use actix_web::error::ErrorUnauthorized;
use futures::future::{ok, Ready};
use futures::Future;
use reqwest::Body;

use crate::domain::vo::{JWTToken, RespVO};
use actix_web::{Error};
use crate::service::CONTEXT;
pub struct Auth;

//
// pub struct AuthMiddleware<S> {
//     service: Rc<RefCell<S>>,
// }
//
// impl<S> Service<ServiceRequest> for AuthMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<Body>, Error = Error> + 'static,
//     S::Future: 'static,
// {
//     type Response = ServiceResponse<Body>;
//     type Error = <S as actix_web::dev::Service<ServiceRequest>>::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }
//
//     fn call(&mut self, req: ServiceRequest) -> Self::Future {
//         let mut svc = self.service.clone();
//
//         Box::pin(async move {
//             let value = HeaderValue::from_str("").unwrap();
//             let token = req.headers().get("access_token").unwrap_or(&value);
//             let path = req.path().to_string();
//             if !is_white_list_api(&path) {
//                 //非白名单检查token是否有效
//                 let token_value = token.to_str().unwrap_or("");
//                 match checked_token(token_value, &path).await {
//                     Ok(data) => {
//                         match check_auth(&data, &path).await {
//                             Ok(_) => {}
//                             Err(e) => {
//                                 //仅提示拦截
//                                 let resp: RespVO<String> = RespVO {
//                                     code: Some("-1".to_string()),
//                                     msg: Some(format!("无权限访问:{}", e.to_string())),
//                                     data: None,
//                                 };
//                                 return Ok(req.into_response(resp.resp_json()));
//                             }
//                         }
//                     }
//                     Err(e) => {
//                         //401 http状态码会强制前端退出当前登陆状态
//                         let resp: RespVO<String> = RespVO {
//                             code: Some("-1".to_string()),
//                             msg: Some(format!("Unauthorized for:{}", e.to_string())),
//                             data: None,
//                         };
//                         return Err(error::ErrorUnauthorized(serde_json::json!(&resp).to_string()));
//                     }
//                 }
//             }
//             //调用接口服务
//             let resp = svc.call(req).await?;
//             Ok(resp)
//         })
//     }
// }

///是否处在白名单接口中
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if x.contains(path) {
            return true;
        }
    }
    return false;
}

///校验token是否有效，未过期
pub async fn checked_token(token: &str, path: &str) -> Result<JWTToken, crate::error::Error> {
    //check token alive
    let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match token {
        Ok(token) => {
            return Ok(token);
        }
        Err(e) => {
            return Err(crate::error::Error::from(e.to_string()));
        }
    }
}

///权限校验
pub async fn check_auth(token: &JWTToken, path: &str) -> Result<(), crate::error::Error> {
    let sys_res = CONTEXT.sys_res_service.finds_all().await?;
    //权限校验
    for token_permission in &token.permissions {
        for x in &sys_res {
            match &x.permission {
                Some(permission) => match &x.path {
                    None => {}
                    Some(x_path) => {
                        if permission.eq(token_permission) && path.contains(x_path) {
                            return Ok(());
                        }
                    }
                },
                _ => {}
            }
        }
    }
    return Err(crate::error::Error::from("无权限访问!"));
}
