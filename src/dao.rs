use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use fast_log::log::RuntimeType;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;
use serde_json::json;


pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  pub static ref RB:Rbatis<'static>={
     let mut rb = Rbatis::new();
     let mut del=RbatisLogicDeletePlugin::new("del");
     rb.logic_plugin = Some(Box::new(del));
     rb
  };
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