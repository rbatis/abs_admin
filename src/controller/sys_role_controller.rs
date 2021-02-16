use actix_web::{web, Responder};

use crate::domain::dto::{IdDTO, RoleAddDTO, RoleEditDTO, RolePageDTO};
use crate::domain::vo::RespVO;
use crate::service::SYS_ROLE_SERVICE;

/// 角色添加
pub async fn add(arg: web::Json<RoleAddDTO>) -> impl Responder {
    let vo = SYS_ROLE_SERVICE.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///角色分页
pub async fn page(arg: web::Json<RolePageDTO>) -> impl Responder {
    let vo = SYS_ROLE_SERVICE.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///角色修改
pub async fn update(arg: web::Json<RoleEditDTO>) -> impl Responder {
    let vo = SYS_ROLE_SERVICE.edit(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///角色删除
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let role_id = arg.0.id.unwrap_or_default();
    let vo = SYS_ROLE_SERVICE.remove_role_relation(&role_id).await;
    return RespVO::from_result(&vo).resp_json();
}
