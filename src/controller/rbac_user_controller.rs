use axum::extract::Request;
use axum::response::IntoResponse;
use axum::Json;

use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserRolePageDTO};
use crate::domain::vo::{JWTToken, RespVO, SignInVO};
use crate::error::Error;
use crate::error_info;
use crate::context::CONTEXT;
use crate::middleware::auth_axum::TOKEN_KEY;

pub async fn login(arg: Json<SignInDTO>) -> impl IntoResponse {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.sys_user_service.sign_in(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn info(req: Request) -> impl IntoResponse {
    let token = req.headers().get(TOKEN_KEY);
    match token {
        Some(token) => {
            let token = token.to_str().unwrap_or("");
            let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
            if token.is_err() {
                return RespVO::<SignInVO>::from_error(token.err().unwrap().to_string());
            }
            let user_data = CONTEXT
                .sys_user_service
                .get_user_info_by_token(&token.unwrap())
                .await;
            RespVO::from_result(user_data)
        }
        _ => RespVO::<SignInVO>::from_error(error_info!("access_token_empty")),
    }
}

pub async fn add(arg: Json<UserAddDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_user_service.add(arg.0).await;
    RespVO::from_result(vo)
}

pub async fn page(arg: Json<UserRolePageDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_user_service.role_page(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn detail(arg: Json<IdDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_user_service.detail(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn update(arg: Json<UserEditDTO>) -> impl IntoResponse {
    if let (Some(account), Some(state)) = (arg.0.account.as_ref(), arg.0.state.as_ref()) {
        if account == "00000000000" && *state == 0 {
            return RespVO::<u64>::from_result(Err(Error::from(error_info!(
                "cannot_disable_admin"
            ))))
            ;
        }
    }
    let vo = CONTEXT.sys_user_service.edit(arg.0).await;
    RespVO::from_result(vo)
}

pub async fn remove(arg: Json<IdDTO>) -> impl IntoResponse {
    let vo = CONTEXT
        .sys_user_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    RespVO::from_result(vo)
}
