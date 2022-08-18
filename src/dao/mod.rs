use crate::config::config::ApplicationConfig;
use rbatis::rbatis::Rbatis;
pub mod mapper;

///实例化 rbatis orm 连接池
pub fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let mut rbatis = Rbatis::new();
    if config.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是仍使用debug模式！请修改 application.yml 中debug配置 ，并删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    return rbatis;
}
