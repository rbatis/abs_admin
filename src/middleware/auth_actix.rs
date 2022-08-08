use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::BytesMut,
    Error, HttpMessage,
};
use futures_util::{future::LocalBoxFuture, stream::StreamExt};

pub struct Auth;

impl<S: 'static, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
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

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        //let token = req.headers().get("access_token").map(|v|v.to_str().unwrap_or_default().to_string()).unwrap_or_default();
        //                 let path = req.path().to_string();
        //                 let fut = srv.call(req);
        //                 Box::pin(async move {
        //                     //debug mode not enable auth
        //                     if !CONTEXT.config.debug{
        //                         if !is_white_list_api(&path) {
        //                             //非白名单检查token是否有效
        //                             match checked_token(&token, &path).await {
        //                                 Ok(data) => {
        //                                     match check_auth(&data, &path).await {
        //                                         Ok(_) => {}
        //                                         Err(e) => {
        //                                             //仅提示拦截
        //                                             let resp: RespVO<String> = RespVO {
        //                                                 code: Some("-1".to_string()),
        //                                                 msg: Some(format!("无权限访问:{}", e.to_string())),
        //                                                 data: None,
        //                                             };
        //                                             return Err(ErrorUnauthorized(serde_json::json!(&resp).to_string()));
        //                                         }
        //                                     }
        //                                 }
        //                                 Err(e) => {
        //                                     //401 http状态码会强制前端退出当前登陆状态
        //                                     let resp: RespVO<String> = RespVO {
        //                                         code: Some("-1".to_string()),
        //                                         msg: Some(format!("Unauthorized for:{}", e.to_string())),
        //                                         data: None,
        //                                     };
        //                                     return Err(ErrorUnauthorized(serde_json::json!(&resp).to_string()));
        //                                 }
        //                             }
        //                         }
        //                     }
        //                     let res = fut.await?;
        //                     Ok(res)
        //                 })

        let svc = self.service.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            println!("request body: {body:?}");
            let res = svc.call(req).await?;

            println!("response: {:?}", res.headers());
            Ok(res)
        })
    }
}