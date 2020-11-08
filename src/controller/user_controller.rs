use actix_web::{Responder, web};

use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserPageDTO};
use crate::domain::vo::RespVO;
use crate::service::SYS_USER_SERVICE;

/// 用户登陆
pub async fn login(arg: web::Json<SignInDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE.sign_in(&arg.0).await;
    return RespVO::from_result(&vo).resp();
}


/// 用户添加
pub async fn add(arg: web::Json<UserAddDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE.add(&arg.0).await;
    return RespVO::from_result(&vo).resp();
}

///用户分页
pub async fn page(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE.page(&arg.0).await;
    return RespVO::from_result(&vo).resp();
}

///用户修改
pub async fn edit(arg: web::Json<UserEditDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE.edit(&arg.0).await;
    return RespVO::from_result(&vo).resp();
}

///用户删除
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = SYS_USER_SERVICE.remove(&arg.0.id.unwrap_or("".to_string())).await;
    return RespVO::from_result(&vo).resp();
}