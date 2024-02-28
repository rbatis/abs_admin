use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;
use axum::Json;
use axum::response::IntoResponse;

///Check whether the token and path are valid and accessible
pub async fn check(arg: Json<SysAuthDTO>) -> impl IntoResponse {
    let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
    RespVO::from_result(r).json()
}
