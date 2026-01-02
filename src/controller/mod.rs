use crate::domain::vo::RespVO;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod file_controller;
pub mod img_controller;
pub mod rbac_permission_controller;
pub mod rbac_role_controller;
pub mod rbac_user_controller;
pub mod sys_auth_controller;
pub mod sys_dict_controller;

impl<T: Serialize + DeserializeOwned> IntoResponse for RespVO<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
