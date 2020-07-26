use actix_web::{HttpResponse, Responder, web};
use rbatis::plugin::page::{Page};
use crate::dao::RB;
use crate::domain::BizRes;
use crate::service::RES_SERVICE;
use crate::service::CACHE_SERVICE;
use crate::bean::dto::ResPageDTO;

/// 资源分页(json请求)
pub async fn res_page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = RES_SERVICE.page(&page.0).await;
    if data.is_err() {
        return HttpResponse::Ok().body(data.err().unwrap().to_string());
    }
    CACHE_SERVICE.put("res_page",data.as_ref().unwrap().to_string().as_str()).await;
    HttpResponse::Ok().content_type("json").body(data.unwrap().to_string())
}