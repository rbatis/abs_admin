use crate::domain::dto::{EmptyDTO, IdDTO, PermissionAddDTO, ResEditDTO, ResPageDTO};
use crate::domain::table::SysPermission;
use crate::domain::vo::RespVO;
use crate::error_info;
use crate::service::CONTEXT;
use axum::Json;
use axum::response::IntoResponse;

pub async fn page(page: Json<ResPageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_permission_service.page(&page.0).await;
    RespVO::from_result(data).json()
}

pub async fn all(_page: Json<EmptyDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_permission_service.finds_all().await;
    RespVO::from_result(data).json()
}

pub async fn layer_top(_page: Json<EmptyDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_permission_service.finds_layer_top().await;
    RespVO::from_result(data).json()
}

pub async fn add(mut arg: Json<PermissionAddDTO>) -> impl IntoResponse {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error(error_info!("arg.name_empty"))
            .json();
    }
    if arg.permission.is_none() {
        return RespVO::<u64>::from_error(error_info!("arg.permission_empty"))
            .json();
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = SysPermission::from(arg.0);
    let data = CONTEXT.sys_permission_service.add(&res).await;
    let _ = CONTEXT.sys_permission_service.update_cache().await;
    RespVO::from_result(data).json()
}

pub async fn update(arg: Json<ResEditDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_permission_service.edit(&arg.0).await;
    let _ = CONTEXT.sys_permission_service.update_cache().await;
    RespVO::from_result(data).json()
}

pub async fn remove(arg: Json<IdDTO>) -> impl IntoResponse {
    let data = CONTEXT
        .sys_permission_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    let _ = CONTEXT.sys_permission_service.update_cache().await;
    RespVO::from_result(data).json()
}
