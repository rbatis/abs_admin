use rbatis::rbatis::Rbatis;

use redis_service::*;
use sys_res_service::*;
use sys_role_res_service::*;
use sys_role_service::*;
use sys_user_role_service::*;
use sys_user_service::*;

use crate::config::app_config::ApplicationConfig;
use crate::service::mem_cache_service::MemCacheService;

mod redis_service;
mod sys_config_service;
mod sys_res_service;
mod sys_role_res_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_user_role_service;
mod sys_user_service;
mod mem_cache_service;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub redis_service: RedisService,
    pub mem_cache_service: MemCacheService,
    pub sys_res_service: SysResService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_res_service: SysRoleResService,
    pub sys_user_role_service: SysUserRoleService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            rbatis: rbatis::core::runtime::task::block_on(async { crate::dao::init_rbatis(&config).await}),
            redis_service: RedisService::new(&config.redis_url),
            mem_cache_service: MemCacheService::default(),
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
