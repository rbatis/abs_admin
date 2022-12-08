use crate::domain::dto::{EmptyDTO, IdDTO, ResAddDTO, ResEditDTO, ResPageDTO};
use crate::domain::table::SysRes;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;
use actix_web::{web, Responder};

pub async fn page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_res_service.page(&page.0).await;
    RespVO::from_result(&data).resp_json()
}

pub async fn all(page: web::Json<EmptyDTO>) -> impl Responder {
    let data = CONTEXT.sys_res_service.finds_all().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn layer_top(page: web::Json<EmptyDTO>) -> impl Responder {
    let data = CONTEXT.sys_res_service.finds_layer_top().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn add(mut arg: web::Json<ResAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error_info("资源名字不能为空!").resp_json();
    }
    if arg.permission.is_none() {
        return RespVO::<u64>::from_error_info("资源permission不能为空!").resp_json();
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = SysRes::from(arg.0);
    let data = CONTEXT.sys_res_service.add(&res).await;
    CONTEXT.sys_res_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn update(arg: web::Json<ResEditDTO>) -> impl Responder {
    let data = CONTEXT.sys_res_service.edit(&arg.0).await;
    CONTEXT.sys_res_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .sys_res_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    CONTEXT.sys_res_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}
