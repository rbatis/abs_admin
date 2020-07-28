use cache_service::CacheService;
use res_service::ResService;
use admin_user_service::AdminUserService;
use crate::config::BOOT_CONFIG;
mod res_service;
mod cache_service;
mod admin_user_service;
mod role_service;


lazy_static! {
   pub static ref RES_SERVICE:ResService = ResService{};
   pub static ref CACHE_SERVICE:CacheService = CacheService::new(&BOOT_CONFIG.redis_url);
   pub static ref ADMIN_USER_SERVICE:AdminUserService = AdminUserService{};
}
