use abs_admin::controller::{
    img_controller, sys_auth_controller, sys_dict_controller, sys_permission_controller,
    sys_role_controller, sys_user_controller,
};
use abs_admin::middleware::auth_actix::Auth;
use abs_admin::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use abs_admin::domain::table;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .body("[abs_admin] Hello !")
}

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    abs_admin::config::log::init_log();
    //database
    CONTEXT.init_database().await;
    table::sync_tables(&CONTEXT.rb).await;
    table::sync_tables_data(&CONTEXT.rb).await;
    //router
    HttpServer::new(|| {
        App::new()
            .wrap(Auth {})
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
            .route(
                "/admin/sys_permission_update",
                web::post().to(sys_permission_controller::update),
            )
            .route(
                "/admin/sys_permission_remove",
                web::post().to(sys_permission_controller::remove),
            )
            .route(
                "/admin/sys_permission_add",
                web::post().to(sys_permission_controller::add),
            )
            .route(
                "/admin/sys_permission_page",
                web::post().to(sys_permission_controller::page),
            )
            .route(
                "/admin/sys_permission_all",
                web::post().to(sys_permission_controller::all),
            )
            .route(
                "/admin/sys_permission_layer_top",
                web::post().to(sys_permission_controller::layer_top),
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
