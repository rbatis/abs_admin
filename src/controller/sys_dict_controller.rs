use actix_web::{web, Responder};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;

use crate::domain::domain::SysDict;
use crate::domain::dto::{DictAddDTO, DictEditDTO, DictPageDTO, EmptyDTO, IdDTO};
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;
use rbatis::plugin::snowflake::new_snowflake_id;

/// 字典分页(json请求)
pub async fn page(page: web::Json<DictPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.page(&page.0).await;
    RespVO::from_result(&data).resp_json()
}

//字典添加
pub async fn add(mut arg: web::Json<DictAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error_info("", "字典名字不能为空!").resp_json();
    }
    if arg.code.is_none() {
        return RespVO::<u64>::from_error_info("", "字典code不能为空!").resp_json();
    }
    if arg.state.is_none() {
        arg.state = Some(1);
    }
    let res = SysDict {
        id: arg.name.clone().into(),
        name: arg.name.clone(),
        code: arg.code.clone(),
        state: arg.state.clone(),
        create_date: NaiveDateTime::now().into(),
    };
    let data = CONTEXT.sys_dict_service.add(&res).await;
    CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

///字典修改
pub async fn update(arg: web::Json<DictEditDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.edit(&arg.0).await;
    CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

///字典删除
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .sys_dict_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}
