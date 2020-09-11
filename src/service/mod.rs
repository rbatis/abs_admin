mod sys_res_service;
mod redis_service;
mod sys_user_service;
mod sys_role_service;

use redis_service::RedisService;
use sys_res_service::SysResService;
use sys_user_service::SysUserService;
use crate::config::CONFIG;
use sys_role_service::SysRoleService;




lazy_static! {
   /// redis
   pub static ref REDIS_SERVICE: RedisService = RedisService::new(&CONFIG.redis_url);
   /// sys services
   pub static ref SYS_RES_SERVICE: SysResService = SysResService{};
   pub static ref SYS_USER_SERVICE: SysUserService = SysUserService{};
   pub static ref SYS_ROLE_SERVICE: SysRoleService = SysRoleService{};
}
