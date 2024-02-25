use crate::domain::vo::RespVO;
use crate::middleware::auth::{check_auth, checked_token, is_white_list_api};
use crate::service::CONTEXT;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use actix_web::body::BoxBody;

pub struct Auth;

impl<S: 'static> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
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

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    #[inline]
    fn poll_ready(
        &self,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Into::into)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        let token = req
            .headers()
            .get("access_token")
            .map(|v| v.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        let path = req.path().to_string();
        // let fut = srv.call(req);
        let mut refresh_token = token.clone();
        Box::pin(async move {
            //debug mode not enable auth
            if !CONTEXT.config.debug {
                if !is_white_list_api(&path) {
                    match checked_token(&token, &path).await {
                        Ok(data) => match check_auth(&data, &path).await {
                            Ok(_) => {
                                //Jwt resolution determines whether the expiration time is less than 10 minutes and automatically renews the contract.
                                let now = rbatis::rbdc::DateTime::now().unix_timestamp() as usize;
                                if (data.exp - now) < CONTEXT.config.jwt_refresh_token {
                                    refresh_token = data
                                        .refresh(&CONTEXT.config.jwt_secret, CONTEXT.config.jwt_exp)
                                        .unwrap();
                                }
                            }
                            Err(e) => {
                                let resp: RespVO<String> =
                                    RespVO::from_error(e.to_string());
                                return Ok(req.into_response(resp.resp_json()));
                            }
                        },
                        Err(e) => {
                            //401 http code will exit login
                            let resp: RespVO<String> = RespVO::from_error(format!("Unauthorized for:{}", e.to_string()));
                            return Err(ErrorUnauthorized(serde_json::json!(&resp).to_string()));
                        }
                    }
                }
            }
            let mut res = svc.call(req).await?;
            res.response_mut().headers_mut().insert(
                HeaderName::from_static("access_token"),
                HeaderValue::from_str(refresh_token.as_str()).unwrap(),
            );
            Ok(res)
        })
    }
}
