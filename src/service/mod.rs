/// 服务层
///
/// 缓存抽象服务
mod cache_service;
/// 内存缓存服务
mod mem_service;
/// redis服务
mod redis_service;
/// 系统配置服务
mod sys_config_service;
/// 系统字典服务
mod sys_dict_service;
/// 系统权限资源服务
mod sys_res_service;
/// 系统角色-权限资源服务
mod sys_role_res_service;
/// 系统角色服务
mod sys_role_service;
/// 系统短信消息服务
mod sys_sms_service;
/// 系统用户/角色服务
mod sys_user_role_service;
/// 系统用户服务
mod sys_user_service;
/// 系统授权服务
mod sys_auth_service;

pub use crate::config::config::ApplicationConfig;
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
pub use sys_auth_service::*;

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
    pub sys_auth_service: SysAuthService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        match config.cache_type.as_str() {
            "mem" => {
                println!("[abs_admin] cache_type: mem");
            }
            "redis" => {
                println!("[abs_admin] cache_type: redis");
            }
            e => {
                panic!("[abs_admin] unsupport of cache_type: \"{}\"", e);
            }
        }
        ServiceContext {
            rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&config).await
            }),
            cache_service: CacheService::new(&config),
            sys_res_service: SysResService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_res_service: SysRoleResService {},
            sys_user_role_service: SysUserRoleService {},
            sys_dict_service: SysDictService {},
            sys_auth_service: SysAuthService{},
            config,
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
