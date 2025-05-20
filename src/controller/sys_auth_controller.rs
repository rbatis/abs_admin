use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
use crate::context::CONTEXT;
use actix_web::{web, HttpResponse, Responder};

///Check whether the token and path are valid and accessible
pub async fn check(arg: web::Json<SysAuthDTO>) -> impl Responder {
    let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
    HttpResponse::Ok().json(RespVO::from_result(r))
}
