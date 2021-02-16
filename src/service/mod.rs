mod redis_service;
mod sys_config_service;
mod sys_res_service;
mod sys_role_res_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_user_role_service;
mod sys_user_service;

use redis_service::*;
use sys_res_service::*;
use sys_role_res_service::*;
use sys_role_service::*;
use sys_user_role_service::*;
use sys_user_service::*;

use crate::config::CONFIG;

lazy_static! {
   /// redis
   pub static ref REDIS_SERVICE: RedisService = RedisService::new(&CONFIG.redis_url);
   /// sys services
   pub static ref SYS_RES_SERVICE: SysResService = SysResService{};
   pub static ref SYS_USER_SERVICE: SysUserService = SysUserService{};
   pub static ref SYS_ROLE_SERVICE: SysRoleService = SysRoleService{};
   pub static ref SYS_ROLE_RES_SERVICE: SysRoleResService = SysRoleResService{};
   pub static ref SYS_USER_ROLE_SERVICE: SysUserRoleService = SysUserRoleService{};
}
