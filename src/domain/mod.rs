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
            r#"please edit application.yml   “debug: false” "#
        );
    }
    return rbatis;
}
