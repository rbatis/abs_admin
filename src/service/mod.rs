mod sys_res_service;
mod cache_service;
mod sys_user_service;
mod sys_role_service;

use cache_service::CacheService;
use sys_res_service::SysResService;
use sys_user_service::SysUserService;
use crate::config::CONFIG;
use sys_role_service::SysRoleService;




lazy_static! {
   ///chache
   pub static ref CACHE_SERVICE: CacheService = CacheService::new(&CONFIG.redis_url);
   ///sys services
   pub static ref SYS_RES_SERVICE: SysResService = SysResService{};
   pub static ref SYS_USER_SERVICE: SysUserService = SysUserService{};
   pub static ref SYS_ROLE_SERVICE: SysRoleService = SysRoleService{};
}
