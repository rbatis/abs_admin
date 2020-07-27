use actix_web::{HttpResponse, Responder, web};
use rbatis::plugin::page::{Page};
use crate::dao::RB;
use crate::service::RES_SERVICE;
use crate::service::CACHE_SERVICE;
use crate::domain::dto::ResPageDTO;

/// 资源分页(json请求)
pub async fn res_page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = RES_SERVICE.page(&page.0).await;
    if data.is_err() {
        return HttpResponse::Ok().body(data.err().unwrap().to_string());
    }
    CACHE_SERVICE.put_json("res_page", &data.as_ref().unwrap().to_string()).await;
    let cached_res: String = CACHE_SERVICE.get_json("res_page").await.unwrap();
    HttpResponse::Ok().content_type("json").body(data.unwrap().to_string())
}