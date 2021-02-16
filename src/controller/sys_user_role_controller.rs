use actix_web::{web, Responder};

use crate::domain::dto::{IdDTO, UserRoleAddDTO, UserRoleEditDTO, UserRolePageDTO};
use crate::domain::vo::RespVO;
use crate::service::SYS_USER_ROLE_SERVICE;

/// 用户角色添加
pub async fn add(arg: web::Json<UserRoleAddDTO>) -> impl Responder {
    let vo = SYS_USER_ROLE_SERVICE.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户角色分页
pub async fn page(arg: web::Json<UserRolePageDTO>) -> impl Responder {
    let vo = SYS_USER_ROLE_SERVICE.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户角色修改
pub async fn update(arg: web::Json<UserRoleEditDTO>) -> impl Responder {
    let vo = SYS_USER_ROLE_SERVICE.edit(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户角色删除
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = SYS_USER_ROLE_SERVICE
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    return RespVO::from_result(&vo).resp_json();
}
