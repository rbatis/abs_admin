use crate::domain::dto::{IdDTO};
use crate::domain::vo::RespVO;
use crate::context::CONTEXT;
use axum::response::IntoResponse;
use axum::Json;
use crate::domain::dto::rbac::{SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO};


pub async fn layer_top() -> impl IntoResponse {
    let vo = CONTEXT.rbac_role_service.find_all().await;
    RespVO::from_result(vo)
}

pub async fn add(arg: Json<SysRoleResAddDTO>) -> impl IntoResponse {
    let vo = CONTEXT.rbac_role_permission_service.add(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn page(arg: Json<SysRoleResPageDTO>) -> impl IntoResponse {
    let vo = CONTEXT.rbac_role_permission_service.page(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn update(arg: Json<SysRoleResUpdateDTO>) -> impl IntoResponse {
    let vo = CONTEXT.rbac_role_permission_service.edit(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn remove(arg: Json<IdDTO>) -> impl IntoResponse {
    let role_id = arg.0.id.unwrap_or_default();
    let vo = CONTEXT
        .rbac_role_permission_service
        .remove_role(&role_id)
        .await;
    RespVO::from_result(vo)
}
