use crate::domain::dto::{IdDTO};
use crate::domain::table::rbac::RbacPermission;
use crate::domain::vo::RespVO;
use crate::error_info;
use crate::context::CONTEXT;
use actix_web::{web, HttpResponse, Responder};
use crate::domain::dto::rbac::{PermissionAddDTO, ResEditDTO, ResPageDTO};


pub async fn layer_top() -> impl Responder {
    let data = CONTEXT.rbac_permission_service.finds_all().await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}

pub async fn page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = CONTEXT.rbac_permission_service.page(&page.0).await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}

pub async fn add(mut arg: web::Json<PermissionAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return HttpResponse::Ok().json(RespVO::<u64>::from_error(error_info!("arg.name_empty")));
    }
    if arg.permission.is_none() {
        return HttpResponse::Ok().json(RespVO::<u64>::from_error(error_info!("arg.permission_empty")));
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = RbacPermission::from(arg.0);
    let data = CONTEXT.rbac_permission_service.add(&res).await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}

pub async fn update(arg: web::Json<ResEditDTO>) -> impl Responder {
    let data = CONTEXT.rbac_permission_service.edit(&arg.0).await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}

pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .rbac_permission_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    HttpResponse::Ok().json(RespVO::from_result(data))
}
