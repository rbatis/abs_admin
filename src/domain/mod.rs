use crate::service::ApplicationConfig;
use rbatis::Rbatis;
pub mod mapper;
pub mod dto;
pub mod table;
pub mod vo;

pub fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}
