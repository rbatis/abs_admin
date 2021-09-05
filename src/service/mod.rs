use rbatis::rbatis::Rbatis;

mod cache_service;
mod mem_service;
mod redis_service;
mod sys_config_service;
mod sys_dict_service;
mod sys_res_service;
mod sys_role_res_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_user_role_service;
mod sys_user_service;

pub use crate::config::app_config::ApplicationConfig;
pub use cache_service::*;
pub use mem_service::*;
pub use redis_service::*;
pub use sys_config_service::*;
pub use sys_dict_service::*;
pub use sys_res_service::*;
pub use sys_role_res_service::*;
pub use sys_role_service::*;
pub use sys_sms_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub cache_service: CacheService,
    pub sys_res_service: SysResService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_res_service: SysRoleResService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_dict_service: SysDictService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        match config.cache_type.as_str() {
            "mem" => {
                log::info!("[abs_admin] cache_type: mem");
            }
            "redis" => {
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
            cache_service: CacheService::new(),
            sys_res_service: SysResService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_res_service: SysRoleResService {},
            sys_user_role_service: SysUserRoleService {},
            sys_dict_service: SysDictService {},
            config,
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
