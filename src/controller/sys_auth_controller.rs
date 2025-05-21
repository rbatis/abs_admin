use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
use crate::context::CONTEXT;
use axum::response::IntoResponse;
use axum::Json;

///Check whether the token and path are valid and accessible
pub async fn check(arg: Json<SysAuthDTO>) -> impl IntoResponse {
    let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
    RespVO::from_result(r)
}
