use crate::domain::dto::{IdDTO, SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO};
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;
use actix_web::{web, Responder};

/// 角色（关联资源）添加
pub async fn add(arg: web::Json<SysRoleResAddDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_res_service.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///角色分页
pub async fn page(arg: web::Json<SysRoleResPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_res_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///角色详情
pub async fn detail(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_res_service.detail(&arg.0).await;
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
    let vo = CONTEXT.sys_role_res_service.remove_role(&role_id).await;
    return RespVO::from_result(&vo).resp_json();
}
