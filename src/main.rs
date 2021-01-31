use abs_admin::config::CONFIG;
use abs_admin::controller::{
    img_controller, sys_res_controller, sys_role_controller, sys_user_controller,
    sys_user_role_controller,
};
use abs_admin::dao::RB;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello ! Please use Post(Json) request /login,/role_page,/res_page....more http interface,you can install postman for import postman.json ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    abs_admin::config::log::init_log();
    //ORM
    RB.link(&CONFIG.mysql_url).await.unwrap();
    info!(
        " - Local:   http://{}",
        CONFIG.server_url.replace("0.0.0.0", "localhost")
    );
    //路由
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route(
                "/sys_res_update",
                web::post().to(sys_res_controller::update),
            )
            .route(
                "/sys_res_remove",
                web::post().to(sys_res_controller::remove),
            )
            .route("/sys_res_add", web::post().to(sys_res_controller::add))
            .route("/sys_res_page", web::post().to(sys_res_controller::page))
            .route("/sys_login", web::post().to(sys_user_controller::login))
            //TODO .route("/sys_log_out", web::post().to(user_controller::log_out))
            .route("/sys_user_add", web::post().to(sys_user_controller::add))
            .route("/sys_user_page", web::post().to(sys_user_controller::page))
            .route(
                "/sys_user_remove",
                web::post().to(sys_user_controller::remove),
            )
            .route(
                "/sys_user_update",
                web::post().to(sys_user_controller::update),
            )
            .route("/sys_role_add", web::post().to(sys_role_controller::add))
            .route(
                "/sys_role_update",
                web::post().to(sys_role_controller::update),
            )
            .route(
                "/sys_role_delete",
                web::post().to(sys_role_controller::remove),
            )
            .route("/sys_role_page", web::post().to(sys_role_controller::page))
            .route(
                "/sys_user_role_add",
                web::post().to(sys_user_role_controller::add),
            )
            .route(
                "/sys_user_role_delete",
                web::post().to(sys_user_role_controller::remove),
            )
            .route(
                "/sys_user_role_update",
                web::post().to(sys_user_role_controller::update),
            )
            .route(
                "/sys_user_role_page",
                web::post().to(sys_user_role_controller::page),
            )
            .route("/captcha", web::get().to(img_controller::captcha))
            .route("/qrcode", web::get().to(img_controller::qrcode))
    })
    .bind(&CONFIG.server_url)?
    .run()
    .await
}
