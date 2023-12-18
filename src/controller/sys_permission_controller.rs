use crate::domain::dto::{EmptyDTO, IdDTO, PermissionAddDTO, ResEditDTO, ResPageDTO};
use crate::domain::table::SysPermission;
use crate::domain::vo::RespVO;
use crate::error_info;
use crate::service::CONTEXT;
use actix_web::{web, Responder};

pub async fn page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_permission_service.page(&page.0).await;
    RespVO::from_result(&data).resp_json()
}

pub async fn all(_page: web::Json<EmptyDTO>) -> impl Responder {
    let data = CONTEXT.sys_permission_service.finds_all().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn layer_top(_page: web::Json<EmptyDTO>) -> impl Responder {
    let data = CONTEXT.sys_permission_service.finds_layer_top().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn add(mut arg: web::Json<PermissionAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error_info("arg.name_empty", &error_info!("arg.name_empty"))
            .resp_json();
    }
    if arg.permission.is_none() {
        return RespVO::<u64>::from_error_info(
            "arg.permission_empty",
            &error_info!("arg.permission_empty"),
        )
        .resp_json();
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = SysPermission::from(arg.0);
    let data = CONTEXT.sys_permission_service.add(&res).await;
    let _ = CONTEXT.sys_permission_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn update(arg: web::Json<ResEditDTO>) -> impl Responder {
    let data = CONTEXT.sys_permission_service.edit(&arg.0).await;
    let _ = CONTEXT.sys_permission_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .sys_permission_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    let _ = CONTEXT.sys_permission_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}
