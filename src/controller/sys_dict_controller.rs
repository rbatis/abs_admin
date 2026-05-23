use crate::context::CONTEXT;
use crate::domain::dto::{DictAddDTO, DictEditDTO, DictPageDTO, IdDTO};
use crate::domain::table::sys_dict::SysDict;
use crate::domain::vo::RespVO;
use crate::error_info;
use actix_web::web;

pub async fn page(page: web::Json<DictPageDTO>) -> RespVO<serde_json::Value> {
    let data = CONTEXT.sys_dict_service.page(&page.0).await;
    RespVO::from_result(data.map(|v| serde_json::to_value(v).unwrap_or_default()))
}

pub async fn add(mut arg: web::Json<DictAddDTO>) -> RespVO<u64> {
    if arg.name.is_none() {
        return RespVO::from_error(error_info!("empty"));
    }
    if arg.code.is_none() {
        return RespVO::from_error(error_info!("empty"));
    }
    if arg.state.is_none() {
        arg.state = Some(1);
    }
    let res = SysDict::from(arg.0);
    let data = CONTEXT.sys_dict_service.add(&res).await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(data)
}

pub async fn update(arg: web::Json<DictEditDTO>) -> RespVO<u64> {
    let data = CONTEXT.sys_dict_service.edit(&arg.0).await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(data)
}

pub async fn remove(arg: web::Json<IdDTO>) -> RespVO<u64> {
    let data = CONTEXT
        .sys_dict_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    let _ = CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(data)
}
