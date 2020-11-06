use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use abs_admin::config::CONFIG;
use abs_admin::dao::RB;
use abs_admin::controller::{res_controller, role_controller, user_controller};

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
            .route("/res_add", web::post().to(res_controller::add))
            .route("/res_page", web::post().to(res_controller::page))
            .route("/login", web::post().to(user_controller::login))
            .route("/user_add", web::post().to(user_controller::add))
            .route("/user_page", web::post().to(user_controller::page))
            .route("/role_add", web::post().to(role_controller::add))
            .route("/role_page", web::post().to(role_controller::page))
    })
        .bind(&CONFIG.server_url)?
        .run()
        .await
}

