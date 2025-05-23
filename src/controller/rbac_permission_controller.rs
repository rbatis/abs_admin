use crate::domain::dto::{IdDTO};
use crate::domain::table::rbac::RbacPermission;
use crate::domain::vo::RespVO;
use crate::error_info;
use crate::context::CONTEXT;
use axum::response::IntoResponse;
use axum::Json;
use crate::domain::dto::rbac::{PermissionAddDTO, ResEditDTO, PermissionPageDTO};


pub async fn layer_top() -> impl IntoResponse {
    let data = CONTEXT.rbac_permission_service.finds_all().await;
    RespVO::from_result(data)
}

pub async fn page(page: Json<PermissionPageDTO>) -> impl IntoResponse {
    let data = CONTEXT.rbac_permission_service.page(&page.0).await;
    RespVO::from_result(data)
}

pub async fn add(mut arg: Json<PermissionAddDTO>) -> impl IntoResponse {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error(error_info!("arg.name_empty"));
    }
    if arg.permission.is_none() {
        return RespVO::<u64>::from_error(error_info!("arg.permission_empty"));
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = RbacPermission::from(arg.0);
    let data = CONTEXT.rbac_permission_service.add(&res).await;
    RespVO::from_result(data)
}

pub async fn update(arg: Json<ResEditDTO>) -> impl IntoResponse {
    let data = CONTEXT.rbac_permission_service.edit(&arg.0).await;
    RespVO::from_result(data)
}

pub async fn remove(arg: Json<IdDTO>) -> impl IntoResponse {
    let data = CONTEXT
        .rbac_permission_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    RespVO::from_result(data)
}
