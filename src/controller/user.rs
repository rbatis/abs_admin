use actix_web::{web, Responder, HttpResponse};
use crate::domain::dto::{SignInDTO, UserAddDTO, UserPageDTO};
use crate::service::ADMIN_USER_SERVICE;
use crate::domain::vo::{RespVO, SignInVO};

/// 用户登陆
pub async fn user_login(arg: web::Json<SignInDTO>) -> impl Responder {
    let vo=ADMIN_USER_SERVICE.sign_in(&arg.0).await;
    return HttpResponse::Ok().content_type("json").body(RespVO::<SignInVO>::make(&vo).to_string());
}


/// 用户添加
pub async fn user_add(arg: web::Json<UserAddDTO>) -> impl Responder {
    let vo=ADMIN_USER_SERVICE.add(&arg.0).await;
    if vo.is_err(){
        return HttpResponse::Ok().content_type("json").body(vo.err().unwrap().to_string());
    }
    return HttpResponse::Ok().content_type("json").body(vo.unwrap().to_string());
}


pub async fn user_page(arg: web::Json<UserPageDTO>)-> impl Responder {
    let vo=ADMIN_USER_SERVICE.page(&arg.0).await;
    if vo.is_err(){
        return HttpResponse::Ok().content_type("json").body(vo.err().unwrap().to_string());
    }
    return HttpResponse::Ok().content_type("json").body(vo.unwrap().to_string());
}