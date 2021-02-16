use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;
pub mod mapper;

pub fn init_rbatis() -> Rbatis {
    let mut rbatis = Rbatis::new();
    //logic plugin 设置逻辑删除插件
    rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt("del", 1, 0)));
    return rbatis;
}
