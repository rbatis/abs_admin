use axum::extract::DefaultBodyLimit;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::cors::{Any, CorsLayer};
use abs_admin::controller::{
    img_controller, sys_auth_controller, sys_dict_controller, rbac_permission_controller,
    rbac_role_controller, rbac_user_controller,
};
use abs_admin::domain::table;
use abs_admin::context::CONTEXT;
use axum::Router;
use axum::routing::{get, post};
use rbs::to_value;
use tower_http::{
    services::{ServeDir,ServeFile},
};
use abs_admin::domain::vo::RespVO;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    abs_admin::config::log::init_log();
    if CONTEXT.config.debug {
        log::info!("[abs_admin] {}", to_value!(&CONTEXT.config));
        log::info!("[abs_admin] ///////////////////// Start On Debug Mode //////////////////////////////");
    } else {
        log::info!("[abs_admin] ///////////////////// Start On Release Mode ////////////////////////////");
    }
    //database
    CONTEXT.init_database().await;
    table::sync_tables(&CONTEXT.rb).await;
    table::sync_tables_data(&CONTEXT.rb).await;
    log::info!("Serve: http://{}",CONTEXT.config.server_url.replace("0.0.0.0", "127.0.0.1"));
    log::info!("[abs_admin] ////////////////////////////////////////////////////////////////////////");
    //router
    let no_auth_router = Router::new()
        .nest_service("/", ServeDir::new("dist/").not_found_service(ServeFile::new("dist/index.html")));
    let auth_router = Router::new()
        .route("/admin/", get(|| async { RespVO::from("hello".to_string()) }))
        .route("/admin/sys_login", post(rbac_user_controller::login))
        .route("/admin/sys_user_info", post(rbac_user_controller::info))
        .route("/admin/sys_user_detail", post(rbac_user_controller::detail))
        .route("/admin/sys_permission_update", post(rbac_permission_controller::update))
        .route("/admin/sys_permission_remove", post(rbac_permission_controller::remove))
        .route("/admin/sys_permission_add", post(rbac_permission_controller::add))
        .route("/admin/sys_permission_page", post(rbac_permission_controller::page))
        .route("/admin/sys_permission_layer_top", post(rbac_permission_controller::layer_top))
        .route("/admin/sys_user_add", post(rbac_user_controller::add))
        .route("/admin/sys_user_page", post(rbac_user_controller::page))
        .route("/admin/sys_user_remove", post(rbac_user_controller::remove))
        .route("/admin/sys_user_update", post(rbac_user_controller::update))
        .route("/admin/sys_role_add", post(rbac_role_controller::add))
        .route("/admin/sys_role_update", post(rbac_role_controller::update))
        .route("/admin/sys_role_delete", post(rbac_role_controller::remove))
        .route("/admin/sys_role_page", post(rbac_role_controller::page))
        .route("/admin/sys_role_layer_top", post(rbac_role_controller::layer_top))
        .route("/admin/sys_dict_add", post(sys_dict_controller::add))
        .route("/admin/sys_dict_update", post(sys_dict_controller::update))
        .route("/admin/sys_dict_remove", post(sys_dict_controller::remove))
        .route("/admin/sys_dict_page", post(sys_dict_controller::page))
        .route("/admin/auth/check", post(sys_auth_controller::check))
        .route("/admin/captcha", get(img_controller::captcha))
        .layer(axum::middleware::from_fn(abs_admin::middleware::auth_axum::auth));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let listener = tokio::net::TcpListener::bind(&CONTEXT.config.server_url).await.unwrap();
    //no_auth_router+auth_router+cors+body_limit
    let app = Router::new()
        .merge(no_auth_router)
        .merge(auth_router)
        .layer(cors)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .layer(RequestBodyLimitLayer::new(50 * 1024 * 1024));
    axum::serve(listener, app).await
}
