use crate::domain::dto::{DictAddDTO, DictEditDTO, DictPageDTO, IdDTO};
use crate::domain::vo::RespVO;
use crate::error_info;
use crate::context::CONTEXT;
use actix_web::{web, HttpResponse, Responder};
use crate::domain::table::sys_dict::SysDict;

pub async fn page(page: web::Json<DictPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.page(&page.0).await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}

pub async fn add(mut arg: web::Json<DictAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return HttpResponse::Ok().json(RespVO::<u64>::from_error(error_info!("empty")));
    }
    if arg.code.is_none() {
        return HttpResponse::Ok().json(RespVO::<u64>::from_error(error_info!("empty")));
    }
    if arg.state.is_none() {
        arg.state = Some(1);
    }
    let res = SysDict::from(arg.0);
    let data = CONTEXT.sys_dict_service.add(&res).await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}

pub async fn update(arg: web::Json<DictEditDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.edit(&arg.0).await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}

pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .sys_dict_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}
