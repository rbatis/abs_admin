use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use crate::domain::vo::RespVO;

pub mod img_controller;
pub mod sys_auth_controller;
pub mod sys_dict_controller;
pub mod rbac_permission_controller;
pub mod rbac_role_controller;
pub mod rbac_user_controller;


impl <T:Serialize>Responder for RespVO<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}