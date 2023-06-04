use ntex::web;
use ntex::web::Responder;
use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

///Check whether the token and path are valid and accessible
pub async fn check(arg: web::types::Json<SysAuthDTO>) -> impl Responder {
    let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
    RespVO::from_result(&r).resp_json()
}
