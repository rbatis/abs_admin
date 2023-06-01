use crate::service::ApplicationConfig;
use rbatis::RBatis;
pub mod dto;
pub mod mapper;
pub mod table;
pub mod vo;

pub fn init_rbatis(config: &ApplicationConfig) -> RBatis {
    let rbatis = RBatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}
