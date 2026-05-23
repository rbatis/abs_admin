use actix_web::web;
use actix_web::HttpRequest;

use crate::context::CONTEXT;
use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserRolePageDTO};
use crate::domain::vo::{JWTToken, RespVO, SignInVO};
use crate::error::Error;
use crate::error_info;
use crate::middleware::auth_actix::TOKEN_KEY;

pub async fn login(arg: web::Json<SignInDTO>) -> RespVO<SignInVO> {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.sys_user_service.sign_in(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn info(req: HttpRequest) -> RespVO<SignInVO> {
    let token = req.headers().get(TOKEN_KEY);
    match token {
        Some(token) => {
            let token = token.to_str().unwrap_or("");
            let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
            if let Err(e) = &token {
                return RespVO::from_error(e.to_string());
            }
            let user_data = CONTEXT
                .sys_user_service
                .get_user_info_by_token(&token.unwrap_or_default())
                .await;
            RespVO::from_result(user_data)
        }
        _ => RespVO::from_error(error_info!("access_token_empty")),
    }
}

pub async fn add(arg: web::Json<UserAddDTO>) -> RespVO<u64> {
    let vo = CONTEXT.sys_user_service.add(arg.0).await;
    RespVO::from_result(vo)
}

pub async fn page(arg: web::Json<UserRolePageDTO>) -> RespVO<serde_json::Value> {
    let vo = CONTEXT.sys_user_service.role_page(&arg.0).await;
    RespVO::from_result(vo.map(|v| serde_json::to_value(v).unwrap_or_default()))
}

pub async fn detail(arg: web::Json<IdDTO>) -> RespVO<serde_json::Value> {
    let vo = CONTEXT.sys_user_service.detail(&arg.0).await;
    RespVO::from_result(vo.map(|v| serde_json::to_value(v).unwrap_or_default()))
}

pub async fn update(arg: web::Json<UserEditDTO>) -> RespVO<u64> {
    if let (Some(account), Some(state)) = (arg.0.account.as_ref(), arg.0.state.as_ref()) {
        if account == "00000000000" && *state == 0 {
            return RespVO::from_result(Err(Error::from(error_info!("cannot_disable_admin"))));
        }
    }
    let vo = CONTEXT.sys_user_service.edit(arg.0).await;
    RespVO::from_result(vo)
}

pub async fn remove(arg: web::Json<IdDTO>) -> RespVO<u64> {
    let vo = CONTEXT
        .sys_user_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    RespVO::from_result(vo)
}
