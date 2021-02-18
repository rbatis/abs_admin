use actix_web::{web, Responder};

use crate::domain::domain::SysRoleRes;
use crate::domain::dto::{
    IdDTO, RoleAddDTO, RoleEditDTO, RolePageDTO, SysRoleResAddDTO, SysRoleResUpdateDTO,
};
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

/// 角色（关联资源）添加
pub async fn add(arg: web::Json<SysRoleResAddDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_res_service.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///角色分页
pub async fn page(arg: web::Json<RolePageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///角色（关联资源）修改
pub async fn update(arg: web::Json<SysRoleResUpdateDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_res_service.edit(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///角色删除
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let role_id = arg.0.id.unwrap_or_default();
    let vo = CONTEXT
        .sys_role_res_service
        .remove_role(&role_id)
        .await;
    return RespVO::from_result(&vo).resp_json();
}
