use crate::config::config::ApplicationConfig;
use rbatis::rbatis::Rbatis;

pub mod mapper;
///实例化 rbatis orm 连接池
pub fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let mut rbatis = Rbatis::new();
    if config.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    return rbatis;
}
