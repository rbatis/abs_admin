use actix_web::{web, HttpRequest, Responder};

use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserRolePageDTO};
use crate::domain::vo::{JWTToken, RespVO, ServiceCode};
use crate::service::CONTEXT;

pub async fn login(arg: web::Json<SignInDTO>) -> impl Responder {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.sys_user_service.sign_in(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn info(req: HttpRequest) -> impl Responder {
    let token = req.headers().get("access_token");
    return match token {
        Some(token) => {
            let token = token.to_str().unwrap_or("");
            let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
            if token.is_err() {
                return RespVO::from_result(&token).resp_json();
            }
            let user_data = CONTEXT
                .sys_user_service
                .get_user_info_by_token(&token.unwrap())
                .await;
            RespVO::from_result(&user_data).resp_json()
        }
        _ => RespVO::<String>::from_code_info(ServiceCode::AccessTokenIsEmpty, "").resp_json(),
    };
}

pub async fn add(arg: web::Json<UserAddDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.add(arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn page(arg: web::Json<UserRolePageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_role_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn detail(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.detail(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn update(arg: web::Json<UserEditDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.edit(arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = CONTEXT
        .sys_user_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    return RespVO::from_result(&vo).resp_json();
}
