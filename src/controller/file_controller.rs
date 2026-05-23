use crate::context::CONTEXT;
use crate::domain::vo::RespVO;
use crate::middleware::auth_actix::JwtAuth;
use actix_web::web;
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    pub name: String,

    #[serde(default)]
    pub filename: Option<String>,
}

pub async fn upload(
    jwt_auth: JwtAuth,
    query: web::Query<QueryParams>,
    body: web::Bytes,
) -> RespVO<String> {
    let filename = query.filename.clone().unwrap_or_else(|| "upload".to_string());
    let path = CONTEXT
        .storage_service
        .upload(
            format!("target/upload/{}/{}", jwt_auth.id, filename),
            body.to_vec(),
        )
        .await
        .map(|v| format!("{}{}", CONTEXT.config.storage_host, v));
    RespVO::from_result(path)
}

pub async fn delete(query: web::Query<QueryParams>) -> RespVO<()> {
    let result = CONTEXT.storage_service.remove(query.name.clone()).await;
    RespVO::from_result(result)
}

pub async fn download(query: web::Query<QueryParams>) -> HttpResponse {
    let data = CONTEXT.storage_service.download(query.name.clone()).await;
    match data {
        Ok(data) => HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(data),
        Err(e) => HttpResponse::Ok()
            .content_type("json")
            .body(serde_json::json!(&RespVO::<()>::from_error(e.to_string())).to_string()),
    }
}
