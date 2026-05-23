use crate::context::CONTEXT;
use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::{JWTToken, RespVO};
use actix_web::web;

pub async fn check(arg: web::Json<SysAuthDTO>) -> RespVO<JWTToken> {
    let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
    RespVO::from_result(r)
}
