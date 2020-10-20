#![allow(unused_variables)]//允许未使用的变量
#![allow(dead_code)]//允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis_macro_driver;

#[macro_use]
pub mod util;
pub mod domain;
pub mod dao;
pub mod controller;
pub mod service;
pub mod config;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use fast_log::log::RuntimeType;

use config::CONFIG;
use dao::RB;

use crate::controller::{res_controller, role_controller, user_controller};



async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello ! Please use Post(Json) request /login,/role_page,/res_page....more http interface,you can install postman for import postman.json ")
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    //日志
    fast_log::log::init_log(&CONFIG.log_path, &RuntimeType::Std).unwrap();
    //ORM
    RB.link(&CONFIG.mysql_url).await.unwrap();
    //http路由
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


