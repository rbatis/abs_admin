use rbatis::rbatis::Rbatis;

use redis_service::*;
use sys_res_service::*;
use sys_role_res_service::*;
use sys_role_service::*;
use sys_user_role_service::*;
use sys_user_service::*;

use crate::config::CONFIG;

mod redis_service;
mod sys_config_service;
mod sys_res_service;
mod sys_role_res_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_user_role_service;
mod sys_user_service;

pub struct ServiceContext {
    pub rbatis: Rbatis,
    pub redis_service: RedisService,
    pub sys_res_service: SysResService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_res_service: SysRoleResService,
    pub sys_user_role_service: SysUserRoleService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            rbatis: crate::dao::init_rbatis(),
            redis_service: RedisService::new(&CONFIG.redis_url),
            sys_res_service: SysResService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_res_service: SysRoleResService {},
            sys_user_role_service: SysUserRoleService {},
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
