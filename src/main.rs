use abs_admin::controller::{
    img_controller, sys_auth_controller, sys_dict_controller, sys_res_controller,
    sys_role_controller, sys_user_controller,
};
use abs_admin::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use abs_admin::middleware::auth_actix::Auth;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .body("[abs_admin] Hello !")
}

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    abs_admin::config::log::init_log();
    //连接数据库
    CONTEXT.link_db().await;
    //路由
    HttpServer::new(|| {
        App::new()
            .wrap(Auth{})
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
            .route(
                "/admin/auth/check",
                web::post().to(sys_auth_controller::check),
            )
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
