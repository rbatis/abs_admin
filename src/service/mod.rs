mod cache_service;
mod mem_service;
mod redis_service;
mod sys_auth_service;
mod sys_config_service;
mod sys_dict_service;
mod sys_permission_service;
mod sys_role_permission_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_trash_service;
mod sys_user_role_service;
mod sys_user_service;

pub use crate::config::config::ApplicationConfig;
pub use cache_service::*;
pub use mem_service::*;
use once_cell::sync::Lazy;
use rbatis::rbatis::RBatis;
pub use redis_service::*;
pub use sys_auth_service::*;
pub use sys_config_service::*;
pub use sys_dict_service::*;
pub use sys_permission_service::*;
pub use sys_role_permission_service::*;
pub use sys_role_service::*;
pub use sys_sms_service::*;
pub use sys_trash_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;

/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::service::CONTEXT.rb.clone()
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: RBatis,
    pub cache_service: CacheService,
    pub sys_permission_service: SysPermissionService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_permission_service: SysRoleResService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_dict_service: SysDictService,
    pub sys_auth_service: SysAuthService,
    pub sys_trash_service: SysTrashService,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        log::info!(
            "[abs_admin] rbatis pool init ({})...",
            self.config.database_url
        );
        let driver = rbdc_sqlite::driver::SqliteDriver {};
        let driver_name = format!("{:?}", driver);
        self.rb
            .init(driver, &self.config.database_url)
            .expect("[abs_admin] rbatis pool init fail!");
        self.rb.acquire().await.expect(&format!(
            "rbatis connect database(driver={},url={}) fail",
            driver_name, self.config.database_url
        ));
        log::info!(
            "[abs_admin] rbatis pool init success! pool state = {:?}",
            self.rb.get_pool().expect("pool not init!").status()
        );
        log::info!(
            " - Local:   http://{}",
            self.config.server_url.replace("0.0.0.0", "127.0.0.1")
        );
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            rb: crate::domain::init_rbatis(&config),
            cache_service: CacheService::new(&config).unwrap(),
            sys_permission_service: SysPermissionService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_permission_service: SysRoleResService {},
            sys_user_role_service: SysUserRoleService {},
            sys_dict_service: SysDictService {},
            sys_auth_service: SysAuthService {},
            sys_trash_service: SysTrashService {},
            config,
        }
    }
}
