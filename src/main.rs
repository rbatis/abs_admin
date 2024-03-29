use abs_admin::controller::{
    img_controller, sys_auth_controller, sys_dict_controller, sys_permission_controller,
    sys_role_controller, sys_user_controller,
};
use abs_admin::domain::table;
use abs_admin::service::CONTEXT;
use axum::Router;
use axum::routing::{get, post};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    abs_admin::config::log::init_log();
    //database
    CONTEXT.init_database().await;
    table::sync_tables(&CONTEXT.rb).await;
    table::sync_tables_data(&CONTEXT.rb).await;
    //router
    let app = Router::new()
        .route("/", get(|| async { "[abs_admin] Hello !" }))
        .route("/admin/sys_login", post(sys_user_controller::login))
        .route("/admin/sys_user_info", post(sys_user_controller::info))
        .route("/admin/sys_user_detail", post(sys_user_controller::detail))
        .route("/admin/sys_permission_update", post(sys_permission_controller::update))
        .route("/admin/sys_permission_remove", post(sys_permission_controller::remove))
        .route("/admin/sys_permission_add", post(sys_permission_controller::add))
        .route("/admin/sys_permission_page", post(sys_permission_controller::page))
        .route("/admin/sys_permission_all", post(sys_permission_controller::all))
        .route("/admin/sys_permission_layer_top", post(sys_permission_controller::layer_top))
        .route("/admin/sys_user_add", post(sys_user_controller::add))
        .route("/admin/sys_user_page", post(sys_user_controller::page))
        .route("/admin/sys_user_remove", post(sys_user_controller::remove))
        .route("/admin/sys_user_update", post(sys_user_controller::update))
        .route("/admin/sys_role_add", post(sys_role_controller::add))
        .route("/admin/sys_role_update", post(sys_role_controller::update))
        .route("/admin/sys_role_delete", post(sys_role_controller::remove))
        .route("/admin/sys_role_page", post(sys_role_controller::page))
        .route("/admin/sys_role_layer_top", post(sys_role_controller::layer_top))
        .route("/admin/sys_dict_add", post(sys_dict_controller::add))
        .route("/admin/sys_dict_update", post(sys_dict_controller::update))
        .route("/admin/sys_dict_remove", post(sys_dict_controller::remove))
        .route("/admin/sys_dict_page", post(sys_dict_controller::page))
        .route("/admin/auth/check", post(sys_auth_controller::check))
        .route("/admin/captcha", get(img_controller::captcha))
        .layer(axum::middleware::from_fn(abs_admin::middleware::auth_axum::auth));
    let listener = tokio::net::TcpListener::bind(&CONTEXT.config.server_url).await.unwrap();
    axum::serve(listener, app).await
}
