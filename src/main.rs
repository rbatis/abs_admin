use abs_admin::controller::{
    img_controller, sys_res_controller, sys_role_controller, sys_user_controller,
};
use abs_admin::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;

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
            .wrap(abs_admin::middleware::auth::Auth)
            .route("/", web::get().to(index))
            .route("/api/sys_login", web::post().to(sys_user_controller::login))
            .route(
                "/api/sys_user_info",
                web::post().to(sys_user_controller::info),
            )
            .route(
                "/api/sys_user_detail",
                web::post().to(sys_user_controller::detail),
            )
            //TODO .route("/sys_log_out", web::post().to(user_controller::log_out))
            .route(
                "/api/sys_res_update",
                web::post().to(sys_res_controller::update),
            )
            .route(
                "/api/sys_res_remove",
                web::post().to(sys_res_controller::remove),
            )
            .route("/api/sys_res_add", web::post().to(sys_res_controller::add))
            .route(
                "/api/sys_res_page",
                web::post().to(sys_res_controller::page),
            )
            .route("/api/sys_res_all", web::post().to(sys_res_controller::all))
            .route(
                "/api/sys_res_layer_top",
                web::post().to(sys_res_controller::layer_top),
            )
            .route(
                "/api/sys_user_add",
                web::post().to(sys_user_controller::add),
            )
            .route(
                "/api/sys_user_page",
                web::post().to(sys_user_controller::page),
            )
            .route(
                "/api/sys_user_remove",
                web::post().to(sys_user_controller::remove),
            )
            .route(
                "/api/sys_user_update",
                web::post().to(sys_user_controller::update),
            )
            .route(
                "/api/sys_role_add",
                web::post().to(sys_role_controller::add),
            )
            .route(
                "/api/sys_role_update",
                web::post().to(sys_role_controller::update),
            )
            .route(
                "/api/sys_role_delete",
                web::post().to(sys_role_controller::remove),
            )
            .route(
                "/api/sys_role_page",
                web::post().to(sys_role_controller::page),
            )
            .route(
                "/api/sys_role_layer_top",
                web::post().to(sys_role_controller::layer_top),
            )
            .route("/api/captcha", web::get().to(img_controller::captcha))
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
