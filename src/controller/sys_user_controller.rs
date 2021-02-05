use actix_web::{web, HttpRequest, Responder};

use crate::config::CONFIG;
use crate::dao::RB;
use crate::domain::domain::SysUser;
use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserPageDTO};
use crate::domain::vo::{JWTToken, RespVO, SignInVO};
use crate::service::SYS_USER_SERVICE;
use actix_http::http::HeaderValue;
use rbatis::core::Error;
use rbatis::crud::CRUD;

/// 用户登陆
pub async fn login(arg: web::Json<SignInDTO>) -> impl Responder {
    log::info!("login:{:?}", arg.0);
    let vo = SYS_USER_SERVICE.sign_in(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 用户信息
pub async fn info(req: HttpRequest) -> impl Responder {
    let token = req.headers().get("access_token");
    match token {
        Some(token) => {
            let token = token.to_str().unwrap_or("");
            let token = JWTToken::verify(&CONFIG.jwt_secret, token);
            if token.is_err() {
                return RespVO::from_result(&token).resp_json();
            }
            let user_data = SYS_USER_SERVICE
                .get_user_info_by_token(&token.unwrap())
                .await;
            return RespVO::from_result(&user_data).resp_json();
        }
        _ => {
            return RespVO::<String>::from_error_info("access_token is empty!", "").resp_json();
        }
    }
}

/// 用户添加
pub async fn add(arg: web::Json<UserAddDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户分页
pub async fn page(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户修改
pub async fn update(arg: web::Json<UserEditDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE.edit(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户删除
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE
        .remove(&arg.0.id.unwrap_or("".to_string()))
        .await;
    return RespVO::from_result(&vo).resp_json();
}
