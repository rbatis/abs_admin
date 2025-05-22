use actix_web::{web, App, HttpServer};
use actix_files::{Files, NamedFile};
use actix_service::fn_service;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::scope;
use abs_admin::controller::{
    img_controller, sys_auth_controller, sys_dict_controller, rbac_permission_controller,
    rbac_role_controller, rbac_user_controller,
};
use abs_admin::domain::table;
use abs_admin::context::CONTEXT;
use rbs::value;
use abs_admin::domain::vo::RespVO;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    abs_admin::config::log::init_log();
    if CONTEXT.config.debug {
        log::info!("[abs_admin] {}", value!(&CONTEXT.config));
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

    // Start HTTP server
    HttpServer::new(|| {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .route("/admin/", web::get().to(|| async { web::Json(RespVO::from("hello".to_string())) }))
            .route("/admin/sys_login", web::post().to(rbac_user_controller::login))
            .route("/admin/captcha", web::get().to(img_controller::captcha))
            //auth paths
            .service(
                scope("/admin")
                    .route("/sys_user_info", web::post().to(rbac_user_controller::info))
                    .route("/sys_user_detail", web::post().to(rbac_user_controller::detail))
                    .route("/sys_permission_update", web::post().to(rbac_permission_controller::update))
                    .route("/sys_permission_remove", web::post().to(rbac_permission_controller::remove))
                    .route("/sys_permission_add", web::post().to(rbac_permission_controller::add))
                    .route("/sys_permission_page", web::post().to(rbac_permission_controller::page))
                    .route("/sys_permission_layer_top", web::post().to(rbac_permission_controller::layer_top))
                    .route("/sys_user_add", web::post().to(rbac_user_controller::add))
                    .route("/sys_user_page", web::post().to(rbac_user_controller::page))
                    .route("/sys_user_remove", web::post().to(rbac_user_controller::remove))
                    .route("/sys_user_update", web::post().to(rbac_user_controller::update))
                    .route("/sys_role_add", web::post().to(rbac_role_controller::add))
                    .route("/sys_role_update", web::post().to(rbac_role_controller::update))
                    .route("/sys_role_delete", web::post().to(rbac_role_controller::remove))
                    .route("/sys_role_page", web::post().to(rbac_role_controller::page))
                    .route("/sys_role_layer_top", web::post().to(rbac_role_controller::layer_top))
                    .route("/sys_dict_add", web::post().to(sys_dict_controller::add))
                    .route("/sys_dict_update", web::post().to(sys_dict_controller::update))
                    .route("/sys_dict_remove", web::post().to(sys_dict_controller::remove))
                    .route("/sys_dict_page", web::post().to(sys_dict_controller::page))
                    .route("/auth/check", web::post().to(sys_auth_controller::check))
                    .wrap_fn(abs_admin::middleware::auth_actix::auth)
            )
            .service(
                Files::new("/", "dist/")
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async("dist/index.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    }))
            )
            .wrap(cors)
            .app_data(
                web::PayloadConfig::default()
                    .limit(50 * 1024 * 1024))
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
