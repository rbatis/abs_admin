use axum::response::IntoResponse;
use axum::Json;

use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserRolePageDTO};
use crate::domain::vo::{JWTToken, RespVO};
use crate::error_info;
use crate::service::CONTEXT;
use log::info;

pub async fn login(arg: Json<SignInDTO>) -> impl IntoResponse {
    info!("user login: {:?}", arg.0);
    let vo = CONTEXT.sys_user_service.sign_in(&arg.0).await;
   
    RespVO::from_result(vo)
}

pub async fn info(token: JWTToken) -> impl IntoResponse {
    info!("user info: {:?}", token);
    let user_data = CONTEXT
            .sys_user_service
            .get_user_info_by_token(&token)
            .await;
    RespVO::from_result(user_data)
}

pub async fn add(arg: Json<UserAddDTO>) -> impl IntoResponse {
    info!("user add: {:?}", arg.0);
    let vo = CONTEXT.sys_user_service.add(arg.0).await;
    RespVO::from_result(vo)
}

pub async fn page(arg: Json<UserRolePageDTO>) -> impl IntoResponse {
    info!("user page: {:?}", arg.0);
    let vo = CONTEXT.sys_user_role_service.page(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn detail(arg: Json<IdDTO>) -> impl IntoResponse {
    info!("user detail: {:?}", arg.0);
    let vo = CONTEXT.sys_user_service.detail(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn update(arg: Json<UserEditDTO>) -> impl IntoResponse {
    info!("user update: {:?}", arg.0);
    if let (Some(account), Some(state)) = (arg.0.account.as_ref(), arg.0.state.as_ref()) {
        if account == "00000000000" && *state == 0 {
            return RespVO::<u64>::from_result(Err(error_info!(
                "cannot_disable_admin"
            )))
            ;
        }
    }
    let vo = CONTEXT.sys_user_service.edit(arg.0).await;
    RespVO::from_result(vo)
}

pub async fn remove(arg: Json<IdDTO>) -> impl IntoResponse {
    info!("user remove: {:?}", arg.0);
    let vo = CONTEXT
        .sys_user_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    RespVO::from_result(vo)
}
