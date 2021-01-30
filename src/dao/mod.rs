use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

pub mod mapper;

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  pub static ref RB:Rbatis={
     let mut rbatis = Rbatis::new();
     //logic plugin 设置逻辑删除插件
     rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt("del",1,0)));
     return rbatis;
  };
}