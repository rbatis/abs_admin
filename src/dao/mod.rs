use crate::config::CONFIG;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

pub mod mapper;

pub fn init_rbatis() -> Rbatis {
    let mut rbatis = Rbatis::new();
    //logic plugin 设置逻辑删除插件
    rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt("del", 1, 0)));
    if CONFIG.debug == false && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    return rbatis;
}
