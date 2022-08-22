use rbatis::Rbatis;
use crate::service::ApplicationConfig;

/// * 数据传输层（dto，Data Transfer Object ）,存放接口传输的结构体
pub mod dto;
///DDD分层架构，分为
///
/// * 数据库结构,该层存放数据库结构体模型
pub mod table;
/// * 展示层（vo，View Object），存放展示的结构体
pub mod vo;
/// 数据库查询实现
pub mod mapper;


///实例化 rbatis orm 连接池
pub fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if config.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是仍使用debug模式！请修改 application.yml 中debug配置 ，并删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    return rbatis;
}
