use actix_web::{HttpResponse, Responder, web};
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::wrapper::Wrapper;
use rbatis_core::db::DriverType;

use crate::dao::RB;
use crate::domain::BizRes;
use crate::dto::ResPageDTO;

/// 资源分页(json请求)
pub async fn res_page(page: web::Json<ResPageDTO>) -> impl Responder {
    let w = Wrapper::new(&DriverType::Mysql);
    let page_req=PageRequest::new(page.page.unwrap_or(1), page.size.unwrap_or(10));
    let data: rbatis_core::Result<Page<BizRes>> = RB.fetch_page_by_wrapper("", &w, &page_req).await;
    if data.is_err() {
        return HttpResponse::Ok().body(data.err().unwrap().to_string());
    }
    let result = serde_json::to_string(&data.unwrap()).unwrap();
    HttpResponse::Ok().content_type("json").body(result)
}