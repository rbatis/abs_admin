use axum::body::Body;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use axum::extract::Multipart;
use serde::Deserialize;
use crate::context::CONTEXT;
use crate::domain::vo::RespVO;
use crate::middleware::auth_axum::JwtAuth;

///上传，返回外网访问地址
pub async fn upload(jwt_auth: JwtAuth, mut multipart: Multipart) -> impl IntoResponse {
    let file_opt = match multipart.next_field().await {
        Ok(f) => f,
        Err(e) => return RespVO::from_error(format!("read multipart error: {}", e)),
    };

    let Some(file) = file_opt else {
        return RespVO::from_error("no file".to_string());
    };

    let Some(filename) = file.file_name().map(|s| s.to_string()) else {
        return RespVO::from_error("no filename".to_string());
    };

    let data = match file.bytes().await {
        Ok(d) => d,
        Err(e) => return RespVO::from_error(format!("read bytes error: {}", e)),
    };

    let path = CONTEXT
        .storage_service
        .upload(format!("/upload/{}/{}", jwt_auth.id, filename), data.to_vec())
        .await
        .map(|v| format!("{}{}", CONTEXT.config.storage, v));
    RespVO::from_result(path)
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub name: String,
}

///内网流量下载
pub async fn download(Query(param): Query<QueryParams>) -> impl IntoResponse {
    let data = CONTEXT.storage_service.download(param.name).await;
    match data {
        Ok(data) => Response::new(Body::from(data)).into_response(),
        Err(e) => {
            let resp = match Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Cache-Control", "no-cache")
                .header("Content-Type", "json")
                .body(Body::from(
                    serde_json::json!(&RespVO::<()>::from_error(e.to_string())).to_string(),
                )) {
                Ok(r) => r,
                Err(_) => return Response::builder()
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Cache-Control", "no-cache")
                    .header("Content-Type", "json")
                    .body(Body::from(r#"{"code":500,"msg":"internal error"}"#))
                    .unwrap_or_else(|_| Response::new(Body::from("internal error"))),
            };
            resp
        }
    }
}
