use rbatis::rbatis::Rbatis;


mod mem_service;
mod redis_service;
mod sys_config_service;
mod sys_res_service;
mod sys_role_res_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_user_role_service;
mod sys_user_service;
mod cache_service;

pub use mem_service::*;
pub use redis_service::*;
pub use sys_config_service::*;
pub use sys_sms_service::*;
pub use sys_res_service::*;
pub use sys_role_res_service::*;
pub use sys_role_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;
pub use cache_service::*;
pub use crate::config::app_config::ApplicationConfig;


pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub redis_service: RedisService,
    pub mem_service: MemService,
    pub cache_service: CacheService,
    pub sys_res_service: SysResService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_res_service: SysRoleResService,
    pub sys_user_role_service: SysUserRoleService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        let cache_type;
        match config.cache_type.as_str() {
            "mem" => {
                cache_type = CacheProxyType::Mem;
                log::info!("[abs_admin] cache_type: mem");
            }
            "redis" => {
                cache_type = CacheProxyType::Redis;
                log::info!("[abs_admin] cache_type: redis");
            }
            e => {
                panic!("[abs_admin] unsupport cache type of {}", e);
            }
        }
        ServiceContext {
            rbatis: rbatis::core::runtime::task::block_on(async {
                crate::dao::init_rbatis(&config).await
            }),
            redis_service: RedisService::new(&config.redis_url),
            mem_service: MemService::default(),
            cache_service: CacheService {
                inner: cache_type,
            },
            sys_res_service: SysResService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_res_service: SysRoleResService {},
            sys_user_role_service: SysUserRoleService {},
            config,
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
