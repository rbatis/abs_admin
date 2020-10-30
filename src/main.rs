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
use config::CONFIG;
use dao::RB;

use crate::controller::{res_controller, role_controller, user_controller};
use fast_log::plugin::file::FileAppender;
use fast_log::plugin::console::ConsoleAppender;
use fast_log::filter::{ModuleFilter, NoFilter};
use fast_log::fast_log::LogAppender;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello ! Please use Post(Json) request /login,/role_page,/res_page....more http interface,you can install postman for import postman.json ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    let mut appenders: Vec<Box<dyn LogAppender>> = vec![
        Box::new(FileAppender::new("requests.log"))
    ];
    if CONFIG.debug {
        appenders.push(Box::new(ConsoleAppender {}));
    }
    //自定义日志过滤
    fast_log::init_custom_log(appenders, 1000, log::Level::Info, Box::new(
        //NoFilter{}
        ModuleFilter{ contains: vec!["rbatis".to_string(),
                                     "actix".to_string(),
                                     "crate".to_string(),
                                     "abs_admin".to_string()] }
    ));


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


