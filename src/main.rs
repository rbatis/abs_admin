#[macro_use]
extern crate lazy_static;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use fast_log::log::RuntimeType;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;
use serde_json::json;

pub mod domain;

pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  static ref RB:Rbatis<'static>={
     let mut rb = Rbatis::new();
     rb.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new("del")));
     rb
  };
}

async fn index() -> impl Responder {
    let v: serde_json::Value = RB.fetch("", "SELECT count(1) FROM biz_activity where delete_flag = 1;").await.unwrap();
    println!("{}", v.to_string());
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
    RB.link(MYSQL_URL).await.unwrap();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
        // .route("/again", web::get().to(index2))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}


#[tokio::main]
#[test]
async fn test_rbatis() {
    fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
    RB.link(MYSQL_URL).await.unwrap();
    let arg = &vec![json!(1)];
    let v: serde_json::Value = RB.fetch_prepare("", "SELECT count(1) FROM biz_activity where delete_flag = ?;", arg).await.unwrap();
    println!("{}", v.to_string());
}
