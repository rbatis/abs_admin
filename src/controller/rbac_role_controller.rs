use crate::domain::dto::{IdDTO};
use crate::domain::vo::RespVO;
use crate::context::CONTEXT;
use actix_web::{web, HttpResponse, Responder};
use crate::domain::dto::rbac::{SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO};


pub async fn layer_top() -> impl Responder {
    let vo = CONTEXT.rbac_role_service.find_all().await;
    HttpResponse::Ok().json(RespVO::from_result(vo))
}

pub async fn add(arg: web::Json<SysRoleResAddDTO>) -> impl Responder {
    let vo = CONTEXT.rbac_role_permission_service.add(&arg.0).await;
    HttpResponse::Ok().json(RespVO::from_result(vo))
}

pub async fn page(arg: web::Json<SysRoleResPageDTO>) -> impl Responder {
    let vo = CONTEXT.rbac_role_permission_service.page(&arg.0).await;
    HttpResponse::Ok().json(RespVO::from_result(vo))
}

pub async fn update(arg: web::Json<SysRoleResUpdateDTO>) -> impl Responder {
    let vo = CONTEXT.rbac_role_permission_service.edit(&arg.0).await;
    HttpResponse::Ok().json(RespVO::from_result(vo))
}

pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let role_id = arg.0.id.unwrap_or_default();
    let vo = CONTEXT
        .rbac_role_permission_service
        .remove_role(&role_id)
        .await;
    HttpResponse::Ok().json(RespVO::from_result(vo))
}
