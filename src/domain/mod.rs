use crate::service::ApplicationConfig;
use rbatis::Rbatis;
/// 数据库查询实现
pub mod mapper;
/// DDD分层架构，数据传输层（dto，Data Transfer Object ）,存放接口传输的结构体
pub mod dto;
/// DDD分层架构，数据库结构,该层存放数据库结构体模型
pub mod table;
/// DDD分层架构，展示层（View Object），存放展示的结构体
pub mod vo;

///实例化 rbatis orm 连接池
pub fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为false！"#
        );
    }
    return rbatis;
}
