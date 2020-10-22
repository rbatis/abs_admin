pub mod mapper;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  pub static ref RB:Rbatis={
     let mut rbatis = Rbatis::new();
     //logic plugin 设置逻辑删除插件
     rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt("del",1,0)));
     return rbatis;
  };
}

///测试
#[cfg(test)]
mod test {
    use fast_log::log::RuntimeType;
    use serde_json::json;
    use crate::dao::RB;
    use crate::config::CONFIG;

    #[async_std::test]
    async fn test_rbatis() {
        fast_log::log::init_log("requests.log", &RuntimeType::Std).unwrap();
        RB.link(&CONFIG.mysql_url).await.unwrap();
        let arg = &vec![json!(1)];
        let v: serde_json::Value = RB.fetch_prepare("", "SELECT count(1) FROM biz_activity where delete_flag = ?;", arg).await.unwrap();
        println!("{}", v.to_string());
    }
}


