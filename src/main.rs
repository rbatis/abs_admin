use std::sync::Arc;
use actix_http::header::HeaderValue;
use abs_admin::controller::{img_controller, sys_auth_controller, sys_dict_controller, sys_res_controller, sys_role_controller, sys_user_controller};
use abs_admin::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::error::ErrorUnauthorized;
use log::info;
use abs_admin::domain::vo::RespVO;
use abs_admin::middleware::auth::{check_auth, checked_token, is_white_list_api};
use actix_web::dev::{Service, ServiceResponse};

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .set_header("Access-Control-Allow-Origin", "*")
        .set_header("Cache-Control", "no-cache")
        .body("[abs_admin] Hello !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    abs_admin::config::log::init_log();
    info!(
        " - Local:   http://{}",
        CONTEXT.config.server_url.replace("0.0.0.0", "127.0.0.1")
    );
    //路由
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                let value = HeaderValue::from_str("").unwrap();
                let token = req.headers().get("access_token").unwrap_or(&value).clone();
                let path = req.path().to_string();
                let fut = srv.call(req);
                Box::pin(async move {
                    if !is_white_list_api(&path) {
                        //非白名单检查token是否有效
                        let token_value = token.to_str().unwrap_or("");
                        match checked_token(token_value, &path).await {
                            Ok(data) => {
                                match check_auth(&data, &path).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        //仅提示拦截
                                        let resp: RespVO<String> = RespVO {
                                            code: Some("-1".to_string()),
                                            msg: Some(format!("无权限访问:{}", e.to_string())),
                                            data: None,
                                        };
                                        return Err(ErrorUnauthorized(serde_json::json!(&resp).to_string()));
                                    }
                                }
                            }
                            Err(e) => {
                                //401 http状态码会强制前端退出当前登陆状态
                                let resp: RespVO<String> = RespVO {
                                    code: Some("-1".to_string()),
                                    msg: Some(format!("Unauthorized for:{}", e.to_string())),
                                    data: None,
                                };
                                return Err(ErrorUnauthorized(serde_json::json!(&resp).to_string()));
                            }
                        }
                    }
                    let mut res = fut.await?;
                    Ok(res)
                })
            })
            .route("/", web::get().to(index))
            .route(
                "/admin/sys_login",
                web::post().to(sys_user_controller::login),
            )
            .route(
                "/admin/sys_user_info",
                web::post().to(sys_user_controller::info),
            )
            .route(
                "/admin/sys_user_detail",
                web::post().to(sys_user_controller::detail),
            )
            //TODO .route("/sys_log_out", web::post().to(user_controller::log_out))
            .route(
                "/admin/sys_res_update",
                web::post().to(sys_res_controller::update),
            )
            .route(
                "/admin/sys_res_remove",
                web::post().to(sys_res_controller::remove),
            )
            .route(
                "/admin/sys_res_add",
                web::post().to(sys_res_controller::add),
            )
            .route(
                "/admin/sys_res_page",
                web::post().to(sys_res_controller::page),
            )
            .route(
                "/admin/sys_res_all",
                web::post().to(sys_res_controller::all),
            )
            .route(
                "/admin/sys_res_layer_top",
                web::post().to(sys_res_controller::layer_top),
            )
            .route(
                "/admin/sys_user_add",
                web::post().to(sys_user_controller::add),
            )
            .route(
                "/admin/sys_user_page",
                web::post().to(sys_user_controller::page),
            )
            .route(
                "/admin/sys_user_remove",
                web::post().to(sys_user_controller::remove),
            )
            .route(
                "/admin/sys_user_update",
                web::post().to(sys_user_controller::update),
            )
            .route(
                "/admin/sys_role_add",
                web::post().to(sys_role_controller::add),
            )
            .route(
                "/admin/sys_role_update",
                web::post().to(sys_role_controller::update),
            )
            .route(
                "/admin/sys_role_delete",
                web::post().to(sys_role_controller::remove),
            )
            .route(
                "/admin/sys_role_page",
                web::post().to(sys_role_controller::page),
            )
            .route(
                "/admin/sys_role_layer_top",
                web::post().to(sys_role_controller::layer_top),
            )
            .route("/admin/captcha", web::get().to(img_controller::captcha))
            .route(
                "/admin/sys_dict_add",
                web::post().to(sys_dict_controller::add),
            )
            .route(
                "/admin/sys_dict_update",
                web::post().to(sys_dict_controller::update),
            )
            .route(
                "/admin/sys_dict_remove",
                web::post().to(sys_dict_controller::remove),
            )
            .route(
                "/admin/sys_dict_page",
                web::post().to(sys_dict_controller::page),
            )
            .route("/admin/auth/check",
                   web::post().to(sys_auth_controller::check),
            )
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}
