#[macro_use]
extern crate lazy_static;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use fast_log::log::RuntimeType;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;
use serde_json::json;

use crate::controller::{res, user};

pub mod domain;
pub mod dao;
pub mod controller;
pub mod service;
pub mod config;
pub mod util;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
    dao::RB.link(dao::MYSQL_URL).await.unwrap();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/res_page", web::post().to(res::res_page))
            .route("/login",web::post().to(user::user_login))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}


