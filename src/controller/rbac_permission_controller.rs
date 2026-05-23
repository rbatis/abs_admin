use crate::context::CONTEXT;
use crate::domain::dto::rbac::{PermissionAddDTO, PermissionPageDTO, ResEditDTO};
use crate::domain::dto::IdDTO;
use crate::domain::table::rbac::RbacPermission;
use crate::domain::vo::RespVO;
use crate::error_info;
use actix_web::web;
use serde::Serialize;

pub async fn layer_top() -> RespVO<impl Serialize> {
    let data = CONTEXT.rbac_permission_service.finds_all().await;
    RespVO::from_result(data)
}

pub async fn page(page: web::Json<PermissionPageDTO>) -> RespVO<impl Serialize> {
    let data = CONTEXT.rbac_permission_service.page(&page.0).await;
    RespVO::from_result(data)
}

pub async fn add(mut arg: web::Json<PermissionAddDTO>) -> RespVO<u64> {
    if arg.name.is_none() {
        return RespVO::from_error(error_info!("arg.name_empty"));
    }
    if arg.permission.is_none() {
        return RespVO::from_error(error_info!("arg.permission_empty"));
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = RbacPermission::from(arg.0);
    let data = CONTEXT.rbac_permission_service.add(&res).await;
    RespVO::from_result(data)
}

pub async fn update(arg: web::Json<ResEditDTO>) -> RespVO<u64> {
    let data = CONTEXT.rbac_permission_service.edit(&arg.0).await;
    RespVO::from_result(data)
}

pub async fn remove(arg: web::Json<IdDTO>) -> RespVO<u64> {
    let data = CONTEXT
        .rbac_permission_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    RespVO::from_result(data)
}
