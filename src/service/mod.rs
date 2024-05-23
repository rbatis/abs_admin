mod cache_service;
mod cache_mem_service;
mod cache_redis_service;
mod sys_auth_service;
mod sys_dict_service;
mod sys_permission_service;
mod sys_role_permission_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_trash_service;
mod sys_user_role_service;
mod sys_user_service;

pub use crate::config::ApplicationConfig;
pub use cache_service::*;
pub use cache_mem_service::*;
use once_cell::sync::Lazy;
use rbatis::rbatis::RBatis;
pub use cache_redis_service::*;
use std::sync::Arc;
use std::time::Duration;
pub use sys_auth_service::*;
pub use sys_dict_service::*;
pub use sys_permission_service::*;
pub use sys_role_permission_service::*;
pub use sys_role_service::*;
pub use sys_sms_service::*;
pub use sys_trash_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;

/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(ServiceContext::new);

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::CONTEXT.rb
    };
}

#[derive(Default)]
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
    pub sys_sms_service: SysSmsService,
}

impl ServiceContext {
    pub fn new() -> Self {
        let mut config = ApplicationConfig::new();
        if cfg!(debug_assertions) == false && config.debug.eq(&true) {
            config.debug = false;
        }
        ServiceContext {
            rb: {
                RBatis::new()
            },
            
            cache_service: CacheService::new(&config).unwrap(),
            config,
          
            ..Default::default()
        }
    }
    /// init database pool
    pub async fn init_database(&self) {
        log::info!("[abs_admin] rbatis pool init ({})...", self.config.db_url);
        //include auto choose driver struct by 'config.db_url'
        self.rb
            .link(include!("../../target/driver.rs"), &self.config.db_url)
            .await
            .expect("[abs_admin] rbatis pool init fail!");
        self.rb.intercepts.push(Arc::new(SysTrashService::new()));
        let pool = self.rb.get_pool().unwrap();
        //max connections
        pool.set_max_open_conns(self.config.db_pool_len as u64)
            .await;
        //max timeout
        pool.set_timeout(Some(Duration::from_secs(
                self.config.db_pool_timeout as u64,
            )))
            .await;
        log::info!(
            "[abs_admin] rbatis pool init success! pool state = {}",
            self.rb.get_pool().expect("pool not init!").state().await
        );
        log::info!(
            "Serve:   http://{}",
            self.config.server_url.replace("0.0.0.0", "127.0.0.1")
        );
    }
}
