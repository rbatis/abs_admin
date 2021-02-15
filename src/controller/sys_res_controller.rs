use actix_web::{web, Responder};
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;

use crate::domain::domain::SysRes;
use crate::domain::dto::{IdDTO, ResAddDTO, ResEditDTO, ResPageDTO};
use crate::domain::vo::RespVO;
use crate::service::SYS_RES_SERVICE;

/// 资源分页(json请求)
pub async fn page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = SYS_RES_SERVICE.page(&page.0).await;
    RespVO::from_result(&data).resp_json()
}

/// 资源全部(json请求)
pub async fn all(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = SYS_RES_SERVICE.finds_all().await;
    RespVO::from_result(&data).resp_json()
}

///资源添加
pub async fn add(mut arg: web::Json<ResAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error_info("", "资源名字不能为空!").resp_json();
    }
    if arg.permission.is_none() {
        return RespVO::<u64>::from_error_info("", "资源permission不能为空!").resp_json();
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = SysRes {
        id: Some(
            rbatis::plugin::snowflake::async_snowflake_id()
                .await
                .to_string(),
        ),
        parent_id: arg.parent_id.clone(),
        name: arg.name.clone(),
        permission: arg.permission.clone(),
        path: arg.path.clone(),
        del: Some(0),
        create_date: Some(NaiveDateTime::now()),
    };
    let data = SYS_RES_SERVICE.add(&res).await;
    RespVO::from_result(&data).resp_json()
}

///资源修改
pub async fn update(arg: web::Json<ResEditDTO>) -> impl Responder {
    let data = SYS_RES_SERVICE.edit(&arg.0).await;
    RespVO::from_result(&data).resp_json()
}

///资源删除
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = SYS_RES_SERVICE
        .remove(&arg.0.id.unwrap_or("".to_string()))
        .await;
    RespVO::from_result(&data).resp_json()
}
