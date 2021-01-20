use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use abs_admin::config::CONFIG;
use abs_admin::controller::{
    img_controller, res_controller, role_controller, user_controller, user_role_controller,
};
use abs_admin::dao::RB;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello ! Please use Post(Json) request /login,/role_page,/res_page....more http interface,you can install postman for import postman.json ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    abs_admin::config::log::init_log();
    //ORM
    RB.link(&CONFIG.mysql_url).await.unwrap();
    //路由
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/res_update", web::post().to(res_controller::update))
            .route("/res_remove", web::post().to(res_controller::remove))
            .route("/res_add", web::post().to(res_controller::add))
            .route("/res_page", web::post().to(res_controller::page))
            .route("/login", web::post().to(user_controller::login))
            //TODO .route("/log_out", web::post().to(user_controller::log_out))
            .route("/user_add", web::post().to(user_controller::add))
            .route("/user_page", web::post().to(user_controller::page))
            .route("/user_remove", web::post().to(user_controller::remove))
            .route("/user_update", web::post().to(user_controller::edit))
            .route("/role_add", web::post().to(role_controller::add))
            .route("/role_update", web::post().to(role_controller::edit))
            .route("/role_delete", web::post().to(role_controller::remove))
            .route("/role_page", web::post().to(role_controller::page))
            .route("/user_role_add", web::post().to(user_role_controller::add))
            .route(
                "/user_role_delete",
                web::post().to(user_role_controller::remove),
            )
            .route(
                "/user_role_update",
                web::post().to(user_role_controller::edit),
            )
            .route(
                "/user_role_page",
                web::post().to(user_role_controller::page),
            )
            .route("/captcha", web::get().to(img_controller::captcha))
    })
    .bind(&CONFIG.server_url)?
    .run()
    .await
}
