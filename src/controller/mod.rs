use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::domain::vo::RespVO;

pub mod img_controller;
pub mod sys_auth_controller;
pub mod sys_dict_controller;
pub mod rbac_permission_controller;
pub mod rbac_role_controller;
pub mod rbac_user_controller;


impl<T: Serialize + DeserializeOwned> IntoResponse for RespVO<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}