use ntex::web;
use ntex::web::Responder;
use crate::domain::dto::{
    EmptyDTO, IdDTO, SysRoleResAddDTO, SysRoleResPageDTO, SysRoleResUpdateDTO,
};
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

pub async fn add(arg: web::types::Json<SysRoleResAddDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_permission_service.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn page(arg: web::types::Json<SysRoleResPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_permission_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn layer_top(_arg: web::types::Json<EmptyDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_service.finds_layer().await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn update(arg: web::types::Json<SysRoleResUpdateDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_permission_service.edit(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn remove(arg: web::types::Json<IdDTO>) -> impl Responder {
    let role_id = arg.0.id.unwrap_or_default();
    let vo = CONTEXT.sys_role_permission_service.remove_role(&role_id).await;
    return RespVO::from_result(&vo).resp_json();
}
