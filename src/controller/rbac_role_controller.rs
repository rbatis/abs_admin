use crate::context::CONTEXT;
use crate::domain::dto::rbac::{SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO};
use crate::domain::dto::IdDTO;
use crate::domain::vo::RespVO;
use actix_web::web;
use serde::Serialize;

pub async fn layer_top() -> RespVO<impl Serialize> {
    let vo = CONTEXT.rbac_role_service.find_all().await;
    RespVO::from_result(vo)
}

pub async fn add(arg: web::Json<SysRoleResAddDTO>) -> RespVO<impl Serialize> {
    let vo = CONTEXT.rbac_role_permission_service.add(&arg.0).await;
   RespVO::from_result(vo)
}

pub async fn page(arg: web::Json<SysRoleResPageDTO>) -> RespVO<impl Serialize> {
    let vo = CONTEXT.rbac_role_permission_service.page(&arg.0).await;
   RespVO::from_result(vo)
}

pub async fn update(arg: web::Json<SysRoleResUpdateDTO>) -> RespVO<u64> {
    let vo = CONTEXT.rbac_role_permission_service.edit(&arg.0).await;
    RespVO::from_result(vo)
}

pub async fn remove(arg: web::Json<IdDTO>) -> RespVO<u64> {
    let role_id = arg.0.id.unwrap_or_default();
    let vo = CONTEXT
        .rbac_role_permission_service
        .remove_role(&role_id)
        .await;
    RespVO::from_result(vo)
}
