use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  pub static ref RB:Rbatis={
     let mut rb = Rbatis::new();
     let del = RbatisLogicDeletePlugin::new("del");
     rb.logic_plugin = Some(Box::new(del));
     rb
  };
}



mod test {
    #![allow(unused_imports)]
    use fast_log::log::RuntimeType;
    use serde_json::json;
    use crate::dao::RB;
    use crate::config::CONFIG;

    #[tokio::main]
    #[test]
    async fn test_rbatis() {
        fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
        RB.link(&CONFIG.mysql_url).await.unwrap();
        let arg = &vec![json!(1)];
        let v: serde_json::Value = RB.fetch_prepare("", "SELECT count(1) FROM biz_activity where delete_flag = ?;", arg).await.unwrap();
        println!("{}", v.to_string());
    }
}


