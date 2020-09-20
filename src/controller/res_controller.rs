use actix_web::{Responder, web};
use chrono::NaiveDateTime;
use rbatis_core::value::DateTimeNow;
use uuid::Uuid;

use crate::domain::domain::SysRes;
use crate::domain::dto::{ResAddDTO, ResPageDTO, IdDTO, ResEditDTO};
use crate::domain::vo::RespVO;
use crate::service::SYS_RES_SERVICE;

/// 资源分页(json请求)
pub async fn page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = SYS_RES_SERVICE.page(&page.0).await;
    RespVO::from_result(&data).resp()
    // REDIS_SERVICE.put_json("res_page", &data.as_ref().unwrap().to_string()).await;
    // let cached_res: String = REDIS_SERVICE.get_json("res_page").await.unwrap();
}

///资源添加
pub async fn add(mut arg: web::Json<ResAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error_info("", "资源名字不能为空!").resp();
    }
    if arg.permission.is_none() {
        return RespVO::<u64>::from_error_info("", "资源permission不能为空!").resp();
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = SysRes {
        id: Some(Uuid::new_v4().to_string()),
        parent_id: arg.parent_id.clone(),
        name: arg.name.clone(),
        permission: arg.permission.clone(),
        path: arg.path.clone(),
        del: Some(1),
        create_time: Some(NaiveDateTime::now()),
        childs: None
    };
    let data = SYS_RES_SERVICE.add(&res).await;
    RespVO::from_result(&data).resp()
}

///资源修改
pub async fn edit(arg: web::Json<ResEditDTO>) -> impl Responder {
    let data = SYS_RES_SERVICE.edit(&arg.0).await;
    RespVO::from_result(&data).resp()
}


///资源删除
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = SYS_RES_SERVICE.remove(&arg.0.id.unwrap_or("".to_string())).await;
    RespVO::from_result(&data).resp()
}