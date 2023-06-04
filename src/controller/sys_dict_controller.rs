use ntex::web;
use ntex::web::Responder;
use crate::domain::dto::{DictAddDTO, DictEditDTO, DictPageDTO, IdDTO};
use crate::domain::table::SysDict;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

pub async fn page(page: web::types::Json<DictPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.page(&page.0).await;
    RespVO::from_result(&data).resp_json()
}

pub async fn add(mut arg: web::types::Json<DictAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error_info("empty", "字典名字不能为空").resp_json();
    }
    if arg.code.is_none() {
        return RespVO::<u64>::from_error_info("empty", "字典code不能为空").resp_json();
    }
    if arg.state.is_none() {
        arg.state = Some(1);
    }
    let res = SysDict::from(arg.0);
    let data = CONTEXT.sys_dict_service.add(&res).await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn update(arg: web::types::Json<DictEditDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.edit(&arg.0).await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

pub async fn remove(arg: web::types::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .sys_dict_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}
