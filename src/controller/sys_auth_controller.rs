use actix_web::{Responder, web};
use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

///检测token以及path 是否有效且允许访问
pub async fn check(arg: web::Json<SysAuthDTO>) -> impl Responder {
    let r= CONTEXT.sys_auth_service.check_auth(arg.0).await;
    RespVO::from_result(&r).resp_json()
}